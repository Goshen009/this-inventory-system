





// use crate::scriptable_objects::ScriptableObjectGeneral;
use crate::scriptable_objects::inventory_item::{self, FoodItems, InventoryItem};
use crate::scriptable_objects::player_stats::PlayerStats;

#[test]
fn test_to_json() {
    
}

// #[test]
// fn test_writing_to_file() {
//     let inventory = InventoryItem::read_from_file(&InventoryItem::default(), INVENTORY_FILEPATH);

//     assert_eq!(r#"{"name":"Aviona"}"#, inventory.to_json());
// }

// #[test]
// fn test_conversion_on_inventory_item_struct_serialize() {
//     //  let inventory = InventoryItem {
//     //     name: "Ade".to_string(),
//     // };

//     // assert_eq!(r#"{"name":"Ade"}"#, inventory.to_json());
// }

// #[test]
// fn test_conversion_on_inventory_item_struct_deserialize() {
//     // let json = r#"{"name":"Ade"}"#;
//     // let mut inventory = InventoryItem::new();
//     // inventory = inventory.to_struct(json.to_string());

//     // let inventory_struct_to_string = format!("{:?}", inventory);

//     // assert_eq!(r#"InventoryItem { name: "Ade" }"#,
//     //     inventory_struct_to_string);
// }


// #[test]
// fn test_conversion_on_founder_item_struct_serialize() {
//     let founder = FounderItem {
//         bag : true,
//         keep: 34.34
//     };

//     assert_eq!(r#"{"bag":true,"keep":34.34}"#, founder.to_json());
// }

// #[test]
// fn test_conversion_on_founder_item_struct_deserialize() {
//     let json = r#"{"bag":true,"keep":34.34}"#;
//     let mut founder = FounderItem::default();
//     founder = founder.to_struct(json.to_string());

//     let founder_struct_to_string = format!("{:?}", founder);

//     assert_eq!(r#"FounderItem { bag: true, keep: 34.34 }"#,
//         founder_struct_to_string);
// }