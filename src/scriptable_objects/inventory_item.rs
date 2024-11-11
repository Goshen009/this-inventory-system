




use super::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    number: f64,
    food: FoodItems,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoodItems {
    number : f32,
}

impl Default for InventoryItem {
    fn default() -> Self {
        InventoryItem {
            number: 0.0,
            food : FoodItems {
                number: 90.0
            }
        }
    }
}

impl InventoryItem {
    
}