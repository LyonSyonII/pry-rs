mod structs;

fn main() {
    let cli = structs::Cli::read();
    println!("{cli:?}")
}
