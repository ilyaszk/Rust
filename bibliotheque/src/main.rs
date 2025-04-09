struct Livre {
    titre : String,
    auteur : String,
    num_isbn : String,
    date_publication : String,
}
impl Livre {
    fn new(titre: &str, auteur: &str, num_isbn: &str, date_publication: &str) -> Livre {
        Livre {
            titre: titre.into(),//on utilise la méthode into() pour convertir une chaîne de caractères en String
            auteur: auteur.into(),
            num_isbn: num_isbn.into(),
            date_publication: date_publication.into(),
        }
    }

    fn afficher(&self) {
        println!("Titre : {}, Auteur : {}, ISBN : {}, Date de publication : {}", self.titre, self.auteur, self.num_isbn, self.date_publication);
    }
}

struct Bibliotheque {
    livres : Vec<Livre>
}

impl Bibliotheque {
    
    fn new() -> Bibliotheque {
        Bibliotheque { livres : Vec::new() }
    }
    fn ajouter_livre(&mut self, livre : Livre) {
        self.livres.push(livre);
    }
    fn rechercher_livre(&self, titre : &str) -> Option<&Livre> {
        for livre in &self.livres {
            if livre.titre == titre {
                return Some(livre);
            }
        }
        None
    }
    fn afficher_livres(&self) {
        for livre in &self.livres {
            livre.afficher();
        }
    }
    fn retirer_livre(&mut self, titre : &str) {
        self.livres.retain(|livre| livre.titre != titre);
    }

    fn saisirUnLivre(&mut self) {
        
        println!("Titre du livre :");
        let mut titre = String::new();
        std::io::stdin().read_line(&mut titre).expect("Erreur de lecture");

        println!("Auteur du livre :");
        let mut auteur = String::new();
        std::io::stdin().read_line(&mut auteur).expect("Erreur de lecture");

        println!("ISBN du livre :");
        let mut num_isbn = String::new();
        std::io::stdin().read_line(&mut num_isbn).expect("Erreur de lecture");

        println!("Date de publication du livre :");
        let mut date_publication = String::new();
        std::io::stdin().read_line(&mut date_publication).expect("Erreur de lecture");

        let livre = Livre::new(titre.trim(), auteur.trim(), num_isbn.trim(), date_publication.trim());
        
        self.ajouter_livre(livre);
    }
}

fn main() {
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

    let mut continuer = true;
    while continuer {
        afficher_menu();

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                bibliotheque.saisirUnLivre();
            },
            "2" => {
                println!("Titre du livre à rechercher :");
                let mut titre = String::new();
                std::io::stdin().read_line(&mut titre).expect("Erreur de lecture");
                match bibliotheque.rechercher_livre(titre.trim()) {
                    Some(livre) => livre.afficher(),
                    None => println!("Livre non trouvé"),
                }
            },
            "3" => bibliotheque.afficher_livres(),
            "4" => {
                println!("Titre du livre à retirer :");
                let mut titre = String::new();
                std::io::stdin().read_line(&mut titre).expect("Erreur de lecture");
                bibliotheque.retirer_livre(titre.trim());
            },
            "5" => {
                println!("Au revoir !");
                continuer = false;
            }
            _ => println!("Choix invalide"),

        }
    }
}

fn afficher_menu() {
    println!("\nMenu :");
    println!("1- Ajouter un livre");
    println!("2- Rechercher un livre");
    println!("3- Afficher tous les livres");
    println!("4- Retirer un livre");
    println!("5- Quitter");
}
