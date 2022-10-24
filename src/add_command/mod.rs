use clap::{AppSettings, Args, Subcommand};
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
    DataSource(DataSource)
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
    pub web: bool
}

impl PleasantErrorHandler for DataSource{}

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
        let result = fs::create_dir(
            format!("data_sources/{}", self.name.display())
        );

        self.validate_create_folder_result(&self.name, &result);
    }
}

#[cfg(test)]
mod tests;
