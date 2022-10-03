use clap::Args;
use console::Style;
use log::{debug, info};
use ptree::{item, TreeBuilder};
use std::{collections, fs, io, path};

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


    /// Writes a better error message to stderr for already exists errors
    /// 
    /// Writes a more user friendly error message to stderr and then panics with
    /// the error that caused the problem.
    /// 
    /// # Arguments
    /// 
    /// * `err` A reference to the io::Error that is being handled
    fn _already_exists_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {}\n",
            self.name.display(),
            self._colour(9)
                .apply_to("already exists, please pick a different name")
        );
        panic!("{} exists\n{:?}", &self.name.display(), err);
    }

    /// Writes a better error message to stderr for not found errors
    /// 
    /// Writes a more user friendly error message to stderr and then panics with
    /// the error that caused the problem.
    /// 
    /// # Arguments
    /// 
    /// * `err` A reference to the io::Error that is being handled
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

    /// Writes a better error message to stderr for permission denied errors
    /// 
    /// Writes a more user friendly error message to stderr and then panics with
    /// the error that caused the problem.
    /// 
    /// # Arguments
    /// 
    /// * `err` A reference to the io::Error that is being handled
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

    /// Writes a better error message to stderr for all other errors
    /// 
    /// Writes a more user friendly error message to stderr and then panics with
    /// the error that caused the problem. This message is used when the error
    /// kind is not AlreadyExists, NotFound or PermissionDenied
    /// 
    /// # Arguments
    /// 
    /// * `err` A reference to the io::Error that is being handled
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

    /// Handles the errors from the folder creation and prints friendlier messages
    /// 
    /// This function matches on the three possible errors that might be returned
    /// by the fs::create_dir_all function. Once matched it writes a helpful
    /// message to stderr and then panics with the error. If it matches any other
    /// error the user is pointed to the github issues page for the project.
    /// 
    /// # Arguments
    /// 
    /// * `err` - The result from the root folder creation.
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

        self._validate_create_result(&result);
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
            let full_subdir = &format!("{}/{}/", self.name.display(), subdir);
            fs::create_dir(full_subdir)
                .unwrap_or_else(|_| panic!("Unable to create {}", full_subdir));

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
