use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}


//CHECK IF DIRECTOTY ALREADY ESISTS
use std::path::Path;

fn main() {
    println!("{}", Path::new("/etc/hosts").exists());
}
