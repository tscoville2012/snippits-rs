#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate pulldown_cmark;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use errors::*;

pub mod snippits;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{
        foreign_links {
        }
    }
}

#[error(403)]
fn forbidden() -> &'static str {
    "Unauthorized!"
}

#[error(404)]
fn not_found() -> &'static str {
    "Not Found!"
}

#[error(500)]
fn server_error() -> &'static str {
    "Whoops!"
}

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).chain_err(|| "File not found!")
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                static_content,
                snippits::index,
                snippits::new_post_submit,
                snippits::new_snippit,
                snippits::up_vote,
            ],
        )
        .catch(errors![forbidden, server_error, not_found])
        .launch();
}
