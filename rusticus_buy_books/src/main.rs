use modulos::database::Database;
use std::io;
use std::path::Path;

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
    println!("| 4. Pesquisar produto      |");
    println!("| 5. Sair                   |");
    println!("+---------------------------+");
}

fn main() {
    let db_path = "compras.db";
    let database = Database::new(db_path).expect("Erro ao criar a conexão com o banco de dados");

    if !Path::new(db_path).exists() {
        database.cria_db().expect("Erro ao criar banco de dados");
    }

    loop {
        show_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler a entrada");

        match choice.trim() {
            "1" => {
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

                if let Err(e) = database.add_produto(&nome.trim(), &marca.trim(), conteudo, preco) {
                    eprintln!("Erro ao adicionar produto: {}", e);
                }
            }
            "2" => {
                match database.show_produtos() {
                    Ok(produtos) => {
                        for (id, nome, marca, conteudo, preco) in produtos {
                            println!("ID: {}, Nome: {}, Marca: {}, Conteúdo: {}, Preço: {}", id, nome, marca, conteudo, preco);
                        }
                    }
                    Err(e) => eprintln!("Erro ao mostrar produtos: {}", e),
                }
            }
            "3" => {
                println!("Digite o ID do produto que deseja deletar: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Falha ao ler o ID");

                let id: i32 = id.trim().parse().expect("ID inválido, digite um número inteiro");

                if let Err(e) = database.delete_produto(id) {
                    eprintln!("Erro ao deletar produto: {}", e);
                }
            }
            "4" => {
                println!("Digite o nome do produto que deseja pesquisar: ");
                let mut query = String::new();
                io::stdin().read_line(&mut query).expect("Falha ao ler a pesquisa");

                match database.search_produtos(&query.trim()) {
                    Ok(produtos) => {
                        for (id, nome, marca, conteudo, preco) in produtos {
                            println!("ID: {}, Nome: {}, Marca: {}, Conteúdo: {}, Preço: {}", id, nome, marca, conteudo, preco);
                        }
                    }
                    Err(e) => eprintln!("Erro ao pesquisar produtos: {}", e),
                }
            }
            "5" => break,
            _ => println!("Opção inválida, por favor, escolha entre 1, 2, 3, 4 ou 5."),
        }
    }
}
