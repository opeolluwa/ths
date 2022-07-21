//this module will take the thunder storm object and build the project directory using the given config
#[path = "./generate_readme.rs"]
mod generate_readme;
use crate::globals::setup::ThunderStorm;
use generate_readme as readme;
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub struct Application {}
impl Application {
    ///the scaffold method accepts the thunderstorm instance to create the project directory in the given path
    /// #Example
    /// ```bash
    /// ths create --path ./app
    /// ths create --path ./../app
    /// ths create --path ./some_dir/app
    /// ```
    pub fn new(app: ThunderStorm) {
        fs::create_dir_all(app.path.clone()).unwrap(); //create the root directory
        fs::create_dir(app.path.clone() + "/src").unwrap(); //create the src directory in root directory
        for folder in app.dirs.clone() {
            //loop through the directories to create in the src directory
            fs::create_dir(format!("{}/src/{}", app.path.clone(), folder)).unwrap();
        }

        //loop through the thunderstorm instance and create the required files in the project directory
        //if typescript is an option
        if app.lang == "typescript" || app.use_typescript {
            fetch_template(
                app.path.clone(),
                "tsconfig.json".to_string(),
                include_str!("./../../resources/tsconfig.json").to_string(),
            );
        }

        //if readme is an option
        if app.include_readme {
            fetch_template(
                app.path.clone(),
                "README.md".to_string(),
                readme::generate_content(
                    app.application_name.to_string(),
                    app.application_description.to_string(),
                ),
            );
        }

        //if gitignore is an option
        if app.use_git {
            fetch_template(
                app.path.clone(),
                ".gitignore".to_string(),
                include_str!("./../../resources/.gitignore").to_string(),
            );
        }

        //if env is an option
        if app.use_env {
            fetch_template(app.path.clone(), ".env".to_string(), "".to_string());

            //if env is an option, create a .env.example file
            fetch_template(
                app.path.clone(),
                ".env.example".to_string(),
                include_str!("./../../resources/.env.example").to_string(),
            );
        }

        //if include package.json is an option
        if app.include_package_json {
            fetch_template(
                app.path.clone(),
                "package.json".to_string(),
                include_str!("./../../resources/package.json").to_string(),
            );
        }
    }

    ///the scaffold method accepts the thunderstorm instance to create the project directory in the PWD
    /// creating only the missing directories and files
    /// #Example
    /// ```bash
    /// ths init
    /// ```
    pub fn new_pwd(app: ThunderStorm) {
        fs::create_dir(app.path.clone() + "/src").unwrap(); //create the src directory in root directory
        for folder in app.dirs.clone() {
            //loop through the directories to create in the src directory
            fs::create_dir(format!("{}/src/{}", app.path.clone(), folder)).unwrap();
        }

        //loop through the thunderstorm instance and create the required files in the project directory
        //if typescript is an option
        if app.lang == "typescript" || app.use_typescript {
            fetch_template(
                app.path.clone(),
                "tsconfig.json".to_string(),
                include_str!("./../../resources/tsconfig.json").to_string(),
            );
        }

        //if readme is an option
        if app.include_readme {
            fetch_template(
                app.path.clone(),
                "README.md".to_string(),
                readme::generate_content(
                    app.application_name.to_string(),
                    app.application_description.to_string(),
                ),
            );
        }

        //if gitignore is an option
        if app.use_git {
            fetch_template(
                app.path.clone(),
                ".gitignore".to_string(),
                include_str!("./../../resources/.gitignore").to_string(),
            );
        }

        //if env is an option
        if app.use_env {
            fetch_template(app.path.clone(), ".env".to_string(), "".to_string());

            //if env is an option, create a .env.example file
            fetch_template(
                app.path.clone(),
                ".env.example".to_string(),
                include_str!("./../../resources/.env.example").to_string(),
            );
        }

        //if include package.json is an option
        if app.include_package_json {
            fetch_template(
                app.path.clone(),
                "package.json".to_string(),
                include_str!("./../../resources/package.json").to_string(),
            );
        }
    }
}

/// This function reads the contents of a file and writes it to another (a new) file.
/// @param: base_path: String, the root directory of the application
/// @param: file_name: String, the  name of the file to create
/// @param: path: String, the path to the directory to copy the file content from
/// @return: String, the contents of the file
fn fetch_template(base_path: String, file_name: String, content: String) {
    // content_source.read_to_string(&mut content).unwrap();
    //write the contents to the  new file
    let file_path = format!("{}/{}", base_path, file_name);
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
