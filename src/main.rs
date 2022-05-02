use clap::Parser;
use npmjs_verify::{cli::Args, npmjs};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let npmjs_token = std::env::var("NPMJS_TOKEN").ok();
    let npmjs_client = npmjs::Client::new(npmjs_token).unwrap();

    if let Some(package) = npmjs_client.package(&args.package).await.unwrap() {
        info!("Found {}", package.name);
        let futures = package
            .versions
            .values()
            .map(|version| (version, npmjs_verify::verify(&version)));
        for (version, future) in futures {
            if let Some(result) = future.await {
                info!("{}: {}", version.version, result);
            } else {
                info!("{}: Can not verify", version.version);
            }
        }
    } else {
        info!("{} not found", &args.package);
    }
}
