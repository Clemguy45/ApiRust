mod basic_auth;
mod schema;
mod models;
mod repositories;

#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use crate::basic_auth::basic_auth::BasicAuth;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use diesel::result::Error::{NotFound};
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};


#[database("sqlite")]
struct DbConn(SqliteConnection);
#[get("/rustaceans")]
async fn get_rustaceans(_auth : BasicAuth, db: DbConn) -> Result<Value,Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::all(c, 1000)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) -> Result<Value,Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e|
                match e {
                    NotFound => Custom(Status::NotFound, json!(e.to_string())),
                    _ => Custom(Status::InternalServerError, json!(e.to_string()))
                }
            )
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn add_rustaceans(_auth : BasicAuth, db : DbConn, new_rustacean : Json<NewRustacean>) -> Result<Value,Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(id : i32, _auth : BasicAuth, rustacean: Json<Rustacean>, db : DbConn) ->  Result<Value,Custom<Value>> {
    db.run( move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e|
                match e {
                    NotFound => Custom(Status::NotFound, json!(e.to_string())),
                    _ => Custom(Status::InternalServerError, json!(e.to_string()))
                }
            )
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) ->  Result<status::NoContent,Custom<Value>> {
    db.run( move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e|                 match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string()))
            })
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

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build>{
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve database connection")
        .run(|c|{
            c.run_pending_migrations(MIGRATIONS)
                .expect("Unable to run migrations");
        }).await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel Migrations", run_db_migrations))
        .launch()
        .await;
}