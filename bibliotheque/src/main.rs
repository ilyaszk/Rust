use eframe::egui;

struct Livre {
    titre: String,
    auteur: String,
    num_isbn: String,
    date_publication: String,
}

impl Livre {
    fn new(titre: &str, auteur: &str, num_isbn: &str, date_publication: &str) -> Livre {
        Livre {
            titre: titre.into(),
            auteur: auteur.into(),
            num_isbn: num_isbn.into(),
            date_publication: date_publication.into(),
        }
    }

    fn afficher(&self) -> String {
        format!(
            "Titre: {}, Auteur: {}, ISBN: {}, Date: {}",
            self.titre, self.auteur, self.num_isbn, self.date_publication
        )
    }
}

struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Default for Bibliotheque {
    fn default() -> Self {
        Self { livres: Vec::new() }
    }
}

impl Bibliotheque {
    fn new() -> Self {
        Self::default()
    }

    fn ajouter_livre(&mut self, livre: Livre) {
        self.livres.push(livre);
    }

    fn rechercher_livre(&self, titre: &str) -> Option<&Livre> {
        self.livres.iter().find(|livre| livre.titre == titre)
    }

    fn retirer_livre(&mut self, titre: &str) {
        self.livres.retain(|livre| livre.titre != titre);
    }
}

#[derive(Default)]
struct MonApp {
    bibliotheque: Bibliotheque,
    titre: String,
    auteur: String,
    isbn: String,
    date: String,
    recherche: String,
    message: String,
}

impl MonApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut bibliotheque = Bibliotheque::new();

        let livres = vec![
            Livre::new("Le Petit Prince", "Antoine de Saint-Exupéry", "978-2-07-061275-8", "1943"),
            Livre::new("1984", "George Orwell", "978-2-07-036822-0", "1949"),
            Livre::new("Le Meilleur des mondes", "Aldous Huxley", "978-2-07-036822-0", "1932"),
            Livre::new("Fahrenheit 451", "Ray Bradbury", "978-2-07-036822-0", "1953"),
            Livre::new("Le Seigneur des Anneaux", "J.R.R. Tolkien", "978-2-07-036822-0", "1954"),
            Livre::new("Harry Potter à l'école des sorciers", "J.K. Rowling", "978-2-07-036822-0", "1997"),
        ];

        for livre in livres {
            bibliotheque.ajouter_livre(livre);
        }

        Self {
            bibliotheque,
            ..Default::default()
        }
    }
}

impl eframe::App for MonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📚 Bibliothèque");

            ui.separator();
            ui.label("Ajouter un nouveau livre :");

            ui.horizontal(|ui| {
                ui.label("Titre :");
                ui.text_edit_singleline(&mut self.titre);
            });
            ui.horizontal(|ui| {
                ui.label("Auteur :");
                ui.text_edit_singleline(&mut self.auteur);
            });
            ui.horizontal(|ui| {
                ui.label("ISBN :");
                ui.text_edit_singleline(&mut self.isbn);
            });
            ui.horizontal(|ui| {
                ui.label("Date de publication (YYYY) :");
                ui.text_edit_singleline(&mut self.date);
            });

            if ui.button("Ajouter le livre").clicked() {
                if self.date.trim().len() == 4 && self.date.trim().chars().all(|c| c.is_digit(10)) {
                    let livre = Livre::new(&self.titre, &self.auteur, &self.isbn, &self.date);
                    self.bibliotheque.ajouter_livre(livre);
                    self.message = "📖 Livre ajouté avec succès !".to_string();
                    self.titre.clear();
                    self.auteur.clear();
                    self.isbn.clear();
                    self.date.clear();
                } else {
                    self.message = "❌ La date doit être au format YYYY.".to_string();
                }
            }

            ui.label(&self.message);
            ui.separator();

            ui.heading("🔍 Rechercher un livre");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.recherche);
                if ui.button("Rechercher").clicked() {
                    match self.bibliotheque.rechercher_livre(&self.recherche) {
                        Some(livre) => {
                            self.message = format!("✅ Livre trouvé : {}", livre.afficher());
                        }
                        None => {
                            self.message = "❌ Livre non trouvé".to_string();
                        }
                    }
                }
            });

            ui.separator();

            ui.heading("📚 Liste des livres");
            let mut livres_a_retirer = Vec::new();
            for livre in &self.bibliotheque.livres {
                ui.horizontal(|ui| {
                    ui.label(livre.afficher());
                    if ui.button("Retirer").clicked() {
                        livres_a_retirer.push(livre.titre.clone());
                    }
                });
            }
            
            for titre in livres_a_retirer {
                self.bibliotheque.retirer_livre(&titre);
                self.message = format!("❌ Livre retiré : {}", titre);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Ma Bibliothèque", options, Box::new(|cc| Box::new(MonApp::new(cc))))
}
