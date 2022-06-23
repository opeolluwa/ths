// mod dirs
extern crate dialoguer;
use dialoguer::{theme::CustomPromptCharacterTheme, Input};

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
    pub name: String,
    pub lang: String,
    pub path: String,
    pub dirs: Vec<String>,
}

impl ThunderStorm {
    pub fn new(lang: String, path: String) -> ThunderStorm {
        //compute the application name
        let theme = CustomPromptCharacterTheme::new('>');
        let application_name: String = Input::with_theme(&theme)
            .with_prompt("Provide the name of the application ")
            .interact()
            .unwrap();

        let directory = dirs::create();
        ThunderStorm {
            lang: lang,
            path: path,
            name: application_name,
            dirs: directory,
        }
    }
}
