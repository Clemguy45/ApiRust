mod basic_auth;
mod schema;

#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket_sync_db_pools::database;
use crate::basic_auth::basic_auth::BasicAuth;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);
#[get("/rustaceans")]
fn get_rustaceans(_auth : BasicAuth, _db: DbConn) -> Value {
    return json!([{"id" : 1, "name" : "John Doe"}, {"id" : 2, "name" : "Toto Ro"}]);

}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id : i32, _auth : BasicAuth) -> Value {
    return json!([{"id" : id, "name" : "John Doe", "email": "john.doe@hotmali.fr"}]);
}

#[post("/rustaceans", format = "json")]
fn add_rustaceans(_auth : BasicAuth) -> Value {
    return json!({"id" : 3, "name" : "Toto Ro", "email": "toto.ro@hotmali.fr"});
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id : i32, _auth : BasicAuth) -> Value {
    return json!({"id" : id, "name" : "Toto Ro", "email": "toto.ro@hotmali.fr"});
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id : i32, _auth : BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() ->Value {
    json!("Not found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Non autorisé")
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
            unauthorized
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}