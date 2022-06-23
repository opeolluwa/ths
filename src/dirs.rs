extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

//prompt user for folders to create, return array of folder names
pub fun create_directories() -> Vec<String>{
  const checkboxes: &[&str; 11] = &[
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

let defaults = &[false; checkboxes.len()];
let selections = Checkboxes::with_theme(&ColorfulTheme::default())
    .with_prompt("Pick your food")
    .items(&checkboxes[..])
    .defaults(&defaults[..])
    .interact()
    .unwrap();

if selections.is_empty() {
    println!("You did not select anything :(");
    return;
} 
//return the array of folder names
selections.iter().map(|i| checkboxes[*i].to_string()).collect()
}


/* else {
    
    println!("You selected these things:");
    for selection in selections {
        println!("  {}", checkboxes[selection]);
    }
} */