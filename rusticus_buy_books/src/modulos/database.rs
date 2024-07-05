use rusqlite::{params, Connection, Result};

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
                Preco FLOAT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_produto(&self, produto: &str, marca: &str, conteudo: f32, preco: f32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO produtos (Produto, Marca, Conteudo, Preco) VALUES (?1, ?2, ?3, ?4)",
            params![produto, marca, conteudo, preco],
        )?;
        Ok(())
    }

    pub fn show_produtos(&self) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let mut stmt = self.conn.prepare("SELECT id, Produto, Marca, Conteudo, Preco FROM produtos")?;
        let produtos = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
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

    pub fn search_produtos(&self, query: &str) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let like_query = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, Produto, Marca, Conteudo, Preco FROM produtos WHERE Produto LIKE ?1 OR Marca LIKE ?1",
        )?;
        let produtos = stmt.query_map(params![like_query], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
        })?;

        let mut result = Vec::new();
        for produto in produtos {
            result.push(produto?);
        }
        Ok(result)
    }
}
