use clap::{arg, Command};
use std::env;
use std::path::Path;
use std::process;

mod utils;

use utils::Config;

fn main() {
    let config = if let Ok(config) = Config::load() {
        config
    } else {
        let config = Config::default();
        config.write().unwrap();
        config
    };
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
    } else if hack {
        config.hacks_dir
    } else {
        config.projects_dir
    };
    change_dir(&path);
    path = path.join(name);
    make_hack(&path);
    println!("{}", path.to_str().unwrap());
}

fn make_hack(path: &Path) {
    if path.try_exists().unwrap() {
        panic!("directory already exists");
    } else {
        process::Command::new("cargo")
            .arg("new")
            .arg(path.file_name().unwrap())
            .output()
            .unwrap();
    };
}

fn change_dir<T: AsRef<Path>>(path: T) {
    env::set_current_dir(path).unwrap();
}
