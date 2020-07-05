
use diesel::{RunQueryDsl, PgConnection, Connection, ConnectionResult};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::{get, post};
use rocket::response::Responder;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::env;
use super::{Field, Gamerecord, Step};
use super::db_entity_converters::{FromDb, IntoDb};
use crate::models as db_models;
use crate::schema::gamerecords;

#[derive(Deserialize)]
pub struct NewGamerecord {
    player_name: Option<String>,
    start_time: String,
    initial_field: Field,
    rule: [[bool; 3]; 3],
    steps: Vec<Step>
}

#[derive(Responder)]
pub enum Response {
    Ok(Json<Gamerecord>),
    BadRequest(BadRequest<String>),
    NotFound(Status),
    InternalServerError(Status),
}

impl std::ops::Try for Response {
    type Ok = Json<Gamerecord>;
    type Error = Response;

    fn into_result(self) -> Result<Json<Gamerecord>, Response> {
        match self {
            Response::Ok(json) => Ok(json),
            response => Err(response)
        }
    }

    fn from_ok(v: Json<Gamerecord>) -> Response {
        Response::Ok(v)
    }

    fn from_error(response: Response) -> Response {
        response
    }
}

impl Response {
    fn ok(gamerecord: Gamerecord) -> Response {
        Response::Ok(Json(gamerecord))
    }

    fn bad_request(reason: String) -> Response {
        Response::BadRequest(BadRequest(Some(reason)))
    }

    fn not_found() -> Response {
        Response::NotFound(Status::NotFound)
    }

    fn internal_server_error() -> Response {
        Response::InternalServerError(Status::InternalServerError)
    }
}

impl IntoDb<db_models::NewGamerecord> for NewGamerecord {
    type Err = String;

    fn into_db(self) -> Result<db_models::NewGamerecord, String> {
        let NewGamerecord {
            player_name,
            start_time,
            initial_field: Field {
                width: initial_field_width,
                height: initial_field_height,
                cells: initial_field_cells
            },
            rule,
            steps
        } = self;

        if initial_field_width <= 0 || initial_field_width >= i16::MAX as i32 {
            return Err("invalid initial field size".to_string());
        }

        if initial_field_height <= 0 || initial_field_height >= i16::MAX as i32 {
            return Err("invalid initial field size".to_string());
        }

        let db_entity = db_models::NewGamerecord {
            player_name,
            start_time,
            initial_field_width:  initial_field_width  as i16,
            initial_field_height: initial_field_height as i16,
            initial_field: initial_field_cells.into_iter().flatten().collect(),
            rule: rule.iter().flatten().map(|&b| b).collect(),
            steps: steps.into_iter().map(|s| s.into_db().unwrap()).collect()
        };

        Ok(db_entity)
    }
}

fn connect_database() -> ConnectionResult<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

#[post("/gamerecords", data = "<gamerecord>")]
pub fn save_gamerecord(gamerecord: Json<NewGamerecord>) -> Response {
    let new_gamerecord = gamerecord.0.into_db()
        .map_err(|message| Response::bad_request(message))?;

    let connection = connect_database()
        .map_err(|_| Response::internal_server_error())?;

    let result = diesel::insert_into(gamerecords::table)
        .values(new_gamerecord)
        .get_result::<db_models::Gamerecord>(&connection)
        .map_err(|_| Response::internal_server_error())?;

    let gamerecord = Gamerecord::from_db(result)
        .map_err(|_| Response::internal_server_error())?;

    Response::ok(gamerecord)
}

#[get("/gamerecords/<gamerecord_id>")]
pub fn load_gamerecord(gamerecord_id: String) -> Response {
    use crate::schema::gamerecords::dsl::*;

    let gamerecord_id = gamerecord_id.parse::<i64>()
        .map_err(|_| Response::not_found())?;

    let connection = connect_database()
        .map_err(|_| Response::internal_server_error())?;

    let mut results: Vec<db_models::Gamerecord> = gamerecords
        .filter(id.eq(gamerecord_id))
        .limit(2)
        .load(&connection)
        .map_err(|_| Response::internal_server_error())?;

    let gamerecord = results.pop().ok_or(Response::not_found())?;

    let gamerecord = Gamerecord::from_db(gamerecord)
        .map_err(|_| Response::internal_server_error())?;

    Response::ok(gamerecord)
}
