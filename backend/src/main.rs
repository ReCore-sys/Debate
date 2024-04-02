pub mod database;
pub mod routes;

#[macro_use]
extern crate rocket;

#[launch]
fn blastoff() -> _ {
    rocket::build().mount("/", routes![routes::login::login])
}
