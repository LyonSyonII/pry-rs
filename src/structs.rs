extern crate clap;
use self::clap::{CommandFactory, ErrorKind, Parser};
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn read() -> Cli {
        let mut cli = Cli::parse();
        let mut cmd = Cli::command();

        // Add piped contents to parse results
        if let Some(read) = read_pipe::read_pipe_split_whitespace() {
            cli.files.extend(read.into_iter().map(PathBuf::from));
            cli.files.sort();
            // Remove duplicates
            cli.files.dedup();
        }

        if cli.files.is_empty() {
            cmd.error(
                ErrorKind::MissingRequiredArgument,
                "The following required arguments were not provided: <FILES>...",
            )
            .exit();
        }

        // Canonicalize paths
        for path in &mut cli.files {
            *path = path.canonicalize().unwrap_or_else(|_| {
                cmd.error(
                    ErrorKind::ValueValidation,
                    format_args!("The file \"{}\" does not exist", path.display()),
                )
                .exit()
            });
        }

        cli
    }
}
