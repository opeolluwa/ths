pub struct ThunderStorm {
    name: String,
    lang: String,
    path: String,
    dirs: Vec<String>,
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
