use clap::Parser;
use npmjs_verify::{cli::Args, npmjs};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if let Some(package) = npmjs::package(&args.package).await.unwrap() {
        info!("Found {}", package.name);
        for version in package.versions.values() {
            if let Some(result) = npmjs_verify::verify(&version) {
                info!("{}: {}", version.version, result);
            } else {
                info!("{}: Can not verify", version.version);
            }
        }
    } else {
        info!("{} not found", &args.package);
    }
}
