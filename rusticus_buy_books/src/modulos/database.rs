use std::io;
use egui::CollapsingHeader;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn cria_db(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS produtos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                Produto TEXT NOT NULL,
                Marca_ou_qualidade TEXT NOT NULL,
                Conteudo FLOAT,
                Preco FLOAT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_produto(&self, nome: &str, marca: &str, conteudo: f32, preco: f32) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO produtos (Produto, Marca_ou_qualidade, Conteudo, Preco) VALUES (?1, ?2, ?3, ?4)",
            (&nome, &marca, &conteudo, &preco),
        )?;
        Ok(())
    }

    pub fn show_produtos(&self) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, Produto, Marca_ou_qualidade, Conteudo, Preco FROM produtos")?;
        
        let produtos = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut results = Vec::new();
        for produto in produtos {
            results.push(produto?);
        }

        Ok(results)
    }

    pub fn delete_produto(&self, id: i32) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM produtos WHERE id = ?1", &[&id])?;
        Ok(())
    }

    pub fn search_produtos(&self, query: &str) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let conn = self.conn.lock().unwrap();
        let like_query = format!("%{}%", query);
        let mut stmt = conn.prepare("SELECT id, Produto, Marca_ou_qualidade, Conteudo, Preco FROM produtos WHERE Produto LIKE ?1")?;
        
        let produtos = stmt.query_map([&like_query], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut results = Vec::new();
        for produto in produtos {
            results.push(produto?);
        }

        Ok(results)
    }
}