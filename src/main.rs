//modules
mod args;
mod commands;
mod globals;
//external crates
use clap::Parser;

fn main() {
   
    //parse the arguments
    let args = args::ThunderArguments::parse();
    //try to  get the action command from the arguments
    let action = args.action;
    println!("{:?}", action);

    //match the sub commands and execute the appropriate commands
    match action {
        args::ThunderSubCommands::Create(create_command) => {
            //create the application
            commands::create::build(create_command.lang, create_command.path);
        }
        args::ThunderSubCommands::Init(init_command) => {
            commands::init::build(init_command.lang);
            // println!("{:?}", action);
        }
        args::ThunderSubCommands::Config(config_command) => {
            println!("{:?}", config_command);
        }
    }

    // println!("{:?}", args);
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
