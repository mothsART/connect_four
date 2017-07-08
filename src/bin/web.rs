#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket_contrib::MsgPack;

#[derive(Serialize, Deserialize)]
struct Message {
    cord_x     : usize,
    cord_y     : usize,
    new_cord_x : usize,
    new_cord_y : usize,
    attack     : bool
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/*
#[post("/", data = "<data>", format = "application/msgpack")]
fn deplacement(data: MsgPack<Message>) -> Result<String, ()> {
    Ok("essai".to_owned())
}
*/

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
    .mount("/static/", routes![files])
    //.mount("/deplacement", routes![deplacement])
}

fn main() {
    rocket().launch();
}
