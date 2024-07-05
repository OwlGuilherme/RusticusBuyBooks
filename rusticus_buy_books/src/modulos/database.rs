use rusqlite::{Connection, Result};
use std::io;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;
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
            (&produto, &marca, &conteudo, &preco),
        )?;
        Ok(())
    }

    pub fn show_produtos(&self) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let mut stmt = self.conn.prepare("SELECT id, Produto, Marca, Conteudo, Preco FROM produtos")?;
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
        self.conn.execute(
            "DELETE FROM produtos WHERE id = ?1", &[&id],
        )?;
        Ok(())
    }

    pub fn search_produtos(&self, termo: &str) -> Result<Vec<(i32, String, String, f32, f32)>> {
        let like_termo = format!("%{}%", termo);
        let mut stmt = self.conn.prepare("SELECT id, Produto, Marca, Conteudo, Preco FROM produtos WHERE Produto LIKE ?1")?;
        let produtos = stmt.query_map([like_termo], |row| {
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

pub fn add_produto(db: &Database) -> Result<()> {
    println!("Digite o nome do produto: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Digite a marca do produto:");
    let mut marca = String::new();
    io::stdin().read_line(&mut marca).expect("Falha ao ler a marca");

    println!("Digite o conteúdo do produto [litros ou kg]: ");
    let mut conteudo = String::new();
    io::stdin().read_line(&mut conteudo).expect("Falha ao ler conteúdo");

    println!("Digite o preço do produto: ");
    let mut preco = String::new();
    io::stdin().read_line(&mut preco).expect("Falha ao ler o preço");

    let conteudo: f32 = conteudo.trim().parse().expect("Conteúdo inválido, digite um número");
    let preco: f32 = preco.trim().parse().expect("Preço inválido, digite um número");

    db.add_produto(&nome.trim(), &marca.trim(), conteudo, preco)?;
    Ok(())
}

pub fn show_produtos(db: &Database) -> Result<()> {
    let produtos = db.show_produtos()?;
    for (id, nome, marca, conteudo, preco) in produtos {
        println!("ID: {}, Nome: {}, Marca: {}, Conteúdo: {}, Preço: {}", id, nome, marca, conteudo, preco);
    }
    Ok(())
}

pub fn delete_produto(db: &Database) -> Result<()> {
    show_produtos(db)?;

    println!("Digite o ID do produto que deseja deletar: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");

    let id: i32 = id.trim().parse().expect("ID inválido, digite um número inteiro");

    db.delete_produto(id)?;
    Ok(())
}
