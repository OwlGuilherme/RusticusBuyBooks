use modulos::database::add_produto;
use modulos::database::show_produtos;
use modulos::database::delete_produto;
//use gui::main_window::show_main_window;

mod modulos {
    pub mod database;
}

mod gui {
    pub mod main_window;
}

fn main() {
    //let _ = modulos::database::cria_db();

    //let _ = add_produto();
    //let _ = show_produtos();
    let _ = delete_produto();

    
}
