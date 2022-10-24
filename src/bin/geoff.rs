use clap::{AppSettings, Parser, Subcommand};

#[cfg(feature = "documentation")]
use std::path;

use geoffrey::create_command::Create;
use geoffrey::add_command::{Add, AddCommands};

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
    Add(Add)
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
        Some(Commands::Add(add)) => {
            match &add.command {
                Some(AddCommands::DataSource(data_source)) => {
                    let data_source_name = format!("data_sources/{}", data_source.name.display()); // create_data_source

                    // method to get metadata contents
                    let metadata_contents: &str;

                    if data_source.database {
                        metadata_contents = include_str!(
                            "../templates/data_sources/database_metadata.md"
                        );
                    } else if data_source.extract {
                        metadata_contents = include_str!(
                            "../templates/data_sources/extract_metadata.md"
                        );
                    } else if data_source.web {
                        metadata_contents = include_str!(
                            "../templates/data_sources/web_metadata.md"
                        );
                    } else {
                        metadata_contents = include_str!(
                            "../templates/data_sources/default_metadata.md"
                        );
                    }

                    // create_metadata
                    let mut data_source_metadata_path = data_source_name.clone();
                    data_source_metadata_path.push_str("/metadata.md");

                    let updated_contents = metadata_contents.replace(
                        "<<<data_source_name>>>", &data_source.name.to_str().unwrap()
                    );

                    // create_data_source
                    std::fs::create_dir(&data_source_name)
                        .unwrap_or_else(|_| panic!("Unable to create {}", &data_source_name));

                    // create_metadata
                    std::fs::write(&data_source_metadata_path, &updated_contents)
                        .unwrap_or_else(|_| panic!("Unable to copy to {}", &data_source_metadata_path));

                    // create tree
                    let gold = console::Style::new().color256(220);
                    let hd = console::Style::new().color256(194);

                    let tree = ptree::TreeBuilder::new(format!(
                        "{} data_sources",
                        gold.apply_to("\u{1F5BF}")
                    ))
                    .begin_child(format!(
                        "{} {}",
                        gold.apply_to("\u{1F5BF}"),
                        data_source.name.display()
                    ))
                    .add_empty_child(
                        format!(
                            "{} metadata.md",
                            hd.apply_to("\u{1F5CE}")
                        )   
                    )
                    .end_child()
                    .build();

                    ptree::print_tree(&tree).unwrap();
                }
                None => {
                    println!("Matched none");
                }
            }
        }
        None => {
            println!("Matched none");
        }
    }
}
