// mod dirs
extern crate dialoguer;
use dialoguer::{theme::CustomPromptCharacterTheme, Confirmation, Input, Select};

mod dirs {
    use dialoguer::{theme::ColorfulTheme, Checkboxes};
    //prompt user for folders to create, return array of folder names
    pub fn create() -> Vec<String> {
        const CHECKBOXES: &[&str; 11] = &[
            "config",
            "controller",
            "migrations",
            "files",
            "middleware",
            "models",
            "lib",
            "routes",
            "templates",
            "utils",
            "views",
        ];

        let defaults = &[false; CHECKBOXES.len()];
        let selections = Checkboxes::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select the desired folders to create")
            .items(&CHECKBOXES[..])
            .defaults(&defaults[..])
            .interact()
            .unwrap();

        if selections.is_empty() {
            println!("You did not select anything :(");
            return vec![];
        }
        //return the array of folder names
        // selections
        let mut dirs = vec![];
        for i in selections {
            dirs.push(CHECKBOXES[i].to_string());
        }
        //return the directory names
        dirs
    }
}
#[derive(Debug)]
pub struct ThunderStorm {
    pub application_name: String,
    pub lang: String,
    pub path: String,
    pub dirs: Vec<String>,
    pub package_manager: String,
    pub use_env: bool,
    // pub use_git: bool,
    // pub use_typescript: bool,
}

impl ThunderStorm {
    pub fn new(lang: String, path: String) -> ThunderStorm {
        //compute the application name
        let application_name = Inquirer::prompt("Application name ".to_string());
        let dirs = dirs::create(); //compute the directories to create

        //inquire the package manager, use of env and typescript
        let known_package_managers: Vec<String> = vec!["npm".to_string(), "yarn".to_string()];
        let package_manager = Inquirer::select(
            "Which package manager do you use?".to_string(),
            known_package_managers,
        );

        //inquire the use of environment
        let use_env =
            Inquirer::confirm("Do you want to use the environment variables?".to_string());
        // let use_git = Inquirer::confirm("Do you want to use git?".to_string());
        // let use_typescript = Inquirer::confirm("Do you want to use typescript?".to_string());

        //return the ThunderStorm object
        ThunderStorm {
            lang,
            path,
            application_name,
            dirs,
            package_manager,
            use_env,
        }
    }
}

//a function that inquires the user for the language to use
struct Inquirer {}
impl Inquirer {
    fn prompt(query: String) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer: String = Input::with_theme(&theme)
            .with_prompt(&query)
            .interact()
            .unwrap();
        answer
    }

    fn confirm(query: String) -> bool {
        let answer: bool = Confirmation::new().with_text(&query).interact().unwrap();
        answer
    }

    fn select(query: String, options: Vec<String>) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer = Select::with_theme(&theme)
            .with_prompt(&query)
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        answer.to_string()
    }
}
