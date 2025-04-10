use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TypePokemon {
    Feu,
    Eau,
    Plante,
    Electrik,
    Tenebre,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    id: Uuid,
    nom: String,
    niveau: u32,
    type_pokemon: TypePokemon,
    experience: u32,
    genre: Genre,
}

impl Pokemon {
    fn new(nom: String, type_pokemon: TypePokemon, genre: Genre) -> Pokemon {
        Pokemon {
            id: Uuid::new_v4(),
            nom,
            niveau: 1,
            type_pokemon,
            experience: 0,
            genre,
        }
    }

    fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        while self.experience >= 100 {
            self.niveau += 1;
            self.experience -= 100;
        }
    }

    fn afficher(&self) {
        println!("=========================");
        println!("Pokémon  ===>");
        println!("ID: {}", self.id);
        println!("Nom: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {:?}", self.type_pokemon);
        println!("Experience: {}", self.experience);
        println!("Genre: {:?}", self.genre);
        println!("=========================");
    }
}

struct PokeDeck {
    pokemons: Vec<Pokemon>,
}

impl PokeDeck {
    fn new() -> PokeDeck {
        PokeDeck {
            pokemons: Vec::new(),
        }
    }

    fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemons.push(pokemon);
    }

    fn afficher_pokemons(&self) {
        for pokemon in &self.pokemons {
            pokemon.afficher();
        }
    }

    fn entrainer_pokemons(&mut self, xp: u32) {
        for pokemon in &mut self.pokemons {
            pokemon.gagner_experience(xp);
        }
    }

    fn peut_reproduire(poke1: &Pokemon, poke2: &Pokemon) -> bool {
        poke1.type_pokemon == poke2.type_pokemon
            && poke1.genre != poke2.genre
            && poke1.niveau >= 5
            && poke2.niveau >= 5
    }

    fn reproduire(&mut self, poke1: &Pokemon, poke2: &Pokemon) {
        if !Self::peut_reproduire(poke1, poke2) {
            println!("Les Pokémon ne peuvent pas se reproduire.");
            return;
        }

        // Générer aléatoirement le genre
        let genre_aleatoire = if rand::thread_rng().gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        // Générer aléatoirement le nom
        let noms = [
            "Pikachu", "Bulbizarre", "Salamèche", "Carapuce", 
            "Évoli", "Miaous", "Rondoudou", "Psykokwak",
            "Rattata", "Miaouss", "Goupix", "Nosferapti"
        ];
        let nom_aleatoire = noms[rand::thread_rng().gen_range(0..noms.len())].to_string();

        let new_poke = Pokemon::new(nom_aleatoire, poke1.type_pokemon, genre_aleatoire);
        println!("Un nouveau Pokémon est né !!!");
        new_poke.afficher();
        self.ajouter_pokemon(new_poke);
    }

    // Nouvelles fonctionnalités pour trier les Pokémon
    fn trier_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| b.niveau.cmp(&a.niveau));
    }

    fn trier_par_type(&mut self) {
        self.pokemons.sort_by(|a, b| {
            let type_a = format!("{:?}", a.type_pokemon);
            let type_b = format!("{:?}", b.type_pokemon);
            type_a.cmp(&type_b)
        });
    }
    
    // Sauvegarder les Pokémon dans un fichier
    fn sauvegarder(&self, fichier: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(fichier)?;

        writeln!(file, "ID,Nom,Niveau,Type,Experience,Genre")?;
        
        for pokemon in &self.pokemons {
            writeln!(
                file,
                "{},{},{},{:?},{},{:?}",
                pokemon.id,
                pokemon.nom,
                pokemon.niveau,
                pokemon.type_pokemon,
                pokemon.experience,
                pokemon.genre
            )?;
        }
        
        println!("Pokédex sauvegardé dans le fichier: {}", fichier);
        Ok(())
    }
    
    // Charger les Pokémon depuis un fichier
    fn charger(&mut self, fichier: &str) -> io::Result<()> {
        if !Path::new(fichier).exists() {
            println!("Le fichier {} n'existe pas.", fichier);
            return Ok(());
        }

        if self.pokemons.len() > 0 {
            println!("Le Pokédex n'est pas vide. Veuillez le vider avant de charger un nouveau fichier.");
            return Ok(());
        }
        
        let mut file = File::open(fichier)?;
        let mut contenu = String::new();
        file.read_to_string(&mut contenu)?;
        
        let mut lignes = contenu.lines();
        
        // Ignorer l'en-tête
        if let Some(_) = lignes.next() {
            for ligne in lignes {
                let parts: Vec<&str> = ligne.split(',').collect();
                if parts.len() == 6 {
                    let id = Uuid::parse_str(parts[0]).unwrap_or_else(|_| Uuid::new_v4());
                    let nom = parts[1].to_string();
                    let niveau = parts[2].parse::<u32>().unwrap_or(1);
                    
                    let type_pokemon = match parts[3] {
                        "Feu" => TypePokemon::Feu,
                        "Eau" => TypePokemon::Eau,
                        "Plante" => TypePokemon::Plante,
                        "Electrik" => TypePokemon::Electrik,
                        "Tenebre" => TypePokemon::Tenebre,
                        _ => TypePokemon::Feu,
                    };
                    
                    let experience = parts[4].parse::<u32>().unwrap_or(0);
                    
                    let genre = match parts[5] {
                        "Male" => Genre::Male,
                        "Femelle" => Genre::Femelle,
                        _ => Genre::Male,
                    };
                    
                    let pokemon = Pokemon {
                        id,
                        nom,
                        niveau,
                        type_pokemon,
                        experience,
                        genre,
                    };
                    
                    self.pokemons.push(pokemon);
                }
            }
        }
        
        println!("{} Pokémon chargés depuis le fichier: {}", self.pokemons.len(), fichier);
        Ok(())
    }
}

