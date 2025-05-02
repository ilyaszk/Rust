use std::io::{self, Write};
use rand::{Rng, thread_rng};
use std::collections::HashSet;

// Fonction pour lire une entrée utilisateur
fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin()
        .read_line(&mut entree)
        .expect("Échec de la lecture de l'entrée");
    entree.trim().to_string()
}

// Fonction pour générer un mot de passe aléatoire (simplifiée)
fn generer_mot_de_passe(longueur: usize, caracteres_exclus: &HashSet<char>) -> String {
    // Définir les ensembles de caractères
    let sets = [
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "0123456789",
        "!@#$%^&*()_-+=<>?"
    ];
    
    // Créer le pool de caractères en filtrant les caractères exclus
    let caracteres: Vec<char> = sets.iter()
        .flat_map(|s| s.chars())
        .filter(|c| !caracteres_exclus.contains(c))
        .collect();
    
    // Vérifier qu'il reste des caractères
    if caracteres.is_empty() {
        return "Erreur: tous les caractères ont été exclus!".to_string();
    }
    
    let mut rng = thread_rng();
    
    // Créer un tableau pour stocker le mot de passe
    let mut password: Vec<char> = Vec::with_capacity(longueur);
    
    // Ajouter au moins un caractère de chaque type si possible
    for set in &sets {
        // S'arrêter si la longueur du mot de passe est atteinte
        if password.len() >= longueur {
            break;
        }
        
        // Essayer d'ajouter un caractère de ce set s'il n'est pas entièrement exclu
        let chars: Vec<char> = set.chars().filter(|c| !caracteres_exclus.contains(c)).collect();
        if !chars.is_empty() {
            password.push(chars[rng.gen_range(0..chars.len())]);
        }
    }
    
    // Compléter jusqu'à la longueur demandée
    while password.len() < longueur {
        password.push(caracteres[rng.gen_range(0..caracteres.len())]);
    }
    
    // Mélanger pour éviter un motif prévisible
    password.sort_by(|_, _| {
        if rng.r#gen() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    
    // Convertir en String et retourner
    password.into_iter().collect()
}

fn main() {
    println!("=== GÉNÉRATEUR DE MOT DE PASSE ===");
    
    let mut continuer = true;
    
    while continuer {
        // Demander la longueur du mot de passe
        print!("Entrez la longueur souhaitée du mot de passe: ");
        io::stdout().flush().unwrap();
        let longueur_str = lire_entree();
        
        let longueur = match longueur_str.parse::<usize>() {
            Ok(len) => len,
            Err(_) => {
                println!("Longueur invalide, veuillez entrer un nombre entier positif.");
                continue;
            }
        };
        
        if longueur == 0 {
            println!("La longueur doit être supérieure à zéro.");
            continue;
        }
        
        // Option pour exclure certains caractères
        print!("Voulez-vous exclure certains caractères? [o/N]: ");
        io::stdout().flush().unwrap();
        let exclure_reponse = lire_entree().to_lowercase();
        
        let mut caracteres_exclus = HashSet::new();
        
        if exclure_reponse == "o" || exclure_reponse == "oui" {
            print!("Entrez les caractères à exclure (sans espace): ");
            io::stdout().flush().unwrap();
            let exclus = lire_entree();
            
            for c in exclus.chars() {
                caracteres_exclus.insert(c);
            }
            
            println!("Les caractères suivants seront exclus: {:?}", caracteres_exclus);
        }
        
        // Générer et afficher le mot de passe
        let mot_de_passe = generer_mot_de_passe(longueur, &caracteres_exclus);
        println!("\nVotre mot de passe: {}", mot_de_passe);
        
        // Demander si l'utilisateur veut générer un autre mot de passe
        print!("\nGénérer un autre mot de passe? [o/N]: ");
        io::stdout().flush().unwrap();
        let reponse = lire_entree().to_lowercase();
        
        continuer = reponse == "o" || reponse == "oui";
        
        if continuer {
            println!("\n------------------------------\n");
        }
    }
    
    println!("Au revoir!");
}
