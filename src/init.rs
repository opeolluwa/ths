//load in the inquirer library
#[path = "inquirer.rs"]
mod inquirer;
use inquirer as Commander;

#[path = "dirs.rs"]
mod dirs;
use dirs as Dirs;

#[derive(Debug)]
pub struct ThunderStorm {
    pub application_name: String,
    pub lang: String,
    pub path: String,
    pub dirs: Vec<String>,
    pub package_manager: String,
    pub use_env: bool,
    pub use_git: bool,
    pub use_typescript: bool,
    pub test_suit: String,
    pub include_readme: bool,
    pub include_license: bool,
    pub include_changelog: bool,
    pub include_package_json: bool,
    pub include_gitignore: bool,
    pub include_tsconfig: bool,
    pub include_tslint: bool,
    pub include_eslint: bool,
    pub include_prettier: bool,
}

impl ThunderStorm {
    pub fn new(lang: String, path: String) -> ThunderStorm {
        //compute the application name
        let application_name = Commander::Prompt::new("Application name ".to_string());
        let dirs = Dirs::create(); //compute the directories to create

        //inquire the package manager, use of env and typescript
        let known_package_managers: Vec<String> = vec!["npm".to_string(), "yarn".to_string(), "pnpm".to_string()];
        let package_manager = Commander::Select::new(
            "Which package manager do you use?".to_string(),
            known_package_managers,
        );

        //inquire the user configuration
        let use_env =
            Commander::Confirm::new("Do you want to use the environment variables?".to_string());
        let use_git =
            Commander::Confirm::new("Do you want to initialize as a git repository?".to_string());
        let use_typescript = Commander::Confirm::new("Do you want to use typescript?".to_string());
        let query_test_suit = Commander::Confirm::new("Do you want to setup test suit".to_string());
        let mut test_suit = "".to_string();
        if query_test_suit {
            test_suit = Commander::Prompt::new(
                "Input a space delimited names of testing packages to use".to_string(),
            );
        }
        let include_readme = Commander::Confirm::new("Do initialize project with a README?".to_string());
        let include_license =
            Commander::Confirm::new("Do initialize project with LICENSE?".to_string());
        let include_changelog =
            Commander::Confirm::new("Do initialize project with CHANGELOG?".to_string());
        let include_package_json =
            Commander::Confirm::new("Do initialize project with package.json ?".to_string());
        let include_gitignore =
            Commander::Confirm::new("Do initialize project with gitignore?".to_string());
        let include_tsconfig =
            Commander::Confirm::new("Do initialize project with tsconfig?".to_string());
        let include_tslint =
            Commander::Confirm::new("Do initialize project with tslint?".to_string());
        let include_eslint =
            Commander::Confirm::new("Do initialize project with eslint?".to_string());
        let include_prettier =
            Commander::Confirm::new("Do initialize project with prettier?".to_string());

        //return the ThunderStorm object
        ThunderStorm {
            dirs,
            lang,
            path,
            application_name,
            package_manager,
            use_env,
            use_git,
            use_typescript,
            test_suit,
            include_readme,
            include_license,
            include_changelog,
            include_package_json,
            include_gitignore,
            include_tsconfig,
            include_tslint,
            include_eslint,
            include_prettier,
        }
    }
}
