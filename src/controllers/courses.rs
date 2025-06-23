use rocket::get;
use rocket_dyn_templates::Template;
use crate::models::course::Course;
use std::collections::HashMap;






#[get("/cursos")]
pub fn index() -> Template {
    let cursos = vec![
        Course::new(1, "Curso de Rust para Iniciantes", "Aprenda Rust do zero com exemplos práticos."),
        Course::new(2, "Programação Web com Rocket", "Crie sites rápidos e seguros com Rocket."),
        Course::new(3, "Desenvolvimento Fullstack em Rust", "Domine o backend e frontend usando apenas Rust."),
    ];

    let mut context = HashMap::new();
    context.insert("cursos", &cursos);

    Template::render("courses/index", rocket_dyn_templates::context!{cursos})
}
