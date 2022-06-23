mod sample;
mod dirs;

use clap::Parser;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use std::env;
use std::path::PathBuf;

// use std::fs;
// use dialoguer::Confirm;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct ThunderArguments {
    //path to create the application
    #[clap(short, long, value_parser)]
    path: String,

    //programming language to use
    #[clap(short, long, value_parser, default_value = "javascript")]
    lang: String,
}

fn main() {

    let tt = sample::new("sample name".to_string());
    println!("{}", tt);

    let directory = dirs::create();
    println!("{:?}", directory);
    //display the banner
    let custom_figlet_font = FIGfont::from_file("resources/5lineoblique.flf").unwrap();
    let figure = custom_figlet_font.convert("Thunderstorm");
    println!("{}", figure.unwrap().yellow().bold());

    //parse the arguments
    let args = ThunderArguments::parse();
    let lang = args.lang;
    let mut path = args.path;
    println!("lang {} path {}", lang, path);

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