fn afficher_menu() {
    println!("\nPokémon Manager - Menu Principal");
    println!("=================================");
    println!("1. Ajouter un Pokémon");
    println!("2. Afficher les Pokémon");
    println!("3. Entraîner les Pokémon");
    println!("4. Reproduire des Pokémon");
    println!("5. Trier les Pokémon par niveau");
    println!("6. Trier les Pokémon par type");
    println!("7. Sauvegarder les Pokémon");
    println!("8. Charger les Pokémon");
    println!("9. Quitter");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();
}

fn saisie_pokemon() -> Option<Pokemon> {
    println!("\nMenu de saisie d'un Pokémon :");
    println!("=========================");

    print!("==> Nom du Pokémon (ou 'aléatoire' pour un nom aléatoire): ");
    io::stdout().flush().unwrap();
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    let nom = nom.trim().to_string();

    let nom_final = if nom.to_lowercase() == "aléatoire" || nom.to_lowercase() == "aleatoire" {
        let noms = [
            "Pikachu", "Bulbizarre", "Salamèche", "Carapuce", 
            "Évoli", "Miaous", "Rondoudou", "Psykokwak",
            "Rattata", "Miaouss", "Goupix", "Nosferapti"
        ];
        noms[rand::thread_rng().gen_range(0..noms.len())].to_string()
    } else {
        nom
    };

    println!("==> Type du Pokémon :");
    println!("1. Feu");
    println!("2. Eau");
    println!("3. Plante");
    println!("4. Electrik");
    println!("5. Tenebre");

    let mut type_input = String::new();
    io::stdin().read_line(&mut type_input).expect("Erreur de lecture");
    let type_pokemon = match type_input.trim() {
        "1" => TypePokemon::Feu,
        "2" => TypePokemon::Eau,
        "3" => TypePokemon::Plante,
        "4" => TypePokemon::Electrik,
        "5" => TypePokemon::Tenebre,
        _ => {
            println!("Type invalide !");
            return None;
        }
    };

    println!("==> Genre du Pokémon :");
    println!("1. Male");
    println!("2. Femelle");
    println!("3. Aléatoire");

    let mut genre_input = String::new();
    io::stdin().read_line(&mut genre_input).expect("Erreur de lecture");
    let genre = match genre_input.trim() {
        "1" => Genre::Male,
        "2" => Genre::Femelle,
        "3" => if rand::thread_rng().gen_bool(0.5) { Genre::Male } else { Genre::Femelle },
        _ => {
            println!("Genre invalide !");
            return None;
        }
    };

    Some(Pokemon::new(nom_final, type_pokemon, genre))
}

