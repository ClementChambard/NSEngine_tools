use std::io::Write;

pub struct Verbosity {
    verbosity: u8,
}

impl Verbosity {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let verbosity = if matches.get_flag("quiet") {
            0u8
        } else {
            matches
                .get_one::<u8>("verbose")
                .expect("Getting verbosity shouldn't fail.")
                + 1
        };
        Self { verbosity }
    }
    pub fn msg(&self, min_level: u8, msg: &str) {
        if self.verbosity >= min_level {
            println!("{msg}");
        }
    }
    pub fn create_dir<P>(&self, path: P)
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        // TODO: proper io error handling
        std::fs::create_dir(&path).expect(&format!("Failed to create {path:?} directory."));
        if self.verbosity > 2 {
            println!("Created directory: {path:?}");
        }
    }

    pub fn set_current_dir<P>(&self, path: P)
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        // TODO: proper io error handling
        std::env::set_current_dir(&path)
            .expect(&format!("Failed to navigate to {path:?} directory."));
        if self.verbosity > 2 {
            println!("Navigating to {path:?}");
        }
    }

    pub fn create_file<P>(&self, path: P) -> std::fs::File
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        // TODO: proper io error handling
        let file =
            std::fs::File::create("project.ns").expect("Creating default project.ns failed.");
        if self.verbosity > 2 {
            println!("Created file: {path:?}");
        }
        file
    }

    pub fn create_file_with_data<P>(&self, path: P, data: &str)
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        let mut file = self.create_file(&path);
        write!(file, "{}", data).expect(&format!("Writing {path:?} failed."));
        if self.verbosity > 2 {
            println!("Wrote file: {path:?}");
        }
    }

    pub fn remove_file<P>(&self, path: P)
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        std::fs::remove_file(&path).expect(&format!("Removing {path:?} failed."));
        if self.verbosity > 2 {
            println!("Removed file: {path:?}");
        }
    }

    pub fn fs_copy<P, Q>(&self, from: P, to: Q)
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
        Q: AsRef<std::path::Path> + std::fmt::Debug,
    {
        std::fs::copy(&from, &to).expect(&format!("Copying {from:?} to {to:?} failed."));
        if self.verbosity > 2 {
            println!("Copied {from:?} to {to:?}");
        }
    }
}
