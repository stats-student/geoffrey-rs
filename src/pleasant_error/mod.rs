use console::Style;
use std::{io, path};

pub trait PleasantErrorHandler {
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
    fn already_exists_err_msg(&self, name: &path::PathBuf, err: &io::Error) -> (){
        eprintln!(
            "{} {}\n",
            name.display(),
            self._colour(9)
                .apply_to("already exists, please pick a different name")
        );
        panic!("{} exists\n{:?}", name.display(), err);
    }

    /// Writes a better error message to stderr for not found errors
    /// 
    /// Writes a more user friendly error message to stderr and then panics with
    /// the error that caused the problem.
    /// 
    /// # Arguments
    /// 
    /// * `err` A reference to the io::Error that is being handled
    fn not_found_err_msg(&self, name: &path::PathBuf, err: &io::Error) {
        eprintln!(
            "{} {} {}",
            self._colour(9).apply_to("One or more parents of"),
            name.display(),
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
    fn permission_denied_err_msg(&self, name: &path::PathBuf, err: &io::Error) {
        eprintln!(
            "{} {} {}",
            self._colour(9)
                .apply_to("You don't have permission to create"),
            name.display(),
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
    fn generic_err_msg(&self, err: &io::Error) {
        eprintln!(
            "{} {}\n",
            self._colour(9).apply_to(
                "There was an unknown error creating the directory,
                if you need help with this you can raise an issue here:"
            ),
            self._colour(12)
                .apply_to("https://github.com/stats-student/geoffrey-rs/issues")
        );
        panic!("Unknown error\n{:?}", err);
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
    fn validate_create_folder_result(&self, name: &path::PathBuf, err: &io::Result<()>) {
        match err {
            Ok(_) => (),
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::AlreadyExists => self.already_exists_err_msg(name, err),
                    io::ErrorKind::NotFound => self.not_found_err_msg(name, err),
                    io::ErrorKind::PermissionDenied => self.permission_denied_err_msg(name, err),
                    _ => {
                        self.generic_err_msg(err);
                    } // _
                } // match err.kind()
            } // Err(err)
        } // match result
    }
}

#[cfg(test)]
mod tests;
