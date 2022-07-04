
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
        }