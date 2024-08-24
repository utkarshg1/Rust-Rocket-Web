#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Please provide path ./hello/<name>/<age>"
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello , {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
}
