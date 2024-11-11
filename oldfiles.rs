





use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::io::ErrorKind;

fn main() {
    let mut filex = File::open("txt").unwrap_or_else(|error| {
        println!("Failed to read {}", error);
        File::create("txt").expect("fialed")
    });

    // let file = open_or_create_file()?;

    // write_to_file(file.try_clone()?, "Line to be huh\n")?;

    // let file = open_or_create_file()?;
    // let content = read_from_file(file)?;
    // println!("File content: {}", content);

    // Ok(())

    // let file = inventory::read_inventory_data(); 
    // This will return a Struct
    two();
}

// pub mod inventory {
//     pub struct Inventory_Object {
//         username : String,
//     }

//     pub fn read_inventory_data() -> Inventory_Object {
//         // first gets file
//         let file_string = read_write::get_inventory_file();

//         // then converts the JSON to struct
//         // and returns that
//         parser::convert_json_to_struct(&file_string)
//     }

//     pub mod read_write {
//         use std::fs::{File, OpenOptions};
//         use std::io::{self, Read, Write};
//         use std::path::Path;

//         // let a = match experssion {
//         //     Ok(file) => a,
//         //     Err(error) => panic!("{error}"),
//         // };

//         // let file = File::open("path.txt").unwrap_or_else(|err| {
//         //     if error.kind() == ErrorKind::NotFound {
//         //         File::create("path").unwrap_or_else(|err| {
//         //             panic!("HEHE {:?}", err);
//         //         })
//         //     } else {
//         //         panic!("lasdj");
//         //     }
//         // });

//         pub fn get_inventory_file() -> String {
//             let file_path = "inventory.txt";

//             let file = OpenOptions::new()
//                 .read(true)
//                 .open(file_path);


            


//             // check if file exists
//             let is_file_existing = Path::new(file_path).exists();
//             let mut file_content = String::new();

//             if is_file_existing {

//             } else {
                
//             }
            
            

//             String::from("")
//         }

//         fn open_file(file_name: &str) -> io::Result<File> {
//             let file = OpenOptions::new()
//                 .read(true)
//                 .open(file_name)?;

//             Ok(file)
//         }

//         fn read_file(mut file: File) -> io::Result<String> {
//             let mut content = String::new();
//             file.read_to_string(&mut content)?;

//             Ok(content)
//         }



        
//         fn open_or_create_file() -> io::Result<File> {
//             let file = OpenOptions::new()
//                 .read(true)
//                 .write(true)
//                 .create(true)
//                 .open("inventory.txt")?;
        
//             Ok(file)
//         }

//         fn write_to_file(mut file: File, content: &str) -> io::Result<()> {
//             file.write(content.as_bytes())?;
//             Ok(())

//             // use a .truncate(true) to clear out all the data in the file first
//         }
//     }

//     pub mod parser {
//         use crate::inventory;

//         pub fn convert_json_to_struct(file_string : &String) -> inventory::Inventory_Object {
//             inventory::Inventory_Object {
//                 username : String::from("my username")
//             }
//         }
//     }










//     pub fn get_data_from_inventory() {
//         // this is called on game start
//         let file_path = "inventory.txt";

//         let game_data_string = file_management::get_data_from_file(file_path);
//     }

//     pub mod file_management {
//         use std::fs::{File, OpenOptions};
//         use std::io::{self, Read, Write};
//         use std::path::Path;

//         fn check_if_inventory_file_exists(file_path: &str) -> bool {
//             Path::new(file_path).exists()
//         }

//         pub fn get_data_from_file(file_path: &str) -> String {
//             // this one must return a string and not a Result
//             // because all the errors must be handled in here
//             // and only the contents of the file itself
//             // should be returned

//             let mut file;

//             if check_if_inventory_file_exists(file_path) {
//                 file =  OpenOptions::new().read(true);
//             } else {
//                 file = create_new_inventory(file_path);
//             }

//             read_data_from_file()
//         }

//         fn create_new_inventory(file_path: &str) -> OpenOptions {
//             OpenOptions::new()
//                 .read(true)
//                 .create(true)
//                 .open(file_path);
//         }

//         fn read_data_from_file() -> String{
//             String::new()
//         }
//     }
// }




// let file = File::open("path.txt").unwrap_or_else(|err| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("path").unwrap_or_else(|err| {
//                 panic!("HEHE {:?}", err);
//             })
//         } else {
//             panic!("lasdj");
//         }
//     });

fn two() {
    // let file = open_or_create_file()?;

    // write_to_file(file.try_clone()?, "Line to be huh\n")?;

    // let file = open_or_create_file()?;
    // let content = read_from_file(file)?;
    // println!("File content: {}", content);

    // Ok(())

    
}

fn open_or_create_file() -> io::Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("inventory.txt")?;

    if(1 == 1) {
        return Error("failure");
    }

    Ok(file)
}

fn write_to_file(mut file: File, content: &str) -> io::Result<()> {
    file.write(content.as_bytes())?;
    Ok(())

    // use a .truncate(true) to clear out all the data in the file first
}

fn read_from_file(mut file: File) -> io::Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}