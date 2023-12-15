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
use std::env;
#[tokio::main]
async fn main() {
    let mongo_url = env::var_os("MONGODB_URL").unwrap();
    let mongo_instance = Manager::new(mongo_url.to_str().unwrap()).await.unwrap();
}
