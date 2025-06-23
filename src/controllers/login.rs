use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::http::{Cookie, CookieJar};
use serde::Serialize;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono;
use crate::services::auth::{find_user, open_connection};
use rocket::time;


#[derive(rocket::form::FromForm)]
pub struct LoginData {
    email: String,
    password: String,
}

#[rocket::get("/login")]
pub fn login_form() -> Template {
    Template::render("login/index", rocket_dyn_templates::context! {})
}

#[rocket::post("/login", data = "<form>")]
pub fn login_submit(form: Form<LoginData>, cookies: &CookieJar<'_>) -> Result<Redirect, Template> {
    let data = form.into_inner();

    println!("User: {}, Password: {}", data.email, data.password);

    let conn = match open_connection() {
        Ok(conn) => {
            println!("success connection!");
            conn
        }
        Err(e) => {
            eprintln!("DB connection error: {}", e);
            return Err(Template::render("login/index", rocket_dyn_templates::context! {
                error: "Internal server error"
            }));
        }
    };

    match find_user(&data.email, &data.password, conn) {
        Ok(json_val) => {
            if json_val.get("success").and_then(|v| v.as_bool()) == Some(false) {
                println!("User does not exist");
                return Err(Template::render("login/index", rocket_dyn_templates::context! {
                    error: "Invalid email or password"
                }));
            }

            // Extract user data
            let user_id = json_val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
            let user_email = json_val.get("email").and_then(|v| v.as_str()).unwrap_or("");
            let user_name = json_val.get("nome").and_then(|v| v.as_str()).unwrap_or(""); // Corrigido: era "name"

            #[derive(Serialize)]
            struct Claims {
                sub: u64,
                email: String,
                name: String,
                exp: usize,
            }

            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: user_id,
                email: user_email.to_string(),
                name: user_name.to_string(),
                exp: expiration,
            };

            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
                .expect("JWT token creation failed");

            let mut cookie = Cookie::new("token", token);
            cookie.set_http_only(true);
            cookie.set_path("/"); // Or a more specific path
            cookie.set_expires(time::OffsetDateTime::now_utc().checked_add(time::Duration::hours(24)).unwrap()); // Example: 24 hours
            cookies.add(cookie);
            println!("User authenticated and cookie set.");
        }

        Err(e) => {
            eprintln!("Error querying user: {}", e);
            return Err(Template::render("login/index", rocket_dyn_templates::context! {
                error: "Internal server error"
            }));
        }
    };

    Ok(Redirect::to("/"))
}
