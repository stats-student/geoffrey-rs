use clap::Args;
use console::Style;
use ptree::{item, TreeBuilder};
use std::{fs, io, path};

#[derive(Args)]
pub struct Create {
    /// The name of the project to create
    #[clap(value_parser)]
    pub name: path::PathBuf,

    /// Whether to create the parent directories in the project name
    #[clap(short, long)]
    pub parents: bool,
}

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

    /// Prints the already exists error message to stderr
    ///
    /// This function prints an error message to stderr for errors which have
    /// an **AlreadyExists** error kind.
    ///
    /// # Arguments
    ///
    /// * `err` - The borrowed `io::error`
    fn _already_exists_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {}\n",
            self.name.display(),
            self._colour(9)
                .apply_to("already exists, please pick a different name")
        );
        panic!("{} exists\n{:?}", self.name.display(), err);
    }

    /// Prints the not found error message to stderr
    ///
    /// This function prints an error message to stderr for errors which have
    /// a **NotFound** error kind.
    ///
    /// # Arguments
    ///
    /// * `err` - The borrowed `io::error`
    fn _not_found_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {} {}",
            self._colour(9).apply_to("One or more parents of"),
            self.name.display(),
            self._colour(9).apply_to(
                "doesn't exist. Please create the parent directories\n\
                or pass the `--parents` option\n\
                \n\
                For example:\n\
                geoff create --parents test_project"
            )
        );
        panic!("Parents don't exist\n{:?}", err);
    }

    /// Prints the permission denied error message to stderr
    ///
    /// This function prints an error message to stderr for errors which have
    /// an **PermissionDenied** error kind.
    ///
    /// # Arguments
    ///
    /// * `err` - The borrowed `io::error`
    fn _permission_denied_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {} {}",
            self._colour(9)
                .apply_to("You don't have permission to create"),
            self.name.display(),
            self._colour(9).apply_to(
                ". Please change your permissions or choose a\n\
                different directory to create this project in"
            )
        );
        panic!("Invalid permissions\n{:?}", err);
    }

    /// Prints the generic error message to stderr
    ///
    /// This function prints an error message to stderr for errors which have
    /// an error kind that isn't `AlreadyExists`, `NotFound` or `PermissionDenied`.
    ///
    /// # Arguments
    ///
    /// * `err` - The borrowed `io::error`
    fn _generic_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {}\n",
            self._colour(9).apply_to(
                "There was an unknown error creating the directory,
                if you need help with this you can raise an issue here:"
            ),
            self._colour(12)
                .apply_to("https://github.com/stats-student/geoffrey-rs/issues")
        );
        panic!("{:?}", err);
    }

    /// Handles the different errors that might be encountered
    ///
    /// This function matches on three different error kinds `AlreadyExists`,
    /// `NotFound` or `PermissionDenied`. If a different error kind is encountered
    /// a generic error message is printed.
    ///
    /// # Arguments
    ///
    /// * `err` - The borrowed `io::error`
    fn _validate_create_result(&self, err: &io::Result<()>) {
        match err {
            Ok(_) => (),
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::AlreadyExists => self._already_exists_err_msg(err),
                    io::ErrorKind::NotFound => self._not_found_err_msg(err),
                    io::ErrorKind::PermissionDenied => self._permission_denied_err_msg(err),
                    _ => {
                        self._generic_err_msg(err);
                    } // _
                } // match err.kind()
            } // Err(err)
        } // match result
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
        let result: io::Result<()> = if self.parents {
            fs::create_dir_all(&self.name)
        } else {
            fs::create_dir(&self.name)
        };

        self._validate_create_result(&result);
    } // fn create_root

    /// Creates the subdirectories that geoff manages
    ///
    /// Creates the 4 directories that geoff manages
    /// * data_sources
    /// * explorations
    /// * models
    /// * products
    pub fn create_subdirectories(&self) {
        let subdirs = vec!["data_sources", "explorations", "models", "products"];

        for subdir in subdirs.iter() {
            let full_subdir = &format!("{}/{}/", self.name.display(), subdir);
            fs::create_dir(full_subdir)
                .unwrap_or_else(|_| panic!("Unable to create {}", full_subdir));
        }
    }

    /// Creates the initial files for the project
    ///
    /// Creates 3 files
    /// * README.md - General introduction to the project
    /// * project_scoping.md - The project scoping template to be filled out at the start of each project
    /// * .geoff - A blank file to indicate this directory is managed by geoff
    pub fn create_files(&self) {
        let files = vec!["README.md", "project_scoping.md", ".geoff"];

        for file in files.iter() {
            let full_file_path = &format!("{}/{}", self.name.display(), file);
            fs::write(full_file_path, "")
                .unwrap_or_else(|_| panic!("Unable to create {}", full_file_path));
        }
    }

    /// Creates a tree representation of the directories and files created
    ///
    /// This function creates a tree to be printed showing the different folders and
    /// files created.
    ///
    /// # Return value
    ///
    /// Returns an instance of a StringItem from the ptree crate. This is a struct storing
    /// the data in the tree
    pub fn create_tree(&self) -> item::StringItem {
        let bold = Style::new().bold();

        let tree = TreeBuilder::new(format!("{}", bold.apply_to("test_project")))
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
