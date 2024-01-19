mod basic_auth;
mod schema;
mod models;

#[macro_use] extern crate rocket;

use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::database;
use crate::basic_auth::basic_auth::BasicAuth;
use crate::models::{NewRustacean, Rustacean};
use crate::schema::rustaceans;


#[database("sqlite")]
struct DbConn(SqliteConnection);
#[get("/rustaceans")]
async fn get_rustaceans(_auth : BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let rustaceans = rustaceans::table.order(rustaceans::id.desc())
            .load::<Rustacean>(c)
            .expect("Error loading posts");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) -> Value {
    db.run(move |c| {
        let rustacean = rustaceans::table.find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading post");
        json!(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn add_rustaceans(_auth : BasicAuth, db : DbConn, new_rustacean : Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result =diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error saving new rustacean");
        json!(result)
    }).await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(id : i32, _auth : BasicAuth, rustacean: Json<Rustacean>, db : DbConn) -> Value {
    db.run( move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set(
                (rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned())
                ))
            .execute(c)
            .expect("Error updating rustacean");
        json!(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id : i32, _auth : BasicAuth, db : DbConn) -> status::NoContent {
    db.run( move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean");
        status::NoContent
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