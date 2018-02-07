#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use std::cmp;
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{Form, FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use tera::Context;

#[derive(Serialize)]
struct TemplateContext {
        keyword: String,
        keyword_help_link: String,
        sql_correct: String,
        sql_to_run: String,
        sql_correct_result: Vec<Vec<String>>,
        sql_to_run_result: Vec<Vec<String>>,
        next_q: String,
        prev_q: String,
        is_correct: bool,
        used_correct_word: bool,
}

#[get("/questions/<_question>")]
fn get_db(_question: String) -> Template {
    let c = TemplateContext {
        keyword: "t.keywords[0]".to_string(),
        keyword_help_link: "t.help_link".to_string(),
        sql_correct: "t.sql".to_string(),
        sql_to_run : "asdf".to_string(),
        sql_correct_result: vec![vec!["asd".to_string()]],
        sql_to_run_result: vec![vec!["sql_result".to_string()]],
        next_q: "asdf".to_string(),
        prev_q: "3".to_string(),
        is_correct: true,
        used_correct_word: false,
    };
    Template::render(_question, &c)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get_db,
            ],
        )
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
