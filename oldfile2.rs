






use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
// use std::io::ErrorKind;

// OpenOptions::new().read(true).open(file_path).unwrap()

fn main() {
    let file_name = "";

    // code to load inventory file

    // create_file(&file_name);

    // convert_to_json();
    convert_to_struct();
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct InventoryItem {
    name: String,
    quantiy: u32,
    price: f64
}

fn convert_to_json() -> Result<(), Box<dyn std::error::Error>> {
    let item = InventoryItem {
        name: "Apple".to_string(),
        quantiy: 50,
        price: 0.99
    };

    let json = serde_json::to_string(&item)?;
    write_to_file("newPath.txt", &json);
    // println!("Serialized JSON: {}", json);

    Ok(())
}

fn convert_to_struct() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"name":"Apple","quantiy":50,"price":0.99}"#;

    let mut file = read_file("oop");
    let mut json = String::new();
    
    file.read_to_string(&mut json);

    let item : InventoryItem = serde_json::from_str(&json)?;
    println!("Deserialized Struct: {:?}", item);

    Ok(())
}

fn read_file(file_path : &str) -> File {
    // if check_if_file_exists(file_path) {
    //     File::open(file_path).expect("There's an error here!")
    // } else {
    //     write_new_inventory_file(file_path)
    // }

    File::open("inventory.txt").unwrap()
}

fn write_to_file(file_path : &str, contents:&String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("inventory.txt")
        .unwrap();

    file.write(contents.as_bytes());
}

fn write_new_inventory_file(file_path : &str) -> File {
    todo!("over here");
}

fn create_file(file_path : &str) -> File {
    File::create(file_path).expect("There's a problem here!")
}

fn check_if_file_exists(file_path : &str) -> bool {
    Path::new(file_path).exists()
}