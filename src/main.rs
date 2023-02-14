use xdg::BaseDirectories;
use chrono::offset::Local;
use chrono::DateTime;
use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

const DATE_YMD_TIME: &str = "%a %b %e, %Y - %H:%M";

fn main() {
    // let args: Vec<_> = env::args().collect();
    // if args.len() > 1 {
    //     println!("The first argument is {}", args[1]);
    // } else {
    //     println!("{}", get_current_working_dir().unwrap().to_str().unwrap());
    // }
    let bd = BaseDirectories::with_prefix("notes").unwrap().get_data_home();
    let data_dir = bd.as_path();

    println!("{}", data_dir.to_str().unwrap());

    if !bd.is_dir() {
        fs::create_dir_all(data_dir).unwrap();
    }

    // let config_file = data_dir.join("test.cfg");

    // let _file = fs::write(&config_file, "path-location: 0").unwrap();
    // let file_path = &config_file.to_str().unwrap();
    // let date: DateTime<Local> = config_file.metadata().unwrap().modified().unwrap().into();
    // println!("{}", date.format(DATE_YMD_TIME).to_string());

    let location = get_input("Enter desired file location: \n0: Default XDG_DATA_HOME\n1: Current Directory");
    let file_path = location.trim();
    if file_path == "0" {
        loop {
            let files = get_files(&data_dir);
            let file_index = get_input("Enter file index (q to exit): ");
            if file_index.trim() == "q" {
                println!("Exiting...");
                return;
            }
            let file_index = file_index.trim().parse::<usize>().unwrap();
            if file_index >= files.len() {
                let opt = get_input("Invalid index, would you like to create a new file? (y/n)");
                if opt.trim() == "y" {
                    let file_name = get_input("Enter file name: ");
                    let file_name = file_name.trim();
                    let file = data_dir.join(file_name);
                    let _file = fs::File::create(&file).unwrap();
                    open_editor(&file.to_str().unwrap());
                    return;
                } else {
                    println!("Exiting...");
                }
            } else {
                let file = files[file_index].to_str().unwrap();
                open_editor(file);
            }
        }
    } else if file_path == "1" {
        loop {
            let current_dir = get_current_working_dir().unwrap();
            let files = get_files(&current_dir);
            let file_index = get_input("Enter file index (q to exit): ");
            if file_index.trim() == "q" {
                println!("Exiting...");
                return;
            }
            let file_index = file_index.trim().parse::<usize>().unwrap();
            if file_index >= files.len() {
                let opt = get_input("Invalid index, would you like to create a new file? (y/n)");
                if opt.trim() == "y" {
                    let file_name = get_input("Enter file name: ");
                    let file_name = file_name.trim();
                    let file = current_dir.join(file_name);
                    let _file = fs::File::create(&file).unwrap();
                    open_editor(&file.to_str().unwrap());
                    return;
                } else {
                    println!("Exiting...");
                }
            } else {
                let file = files[file_index].to_str().unwrap();
                open_editor(file);
            }
        }
    } else {
        panic!("invalid option")
    }

}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn get_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    files.sort_by(|a, b| b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap()));
    for (i, file) in files.iter().enumerate() {
        let date: DateTime<Local> = file.metadata().unwrap().modified().unwrap().into();
        println!("{}: {} - {}", i, date.format(DATE_YMD_TIME).to_string(), file.file_name().unwrap().to_str().unwrap());
    }
    files
}

fn open_editor(file_path: &str) {
    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("nano ".to_owned() + file_path)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: Failed to read input");
    input
}