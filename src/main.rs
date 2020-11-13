#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::env;
use std::process::Command;
use std::path::Path;

use lazy_static::lazy_static;
use rocket::Data;
use uuid::Uuid;

lazy_static! {
    static ref ENCODER_PATH: String = env::var("ENCODER_PATH").unwrap();
    static ref OUTPUT_DIR: String = env::var("OUTPUT_DIR").unwrap();
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let tmp_dir = env::temp_dir();
    let temp_dir_str = tmp_dir.to_str().unwrap();
    let paste_id = Uuid::new_v4();
    let filename = format!("{dir}/{id}", dir=temp_dir_str, id = paste_id);

    paste.stream_to_file(Path::new(&filename))?;
    // TODO: error handling
    let _output = Command::new(ENCODER_PATH.to_string())
    .arg("-i")
    .arg(filename)
    .arg("-o")
    .arg(format!("{}/{}.drc", OUTPUT_DIR.to_string(), paste_id))
    .output()
    .expect("failed to execute process");

    Ok(paste_id.to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![upload]).launch();
}