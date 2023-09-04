use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {
    use rocket::{catch, catcher, Catcher};
    use rocket::{catchers, Request};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "Something went wrong...."
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![default, internal_error, not_found]
    }
}
