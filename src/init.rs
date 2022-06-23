//load in the inquirer library
#[path = "inquirer.rs"]
mod inquirer;
use inquirer as Commander;
use Commander::Inquirer;



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
