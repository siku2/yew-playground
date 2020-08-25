use commands::DockerCommandExt;
pub use error::{Error, Result};
use helpers::{BacktraceRequest, EditionRequest};
use protocol::{
    Channel,
    ClippyRequest,
    ClippyResponse,
    CompileRequest,
    CompileResponse,
    FormatRequest,
    FormatResponse,
    MacroExpansionRequest,
    MacroExpansionResponse,
    Mode,
    SandboxStructure,
    ToolVersions,
};
use std::{
    borrow::Cow,
    collections::VecDeque,
    fs::{self, Permissions},
    io,
    os::unix::fs::PermissionsExt,
    path::{Component, Path, PathBuf},
    process::Command,
};
use tempdir::TempDir;

mod commands;
mod error;
mod helpers;

const PUBLIC_DIR_NAME: &str = "public";
const SRC_DIR_NAME: &str = "src";
const BUILD_DIR_NAME: &str = "build";

#[derive(Debug)]
pub struct Sandbox {
    _scratch: TempDir,
    root_dir: PathBuf,
    // other files like index.html
    public_dir: PathBuf,
    // Rust source code
    src_dir: PathBuf,
    // build artefacts
    build_dir: PathBuf,
}
impl Sandbox {
    /// Creates a Sandbox with only the directory structure.
    fn create_empty() -> Result<Self> {
        let scratch = TempDir::new("playground").map_err(Error::UnableToPrepareDir)?;
        let root_dir = scratch
            .path()
            .canonicalize()
            .map_err(Error::UnableToPrepareDir)?;

        let public_dir = root_dir.join(PUBLIC_DIR_NAME);
        fs::create_dir(&public_dir).map_err(Error::UnableToPrepareDir)?;

        let src_dir = root_dir.join(SRC_DIR_NAME);
        fs::create_dir(&src_dir).map_err(Error::UnableToPrepareDir)?;

        let build_dir = root_dir.join(BUILD_DIR_NAME);
        fs::create_dir(&build_dir).map_err(Error::UnableToPrepareDir)?;
        set_permissions_open(&build_dir)?;

        log::debug!("created new sandbox (dir: {:?})", scratch);

        Ok(Self {
            _scratch: scratch,
            root_dir,
            public_dir,
            src_dir,
            build_dir,
        })
    }

    pub fn create_from_template(template_path: &Path) -> Result<Self> {
        let sandbox = Self::create_empty()?;

        copy_dir(&template_path.join(PUBLIC_DIR_NAME), &sandbox.public_dir)
            .map_err(Error::UnableToPrepareDir)?;
        copy_dir(&template_path.join(SRC_DIR_NAME), &sandbox.src_dir)
            .map_err(Error::UnableToPrepareDir)?;

        Ok(sandbox)
    }

    pub fn get_structure(&self) -> Result<SandboxStructure> {
        Ok(SandboxStructure {
            public: create_protocol_directory(&self.root_dir, &self.public_dir)?,
            src: create_protocol_directory(&self.root_dir, &self.src_dir)?,
        })
    }

    pub fn write_to_file(&self, path: &Path, content: &str) -> Result<()> {
        let path = self.get_file_path(path)?;
        fs::write(&path, content).map_err(Error::UnableToWriteFile)?;

        log::debug!(
            "wrote {} bytes of source to {}",
            content.len(),
            path.display()
        );
        Ok(())
    }

    /// Get a path for either the "public" or "src" directory.
    /// The only guarantee is that the resulting path will be an absolute path
    /// to a location within one of the two directories.
    pub fn get_file_path(&self, path: &Path) -> Result<PathBuf> {
        let path = safe_join_path(&self.root_dir, path)?;

        // make absolutely sure that the file is in either "public" or "src".
        if path.starts_with(&self.public_dir) || path.starts_with(&self.src_dir) {
            Ok(path)
        } else {
            Err(Error::InvalidPath(path.to_path_buf()))
        }
    }

    pub fn get_serve_path(&self, path: &Path) -> Result<PathBuf> {
        let mut public_path = safe_join_path(&self.public_dir, path)?;
        if public_path.is_dir() {
            public_path.push("index.html");
        }
        if public_path.is_file() {
            return Ok(public_path);
        }
        safe_join_path(&self.build_dir, path)
    }

    pub fn get_tool_versions(&self) -> Result<ToolVersions> {
        // TODO use correct channel
        commands::get_tool_versions(Channel::Stable)
    }

    pub fn compile(&self, req: &CompileRequest) -> Result<CompileResponse> {
        let command = self.compile_command(req.channel, req.mode, req);
        let output = commands::run_with_timeout(command)?;

        let stdout = helpers::string_from_utf8_vec(output.stdout)?;
        let stderr = helpers::string_from_utf8_vec(output.stderr)?;

        Ok(CompileResponse {
            success: output.status.success(),
            stdout,
            stderr,
        })
    }

