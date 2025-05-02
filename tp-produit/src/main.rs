use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;

// Structure Produit avec nom et quantité
struct Produit {
    nom: String,
    quantite: u32,
}

// Implémentation des méthodes pour Produit
impl Produit {
    // Constructeur
    fn new(nom: String, quantite: u32) -> Self {
        Produit {
            nom,
            quantite,
        }
    }

    // Méthode pour modifier la quantité
    fn modifier_quantite(&mut self, nouvelle_quantite: u32) {
        self.quantite = nouvelle_quantite;
    }

    // Afficher les informations du produit
    fn afficher(&self) {
        println!("Nom: {}, Quantité: {}", self.nom, self.quantite);
    }
}

// Structure pour gérer l'inventaire
struct Inventaire {
    produits: Vec<Produit>,
}

impl Inventaire {
    // Créer un nouvel inventaire vide
    fn new() -> Self {
        Inventaire {
            produits: Vec::new(),
        }
    }

    // Ajouter un produit à l'inventaire
    fn ajouter_produit(&mut self, produit: Produit) {
        self.produits.push(produit);
    }

    // Supprimer un produit par nom
    fn supprimer_produit(&mut self, nom: &str) -> bool {
        let position = self.produits.iter().position(|p| p.nom == nom);
        if let Some(index) = position {
            self.produits.remove(index);
            true
        } else {
            false
        }
    }

    // Modifier la quantité d'un produit
    fn modifier_produit(&mut self, nom: &str, nouvelle_quantite: u32) -> bool {
        for produit in &mut self.produits {
            if produit.nom == nom {
                produit.modifier_quantite(nouvelle_quantite);
                return true;
            }
        }
        false
    }

    // Afficher tous les produits
    fn afficher_produits(&self) {
        if self.produits.is_empty() {
            println!("L'inventaire est vide.");
        } else {
            println!("Liste des produits:");
            for (i, produit) in self.produits.iter().enumerate() {
                print!("{}. ", i + 1);
                produit.afficher();
            }
        }
    }

    // Sauvegarder l'inventaire dans un fichier
    fn sauvegarder(&self, fichier: &str) -> io::Result<()> {
        let mut file = File::create(fichier)?;
        
        for produit in &self.produits {
            writeln!(file, "{},{}", produit.nom, produit.quantite)?;
        }
        
        Ok(())
    }

    // Charger l'inventaire depuis un fichier
    fn charger(&mut self, fichier: &str) -> io::Result<()> {
        if !Path::new(fichier).exists() {
            return Ok(());
        }
        
        let contenu = fs::read_to_string(fichier)?;
        self.produits.clear();
        
        for ligne in contenu.lines() {
            let parts: Vec<&str> = ligne.split(',').collect();
            if parts.len() == 2 {
                if let Ok(quantite) = parts[1].parse::<u32>() {
                    self.produits.push(Produit::new(
                        parts[0].to_string(),
                        quantite,
                    ));
                }
            }
        }
        
        Ok(())
    }
}

// Fonction pour lire une entrée utilisateur
fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin()
        .read_line(&mut entree)
        .expect("Échec de la lecture de l'entrée");
    entree.trim().to_string()
}

fn main() {
    let fichier_inventaire = "inventaire.txt";
    let mut inventaire = Inventaire::new();
    
    // Essayer de charger l'inventaire existant
    if let Err(e) = inventaire.charger(fichier_inventaire) {
        println!("Erreur lors du chargement de l'inventaire: {}", e);
    }

    loop {
        println!("\n=== GESTION D'INVENTAIRE ===");
        println!("1. Afficher tous les produits");
        println!("2. Ajouter un produit");
        println!("3. Modifier la quantité d'un produit");
        println!("4. Supprimer un produit");
        println!("5. Sauvegarder l'inventaire");
        println!("6. Quitter");
        
        print!("Votre choix: ");
        io::stdout().flush().unwrap();
        
        match lire_entree().as_str() {
            "1" => {
                inventaire.afficher_produits();
            },
            "2" => {
                print!("Nom du produit: ");
                io::stdout().flush().unwrap();
                let nom = lire_entree();
                
                print!("Quantité: ");
                io::stdout().flush().unwrap();
                let quantite_str = lire_entree();
                
                if let Ok(quantite) = quantite_str.parse::<u32>() {
                    inventaire.ajouter_produit(Produit::new(nom, quantite));
                    println!("Produit ajouté avec succès!");
                } else {
                    println!("Quantité invalide!");
                }
            },
            "3" => {
                print!("Nom du produit à modifier: ");
                io::stdout().flush().unwrap();
                let nom = lire_entree();
                
                print!("Nouvelle quantité: ");
                io::stdout().flush().unwrap();
                let quantite_str = lire_entree();
                
                if let Ok(quantite) = quantite_str.parse::<u32>() {
                    if inventaire.modifier_produit(&nom, quantite) {
                        println!("Produit modifié avec succès!");
                    } else {
                        println!("Produit non trouvé!");
                    }
                } else {
                    println!("Quantité invalide!");
                }
            },
            "4" => {
                print!("Nom du produit à supprimer: ");
                io::stdout().flush().unwrap();
                let nom = lire_entree();
                
                if inventaire.supprimer_produit(&nom) {
                    println!("Produit supprimé avec succès!");
                } else {
                    println!("Produit non trouvé!");
                }
            },
            "5" => {
                if let Err(e) = inventaire.sauvegarder(fichier_inventaire) {
                    println!("Erreur lors de la sauvegarde: {}", e);
                } else {
                    println!("Inventaire sauvegardé avec succès dans '{}'!", fichier_inventaire);
                }
            },
            "6" => {
                println!("Sauvegarde des données avant de quitter...");
                if let Err(e) = inventaire.sauvegarder(fichier_inventaire) {
                    println!("Erreur lors de la sauvegarde finale: {}", e);
                }
                println!("Au revoir!");
                break;
            },
            _ => {
                println!("Choix invalide, veuillez réessayer.");
            }
        }
    }
}
