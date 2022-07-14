///  a module for creating the application directories
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
