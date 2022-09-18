use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct Create {
    /// The name of the project to create
    #[clap(value_parser)]
    pub name: PathBuf,

    /// Whether to create the parent directories in the project name
    #[clap(short, long)]
    pub parents: bool,
}