use crate::npmjs::{Dist, Version};
use std::{fs::File, future::Future, io::Write, path::Path, process::Output};
use tokio::process::Command;
use tracing::debug;

const SIGNED_BY: &str = "npmregistry";

pub async fn verify(version: &Version) -> Option<bool> {
    let message = match message(version) {
        Some(m) => m,
        None => return None,
    };
    let sig_tempfile = tempfile::NamedTempFile::new().unwrap();
    if write_signature(&version.dist, sig_tempfile.path())
        .unwrap()
        .is_some()
    {
        let output = verify_cmd(SIGNED_BY, sig_tempfile.path(), &message);
        if let Ok(output) = output.await.await {
            let status = output.status.success();
            debug!("{}", String::from_utf8_lossy(&output.stdout));
            debug!("{}", String::from_utf8_lossy(&output.stderr));
            return Some(status);
        } else {
            return Some(false);
        }
    }

    None
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
            detached.as_ref().to_str().unwrap(),
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
