use eframe::{egui, Result};

struct MyApp {
    produto: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            produto: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusticus - Registro de compras");

            ui.horizontal(|ui| {
                ui.label("Produto: ");
                ui.text_edit_singleline(&mut self.produto);
            });

            if ui.button("Adicionar").clicked() {
                println!("Produto: {}", self.produto);
            }

            //ui.label(format!("Produto adicionado: {}", self.produto));
        });
    }
}

pub fn show_main_window() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rusticus",
        options,
        Box::new(|cc| Box::<MyApp>::default()),
    )
}
