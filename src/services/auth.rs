use rusqlite::{Connection, Result};
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::env;
use dotenvy::dotenv;

#[derive(Debug, Serialize)]
pub struct User {
    id: i32,
    nome: String,
    email: String,
    senha_hash: String,
}


pub fn open_connection() -> Result<Connection> {
    dotenv().ok();
    let db_path = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    Connection::open(db_path)
}

/* 

pub fn list_users_json(conn: &Connection) -> Result<Value> {
    let mut stmt = conn.prepare("SELECT id, nome, email, senha_hash FROM usuarios_clientes")?;

    let users: Vec<User> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                nome: row.get(1)?,
                email: row.get(2)?,
                senha_hash: row.get(3)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();

    // Convert to JSON array
    serde_json::to_value(users)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
}

*/


pub fn find_user(email: &str, password: &str, conn: Connection) -> Result<Value> {
    let mut stmt = conn.prepare("SELECT id, nome, email, senha_hash FROM usuarios_clientes WHERE email = ? AND senha_hash = ?")?;
    let user_result = stmt
        .query_row([email, password], |row| {
            Ok(User {
                id: row.get(0)?,
                nome: row.get(1)?,
                email: row.get(2)?,
                senha_hash: row.get(3)?,
            })
        });

    match user_result {
        Ok(u) => Ok(serde_json::to_value(u).unwrap()),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(json!({"success": false, "message": "Usuário não encontrado"})),
        Err(e) => Err(e),
    }
}
