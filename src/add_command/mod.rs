use clap::{AppSettings, Args, Subcommand};
use std::{fs, io, path};

#[derive(Args)]
pub struct Add {
    #[clap(subcommand)]
    pub command: Option<AddCommands>,
}

#[derive(Subcommand)]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub enum AddCommands {
    /// Adds a data source instance
    DataSource(DataSource)
}

#[derive(Args)]
pub struct DataSource {
    /// The name of the data source
    #[clap(value_parser)]
    name: String,
    /// Flag to add a database data source
    #[clap(short, long, conflicts_with_all(&["extract", "web"]))]
    database: bool,
    /// Flag to add an extract data source
    #[clap(short, long, conflicts_with_all(&["database", "web"]))]
    extract: bool,
    /// Flag to add a web data source
    #[clap(short, long, conflicts_with_all(&["extract", "database"]))]
    web: bool
}

impl DataSource {
    fn _geoff_check(&self) -> Result<(), io::Error> {
        if path::Path::new(".geoff").exists() {
            Ok(())
        } else {
            panic!(
                "This directory is not managed by geoff. Please change to a directory that is"
            )
        }
    }

    pub fn create_data_source(&self) -> () {
        let create_result = fs::create_dir(
            format!("data_sources/{}", self.name)
        );
    }
}

#[cfg(test)]
mod tests;
