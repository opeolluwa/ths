mod args;
mod commands;
mod globals;
use clap::Parser;

fn main() {
    let args = args::ThunderArgs::parse();
    //try to  get the action command from the arguments
    let action = args.action;
    //match the sub commands and execute the appropriate commands
    match action {
        //create the application
        args::ThunderSubCommands::Create(create_command) => {
            commands::create::build(create_command.lang, create_command.path);
        }
        //init the application
        args::ThunderSubCommands::Init(init_command) => {
            commands::init::build(init_command.lang);
        }
        //configure the application
        args::ThunderSubCommands::Config(config_command) => {
            commands::config::build(config_command.config_path);
        }
    }
}
