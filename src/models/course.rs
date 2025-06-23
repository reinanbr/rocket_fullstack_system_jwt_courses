use serde::Serialize;

#[derive(Serialize)]
pub struct Course {
    pub id: u32,
    pub title: &'static str,
    pub description: &'static str,
}

impl Course {
    pub fn new(id: u32, title: &'static str, description: &'static str) -> Self {
        Self { id, title, description }
    }
}
