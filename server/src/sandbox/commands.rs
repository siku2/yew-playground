use super::{
    helpers::{self, BacktraceRequest, EditionRequest},
    Channel,
    Error,
    Mode,
    Result,
};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    process::{Command, Output},
    time::Duration,
};

const DOCKER_PROCESS_TIMEOUT_SOFT: Duration = Duration::from_secs(10);
const DOCKER_PROCESS_TIMEOUT_HARD: Duration = Duration::from_secs(12);

pub trait DockerCommandExt {
    fn apply_edition(&mut self, req: impl EditionRequest);
    fn apply_backtrace(&mut self, req: impl BacktraceRequest);
}

impl DockerCommandExt for Command {
    fn apply_edition(&mut self, req: impl EditionRequest) {
        if let Some(edition) = req.edition() {
            self.args(&[
                "--env",
                &format!(
                    "PLAYGROUND_EDITION={}",
                    helpers::cargo_ident_for_edition(edition)
                ),
            ]);
        }
    }

    fn apply_backtrace(&mut self, req: impl BacktraceRequest) {
        if req.backtrace() {
            self.args(&["--env", "RUST_BACKTRACE=1"]);
        }
    }
}

pub fn docker_run() -> Command {
    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("--rm")
        .arg("--cap-drop=ALL")
        // Needed to allow overwriting the file
        .arg("--cap-add=DAC_OVERRIDE")
        .arg("--security-opt=no-new-privileges")
        .args(&["--workdir", "/playground"])
        .args(&["--net", "none"])
        .args(&["--memory", "256m"])
        .args(&["--memory-swap", "320m"])
        .args(&[
            "--env",
            &format!(
                "PLAYGROUND_TIMEOUT={}",
                DOCKER_PROCESS_TIMEOUT_SOFT.as_secs()
            ),
        ])
        .args(&["--pids-limit", "512"]);

    cmd
}

pub fn run_with_timeout(mut command: Command) -> Result<Output> {
    // TODO handle timeout
    let _ = DOCKER_PROCESS_TIMEOUT_HARD;
    command.output().map_err(Error::UnableToExecuteCompiler)
}

pub fn set_execution_environment(cmd: &mut Command, req: impl EditionRequest + BacktraceRequest) {
    cmd.apply_edition(&req);
    cmd.apply_backtrace(&req);
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrateInformation {
    pub name: String,
    pub version: String,
    pub id: String,
}

pub fn _list_crates() -> Result<Vec<CrateInformation>> {
    let mut command = docker_run();
    command.args(&[helpers::container_name_for_channel(Channel::Stable)]);
    command.args(&["cat", "crate-information.json"]);
    let output = run_with_timeout(command)?;

    let crate_info: Vec<CrateInformation> =
        serde_json::from_slice(&output.stdout).map_err(Error::UnableToParseCrateInformation)?;

    let crates = crate_info.into_iter().map(Into::into).collect();

    Ok(crates)
}

#[derive(Clone, Debug)]
pub struct Version {
    pub release: String,
    pub commit_hash: String,
    pub commit_date: String,
}

pub fn _rustc_version(channel: Channel) -> Result<Version> {
    let mut command = docker_run();
    command.args(&[helpers::container_name_for_channel(channel)]);
    command.args(&["rustc", "--version", "--verbose"]);

    let output = run_with_timeout(command)?;
    let version_output = helpers::string_from_utf8_vec(output.stdout)?;

    let mut info: BTreeMap<String, String> = version_output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut pieces = line.splitn(2, ':').fuse();
            match (pieces.next(), pieces.next()) {
                (Some(name), Some(value)) => Some((name.trim().into(), value.trim().into())),
                _ => None,
            }
        })
        .collect();

    let release = info.remove("release").ok_or(Error::VersionReleaseMissing)?;
    let commit_hash = info
        .remove("commit-hash")
        .ok_or(Error::VersionHashMissing)?;
    let commit_date = info
        .remove("commit-date")
        .ok_or(Error::VersionDateMissing)?;

    Ok(Version {
        release,
        commit_hash,
        commit_date,
    })
}

pub fn _version_rustfmt() -> Result<Version> {
    let mut command = docker_run();
    command.args(&["rustfmt", "cargo", "fmt", "--version"]);
    cargo_tool_version(command)
}

pub fn _version_clippy() -> Result<Version> {
    let mut command = docker_run();
    command.args(&["clippy", "cargo", "clippy", "--version"]);
    cargo_tool_version(command)
}

// Parses versions of the shape `toolname 0.0.0 (0000000 0000-00-00)`
fn cargo_tool_version(command: Command) -> Result<Version> {
    let output = run_with_timeout(command)?;
    let version_output = helpers::string_from_utf8_vec(output.stdout)?;
    let mut parts = version_output.split_whitespace().fuse().skip(1);

    let release = parts.next().unwrap_or("").into();
    let commit_hash = parts.next().unwrap_or("").trim_start_matches('(').into();
    let commit_date = parts.next().unwrap_or("").trim_end_matches(')').into();

    Ok(Version {
        release,
        commit_hash,
        commit_date,
    })
}

pub fn wasm_pack_build(_channel: Channel, mode: Mode, out_dir: &'static str) -> Vec<&'static str> {
    let mut cmd = vec!["wasm-pack", "build", "--no-typescript"];
    cmd.extend(&["--mode", "no-install"]);
    cmd.extend(&["--target", "web"]);
    cmd.extend(&["--out-dir", out_dir]);

    cmd.push(match mode {
        Mode::Debug => "--dev",
        Mode::Release => "--release",
    });

    cmd
}
