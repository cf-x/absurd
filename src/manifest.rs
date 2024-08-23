use ::std::{fs::File, io::Read};
use toml::{from_str, Value};

pub struct Project {
    // # project
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,
    pub license_file: String,
    pub repository: String,
    pub documentation: String,
    pub readme: String,
    pub auto_update: bool,
    pub edition: String,
    // # config
    pub snippet: u8,
    pub side_effects: bool,
    pub disable_std: bool,
    pub load_std: bool,
    pub disable_bases: bool,
    pub disable_analyzer: bool,
    // # modules
    #[allow(dead_code)]
    pub modules: Vec<(String, String)>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            // # project
            name: "project_name".to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            authors: Vec::new(),
            license: "MIT".to_string(),
            license_file: String::new(),
            repository: String::new(),
            documentation: String::new(),
            readme: String::new(),
            auto_update: false,
            edition: "beta".to_string(),
            // # config
            snippet: 1,
            side_effects: true,
            disable_std: false,
            load_std: true,
            disable_bases: false,
            disable_analyzer: true,
            // # modules
            modules: Vec::new(),
        }
    }

    pub fn load(&mut self) {
        let file = File::open("project.toml");
        if !file.is_err() {
            let mut contents = String::new();
            file.unwrap()
                .read_to_string(&mut contents)
                .expect("@error failed to read manifest");
            let parsed: Value = from_str(&contents).expect("@error failed to parse manifest");
            match parsed.as_table() {
                Some(v) => {
                    if v.contains_key("project") {
                        let table = v.get("project").unwrap();
                        if table.get("name").is_some() {
                            self.name = self.get_str(table, "name");
                        }
                        if table.get("version").is_some() {
                            self.version = self.get_str(table, "version");
                        }
                        if table.get("description").is_some() {
                            self.description = self.get_str(table, "description");
                        }
                        if table.get("authors").is_some() {
                            let authors = self.get_arr(table, "authors");
                            for author in authors {
                                self.authors.push(author.as_str().unwrap().to_string());
                            }
                        }
                        if table.get("license").is_some() {
                            self.license = self.get_str(table, "license");
                        }
                        if table.get("license_file").is_some() {
                            self.license_file = self.get_str(table, "license_file");
                        }
                        if table.get("repository").is_some() {
                            self.repository = self.get_str(table, "repository");
                        }
                        if table.get("documentation").is_some() {
                            self.documentation = self.get_str(table, "documentation");
                        }
                        if table.get("readme").is_some() {
                            self.readme = self.get_str(table, "readme");
                        }
                        if table.get("auto_update").is_some() {
                            self.auto_update = self.get_bool(table, "auto_update");
                        }
                        if table.get("edition").is_some() {
                            self.edition = self.get_str(table, "edition");
                        }
                    }
                    if v.contains_key("config") {
                        let table = v.get("config").unwrap();
                        if table.get("snippet").is_some() {
                            self.snippet = self.get_int(table, "snippet");
                        }
                        if table.get("side_effects").is_some() {
                            self.side_effects = self.get_bool(table, "side_effects");
                        }
                        if table.get("disable_std").is_some() {
                            self.disable_std = self.get_bool(table, "disable_std");
                        }
                        if table.get("load_std").is_some() {
                            self.load_std = self.get_bool(table, "load_std");
                        }
                        if table.get("disable_bases").is_some() {
                            self.disable_bases = self.get_bool(table, "disable_bases");
                        }
                        if table.get("disable_analyzer").is_some() {
                            self.disable_analyzer = self.get_bool(table, "disable_analyzer");
                        }
                    }
                }
                None => {
                    panic!("@error failed to parse manifest")
                }
            }
        }
    }

    fn get_str(&self, table: &Value, name: &str) -> String {
        table.get(name).unwrap().as_str().unwrap().to_string()
    }

    fn get_arr(&self, table: &Value, name: &str) -> Vec<Value> {
        table.get(name).unwrap().as_array().unwrap().clone()
    }

    fn get_bool(&self, table: &Value, name: &str) -> bool {
        table.get(name).unwrap().as_bool().unwrap()
    }

    fn get_int(&self, table: &Value, name: &str) -> u8 {
        table.get(name).unwrap().as_integer().unwrap() as u8
    }
}
