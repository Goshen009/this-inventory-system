




use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};
use std::fmt::Debug;

pub mod inventory_item;
pub mod player_stats;

pub trait ScriptableObjectGeneral {
    fn to_json(&self) -> String;
    fn to_struct(&self, json : String) -> Self;
    fn save_to_file(&self, file_path : &str);
    fn read_from_file(&self, file_path : &str) -> Self;
}

impl <T> ScriptableObjectGeneral for T where T: Serialize + Debug + for<'de> Deserialize<'de> + Default{
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("converting to json failed")
    }
            
    fn to_struct(&self, json : String) -> Self {
        serde_json::from_str(json.as_str()).expect("converting to struct failed")
    }

    fn save_to_file(&self, file_path : &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("error opening file to write to");

        file.write_all(self.to_json().as_bytes()).expect("error writing to file");
    }

    fn read_from_file(&self, file_path : &str) -> Self {
        if  Path::new(file_path).exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(file_path)
                .expect("error opening file to read to");

            let mut json = String::new();
            file.read_to_string(&mut json).expect("error reading from file");

            return self.to_struct(json);
        } else {
            return T::default();
        }
    }
}