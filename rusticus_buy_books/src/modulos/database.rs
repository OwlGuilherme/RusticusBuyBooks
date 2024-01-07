use std::io;
use rusqlite::{Connection, Result};

pub fn cria_db() -> Result<()> {
    let conn = Connection::open("compras.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS produtos (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        Produto TEXT NOT NULL,
        Marca_ou_qualidade TEXT NOT NULL,
        Conteúdo FLOAT
        )",
        [],
    )?;
    Ok(())
}

pub fn add_produto() -> Result<()> {

    let conn = Connection::open("compras.db")?;

    println!("Digite o nome do produto: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Digite a marca do produto:");
    let mut marca = String::new();
    io::stdin().read_line(&mut marca).expect("Falha ao ler a marca");

    println!("Digite o conteúdo do produto [litros ou km]: ");
    let mut conteudo = String::new();
    io::stdin().read_line(&mut conteudo).expect("Falha ao ler conteúdo");

    conn.execute(
        "INSERT INTO produtos (Produto, Marca_ou_qualidade, Conteúdo) VALUES (?1, ?2, ?3)",
        (&nome, &marca, &conteudo),
    )?;
    Ok(())

}
