use clap::Parser;
use futures::StreamExt;
use npmjs_verify::{
    cli::{Cli, Commands},
    npmjs,
    verify::VerifyOutput,
};
use tracing::warn;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let npmjs_token = std::env::var("NPMJS_TOKEN").ok();
    let npmjs_client = npmjs::Client::new(npmjs_token).unwrap();
    let args = Cli::parse();
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    match args.command {
        Commands::Package { name, job_size } => {
            if let Some(outputs) = package(
                &npmjs_client,
                &name,
                job_size.unwrap_or_else(|| num_cpus::get()),
            )
            .await
            {
                for output in outputs {
                    writer.serialize(output).unwrap();
                }
            } else {
                warn!("{} not found", name);
            }
        }
        Commands::User { name, job_size } => {
            let job_size = job_size.unwrap_or_else(|| num_cpus::get());
            if let Some(packages) = npmjs_client.packages(&name).await.unwrap() {
                let futures = packages
                    .keys()
                    .map(|pkg| package(&npmjs_client, pkg, job_size / 2));
                let stream = futures::stream::iter(futures).buffer_unordered(job_size);
                let packages_outputs = stream.collect::<Vec<_>>().await;
                for package_outputs in packages_outputs {
                    if let Some(outputs) = package_outputs {
                        for output in outputs {
                            writer.serialize(output).unwrap();
                        }
                    } else {
                        warn!("{} not found", name);
                    }
                }
            }
        }
    }

    writer.flush().unwrap();
}

async fn package(
    client: &npmjs::Client,
    name: &str,
    futures_buffer: usize,
) -> Option<Vec<VerifyOutput>> {
    if let Some(package) = client.package(name).await.unwrap() {
        let futures = package.versions.values().map(npmjs_verify::verify);
        let stream = futures::stream::iter(futures).buffer_unordered(futures_buffer);

        let mut outputs = stream
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<VerifyOutput>, std::io::Error>>()
            .unwrap();
        outputs.sort_by(|a, b| a.version.cmp(&b.version));

        Some(outputs)
    } else {
        None
    }
}
