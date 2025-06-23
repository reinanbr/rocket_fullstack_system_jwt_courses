use rocket::{routes, Route};
use crate::controllers::{home, courses, login};

pub fn routes() -> Vec<Route> {
    routes![
        home::index,
        courses::index,
        login::login_form,
        login::login_submit
    ]
}
