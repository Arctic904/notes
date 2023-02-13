use xdg::BaseDirectories;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    } else {
        println!("{}", get_current_working_dir().unwrap().to_str().unwrap());
    }
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}