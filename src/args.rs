use clap::*;
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
