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
};
use std::{
    fs::{self, Permissions},
    io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
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
    // other files like index.html
    public_dir: PathBuf,
    // Rust source code
    src_dir: PathBuf,
    // build artefacts
    build_dir: PathBuf,
    // served data
    www_dir: PathBuf,
}
impl Sandbox {
    fn create_empty() -> Result<Self> {
        let scratch = TempDir::new("playground").map_err(Error::UnableToCreateTempDir)?;

        let public_dir = scratch.path().join(PUBLIC_DIR_NAME);
        fs::create_dir(&public_dir).map_err(Error::UnableToCreateTempDir)?;

        let src_dir = scratch.path().join(SRC_DIR_NAME);
        fs::create_dir(&src_dir).map_err(Error::UnableToCreateTempDir)?;

        let build_dir = scratch.path().join(BUILD_DIR_NAME);
        fs::create_dir(&build_dir).map_err(Error::UnableToCreateTempDir)?;
        set_permissions_open(&build_dir)?;

        let www_dir = scratch.path().join("www");

        Ok(Self {
            _scratch: scratch,
            public_dir,
            src_dir,
            build_dir,
            www_dir,
        })
    }

    pub fn create_from_template(template_path: &Path) -> Result<Self> {
        let sandbox = Self::create_empty()?;

        copy_dir(&template_path.join(PUBLIC_DIR_NAME), &sandbox.public_dir)
            .map_err(Error::UnableToCreateTempDir)?;
        copy_dir(&template_path.join(SRC_DIR_NAME), &sandbox.src_dir)
            .map_err(Error::UnableToCreateTempDir)?;

        Ok(sandbox)
    }

    pub fn write_src_file(&self, path: &Path, code: &str) -> Result<()> {
        let path = self.src_dir.join(path);
        fs::write(&path, code).map_err(Error::UnableToWriteFile)?;

        log::debug!("wrote {} bytes of source to {}", code.len(), path.display());
        Ok(())
    }

    pub fn get_src_path(&self, path: &Path) -> Option<PathBuf> {
        let path = self.src_dir.join(path);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    }

    pub fn get_www_path(&self, path: &Path) -> Option<PathBuf> {
        let path = self.www_dir.join(path);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
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
            .args(&execution_cmd);

        log::debug!("compile command: {:?}", cmd);

        cmd
    }

    fn format_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(req);

        cmd.arg("rustfmt").args(&["cargo", "fmt"]);

        log::debug!("format command: {:?}", cmd);

        cmd
    }

    fn clippy_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(&req);

        cmd.arg("clippy").args(&["cargo", "clippy"]);

        log::debug!("clippy command: {:?}", cmd);

        cmd
    }

    fn macro_expand_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();
        cmd.apply_edition(req);

        cmd.arg(helpers::container_name_for_channel(Channel::Nightly))
            .args(&["cargo", "expand"]);

        log::debug!("macro expand command: {:?}", cmd);

        cmd
    }

    fn docker_command(&self) -> Command {
        let mut mount_input_file = self.src_dir.as_os_str().to_os_string();
        mount_input_file.push(":");
        mount_input_file.push("/playground/src");

        let mut mount_output_dir = self.build_dir.as_os_str().to_os_string();
        mount_output_dir.push(":");
        mount_output_dir.push("/playground-result");

        let mut cmd = commands::docker_run();
        cmd.arg("--volume")
            .arg(&mount_input_file)
            .arg("--volume")
            .arg(&mount_output_dir);

        cmd
    }
}
// We must create a world-writable files (rustfmt) and directories so that the
// process inside the Docker container can write into it.
//
// This problem does *not* occur when using the indirection of
// docker-machine.
fn open_permissions() -> Permissions {
    Permissions::from_mode(0o777)
}

fn set_permissions_open(path: &Path) -> Result<()> {
    fs::set_permissions(path, open_permissions()).map_err(Error::UnableToSetPermissions)
}

fn hard_link_dir(src: &Path, dst: &Path) -> io::Result<()> {
    src_dst_transformer(src, dst, &|src, dst| fs::hard_link(src, dst))
}

fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    src_dst_transformer(src, dst, &|src, dst| {
        fs::copy(src, dst)?;
        Ok(())
    })
}

// TODO clean this up
fn src_dst_transformer(
    src: &Path,
    dst: &Path,
    f: &impl Fn(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    for entry in src.read_dir()? {
        let entry = entry?;

        let entry_type = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if entry_type.is_file() {
            f(&entry.path(), &dst_path)?;
        } else if entry_type.is_dir() {
            fs::create_dir(&dst_path)?;
            src_dst_transformer(&entry.path(), &dst_path, f)?;
        }
    }

    Ok(())
}
