// #[macro_use]
// extern crate rocket;
// use std::fmt::format;

// use rocket::{
//     form,
//     tokio::time::{sleep, Duration},
// };

// #[get("/world")]
// fn world() -> &'static str {
//     "Hello, world!"
// }

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }

// #[get("/hello/<name>/<age>/<cool>")]
// fn hello(name: &str, age: u8, cool: bool) -> String {
//     if cool {
//         format!("You're a cool {} year old, {}!", age, name)
//     } else {
//         format!("{}, we need to talk about your coolness.", name)
//     }
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![world, delay, hello])
// }
use mongodb_manager::Manager;
#[tokio::main]
async fn main() {
    let mongo_instance = Manager::new("mongodb://localhost:47017").await;
    // Use mongo_instance for further operations
}