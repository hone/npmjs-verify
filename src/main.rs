use clap::Parser;
use npmjs_verify::{cli::Args, npmjs};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let npmjs_token = std::env::var("NPMJS_TOKEN").ok();
    let npmjs_client = npmjs::Client::new(npmjs_token).unwrap();

    if let Some(user) = args.user {
        if let Some(packages) = npmjs_client.packages(&user).await.unwrap() {
            let futures = packages.keys().map(|pkg| package(&npmjs_client, pkg));
            futures::future::join_all(futures).await;
        }
    } else if let Some(pkg) = args.package {
        package(&npmjs_client, &pkg).await;
    }
}

async fn package(client: &npmjs::Client, name: &str) {
    if let Some(package) = client.package(name).await.unwrap() {
        info!("Found {}", package.name);
        let futures = package.versions.values().map(npmjs_verify::verify);

        let mut outputs = futures::future::join_all(futures).await;
        outputs.sort_by(|a, b| a.version.cmp(&b.version));
        for output in outputs {
            info!("{}: {:?}", output.version, output.result);
        }
    } else {
        info!("{} not found", name);
    }
}
