
use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Gamerecord {
    id: i64,
    player_name: String,
    start_time: NaiveDateTime,
    initial_field: Vec<Vec<bool>>,
    rule: [[bool; 3]; 3],
    steps: Vec<String>
}

