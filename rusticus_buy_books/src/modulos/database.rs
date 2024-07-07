use rusqlite::{params, Connection, Result};
use chrono::prelude::*;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn cria_db(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS produtos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                Produto TEXT NOT NULL,
                Marca TEXT NOT NULL,
                Conteudo FLOAT,
                Unidade TEXT NOT NULL,
                Preco FLOAT,
                Data TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_date() -> String {
        let local: DateTime<Local> = Local::now();
        local.format("%d-%m-%Y").to_string()
    }

    pub fn add_produto(&self, produto: &str, marca: &str, conteudo: f32, unidade: &str, preco: f32) -> Result<()> {

        let data = Database::get_date();

        self.conn.execute(
            "INSERT INTO produtos (Produto, Marca, Conteudo, Unidade, Preco, Data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![produto, marca, conteudo, unidade, preco, data],
        )?;
        Ok(())
    }

    pub fn show_produtos(&self) -> Result<Vec<(i32, String, String, f32, String, f32, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, Produto, Marca, Conteudo, Unidade, Preco, Data FROM produtos")?;
        let produtos = stmt.query_map([], |row| {
            Ok((row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?))
        })?;

        let mut result = Vec::new();
        for produto in produtos {
            result.push(produto?);
        }
        Ok(result)
    }

    pub fn delete_produto(&self, id: i32) -> Result<()> {
        self.conn.execute("DELETE FROM produtos WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn search_produtos(&self, query: &str) -> Result<Vec<(i32, String, String, f32, String, f32, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, produto, marca, conteudo, unidade, preco, data FROM produtos WHERE produto LIKE ?1")?;
        let query = format!("%{}%", query);
        let produtos_iter = stmt.query_map([query], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?;
        
        let mut produtos = Vec::new();
        for produto in produtos_iter {
            produtos.push(produto?);
        }
        Ok(produtos)
    }    
}
