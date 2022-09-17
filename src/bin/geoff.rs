use clap::{AppSettings, Parser, Subcommand};
use std::fs::create_dir;
use std::path::{PathBuf};

#[derive(Parser)]
#[clap(version, about, long_about = None, setting = AppSettings::SubcommandRequiredElseHelp)]
struct Geoffrey {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
enum Commands {
    /// Creates a new data science project managed by geoffrey
    Create {
        /// The name of the project to create
        #[clap(value_parser)]
        name: PathBuf,

        /// Whether to create the parent directories in the project name
        #[clap(short, long)]
        parents: bool,
    },
    /// Builds a documentation site for the project
    BuildDocs {
        /// The location for the documentation website
        #[clap(short, long, default_value = "./docs/", value_parser)]
        output: PathBuf,
    },
    /// Adds a new instance of a data source, exploration, model or product
    Add {
        #[clap(subcommand)]
        command: Option<AddCommands>,
    },
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
enum AddCommands {
    /// Adds a data source instance
    DataSource {
        /// The name of the data source
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Geoffrey::parse();

    match &cli.command {
        Some(Commands::Create { name, parents }) => {
            create_dir(&name).expect("Project folder couldn't be created");

            let name_str = name
                .file_name()
                .unwrap()
                .to_str()
                .expect("name isn't a valid unicode string");

            println!("\u{1F680} {} created!", name_str);
        }
        Some(Commands::BuildDocs { output }) => {
            println!("build docs matched");
        }
        Some(Commands::Add { command }) => {
            println!("add matched");
        }
        None => {
            println!("Matched non");
        }
    }
}
