use clap::Parser;
use npmjs_verify::{
    cli::{Cli, Commands},
    npmjs,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let npmjs_token = std::env::var("NPMJS_TOKEN").ok();
    let npmjs_client = npmjs::Client::new(npmjs_token).unwrap();
    let args = Cli::parse();
    match args.command {
        Commands::Package { name } => {
            package(&npmjs_client, &name).await;
        }
        Commands::User { name } => {
            if let Some(packages) = npmjs_client.packages(&name).await.unwrap() {
                let futures = packages.keys().map(|pkg| package(&npmjs_client, pkg));
                futures::future::join_all(futures).await;
            }
        }
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
