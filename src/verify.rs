use crate::npmjs::data::{Dist, Version};
use std::{fs::File, future::Future, io::Write, path::Path, process::Output};
use tokio::process::Command;
use tracing::debug;

const SIGNED_BY: &str = "npmregistry";

#[derive(Debug)]
/// Possible Verification Values
pub enum Verify {
    Pass,
    Fail,
    Missing,
}

#[derive(Debug)]
/// Named Tuple of the Verify Output
pub struct VerifyOutput {
    pub name: String,
    pub version: node_semver::Version,
    pub result: Verify,
}

impl VerifyOutput {
    pub fn new(version: &Version, result: Verify) -> VerifyOutput {
        VerifyOutput {
            name: version.name.clone(),
            version: version.version.clone(),
            result,
        }
    }
}

/// PGP Verify using Keybase of a NPM Version
pub async fn verify(version: &Version) -> Result<VerifyOutput, std::io::Error> {
    let message = match message(version) {
        Some(m) => m,
        None => return Ok(VerifyOutput::new(version, Verify::Missing)),
    };
    let sig_tempfile = tempfile::NamedTempFile::new()?;
    if write_signature(&version.dist, sig_tempfile.path())?.is_some() {
        let output = verify_cmd(SIGNED_BY, sig_tempfile.path(), &message);
        if let Ok(output) = output.await.await {
            let status = output.status.success();
            debug!("{}", String::from_utf8_lossy(&output.stdout));
            debug!("{}", String::from_utf8_lossy(&output.stderr));
            return Ok(VerifyOutput::new(
                version,
                if status { Verify::Pass } else { Verify::Fail },
            ));
        } else {
            return Ok(VerifyOutput::new(version, Verify::Fail));
        }
    }

    Ok(VerifyOutput::new(version, Verify::Missing))
}

fn message(version: &Version) -> Option<String> {
    version
        .dist
        .integrity
        .as_ref()
        .map(|integrity| format!("{}@{}:{}", version.name, version.version, integrity))
}

async fn verify_cmd(
    signed_by: &str,
    detached: impl AsRef<Path>,
    message: &str,
) -> impl Future<Output = Result<Output, std::io::Error>> {
    Command::new("keybase")
        .args([
            "pgp",
            "verify",
            "--signed-by",
            signed_by,
            "--detached",
            &detached.as_ref().to_string_lossy(),
            "--message",
            message,
        ])
        .output()
}

fn write_signature(dist: &Dist, file: impl AsRef<Path>) -> Result<Option<()>, std::io::Error> {
    if let Some(signature) = dist.npm_signature.as_ref() {
        let mut file = File::create(file)?;
        file.write_all(signature.as_bytes())?;

        Ok(Some(()))
    } else {
        Ok(None)
    }
}
