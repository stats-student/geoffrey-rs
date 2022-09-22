use clap::{AppSettings, Parser, Subcommand};
use std::path;

use geoffrey::create_command::Create;

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
    Create(Create),
    /// Builds a documentation site for the project
    BuildDocs {
        /// The location for the documentation website
        #[clap(short, long, default_value = "./docs/", value_parser)]
        output: path::PathBuf,
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
        Some(Commands::Create(create)) => {
            create.create_root();
            create.create_subdirectories();
            create.create_files();
            let tree = create.create_tree();

            let name_str = create
                .name
                .file_name()
                .unwrap()
                .to_str()
                .expect("name isn't a valid unicode string");

            println!("\u{1F680} {} created!\n", name_str);
            ptree::print_tree(&tree).unwrap();
        }
        Some(Commands::BuildDocs { output:_ }) => {
            println!("build docs matched");
        }
        Some(Commands::Add { command:_ }) => {
            println!("add matched");
        }
        None => {
            println!("Matched non");
        }
    }
}
