//THIS MODULE WILL TAKE THE THUNDER STORM OBJECT AND BUILD THE PROJECT DIRECTORY USING THE GIVEN CONFIG
//-----------------------------------------------------------------------------------------------------

//import "init.rs" module here, essentially the Thunderstorm Struct
use crate::ThunderStorm;
use std::env;
use std::fs;
use std::io::Write;
// use std::path::PathBuf;

#[derive(Debug)]
pub struct Application {}
impl Application {
    //the scaffold method accepts the thunderstorm instance to create the project directory in the given path
    pub fn new(app: ThunderStorm) {
        //create th root directory
        //loop through the thunderstorm instance and create the required folders in the project directory
        fs::create_dir(app.path.clone()).unwrap();
        fs::create_dir(app.path.clone() + "/src").unwrap();
        /* for folder in app.dirs.clone() {
            fs::create_dir(format!("{}/{}", source_dir.clone(), folder)).unwrap();
        } */

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
            let readme_content = r#"
            {

                "name": fields.directory,
                "version": fields.version,
                "description": fields.description,
                "scripts": {
                    "start": node src/index.ts,
                    "dev": `nodemon src/index.ts`,
                    "make":"bash install.sh"
                },
                "dependencies": {
                   
                },
                "devDependencies": {
                    
                },
                "repository": {
                    "type": "git",
                    "url": ""
                },
                "keywords": [
                ],
                "author": "",
                "license": "",
                "bugs": {
                    "url": ""
                },
                "homepage": ""
            }
        }"#;
            fs::write(readme, readme_content).unwrap();
        }
        //create the env.example file in the project directory
        /* if app.use_env {
            let env_example = format!("{}/env.example", app.path.clone());
            let content = std::fs::read_to_string("./meta/env.example")
            .expect("could not read file");
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
