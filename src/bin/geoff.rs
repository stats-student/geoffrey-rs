use clap::{AppSettings, Parser, Subcommand};

#[cfg(feature = "documentation")]
use std::path;

use geoffrey::add_command::{Add, AddCommands};
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
    #[cfg(feature = "documentation")]
    /// Builds a documentation site for the project
    BuildDocs {
        /// The location for the documentation website
        #[clap(short, long, default_value = "./docs/", value_parser)]
        output: path::PathBuf,
    },
    /// Adds a new instance of a data source, exploration, model or product
    Add(Add),
}

fn main() {
    env_logger::init();

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
        #[cfg(feature = "documentation")]
        Some(Commands::BuildDocs { output: _ }) => {
            println!("build docs matched");
        }
        Some(Commands::Add(add)) => match &add.command {
            Some(AddCommands::DataSource(data_source)) => {
                data_source.create_data_source();

                let contents = data_source.retrieve_metadata_contents();
                let updated_contents = data_source.update_placeholders(&contents);
                data_source.create_metadata(&updated_contents);

                let tree = data_source.create_tree();

                let name_str = data_source
                    .name
                    .file_name()
                    .unwrap()
                    .to_str()
                    .expect("name isn't a valid unicode string");

                println!("\u{1F680} {} created!\n", name_str);
                ptree::print_tree(&tree).unwrap();
            }
            None => {
                println!("Matched none");
            }
        },
        None => {
            println!("Matched none");
        }
    }
}
