//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm Struct
use crate::ThunderStorm;
use std::env;
use std::fs;
use std::io::Write;
use std::io::Read;

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
            //create tsconfig.json
            let tsconfig = format!("{}/tsconfig.json", app.path.clone());
            let tsconfig_content = r#"{
                "compilerOptions": {
                    "target": "es5",
                    "module": "commonjs",
                    "moduleResolution": "node",
                    "sourceMap": true,
                    "declaration": true,
                    "emitDecoratorMetadata": true,
                    "experimentalDecorators": true,
                    "removeComments": false,
                    "noImplicitAny": false,
                    "strictNullChecks": true,
                    "outDir": "./dist",
                    "rootDir": "./src",
                    "baseUrl": "./src",
                    "paths": {
                        "*": [
                            "node_modules/*"
                        ]
                    }
                },
                "include": [
                    "src/**/*"
                ]
            }"#;
            fs::write(tsconfig, tsconfig_content).unwrap();
        }

        //if create readme
        if app.include_readme {
            let readme = format!("{}/README.md", app.path.clone());
            let readme_content = r#"# Project TitleSimple overview of use/purpose.
            
            ## Description
            
            An in-depth paragraph about your project and overview of use.
            
            ## Getting Started
            
            ### Dependencies
            
            - Describe any prerequisites, libraries, OS version, etc., needed before installing program.
            - ex. Windows 10
            
            ### Installing
            
            - How/where to download your program
            - Any modifications needed to be made to files/folders
            
            ### Executing program
            
            - How to run the program
            - Step-by-step bullets
            
            ```
            code blocks for commands
            ```
            
            ## Help
            
            Any advise for common problems or issues.
            
            ```
            command to run if program contains helper info
            ```
            
            ## Authors
            
            Contributors names and contact info
            
            ex. Dominique Pizzie  
            ex. [@DomPizzie](https://twitter.com/dompizzie)
            
            ## Version History
            
            - 0.2
              - Various bug fixes and optimizations
              - See [commit change]() or See [release history]()
            - 0.1
              - Initial Release
            
            ## License
            
            This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details
            
            ## Acknowledgments
            
            Inspiration, code snippets, etc.
            
            - [awesome-readme](https://github.com/matiassingers/awesome-readme)
            - [PurpleBooth](https://gist.github.com/PurpleBooth/109311bb0361f32d87a2)
            - [dbader](https://github.com/dbader/readme-template)
            - [zenorocha](https://gist.github.com/zenorocha/4526327)
            - [fvcproductions](https://gist.github.com/fvcproductions/1bfc2d4aecb01a834b46)
            "#;
            fs::write(readme, readme_content).unwrap();
        }
        //create the env.example file in the project directory
       /*  if app.use_env {
            let env_example = format!("{}/env.example", app.path.clone());
            let content =
                std::fs::read_to_string("./meta/env.example").expect("could not read file");
            fs::write(env_example, content).unwrap();
        } */
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

fn create_file(file_name: String, base_path: String, content: Option<String>) -> std::fs::File {
    let mut file = fs::File::create(file_name).unwrap();
    if let Some(c) = content {
        file.write_all(c.as_bytes()).unwrap();
    }
    file
}

fn read_file(file_name: String) -> String {
    let mut file = fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
