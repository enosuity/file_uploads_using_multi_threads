#[macro_use] extern crate rocket;

use diesel::prelude::*;
use diesel::dsl::Limit;
use rocket::{data::{ByteUnit, Limits, ToByteUnit}, launch, routes};
use rocket_dyn_templates::{ Template };
mod services;
use services::{post::*, file_upload::*};
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    // let limits = Limits::default()
    //                .limit("/upload", 128.mebibytes());
    rocket::build()
        .configure(rocket::config::Config {
            port: 5000,
            address: "0.0.0.0".parse().unwrap(),
            limits: Limits::new().limit("file", 512.megabytes()),
            ..Default::default()
        })
        .mount("/", routes![create_post, list, upload_file, get_file])
        .attach(Template::fairing())
}