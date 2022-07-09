use clap::{CommandFactory, ErrorKind, Parser};
use std::path::PathBuf;


#[derive(clap::Parser)]
pub struct Cli {
    files: Vec<PathBuf>
}

impl Cli {
    pub fn read() -> Cli {
        let cli = Cli::parse();
        // Add piped contents to parse results
        if let Some(read) = read_pipe::read_pipe_split_whitespace() {
            cli.files.extend(read.into_iter().map(|s| PathBuf::from(s)))
        }

        if cli.files.is_empty() {
            let cmd = Cli::command();
            cmd.error(ErrorKind::MissingRequiredArgument, "<FILES> is required").exit();
        }

        cli
    }
}