use std::env;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct ThunderStorm {
  /*   /// name of the application
    #[clap(short, long, value_parser)]
    name: String,
 */
    //path to create the application
    #[clap(short, long, value_parser)]
    path: String,

    //programming language to use
    #[clap(short, long, value_parser, default_value = "javascript")]
    lang: String,
}

fn main() {
    let args = ThunderStorm::parse();

    println!("{:?}", args);

    /*
    ///the path of the application to be created is passed as an argument
    let mut application_path: String = env::args().nth(2).unwrap();
    application_path = application_path.to_string();

    if application_path == "." || application_path == "./" {
        application_path = env::current_dir().unwrap().to_str().unwrap().to_string();
        println!(" application will be created in current path. Proceed anyway? Y/n  {}", application_path);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "n" {
            println!("Exiting...");
            return;
        }

    } else if application_path.ends_with("/") {
        application_path = application_path.clone().trim_end_matches("/").to_string();
    } else {
        application_path = application_path.clone();
    }
    fs::create_dir(application_path.clone());
    println!(" path {}", application_path);
    */
}
