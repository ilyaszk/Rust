use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Bienvenue dans le jeu du Juste Prix!");
    
    // Génération d'un nombre aléatoire entre 1 et 100
    let nombre_secret = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Veuillez deviner un nombre entre 1 et 100:");
        
        // Lecture de l'entrée utilisateur
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Échec de la lecture de l'entrée");
        
        // Conversion de la chaîne en nombre
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide!");
                continue;
            }
        };
        
        // Comparaison avec le nombre secret
        match guess.cmp(&nombre_secret) {
            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand!"),
            Ordering::Equal => {
                println!("Gagné! Le nombre était {}", nombre_secret);
                break;
            }
        }
    }
}
