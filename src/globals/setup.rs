//load in the inquirer library
use crate::globals::inquirer as Commander;
#[path = "dirs.rs"]
mod dirs;
use dirs as Dirs;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct ThunderStorm {
    pub application_name: String,
    pub application_description: String,
    pub lang: String,
    pub path: String,
    pub dirs: Vec<String>,
    pub package_manager: String,
    pub use_env: bool,
    pub use_git: bool,
    pub use_typescript: bool,
    // pub test_suit: String,
    pub include_readme: bool,
    pub include_license: bool,
    pub include_package_json: bool,
    pub include_gitignore: bool,
    // pub include_changelog: bool,
    // pub include_tslint: bool,
    // pub include_eslint: bool,
    // pub include_prettier: bool,
}

impl ThunderStorm {
    pub fn parse_options(lang: String, path: String) -> ThunderStorm {
        //display the banner
        let custom_figlet_font =
            FIGfont::from_content(include_str!("./../../resources/roman.flf")).unwrap();
        let figure = custom_figlet_font.convert("thunderStorm");
        println!("{}", figure.unwrap().yellow().bold().on_black().to_string());
        //compute the application name
        let application_name = Commander::Prompt::new("Application name? ".to_string());
        let application_description = Commander::Prompt::new("Application description? ".to_string());

        let dirs = Dirs::create(); //compute the directories to create

        //inquire the package manager, use of env and typescript
        let known_package_managers: Vec<String> =
            vec!["npm".to_string(), "yarn".to_string(), "pnpm".to_string()];
        let package_manager = Commander::Select::new(
            "Which package manager do you use?".to_string(),
            known_package_managers,
        );

        //inquire the user configuration for the project
        let use_git =
            Commander::Confirm::new("Do you want to initialize as a git repository?".to_string());
        let use_typescript = Commander::Confirm::new("Do you want to use typescript?".to_string());
        /* let query_test_suit = Commander::Confirm::new("Do you want to setup test suit".to_string()); */
        /*   let mut test_suit = "".to_string();
               if query_test_suit {
                   test_suit = Commander::Prompt::new(
                       "Input a space delimited names of testing packages to use".to_string(),
                   );
               }
        */
        let include_license =
            Commander::Confirm::new("Do initialize project with LICENSE?".to_string());
        let include_package_json =
            Commander::Confirm::new("Do initialize project with package.json ?".to_string());

        /*    let include_tsconfig =
                   Commander::Confirm::new("Do initialize project with tsconfig?".to_string());
                    let include_changelog =
                   Commander::Confirm::new("Do initialize project with CHANGELOG?".to_string());
               let include_tslint =
                   Commander::Confirm::new("Do initialize project with tslint?".to_string());
               let include_eslint =
                   Commander::Confirm::new("Do initialize project with eslint?".to_string());
               let include_prettier =
                   Commander::Confirm::new("Do initialize project with prettier?".to_string());
        */
        //return the ThunderStorm object
        ThunderStorm {
            dirs,
            lang,
            path,
            application_name,
            application_description,
            package_manager,
            use_env: true,
            use_git,
            use_typescript,
            // test_suit,
            include_readme: true,
            include_license,
            include_package_json,
            include_gitignore: true,
            /*   include_tsconfig,
            include_tslint,
            include_eslint,
            include_changelog,
            include_prettier, */
        }
    }
}
