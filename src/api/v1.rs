
pub mod db_entity_converters;
pub mod gamerecords;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Gamerecord {
    id: String,
    player_name: Option<String>,
    start_time: String,
    initial_field: Field,
    rule: [[bool; 3]; 3],
    steps: Vec<Step>
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    width: i32,
    height: i32,
    cells: Vec<Vec<bool>>
}

#[derive(Serialize, Deserialize)]
pub struct Step {
    time: u64,
    point: (i32, i32)
}
