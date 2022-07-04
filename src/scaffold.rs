//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm Struct
use crate::ThunderStorm;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

// use std::path::PathBuf;

#[derive(Debug)]
pub struct Application {}
impl Application {
    //the scaffold method accepts the thunderstorm instance to create the project directory in the given path
    pub fn new(app: ThunderStorm) {
        //create th root directory
        //loop through the thunderstorm instance and create the required folders in the project directory
        fs::create_dir(app.path.clone()).unwrap(); //create the root directory
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
                "resources/tsconfig.json".to_string(),
            );
        }
    }

    //the scaffold method accepts the thunderstorm instance to create the project directory in the PWD
    pub fn new_pwd(app: ThunderStorm) {
        let path = env::current_dir().unwrap().to_str().unwrap().to_string();
        println!("{:?}", app);
        fs::create_dir(path.clone()).unwrap();

        // fs::create_dir(app.path.clone());
        // println!(" path {}", path);
    }
}

/**
 * This function reads the contents of a file and writes it to another (a new) file.
 * @param: base_path: String, the root directory of the application
 * @param: file_name: String, the  name of the file to create
 * @param: path: String, the path to the directory to copy the file content from
 * @return: String, the contents of the file
 */
fn fetch_template(base_path: String, file_name: String, content_path: String) /* -> std::fs::File */
{
    //open the file to read the contents
    let mut content_source = fs::File::open(content_path).unwrap();
    let mut content = String::new();
    //red the file contents into a string
    content_source.read_to_string(&mut content).unwrap();
    //write the contents to the  new file
    let file_path = format!("{}/{}", base_path, file_name);
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[warn(dead_code)]
fn read_file(file_name: String) -> String {
    let mut file = fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
