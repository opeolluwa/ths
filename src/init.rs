pub struct ThunderStorm {
    pub name: String,
    pub lang: String,
    pub path: String,
    pub dirs: Vec<String>,
}

impl ThunderStorm {
    pub fn new(name: String, lang: String, path: String, dirs: Vec<String>) -> ThunderStorm {
        ThunderStorm {
            name: name,
            lang: lang,
            path: path,
            dirs: dirs,
        }
    }
}
