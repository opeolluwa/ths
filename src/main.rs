//local module "init.rs" to initialize the application
mod init;
use init as Init;
use Init::ThunderStorm;

//local module "scaffold.rs" to scaffold the application
mod scaffold;
use scaffold as Scaffold;
// use Scaffold::Application;

use clap::Parser;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use std::env;
// use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ThunderArguments {
    #[clap(short, long, value_parser)]
    path: String, //path to create the application
    #[clap(short, long, value_parser, default_value = "javascript")]
    lang: String, //programming language to use
}

fn main() {
    //display the banner
    let custom_figlet_font = FIGfont::from_file("resources/5lineoblique.flf").unwrap();
    let figure = custom_figlet_font.convert("Thunderstorm");
    println!("{}", figure.unwrap().yellow().bold());

    //parse the arguments
    let args = ThunderArguments::parse();
    let lang = args.lang;
    let mut path = args.path;

    //check the directory provided
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

        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::new(lang.clone(), path.clone());
        // fs::create_dir(application.path.clone());
        // println!("{:?}", application);
        Scaffold::Application::new(application);
    } else if path.ends_with("/") {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        path = path.clone().trim_end_matches("/").to_string();
        println!("Creating application in {}", current_path.clone() + &path);

        //create
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::new(lang.clone(), path.clone());
        Scaffold::Application::new(application);

        // fs::create_dir(application.path.clone());
        // println!("{:?}", application);
    } else {
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::new(lang.clone(), path.clone());
        Scaffold::Application::new(application);

        // fs::create_dir(application.path.clone());
        // println!("{:?}", application);
    }

    // fs::create_dir(path.clone());
    // println!(" path {}", path);
}
