




use minigame::save_and_load::*;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

pub mod inventory_item {
    use super::*;
    use std::collections::HashMap;

    static INVENTORY_FILEPATH: &str = "inventory.txt";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InventoryItem {
        items: HashMap<String, u8>
    }
    
    impl Default for InventoryItem
    {
        fn default() -> Self {
            let default_map =  HashMap::from([
                ("Iron".to_string(), 0),
                ("Wood".to_string(), 0),
                ("Fish".to_string(), 0),
            ]);

            InventoryItem {
                items: default_map,
            }
        }
    }
    
    impl InventoryItem {
        
    }

    pub fn test_function() {
        let result = read_from_file::<InventoryItem>(INVENTORY_FILEPATH);
        let mut second = match result {
            Ok(value) => value,
            Err(_e) => {
                println!("An error when reading from file! Turning to default");
                InventoryItem::default()
            },
        };
        
        second.items.entry("Fish".to_string()).and_modify(|i| *i += 10);
        println!("Deserialized hashmap is: {:?}", second);
    }
}

pub mod player_stats {
    use super::*;
    // static PLAYERSTATS_FILEPATH: &str ="playerstats.txt";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayerStats {
        name: String,
    }
    
    impl Default for PlayerStats {
        fn default() -> Self {
            PlayerStats {
                name : String::new(),
            }
        }
    }
    
    impl PlayerStats {
        
    }
}


// #[cfg(FALSE)]
// pub mod test_item {
//     use super::*;
//     static TEST_FILEPATH: &str ="test.txt";

//     #[derive(Serialize, Deserialize, Debug)]
//     pub struct TestItem {
//         name: String
//     }

//     impl Default for TestItem {
//         fn default() -> Self {
//             TestItem {
//                 name: "Favie".to_string()
//             }
//         }
//     }

//     impl TestItem {
        
//     }

//     fn test_codes() {
//         let new_item = TestItem::default();

//         // for serialization - private function
//         let json = to_json(&new_item);
//         println!("Serialized struct is: {}", json);

//         // for deserialzation - private function
//         let mut second_item : TestItem = to_struct(json);
//         println!("Deserialized struct is: {:#?}", second_item);

//         // for saving - public function
//         save_to_file(&second_item, TEST_FILEPATH);

//         // for reading - public function
//         let result = read_from_file::<TestItem>(TEST_FILEPATH);
//         let second_item = match result {
//             Ok(value) => value,
//             Err(_e) => {
//                 println!("An error when reading from file! Turning to default");
//                 TestItem::default()
//             },
//         };
//         println!("Deserialized struct is: {:#?}", second_item);

//     }
// }