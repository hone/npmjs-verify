use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "npmjs-verify")]
#[clap(about = "CLI for verifying PGP on NPM Packages")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Package {
        #[clap(required = true)]
        name: String,
        #[clap(short, long)]
        job_size: Option<usize>,
    },
    User {
        #[clap(required = true)]
        name: String,
        #[clap(short, long)]
        job_size: Option<usize>,
    },
}
