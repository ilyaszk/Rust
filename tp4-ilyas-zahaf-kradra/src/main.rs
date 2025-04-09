use std::io::{self, Write};
 
struct CompteBancaire {
    solde: f32,
    recap_actions: Vec<String>,
}

impl CompteBancaire {
    fn new() -> CompteBancaire {
        CompteBancaire { solde: 0.0 , recap_actions: Vec::new() }
    }

    fn update_recap(&mut self, action: String) {
        self.recap_actions.push(action);
    }

    fn afficher_solde(&mut self) {
        self.update_recap("Afficher le solde".to_owned());
        println!("Le solde est de {}", self.solde);
    }

    fn faire_depot(&mut self, montant: f32) {
        self.update_recap(format!("Faire un dépot de {}", montant));
        self.solde += montant;
    }

    fn faire_retrait(&mut self, montant: f32) {
        self.update_recap(format!("Faire un retrait de {}", montant));
        if (self.solde - montant).is_sign_negative(){
            println!("Vous ne pouvez pas retirer plus que le solde");
            self.update_recap("Retrait impossible".to_owned());
        } else {
            self.solde -= montant;
        }
    }
}

fn main() {
    let mut compte = CompteBancaire::new();
    let mut continuer = true;

    while continuer {
        println!("Menu:");
        println!("1- Afficher le solde");
        println!("2- Faire un dépot");
        println!("3- Faire un retrait");
        println!("4- Quitter");
        
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        match choix.trim() {
            "1" => compte.afficher_solde(),
            "2" => {
                println!("Montant du dépot:");
                let mut montant = String::new();
                io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                let montant: f32 = montant.trim().parse().expect("Veuillez entrer un nombre valide");
                compte.faire_depot(montant);
            },
            "3" => {
                println!("Montant du retrait:");
                let mut montant = String::new();
                io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                let montant: f32 = montant.trim().parse().expect("Veuillez entrer un nombre valide");
                compte.faire_retrait(montant);
            },
            "4" => {continuer = false;
                    compte.update_recap("Quitter".to_owned());
                    println!("Génération du récapitulatif:");
                    let mut file = std::fs::File::create("ticket.txt").expect("Erreur de création du fichier");
                    for action in &compte.recap_actions {
                        file.write_all(action.as_bytes()).expect("Erreur d'écriture");
                        file.write_all("\n".as_bytes()).expect("Erreur d'écriture");
                    }
                    println!("Récapitulatif généré");
            },
            _ => println!("Option invalide"),
        }
    }
}

