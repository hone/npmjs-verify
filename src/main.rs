use clap::Parser;
use futures::StreamExt;
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
        Commands::Package { name, job_size } => {
            package(
                &npmjs_client,
                &name,
                job_size.unwrap_or_else(|| num_cpus::get()),
            )
            .await;
        }
        Commands::User { name, job_size } => {
            let job_size = job_size.unwrap_or_else(|| num_cpus::get());
            if let Some(packages) = npmjs_client.packages(&name).await.unwrap() {
                let futures = packages
                    .keys()
                    .map(|pkg| package(&npmjs_client, pkg, job_size / 2));
                let stream = futures::stream::iter(futures).buffer_unordered(job_size);
                stream.collect::<Vec<_>>().await;
            }
        }
    }
}

async fn package(client: &npmjs::Client, name: &str, futures_buffer: usize) {
    if let Some(package) = client.package(name).await.unwrap() {
        info!("Found {}", package.name);
        let futures = package.versions.values().map(npmjs_verify::verify);
        let stream = futures::stream::iter(futures).buffer_unordered(futures_buffer);

        let mut outputs = stream.collect::<Vec<_>>().await;
        outputs.sort_by(|a, b| a.version.cmp(&b.version));
        for output in outputs {
            info!("{}: {:?}", output.version, output.result);
        }
    } else {
        info!("{} not found", name);
    }
}
