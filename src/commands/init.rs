use crate::globals::setup::ThunderStorm;
use crate::globals::scaffold as Scaffold;
use std::env;
use std::path::PathBuf;

/// accept language to use, call on the scaffold to build the application from user config
pub fn build(language: String) {
    //check the current directory if it contains  any files or folder
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let is_empty = PathBuf::from(path.clone())
        .read_dir()
        .map(|mut i| i.next().is_none())
        .unwrap_or(false);

    //prompt the user if he wants to proceed, given the directory is not empty then receive and parse user input
    if !is_empty {
        println!("The selected directory is not empty. Proceed anyway? y/N.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "n" || input.trim() == "N" {
            println!("Exiting...");
            return;
        }
    }

    //pass application instance configuration and pass the config object to Scaffold::application::new()
    let application = ThunderStorm::parse_options(language.clone(), path.clone());
    Scaffold::Application::new_pwd(application);
}
