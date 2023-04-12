use clap::{arg, Command};
use std::env;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let cmd = Command::new("hack").args(&[
        arg!(<name> "The name of the project"),
        arg!(-t - -temp "Create the project in the OS's temporary directory"),
        arg!(-k - -hack "Create the project in your folder for small test projects"),
    ]);
    let matches = cmd.get_matches();
    let name = matches.get_one::<String>("name").unwrap();
    let temp = *matches.get_one::<bool>("temp").unwrap();
    let hack = *matches.get_one::<bool>("hack").unwrap();
    let mut path = if temp {
        env::temp_dir()
    } else {
        dirs::home_dir()
            .unwrap()
            .join("projects")
            .join(if hack { "hacks" } else { "" })
    };
    change_dir(&path);
    path = path.join(name);
    make_hack(&path);
    println!("{}", path.to_str().unwrap());
}

fn make_hack(path: &PathBuf) {
    if !path.try_exists().unwrap() {
        process::Command::new("cargo")
            .arg("new")
            .arg(path.file_name().unwrap())
            .output()
            .unwrap();
    } else {
        panic!("directory already exists");
    };
}

fn change_dir<T: AsRef<Path>>(path: T) {
    env::set_current_dir(path).unwrap();
}
