use modulos::database::{add_produto, show_produtos, delete_produto};
//use gui::main_window::show_main_window;

use std::io;

mod modulos {
    pub mod database;
}

mod gui {
    pub mod main_window;
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
    loop {
        show_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler a entrada");

        match choice.trim() {
            "1" => {
                if let Err(e) = add_produto() {
                    eprintln!("Erro ao acidionar produto: {}", e);
                }
            },
            "2" => {
                if let Err(e) = show_produtos() {
                    eprintln!("Erro ao mostrar produtos: {}", e);
                }
            },
            "3" => {
                if let Err(e) = delete_produto() {
                    eprintln!("Erro ao deletar produto: {}", e);
                }
            },
            "4" => break,
            _ => println!("Opção inválida, por favor, escolha entre 1, 2, 3 ou 4."),
        }
    }    
}