fn reproduction(pokedex: &mut PokeDeck) {
    if pokedex.pokemons.len() < 2 {
        println!("Vous devez avoir au moins deux Pokémon pour les reproduire.");
        return;
    }
    println!("Sélectionnez deux Pokémon à reproduire en entrant leurs IDs :");
    pokedex.afficher_pokemons();
    println!("Sélectionnez le premier Pokémon :");
    let mut id1_input = String::new();
    std::io::stdin()
        .read_line(&mut id1_input)
        .expect("Erreur de lecture");
    let id1 = match id1_input.trim().parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => {
            println!("ID invalide!");
            return;
        }
    };
    
    let poke1 = match pokedex.pokemons.iter().find(|&p| p.id == id1) {
        Some(p) => p.clone(),
        None => {
            println!("Pokémon non trouvé!");
            return;
        }
    };

    println!("Sélectionnez le deuxième Pokémon :");
    let mut id2_input = String::new();
    std::io::stdin()
        .read_line(&mut id2_input)
        .expect("Erreur de lecture");
    let id2 = match id2_input.trim().parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => {
            println!("ID invalide!");
            return;
        }
    };
    
    let poke2 = match pokedex.pokemons.iter().find(|&p| p.id == id2) {
        Some(p) => p.clone(),
        None => {
            println!("Pokémon non trouvé!");
            return;
        }
    };

    println!("Vous avez sélectionné :");
    poke1.afficher();
    poke2.afficher();

    pokedex.reproduire(&poke1, &poke2);
}

fn main() {
    let mut pokedex = PokeDeck::new();
    println!("Bienvenue dans le Pokédex !");
    println!("Vous avez {} Pokémon dans votre élevage.", pokedex.pokemons.len());
    println!("=========================");
    println!("Vous pouvez ajouter des Pokémon, les entraîner, les reproduire et les afficher.");
    println!("=========================");
    let mut continue_game = true;

    while continue_game {
        afficher_menu();
        
        let mut choix = String::new();
        std::io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        match choix.trim() {
            "1" => {
                saisie_pokemon()
                    .map(|pokemon| pokedex.ajouter_pokemon(pokemon))
                    .unwrap_or_else(|| println!("Erreur lors de la saisie du Pokémon"));
            }
            "2" => {
                pokedex.afficher_pokemons();
            }
            "3" => {
                println!("Vos pokémons sont partis pour l'entrainement !");
                pokedex.entrainer_pokemons(rand::thread_rng().gen_range(20..=100));
                println!("Entraînement terminé ! Verifiez vos pokémons, ils ont gagné de l'expérience et peut-être meme un niveau !");
                pokedex.afficher_pokemons();
            }
            "4" => {
                reproduction(&mut pokedex);
            }
            "5" => {
                println!("Tri des Pokémon par niveau (décroissant)...");
                pokedex.trier_par_niveau();
                println!("Pokémon triés par niveau :");
                pokedex.afficher_pokemons();
            }
            "6" => {
                println!("Tri des Pokémon par type...");
                pokedex.trier_par_type();
                println!("Pokémon triés par type :");
                pokedex.afficher_pokemons();
            }
            "7" => {
                print!("Nom du fichier pour la sauvegarde: ");
                io::stdout().flush().unwrap();
                let mut fichier = String::new();
                io::stdin().read_line(&mut fichier).expect("Erreur de lecture");
                let fichier = fichier.trim();
                
                if let Err(err) = pokedex.sauvegarder(fichier) {
                    println!("Erreur lors de la sauvegarde: {}", err);
                }
            }
            "8" => {
                print!("Nom du fichier à charger: ");
                io::stdout().flush().unwrap();
                let mut fichier = String::new();
                io::stdin().read_line(&mut fichier).expect("Erreur de lecture");
                let fichier = fichier.trim();
                
                if let Err(err) = pokedex.charger(fichier) {
                    println!("Erreur lors du chargement: {}", err);
                }
            }
            "9" => {
                println!("Au revoir !");
                continue_game = false;                
            }
            _ => println!("Choix invalide !"),
        }
    }
}