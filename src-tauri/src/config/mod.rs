use serde::Deserialize;
use serde_yaml;
use std::{path::Path, fs::File, io::Read};

#[derive(Deserialize)]
struct YamlConfig {
    path: String,
}

pub struct Config {
    path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // TODO: дополнить
            path: r#"C:\"#.into()
        }
    }
}

impl Config {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

pub fn load_config(path: String) -> Config {
   let result = File::open(path.clone());

   if result.is_err() {
       println!("File not found or cannot be opened: {}", path);
       println!("Default config will be used...");

       return Config::default();
   }

   let mut file_data = String::new();
   let file_read_result = result.unwrap()
       .read_to_string(&mut file_data);

   if file_read_result.is_err() {
       println!("Cannot read config: {}", path);
       println!("Default config will be used...");

       return Config::default();
   }

   let parse_result = serde_yaml::from_str::<YamlConfig>(&file_data);

   if parse_result.is_err() {
       println!("Cannot parse config: {}", path);
       println!("Default config will be used...");

       return Config::default();
   }

   let parse_result = parse_result.unwrap();

   Config::new(parse_result.path)
}
