#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    return json!([{"id" : 1, "name" : "John Doe"}, {"id" : 2, "name" : "Toto Ro"}]);

}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id : i32) -> Value {
    return json!([{"id" : id, "name" : "John Doe", "email": "john.doe@hotmali.fr"}]);
}

#[post("/rustaceans", format = "json")]
fn add_rustaceans() -> Value {
    return json!({"id" : 3, "name" : "Toto Ro", "email": "toto.ro@hotmali.fr"});
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id : i32) -> Value {
    return json!({"id" : id, "name" : "Toto Ro", "email": "toto.ro@hotmali.fr"});
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id : i32) -> status::NoContent {
    status::NoContent
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
        .launch()
        .await;
}