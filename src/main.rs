#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use lib::db;
use lib::db::get_client;
use lib::model::{APN,Storage};
use rocket::State;
use rocket_contrib::json::Json;
use rocket::routes;

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}

#[get("/<id>")]
fn get_apn(id: i64, state: State<Storage>) -> Json<Option<APN>> {
    let mut db = state.database.get().unwrap();
    Json(db::get_apn(id, &mut db).ok().flatten())
}

fn rocket() -> rocket::Rocket {
    let database = get_client();
    let storage = Storage { database };
    rocket::ignite().mount(
        "/apn",
        routes![get_apn],
    ).manage(storage)
}
