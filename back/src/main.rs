#[macro_use]
extern crate rocket;

mod crud;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(crud::stage())
}
