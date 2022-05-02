use clap::Parser;
use npmjs_verify::{cli::Args, npmjs};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(package) = npmjs::package(&args.package).await.unwrap() {
        println!("Found {}", package.name);
        for version in package.versions.values() {
            if let Some(result) = npmjs_verify::verify(&version) {
                println!("{}: {}", version.version, result);
            } else {
                println!("{}: Can not verify", version.version);
            }
        }
    } else {
        println!("{} not found", &args.package);
    }
}
