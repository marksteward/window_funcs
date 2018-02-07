#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use rocket_contrib::Template;


#[get("/")]
fn get_db() -> Template {
    Template::render("all_questions", ())
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
