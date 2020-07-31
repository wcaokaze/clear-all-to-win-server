
use diesel::{Queryable, Insertable};
use crate::schema::gamerecords;

#[derive(Queryable)]
pub struct Gamerecord {
    pub id: i64,
    pub player_name: Option<String>,
    pub start_time: String,
    pub initial_field_width: i16,
    pub initial_field_height: i16,
    pub initial_field: Vec<bool>,
    pub rule: Vec<bool>,
    pub steps: Vec<String>
}

#[derive(Insertable)]
#[table_name = "gamerecords"]
pub struct NewGamerecord {
    pub player_name: Option<String>,
    pub start_time: String,
    pub initial_field_width: i16,
    pub initial_field_height: i16,
    pub initial_field: Vec<bool>,
    pub rule: Vec<bool>,
    pub steps: Vec<String>
}
