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
        (&nome.trim(), &marca.trim(), &conteudo.trim())
    )?;
    Ok(())
}

pub fn show_produtos() -> Result<()> {
    let conn = Connection::open("compras.db")?;

    let mut stmt = conn.prepare("SELECT id, Produto, Marca_ou_qualidade, Conteúdo FROM produtos")?;
    
    let produtos = stmt.query_map([], |row|{
        Ok((row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, f32>(3)?,            
        ))
    })?;

    for produto in produtos {
        if let Ok((id, nome, marca, conteudo)) = produto {
            println!("ID: {}, Nome: {}, Marca: {}, Conteúdo: {}", id, nome, marca, conteudo);
        }
    }
    Ok(())
}

pub fn delete_produto() -> Result<()> {
    let conn = Connection::open("compras.db")?;

    show_produtos()?;

    println!("Digite o ID do produto que deseja deletar: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Falha ao ler o ID");

    // Convertendo a string lida para i32
    let id: i32 = id.trim().parse().expect("ID inválido, digite um número inteiro");

    conn.execute(
        "DELETE FROM produtos WHERE id = ?1", &[&id]
    )?;
    Ok(())
}
