//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm Struct
use crate::ThunderStorm;
use std::fs;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Application {}
impl Application {
    //the scaffold method accepts the thunderstorm instance to create the project directory in the given path
    pub fn new(application: ThunderStorm) {
        println!("{:?}", application);
        fs::create_dir(application.path.clone()).unwrap(); //create th root directory
        //loop through the thunderstorm instance and create the required folders in the project directory
        for folder in application.dirs.clone() {
            fs::create_dir(format!("{}/{}", application.path, folder)).unwrap();
        }
    }

    //the scaffold method accepts the thunderstorm instance to create the project directory in the PWD
    pub fn new_pwd(application: ThunderStorm) {
        let path = env::current_dir().unwrap().to_str().unwrap().to_string();
        println!("{:?}", application);
        fs::create_dir(path.clone()).unwrap();

        // fs::create_dir(application.path.clone());
        // println!(" path {}", path);
    }
}
