use crate::globals::scaffold as Scaffold;
use crate::globals::setup::ThunderStorm;
use std::env;
use std::path::PathBuf;

/// accept language to use, call on the scaffold to build the application from user config
pub fn build(language: String, path: String) {
    //check if the path provided is valid or not
    //check the directory provided
    if path == "." || path == "./" {
       let path = env::current_dir().unwrap().to_str().unwrap().to_string();
        let is_empty = PathBuf::from(path.clone())
            .read_dir()
            .map(|mut i| i.next().is_none())
            .unwrap_or(false);

        if !is_empty {
            //prompt the user if he wants to proceed then receive and parse user input
            println!("The selected directory is not empty. Proceed anyway? y/N.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "n" || input.trim() == "N" {
                println!("Exiting...");
                return;
            }
        }
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::parse_options(language.clone(), path.clone());
        Scaffold::Application::new(application);
    } else if path.ends_with("/") {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        let path = path.clone().trim_end_matches("/").to_string();
        println!("Creating application in {}", current_path.clone() + &path);
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::parse_options(language.clone(), path.clone());
        Scaffold::Application::new(application);
    } else {
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::parse_options(language.clone(), path.clone());
        Scaffold::Application::new(application);
    }
}
