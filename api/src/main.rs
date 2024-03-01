#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is RSI."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
