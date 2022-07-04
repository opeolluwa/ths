//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm Struct
use crate::ThunderStorm;

#[derive(Debug)]
pub struct Application {}

impl Application {
    //the new methods accepts the thunderstorm instance
    pub fn new(application: ThunderStorm) {
        println!("{:?}", application);
        // fs::create_dir(path.clone());
        // println!(" path {}", path);
    }
}
