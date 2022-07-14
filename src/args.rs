use clap::*;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct ThunderArguments {
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
    ///create a new application in provided path
    Create(CreateCommands),
    ///create application in current path
    Init(InitCommands),
    /// extend the thunderStorm configuration
    Config(ConfigCommands),
}

/// create the init command, essentially a struct to hold the init command arguments and options
#[derive(Args, Debug)]
pub struct CreateCommands {
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    ///path to the directory where the application will be created
    path: String, //path to create the application
    #[clap(short, long, value_parser, default_value = "javascript")]
    ///programming language to use for the application, default to JavaScript
    lang: String, //programming language to use
}

/// create the init command, essentially a struct to hold the init command arguments and options
#[derive(Args, Debug)]
pub struct InitCommands {
    #[clap(short, long, value_parser, default_value = "javascript")]
    ///programming language to use for the application, default to JavaScript
    lang: String,
}

/// create the config command, essentially a struct to hold the config command arguments and options
#[derive(Args, Debug)]
pub struct ConfigCommands {
    ///path to .thunderConfig file
    #[clap(short, long, value_parser, default_value = "./.thunderConfig")]
    config_path: String,
}
