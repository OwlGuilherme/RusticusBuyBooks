use crate::modulos::database::Database;
use eframe::egui::{self, CentralPanel, Context};

pub struct MyApp {
    db: Database,
    produto: String,
    marca: String,
    conteudo: String,
    preco: String,
    search_query: String,
    search_results: Vec<(i32, String, String, f32, f32)>,
}

impl MyApp {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            produto: String::new(),
            marca: String::new(),
            conteudo: String::new(),
            preco: String::new(),
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusticus - Gerenciador de Produtos");

            ui.horizontal(|ui| {
                ui.label("Produto:");
                ui.text_edit_singleline(&mut self.produto);
            });

            ui.horizontal(|ui| {
                ui.label("Marca:");
                ui.text_edit_singleline(&mut self.marca);
            });

            ui.horizontal(|ui| {
                ui.label("Conteúdo [litros ou kg]:");
                ui.text_edit_singleline(&mut self.conteudo);
            });

            ui.horizontal(|ui| {
                ui.label("Preço:");
                ui.text_edit_singleline(&mut self.preco);
            });

            if ui.button("Adicionar Produto").clicked() {
                if let Ok(conteudo) = self.conteudo.parse::<f32>() {
                    if let Ok(preco) = self.preco.parse::<f32>() {
                        if let Err(e) = self.db.add_produto(&self.produto, &self.marca, conteudo, preco) {
                            eprintln!("Erro ao adicionar produto: {}", e);
                        } else {
                            self.produto.clear();
                            self.marca.clear();
                            self.conteudo.clear();
                            self.preco.clear();
                        }
                    } else {
                        eprintln!("Erro ao converter preço");
                    }
                } else {
                    eprintln!("Erro ao converter conteúdo");
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Pesquisar Produto:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            if ui.button("Buscar").clicked() {
                self.search_results = self.db.search_produtos(&self.search_query).unwrap_or_default();
            }

            ui.separator();

            let mut produtos_a_remover = Vec::new();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, produto, marca, conteudo, preco) in &self.search_results {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} - {} - {} - {} - {}", id, produto, marca, conteudo, preco));

                        if ui.button("Deletar").clicked() {
                            produtos_a_remover.push(*id);
                        }
                    });
                }
            });

            for id in produtos_a_remover {
                if let Err(e) = self.db.delete_produto(id) {
                    eprintln!("Erro ao deletar produto: {}", e);
                } else {
                    self.search_results.retain(|(pid, _, _, _, _)| *pid != id);
                }
            }
        });
    }
}
