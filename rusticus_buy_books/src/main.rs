use modulos::database::add_produto;

mod modulos {
    pub mod database;
}

fn main() {
    let _ = modulos::database::cria_db();

    let _ = add_produto();
}
