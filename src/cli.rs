use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub package: Option<String>,
    #[clap(short, long)]
    pub user: Option<String>,
}
