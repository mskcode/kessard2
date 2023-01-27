#[macro_use]
extern crate rocket;
extern crate reqwest;

mod api;

#[rocket::main]
async fn main() {
    let rocket = api::initialize_rocket();
    let _ = rocket.launch().await;
}
