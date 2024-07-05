use modulos::database::{Database, add_produto, delete_produto, show_produtos};
use std::io;

mod modulos {
    pub mod database;
}

fn show_menu() {
    println!("+---------------------------+");
    println!("|        Rusticus           |");
    println!("+---------------------------+");
    println!("|    O que deseja fazer?    |");
    println!("+- - - - - - - - - - - - - -+");
    println!("| 1. Adicionar produto      |");
    println!("| 2. Mostrar produtos       |");
    println!("| 3. Deletar produto        |");
    println!("| 4. Sair                   |");
    println!("+---------------------------+");
}

fn main() {
    let db_path = "compras.db";
    let db = Database::new(db_path).expect("Erro ao conectar ao banco de dados");

    // Cria a tabela se não existir
    if let Err(e) = db.cria_db() {
        eprintln!("Erro ao criar banco de dados: {}", e);
        return;
    }

    loop {
        show_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler a entrada");

        match choice.trim() {
            "1" => {
                if let Err(e) = add_produto(&db) {
                    eprintln!("Erro ao adicionar produto: {}", e);
                }
            },
            "2" => {
                if let Err(e) = show_produtos(&db) {
                    eprintln!("Erro ao mostrar produtos: {}", e);
                }
            },
            "3" => {
                if let Err(e) = delete_produto(&db) {
                    eprintln!("Erro ao deletar produto: {}", e);
                }
            },
            "4" => break,
            _ => println!("Opção inválida, por favor, escolha entre 1, 2, 3 ou 4."),
        }
    }
}
