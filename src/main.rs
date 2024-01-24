mod basic_auth;
mod schema;
mod models;
mod repositories;

#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::Error::Custom;
use crate::basic_auth::basic_auth::BasicAuth;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;


#[database("sqlite")]
struct DbConn(SqliteConnection);
#[get("/rustaceans")]
async fn get_rustaceans(_auth : BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::all(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError))
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn add_rustaceans(_auth : BasicAuth, db : DbConn, new_rustacean : Json<NewRustacean>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError))
    }).await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(id : i32, _auth : BasicAuth, rustacean: Json<Rustacean>, db : DbConn) -> Result<Value, Custom<Value>> {
    db.run( move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err|
                match err {
                    Status::NoteFound => Custom(Status::NotFound),
                    _ => Custom(Status::InternalServerError)
                }
                )
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run( move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|err| Custom(Status::InternalServerError))
    }).await
}

#[catch(404)]
fn not_found() ->Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Non autorisé")
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity")
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            add_rustaceans,
            update_rustaceans,
            delete_rustaceans
        ])
        .register("/", catchers![
            not_found,
            unauthorized,
            unprocessable_entity
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}