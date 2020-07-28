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
    ffi::OsStr,
    fmt::Write,
    fs::{self, Permissions},
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
const WWW_DIR_NAME: &str = "www";

#[derive(Debug)]
pub struct Sandbox {
    _scratch: TempDir,
    public_dir: PathBuf,
    src_dir: PathBuf,
    www_dir: PathBuf,
}
impl Sandbox {
    pub fn create() -> Result<Self> {
        let scratch = TempDir::new("playground").map_err(Error::UnableToCreateTempDir)?;
        let public_dir = scratch.path().join(PUBLIC_DIR_NAME);
        let src_dir = scratch.path().join(SRC_DIR_NAME);
        let www_dir = scratch.path().join(WWW_DIR_NAME);

        fs::create_dir(&www_dir).map_err(Error::UnableToCreateOutputDir)?;
        fs::set_permissions(&www_dir, open_permissions())
            .map_err(Error::UnableToSetOutputPermissions)?;

        Ok(Self {
            _scratch: scratch,
            public_dir,
            src_dir,
            www_dir,
        })
    }

    pub fn get_file_path(&self, file: &Path) -> Option<PathBuf> {
        let path = self.www_dir.join(file);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    }

    pub fn compile(&self, req: &CompileRequest) -> Result<CompileResponse> {
        self.write_source_code(&req.code)?;

        let command = self.compile_command(req.channel, req.mode, req);
        let output = commands::run_with_timeout(command)?;

        // The compiler writes the file to a name like
        // `compilation-3b75174cac3d47fb.ll`, so we just find the
        // first with the right extension.
        let file = fs::read_dir(&self.www_dir)
            .map_err(Error::UnableToReadOutput)?
            .flat_map(|entry| entry)
            .map(|entry| entry.path())
            .find(|path| path.extension() == Some(OsStr::new("wat")));

        let stdout = helpers::string_from_utf8_vec(output.stdout)?;
        let mut stderr = helpers::string_from_utf8_vec(output.stderr)?;

        let code = match file {
            Some(file) => helpers::read_file_to_string(&file)?.unwrap_or_else(String::new),
            None => {
                // If we didn't find the file, it's *most* likely that
                // the user's code was invalid. Tack on our own error
                // to the compiler's error instead of failing the
                // request.
                write!(&mut stderr, "\nUnable to locate file",)
                    .expect("Unable to write to a string");
                String::new()
            }
        };

        Ok(CompileResponse {
            success: output.status.success(),
            code,
            stdout,
            stderr,
        })
    }

    pub fn format(&self, req: &FormatRequest) -> Result<FormatResponse> {
        self.write_source_code(&req.code)?;

        let command = self.format_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(FormatResponse {
            success: output.status.success(),
            code: helpers::read_file_to_string(self.src_dir.as_ref())?
                .ok_or(Error::OutputMissing)?,
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    pub fn clippy(&self, req: &ClippyRequest) -> Result<ClippyResponse> {
        self.write_source_code(&req.code)?;

        let command = self.clippy_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(ClippyResponse {
            success: output.status.success(),
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    pub fn macro_expansion(&self, req: &MacroExpansionRequest) -> Result<MacroExpansionResponse> {
        self.write_source_code(&req.code)?;

        let command = self.macro_expansion_command(req);
        let output = commands::run_with_timeout(command)?;

        Ok(MacroExpansionResponse {
            success: output.status.success(),
            stdout: helpers::string_from_utf8_vec(output.stdout)?,
            stderr: helpers::string_from_utf8_vec(output.stderr)?,
        })
    }

    fn write_source_code(&self, code: &str) -> Result<()> {
        fs::write(&self.src_dir, code).map_err(Error::UnableToCreateSourceFile)?;
        fs::set_permissions(&self.src_dir, open_permissions())
            .map_err(Error::UnableToSetSourcePermissions)?;

        log::debug!(
            "Wrote {} bytes of source to {}",
            code.len(),
            self.src_dir.display()
        );
        Ok(())
    }

    fn compile_command(
        &self,
        channel: Channel,
        mode: Mode,
        req: impl EditionRequest + BacktraceRequest,
    ) -> Command {
        let mut cmd = self.docker_command();
        commands::set_execution_environment(&mut cmd, &req);

        let execution_cmd = commands::wasm_pack_build(channel, mode);

        cmd.arg(&helpers::container_name_for_channel(channel))
            .args(&execution_cmd);

        log::debug!("Compilation command is {:?}", cmd);

        cmd
    }

    fn format_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(req);

        cmd.arg("rustfmt").args(&["cargo", "fmt"]);

        log::debug!("Formatting command is {:?}", cmd);

        cmd
    }

    fn clippy_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();

        cmd.apply_edition(&req);

        cmd.arg("clippy").args(&["cargo", "clippy"]);

        log::debug!("Clippy command is {:?}", cmd);

        cmd
    }

    fn macro_expansion_command(&self, req: impl EditionRequest) -> Command {
        let mut cmd = self.docker_command();
        cmd.apply_edition(req);

        cmd.arg(helpers::container_name_for_channel(Channel::Nightly))
            .args(&["cargo", "expand"]);

        log::debug!("Macro expansion command is {:?}", cmd);

        cmd
    }

    fn docker_command(&self) -> Command {
        let mut mount_input_file = self.src_dir.as_os_str().to_os_string();
        mount_input_file.push(":");
        mount_input_file.push("/playground/src");

        let mut mount_output_dir = self.www_dir.as_os_str().to_os_string();
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
