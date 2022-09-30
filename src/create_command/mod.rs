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
    fn _colour(&self, col_256: u8) -> Style {
        Style::new().color256(col_256)
    }

    fn _already_exists_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {}\n",
            self.name.display(),
            self._colour(9)
                .apply_to("already exists, please pick a different name")
        );
        panic!("{} exists\n{:?}", self.name.display(), err);
    }

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

    fn _update_placeholders(&self, file_contents: &&str) -> String {
        file_contents.replace(
            "<<<project_name>>>",
            self.name.file_stem().unwrap().to_str().unwrap(),
        )
    }

    pub fn create_root(&self) {
        info!("Creating project root at {}", self.name.display());

        let result: io::Result<()> = if self.parents {
            fs::create_dir_all(&self.name)
        } else {
            fs::create_dir(&self.name)
        };

        self._validate_create_result(&result);
    } // fn create_root

    pub fn create_subdirectories(&self) {
        let subdirs = vec!["data_sources", "explorations", "models", "products"];

        for subdir in subdirs.iter() {
            info!("Creating project sub directory: {}", subdir);

            let full_subdir = &format!("{}/{}/", self.name.display(), subdir);
            fs::create_dir(full_subdir)
                .unwrap_or_else(|_| panic!("Unable to create {}", full_subdir));
        }
    }

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

            if filename.starts_with(".") {
                info!("Writing {} to root folder", filename);

                let root_path: &String = &format!("{}/{}", self.name.display(), filename);
                fs::write(&root_path, &contents)
                    .unwrap_or_else(|_| panic!("Unable to copy to {}", &root_path));
            } else {
                let updated_contents = self._update_placeholders(&contents);

                info!("Writing {} to root folder", filename);

                let root_path: &String = &format!("{}/{}", self.name.display(), filename);
                fs::write(&root_path, &updated_contents)
                    .unwrap_or_else(|_| panic!("Unable to copy to {}", &root_path));
            }
        }
    }

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
