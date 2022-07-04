//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm
// mod init;
// use init as Init;
// use Init::ThunderStorm;
use crate::ThunderStorm;
#[derive(Debug)]
pub struct Application {}

impl Application {
    //the new methods accepts the thunderstorm instance
   pub fn new(application: ThunderStorm) {
        println!("{:?}", application);
    }
}
