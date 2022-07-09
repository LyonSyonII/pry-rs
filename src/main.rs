use std::path::Path;

mod structs;

fn extract_rar(path: &Path) {
    let extract_to = path.with_extension("").to_string_lossy().into();
    
    unrar::Archive::new(path.to_string_lossy().into()).extract_to(extract_to).unwrap().process().unwrap();
}

fn main() {
    let cli = structs::Cli::read();
    
    for file in cli.files {
        let extension = file.extension().unwrap();
        if extension == "rar" {
            extract_rar(&file);
        } else {
            eprintln!("Extension not recognized");
            std::process::exit(1);
        }
    }
}
