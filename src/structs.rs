use clap::Parser;
use std::path::PathBuf;


#[derive(clap::Parser)]
pub struct Cli {
    files: Vec<PathBuf>
}

impl Cli {
    fn read() -> Cli {
        let cli = Cli::parse();
        if cli.files.is_empty() {

        }
    }
}