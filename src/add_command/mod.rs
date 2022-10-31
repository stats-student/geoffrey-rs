use clap::{AppSettings, Args, Subcommand};
use ptree::{item, TreeBuilder};
use std::{fs, io, path};

use crate::pleasant_error::PleasantErrorHandler;

#[derive(Args)]
pub struct Add {
    #[clap(subcommand)]
    pub command: Option<AddCommands>,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub enum AddCommands {
    /// Adds a data source instance
    DataSource(DataSource),
}

#[derive(Args)]
pub struct DataSource {
    /// The name of the data source
    #[clap(value_parser)]
    pub name: path::PathBuf,
    /// Flag to add a database data source
    #[clap(short, long, conflicts_with_all(&["extract", "web"]))]
    pub database: bool,
    /// Flag to add an extract data source
    #[clap(short, long, conflicts_with_all(&["database", "web"]))]
    pub extract: bool,
    /// Flag to add a web data source
    #[clap(short, long, conflicts_with_all(&["extract", "database"]))]
    pub web: bool,
}

impl PleasantErrorHandler for DataSource {}

impl DataSource {
    fn _geoff_check(&self) -> Result<(), io::Error> {
        if path::Path::new(".geoff").exists() {
            Ok(())
        } else {
            panic!("This directory is not managed by geoff. Please change to a directory that is")
        }
    }

    pub fn create_data_source(&self) -> () {
        let result = fs::create_dir(format!("data_sources/{}", self.name.display()));

        self.validate_create_folder_result(&self.name, &result);
    }

    pub fn retrieve_metadata_contents(&self) -> &str {
        let metadata_contents: &str;

        if self.database {
            metadata_contents = include_str!("../templates/data_sources/database_metadata.md");
        } else if self.extract {
            metadata_contents = include_str!("../templates/data_sources/extract_metadata.md");
        } else if self.web {
            metadata_contents = include_str!("../templates/data_sources/web_metadata.md");
        } else {
            metadata_contents = include_str!("../templates/data_sources/default_metadata.md");
        }

        metadata_contents
    }

    pub fn update_placeholders(&self, text: &&str) -> String {
        let name_str = self
            .name
            .to_str()
            .expect("Unable to convert data source name to str");

        text.replace("<<<data_source_name>>>", name_str)
    }

    pub fn create_metadata(&self, contents: &String) -> () {
        let metadata_path = format!("data_sources/{}/metadata.md", self.name.display());
        fs::write(&metadata_path, contents)
            .unwrap_or_else(|_| panic!("Unable to copy to {}", &metadata_path));
    }

    pub fn create_tree(&self) -> item::StringItem {
        let gold = console::Style::new().color256(220);
        let hd = console::Style::new().color256(194);

        let tree = TreeBuilder::new(format!("{} data_sources", gold.apply_to("\u{1F5BF}")))
            .begin_child(format!(
                "{} {}",
                gold.apply_to("\u{1F5BF}"),
                self.name.display()
            ))
            .add_empty_child(format!("{} metadata.md", hd.apply_to("\u{1F5CE}")))
            .end_child()
            .build();

        tree
    }
}

#[cfg(test)]
mod tests;
