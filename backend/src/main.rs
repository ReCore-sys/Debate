#[macro_use]
extern crate rocket;

use which::which;

use crate::config::get_config;

pub mod database;
pub mod routes;
mod config;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    match which("nats-server.exe") {
        Ok(_) => {}
        Err(_) => {
            println!("Please install nats-server.exe");
            std::process::exit(1);
        }
    }
    match which("surreal") {
        Ok(_) => {}
        Err(_) => {
            println!("Please install surrealdb");
            std::process::exit(1);
        }
    }


    tokio::spawn(async {
        std::process::Command::new("nats-server.exe").spawn()
    });
    tokio::spawn(
        async {
            std::process::Command::new("surreal")
                .arg("start")
                .arg("file:database.db")
                .arg("--auth")
                .arg("--user")
                .arg(get_config().surreal_username)
                .arg("--pass")
                .arg(get_config().surreal_password)
                .arg("--bind")
                .arg("0.0.0.0:9000")
                .spawn().unwrap().wait()
        }
    );
    rocket::build().mount("/", routes![routes::login::login, routes::signup::signup, routes::message::message]).ignite().await.unwrap().launch().await.expect("Web server failed to start");
}