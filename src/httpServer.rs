mod getMyInfo;
pub mod sendMessage;

use crate::types;

use rocket::{self, get, post, routes};
use rocket::serde::{json::Json};

use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Nya!"
}

#[get("/myInfo")]
fn getMyInfoRoute() -> String {
    getMyInfo::get_my_info()
}

#[post("/sendMessage", format = "json", data = "<params>")]
fn sendMessageRoute(params: Json<types::SendMessageParams>) -> &'static str {
    thread::spawn(move || {
        sendMessage::send_message(&params.target, &params.content);
    });
    "qwq"
}

#[rocket::main]
pub async fn start_server() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![index, getMyInfoRoute, sendMessageRoute])
        .launch()
        .await;

    Ok(())
}
