use modulos::database::Database;

mod modulos {
    pub mod database;
}

mod gui {
    pub mod main_window;
}

fn main() {
    let db_path = "compras.db";
    let db = Database::new(db_path).expect("Erro ao conectar ao banco de dados");

    // Cria a tabela se não existir
    if let Err(e) = db.cria_db() {
        eprintln!("Erro ao criar banco de dados: {}", e);
        return;
    }

    // Rodar a GUI
    let app = gui::main_window::MyApp::new(db);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusticus - Gerenciador de Produtos",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).expect("Erro ao iniciar a aplicação");
}
