





pub mod save_and_load {
    use std::path::Path;
    use std::fs::OpenOptions;
    use std::io::{Read, Write};

    use serde::{Serialize, Deserialize};
    use serde_json::Error;

    pub fn to_json<T: Serialize> (obj: &T) -> String {
        serde_json::to_string(obj).expect("converting to json failed")
    }
            
    pub fn to_struct<T: for<'de> Deserialize <'de> + Default> (json : String) -> Result<T, Error> {
        let result = serde_json::from_str(json.as_str())?;
        Ok(result)
    }

    pub fn save_to_file<T: Serialize> (obj: &T, file_path : &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("error opening file to write to");

        let json = to_json(&obj);
        file.write_all(json.as_bytes()).expect("error writing to file");
    }

    pub fn read_from_file<T : for<'de> Deserialize <'de> + Default> (file_path : &str) -> Result<T, Box<dyn std::error::Error>> {
        if  Path::new(file_path).exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(file_path)
                .expect("error opening file to read to");

            let mut json = String::new();
            file.read_to_string(&mut json).expect("error reading from file");

            match to_struct(json) {
                Ok(value) => Ok(value),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            Ok(T::default())
        }
    }
}