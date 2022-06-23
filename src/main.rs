use clap::Parser;
use std::env;
// use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct ThunderStorm {
    //path to create the application
    #[clap(short, long, value_parser)]
    path: String,

    //programming language to use
    #[clap(short, long, value_parser, default_value = "javascript")]
    lang: String,
}

fn main() {
    let args = ThunderStorm::parse();
    let lang = args.lang;
    let mut path = args.path;
    println!("lang {} path {}", lang, path);

    /*
     if the application directory is selected as the current check if its's empty
     to do this, first get the absolute path the check if the path is empty,
     if not empty ask the user if he wants to proceed
     if the path is empty, create the directory
    */
    if path == "." || path == "./" {
        path = env::current_dir().unwrap().to_str().unwrap().to_string();
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
        //else
        println!("Creating application in {}", path);
    } else if path.ends_with("/") {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        path = path.clone().trim_end_matches("/").to_string();
        println!("Creating application in {}", current_path.clone() + &path);

    } else {
        // path = path.clone();
        // path = PathBuf::from(path.clone().to_string().unwrap())
        println!("Creating application in {}", path);
    }

    // fs::create_dir(path.clone());
    // println!(" path {}", path);
}
