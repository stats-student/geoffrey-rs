use clap::Args;
use console::Style;
use log::{debug, info};
use ptree::{item, TreeBuilder};
use std::{collections, fs, io, path};

use crate::pleasant_error::PleasantErrorHandler;

#[derive(Args)]
pub struct Create {
    /// The name of the project to create
    #[clap(value_parser)]
    pub name: path::PathBuf,

    /// Whether to create the parent directories in the project name
    #[clap(short, long)]
    pub parents: bool,
}

impl PleasantErrorHandler for Create{}

impl Create {
    /// Creates a new style class to apply color to text
    ///
    /// # Arguments
    ///
    /// * `col_256` - The xterm number corresponding to the required color
    ///
    /// # Return value
    ///
    /// Returns a Style struct for the color associated with the given xterm
    /// number so the color that can be applied to text.
    fn _colour(&self, col_256: u8) -> Style {
        Style::new().color256(col_256)
    }

    /// Replaces placeholder tags in the template files
    /// 
    /// Each of the template files has tags that are replaced with information
    /// specific to the project. At the moment the only tags that are replaced
    /// are for the project name. Placeholder tags are denoted by 3 angled brackets
    /// e.g "<<<placeholder>>>"
    /// 
    /// # Arguments
    /// 
    /// * `file_contents` - Refernce to a string literal containing the contents
    ///                     of a template file
    /// 
    /// # Returns
    /// 
    /// * A String containing the file_contents with placeholders replaced
    fn _update_placeholders(&self, file_contents: &&str) -> String {
        file_contents.replace(
            "<<<project_name>>>",
            &self.name.file_stem().unwrap().to_str().unwrap(),
        )
    }

    /// Creates the root directory of a new project
    ///
    /// Creates a new folder from the name of a directory or a path to the desired
    /// location.
    ///
    /// If a path is passed, the parents won't be created by default however if the
    /// `--parents` option is passed the parents will also be created.
    ///
    /// # Errors
    ///
    /// * The directory already exists
    /// * The parent(s) of the path don't exist
    /// * The user doesn't have permissions to create the directory
    pub fn create_root(&self) {
        info!("Creating project root at {}", &self.name.display());

        let result: io::Result<()> = if self.parents {
            fs::create_dir_all(&self.name)
        } else {
            fs::create_dir(&self.name)
        };

        self.validate_create_folder_result(&self.name, &result);
    } // fn create_root

    /// Creates the subdirectories within the project root
    ///
    /// Creates the 4 directories that geoff manages
    /// * data_sources
    /// * explorations
    /// * models
    /// * products
    pub fn create_subdirectories(&self) {
        let subdirs = vec!["data_sources", "explorations", "models", "products"];

        for subdir in subdirs.iter() {
            info!("Creating project sub directory: {}", &subdir);

            let full_subdir = &format!("{}/{}/", &self.name.display(), &subdir);
            fs::create_dir(&full_subdir)
                .unwrap_or_else(|_| panic!("Unable to create {}", &full_subdir));
        }
    }


    /// Creates the files within the project root
    ///
    /// Creates 3 files
    /// * README.md - General introduction to the project
    /// * project_scoping.md - The project scoping template to be filled out at the start of each project
    /// * .geoff - A blank file to indicate this directory is managed by geoff
    pub fn create_files(&self) {
        let files = collections::HashMap::from([
            ("README.md", include_str!("../templates/root/README.md")),
            (
                "project_scoping.md",
                include_str!("../templates/root/project_scoping.md"),
            ),
            (".geoff", include_str!("../templates/root/.geoff")),
        ]);

        for (filename, contents) in files.iter() {
            debug!("Replacing placeholders in {}", filename);

            let updated_contents: String;

            if !filename.starts_with(".") {
                updated_contents = self._update_placeholders(&contents);
            } else {
                updated_contents = contents.to_string();
            }

            info!("Writing {} to root folder", filename);

            let root_path: &String = &format!("{}/{}", &self.name.display(), filename);
            fs::write(&root_path, &updated_contents)
                .unwrap_or_else(|_| panic!("Unable to copy to {}", &root_path));
        }
    }

    /// Creates a tree showing the files and folders created
    ///
    /// The tree shows all the files and folders that have been created when running
    /// the `geoff create` command
    pub fn create_tree(&self) -> item::StringItem {
        let bold = Style::new().bold();

        let tree = TreeBuilder::new(format!("{}", bold.apply_to(self.name.display())))
            .add_empty_child(format!(
                "{} data_sources",
                self._colour(220).apply_to("\u{1F5BF}")
            ))
            .add_empty_child(format!(
                "{} explorations",
                self._colour(220).apply_to("\u{1F5BF}")
            ))
            .add_empty_child(format!(
                "{} models",
                self._colour(220).apply_to("\u{1F5BF}")
            ))
            .add_empty_child(format!(
                "{} products",
                self._colour(220).apply_to("\u{1F5BF}")
            ))
            .add_empty_child(format!(
                "{} README.md",
                self._colour(194).apply_to("\u{1F5CE}")
            ))
            .add_empty_child(format!(
                "{} project_scoping.md",
                self._colour(194).apply_to("\u{1F5CE}")
            ))
            .build();

        tree
    }
} // impl Create

#[cfg(test)]
mod tests;
