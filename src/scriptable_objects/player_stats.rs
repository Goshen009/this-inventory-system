










use super::{Serialize, Deserialize};

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