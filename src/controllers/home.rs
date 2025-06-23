use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::{Template, context};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: u64,
    email: String,
    name: String,
    exp: usize,
}


#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Template {
    let title = "Página Inicial";
    let message = "Bem-vindo à aplicação Rocket MVC!";

    let mut username: Option<String> = None;
    let mut exp:Option<usize> = None;

    if let Some(cookie) = cookies.get("token") {
        let token = cookie.value();

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
        let validation = Validation::default();

        match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
            Ok(token_data) => {
                println!("JWT validado com sucesso!");
                username = Some(token_data.claims.name);
                exp = Some(token_data.claims.exp);
            }
            Err(err) => {
                eprintln!("Erro ao decodificar JWT: {}", err);
            }
        }
    } else {
        println!("Cookie 'token' não encontrado.");
    }

    Template::render("home/index", context! {
        title,
        message,
        username,
        exp
    })
}
