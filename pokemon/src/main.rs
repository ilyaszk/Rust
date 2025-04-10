use rand::Rng;
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
        println!("##############################");
        println!("Pokémon");
        println!("=========================");
        println!("ID: {}", self.id);
        println!("Nom: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {:?}", self.type_pokemon);
        println!("Experience: {}", self.experience);
        println!("Genre: {:?}", self.genre);
        println!("----------------------");
        println!("##############################");
        println!("##############################");
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

        let genre_aleatoire = if rand::thread_rng().gen_bool(0.5) {
            Genre::Male
        } else {
            Genre::Femelle
        };

        let new_poke = Pokemon::new("????".to_string(), poke1.type_pokemon, genre_aleatoire);
        println!("Un nouveau Pokémon est né !!!");
        new_poke.afficher();
        self.ajouter_pokemon(new_poke);
    }
}



fn afficher_menu() {
    println!("Choisissez une option :");
    println!("1. Ajouter un Pokémon");
    println!("2. Afficher les Pokémon");
    println!("3. Entraîner les Pokémon");
    println!("4. Reproduire des Pokémon");
    println!("5. Quitter");
}

fn saisie_pokemon() -> Option<Pokemon> {
    use std::io::{self, Write};

    println!("Menu de saisie d'un Pokémon :");
    println!("=========================");

    print!("==> Nom du Pokémon : ");
    io::stdout().flush().unwrap();
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    let nom = nom.trim().to_string();

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

    let mut genre_input = String::new();
    io::stdin().read_line(&mut genre_input).expect("Erreur de lecture");
    let genre = match genre_input.trim() {
        "1" => Genre::Male,
        "2" => Genre::Femelle,
        _ => {
            println!("Genre invalide !");
            return None;
        }
    };

    Some(Pokemon::new(nom, type_pokemon, genre))
}

fn reproduction(pokedex: &mut PokeDeck) {
    if pokedex.pokemons.len() < 2 {
        println!("Vous devez avoir au moins deux Pokémon pour les reproduire.");
        return;
    }
    println!("Sélectionnez deux Pokémon à reproduire en entrant leurs IDs :");

    println!("Sélectionnez le premier Pokémon :");
    pokedex.afficher_pokemons();
    let mut id1_input = String::new();
    std::io::stdin()
        .read_line(&mut id1_input)
        .expect("Erreur de lecture");
    let id1 = id1_input.trim().parse::<Uuid>().expect("ID invalide");
    let poke1 = pokedex
        .pokemons
        .iter()
        .find(|&p| p.id == id1)
        .expect("Pokémon non trouvé")
        .clone();


    println!("Sélectionnez le deuxième Pokémon :");
    pokedex.afficher_pokemons();
    let mut id2_input = String::new();
    std::io::stdin()
        .read_line(&mut id2_input)
        .expect("Erreur de lecture");
    let id2 = id2_input.trim().parse::<Uuid>().expect("ID invalide");
    let poke2 = pokedex
        .pokemons
        .iter()
        .find(|&p| p.id == id2)
        .expect("Pokémon non trouvé")
        .clone();


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
                pokedex.entrainer_pokemons( rand::thread_rng().gen_range(20..=100));
                println!("Entraînement terminé ! Verifiez vos pokémons, ils ont gagné de l'expérience et peut-être meme un niveau !");
                pokedex.afficher_pokemons();
              
            }
            "4" => {
                reproduction(&mut pokedex);
            }
            "5" =>{
                println!("Au revoir !");
                continue_game = false;                
            }
            _ => println!("Choix invalide !"),
        }
        
    }
}
