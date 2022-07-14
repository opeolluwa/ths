//local module "init.rs" to initialize the application
// mod init;
// use init as Init;
// use Init::ThunderStorm;

//local module "scaffold.rs" to scaffold the application
// mod scaffold;
// use scaffold as Scaffold;

//external crates
use clap::Parser;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
// use std::env;
// use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ThunderArguments {
    #[clap(subcommand)]
    action: ThunderSubCommands,
}

/// thunderstorm sub commands
///  "create", "init" and "config"
/// - create takes  path option : ths create --path ./workspace/app
/// - init is used to build application in current path
/// - config to save config files where thunderStorm binary Runs
#[derive(clap::Subcommand, Debug)]
pub enum ThunderSubCommands {
    ///create a new application im provided path
    Create {
        #[clap(short, long, value_parser, forbid_empty_values = true)]
        path: String, //path to create the application
        #[clap(short, long, value_parser, default_value = "javascript")]
        lang: String, //programming language to use
    },
    ///create application in current path
    Init {
        #[clap(short, long, value_parser, default_value = "javascript")]
        lang: String, //programming language to use
    },
    /// extend the thunderStorm configuration
    Config {
        #[clap(short, long, value_parser, default_value = "./")]
        config_path: String, //path to save config files
                             // todo!()
    },
}
fn main() {
    //display the banner
    let custom_figlet_font =
        FIGfont::from_content(include_str!("./../resources/roman.flf")).unwrap();
    let figure = custom_figlet_font.convert("thunderStorm");
    println!("{}", figure.unwrap().yellow().bold().on_black().to_string());

    //parse the arguments
    let args = ThunderArguments::parse();
    println!("{:?}", args);
    /*  let lang = args.lang;
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
        Scaffold::Application::new_pwd(application);
    } else if path.ends_with("/") {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        path = path.clone().trim_end_matches("/").to_string();
        println!("Creating application in {}", current_path.clone() + &path);
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::new(lang.clone(), path.clone());
        Scaffold::Application::new(application);
    } else {
        //pass application instance to Scaffold::application::new()
        let application = ThunderStorm::new(lang.clone(), path.clone());
        Scaffold::Application::new(application);
    } */
}