    pub fn format(&self, req: &FormatRequest) -> Result<FormatResponse> {
        let command = self.format_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(FormatResponse {
            success: output.status.success(),
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    pub fn clippy(&self, req: &ClippyRequest) -> Result<ClippyResponse> {
        let command = self.clippy_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(ClippyResponse {
            success: output.status.success(),
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    pub fn macro_expand(&self, req: &MacroExpansionRequest) -> Result<MacroExpansionResponse> {
        let command = self.macro_expand_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(MacroExpansionResponse {
            success: output.status.success(),
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    fn compile_command(
        &self,
        channel: Channel,
        mode: Mode,
        req: impl EditionRequest + BacktraceRequest,
    ) -> Command {
        let mut cmd = self.docker_command();
        commands::set_execution_environment(&mut cmd, &req);

        let execution_cmd = commands::wasm_pack_build(channel, mode, BUILD_DIR_NAME);

        cmd.arg(&helpers::container_name_for_channel(channel))
            .args(&execution_cmd)
            .args(&["--", "--color=always"]);

        log::debug!("compile command: {:?}", cmd);

        cmd
    }

    fn format_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(req);

        cmd.arg("rustfmt").args(commands::cargo_color()).arg("fmt");

        log::debug!("format command: {:?}", cmd);

        cmd
    }

    fn clippy_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(&req);

        cmd.arg("clippy")
            .args(commands::cargo_color())
            .arg("clippy");

        log::debug!("clippy command: {:?}", cmd);

        cmd
    }

    fn macro_expand_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();
        cmd.apply_edition(req);

        cmd.arg(helpers::container_name_for_channel(Channel::Nightly))
            .args(commands::cargo_color())
            .arg("expand");

        log::debug!("macro expand command: {:?}", cmd);

        cmd
    }

    fn docker_command(&self) -> Command {
        let mut mount_input_file = self.src_dir.as_os_str().to_os_string();
        mount_input_file.push(":");
        mount_input_file.push("/playground/src");

        let mut mount_output_dir = self.build_dir.as_os_str().to_os_string();
        mount_output_dir.push(":");
        mount_output_dir.push("/playground/build");

        let mut cmd = commands::docker_run();
        cmd.arg("--volume")
            .arg(&mount_input_file)
            .arg("--volume")
            .arg(&mount_output_dir);

        cmd
    }
}

/// Safely join two paths.
/// It is assumed that `base` is already safe.
/// The result is a path relative to `base` and only containing normal
/// components.
fn safe_join_path(base: &Path, rel: &Path) -> Result<PathBuf> {
    let has_invalid_comp = rel.components().any(|comp| match comp {
        Component::Normal(_) => false,
        _ => true,
    });
    if has_invalid_comp {
        Err(Error::InvalidPath(rel.to_owned()))
    } else {
        Ok(base.join(rel))
    }
}

fn set_permissions_open(path: &Path) -> Result<()> {
    fs::set_permissions(path, Permissions::from_mode(0o777)).map_err(Error::UnableToSetPermissions)
}

/// Copy the files from `src` to `dst`.
/// `dst` must already exist but further subdirectories from `src` are created
/// automatically.
fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    let mut queue: VecDeque<(Cow<Path>, Cow<Path>)> = VecDeque::new();
    queue.push_back((Cow::from(src), Cow::from(dst)));

    while let Some((src, dst)) = queue.pop_front() {
        for entry in src.read_dir()? {
            let entry = entry?;
            let entry_type = entry.file_type()?;
            let dst_path = dst.join(entry.file_name());

            if entry_type.is_file() {
                fs::copy(&entry.path(), &dst_path)?;
            } else if entry_type.is_dir() {
                fs::create_dir(&dst_path)?;
                queue.push_back((Cow::from(entry.path()), Cow::from(dst_path)));
            }
        }
    }

    Ok(())
}

fn path_to_string(path: &Path) -> Result<String> {
    path.to_str()
        .ok_or_else(|| Error::InvalidPath(path.to_path_buf()))
        .map(String::from)
}

fn rel_path_to_string(base: &Path, rel: &Path) -> Result<String> {
    let path = rel
        .strip_prefix(base)
        .map_err(|_| Error::CorruptSandboxDir)?;
    path_to_string(path)
}

fn _create_protocol_directory(base: &Path, path: &Path) -> Result<protocol::Directory> {
    let mut directories = Vec::new();
    let mut files = Vec::new();

    for entry in path.read_dir().map_err(|_| Error::CorruptSandboxDir)? {
        let entry = entry.map_err(|_| Error::CorruptSandboxDir)?;
        let entry_type = entry.file_type().map_err(|_| Error::CorruptSandboxDir)?;

        if entry_type.is_file() {
            files.push(protocol::File {
                path: rel_path_to_string(base, &entry.path())?,
                name: entry
                    .file_name()
                    .into_string()
                    .map_err(|_| Error::CorruptSandboxDir)?,
            });
        } else if entry_type.is_dir() {
            directories.push(_create_protocol_directory(base, &entry.path())?);
        }
    }

    Ok(protocol::Directory {
        path: rel_path_to_string(base, path)?,
        name: path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(Error::CorruptSandboxDir)?
            .to_owned(),
        directories,
        files,
    })
}

fn create_protocol_directory(base: &Path, rel: &Path) -> Result<protocol::Directory> {
    _create_protocol_directory(base, rel)
}
