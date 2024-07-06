use crate::modulos::database::Database;
use eframe::egui::{self, CentralPanel, Context, ComboBox};

pub struct MyApp {
    db: Database,
    produto: String,
    marca: String,
    conteudo: String,      
    unidade: Unidade,  
    preco: String,
    search_query: String,
    search_results: Vec<(i32, String, String, f32, String, f32)>,
}

#[derive(Debug, PartialEq, Eq)]
enum Unidade {
    Litros,
    Quilogramas,
    Unidades
}

impl MyApp {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            produto: String::new(),
            marca: String::new(),
            conteudo: String::new(),
            unidade: Unidade::Litros,
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
                ui.label("Conteúdo:");
                ui.text_edit_singleline(&mut self.conteudo);
            });

            ui.horizontal(|ui| {
                ui.label("Unidade: ");
                ComboBox::from_label("Escolha a unidade")
                .selected_text(format!("{:?}", self.unidade))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.unidade, Unidade::Litros, "Litros");
                    ui.selectable_value(&mut self.unidade, Unidade::Quilogramas, "Quilogramas");
                    ui.selectable_value(&mut self.unidade, Unidade::Unidades, "Unidades");
                });                
            });

            ui.horizontal(|ui| {
                ui.label("Preço:");
                ui.text_edit_singleline(&mut self.preco);
            });

            if ui.button("Adicionar Produto").clicked() {
                if let Ok(conteudo) = self.conteudo.parse::<f32>() {
                    let unidade = format!("{:?}", self.unidade);
                    if let Ok(preco) = self.preco.parse::<f32>() {
                        if let Err(e) = self.db.add_produto(&self.produto, &self.marca, conteudo, &unidade, preco) {
                            eprintln!("Erro ao adicionar produto: {}", e);
                        } else {
                            self.produto.clear();
                            self.marca.clear();
                            self.conteudo.clear();
                            self.unidade = Unidade::Litros;  // Reset para o valor padrão
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
                ui.label("Buscar produto:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Buscar").clicked() {
                    self.search_results = self.db.search_produtos(&self.search_query).unwrap_or_default();
                }
            });

            ui.separator();

            // Coletar os IDs dos produtos a serem removidos
            let mut ids_to_remove = Vec::new();
            for (id, produto, marca, conteudo, unidade, preco) in &self.search_results {
                ui.horizontal(|ui| {
                    ui.label(format!("{} - {} - {} - {} - {} - {}", id, produto, marca, conteudo, unidade, preco));
                    if ui.button("Deletar").clicked() {
                        ids_to_remove.push(*id);
                    }
                });
            }

            // Remover os produtos fora do loop
            for id in ids_to_remove {
                if let Err(e) = self.db.delete_produto(id) {
                    eprintln!("Erro ao deletar produto: {}", e);
                } else {
                    self.search_results.retain(|(pid, _, _, _, _, _)| pid != &id);
                }
            }
        });
    }
}