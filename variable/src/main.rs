use std::io::{ stdin, stdout, Write};

fn main(){
    //voulez-vous lire ou ecrire un fichier
    stdout().write(b"Voulez-vous lire ou ecrire un fichier? (lire/ecrire): ").unwrap();
    stdout().flush().unwrap();
    let mut choix = String::new();
    stdin().read_line(&mut choix).unwrap();
    let choix = choix.trim();

    if choix == "lire" {
        afficher_fichiers();
        lire_fichier();
    } else if choix == "ecrire" {
        ecrire_fichier();
    } else {
        println!("Choix invalide");
    }
 
 }

 fn afficher_fichiers(){
    // Lister les fichiers dans le repertoire courant
    let fichiers = std::fs::read_dir(".").unwrap();
    for fichier in fichiers {
        let fichier = fichier.unwrap();
        let nom_fichier = fichier.file_name();
        println!("Nom du fichier: {:?}", nom_fichier);
    }
 }


fn lire_fichier(){
    // Demander le nom du fichier
    stdout().write(b"Entrez le nom du fichier: ").unwrap();
    stdout().flush().unwrap();
    let mut nom_fichier = String::new();
    stdin().read_line(&mut nom_fichier).unwrap();
    let nom_fichier = nom_fichier.trim();

    // Lire le contenu du fichier
    let contenu = std::fs::read_to_string(nom_fichier).unwrap();
    println!("Contenu du fichier: {}", contenu);

}

fn ecrire_fichier(){
    // Demander le nom du fichier
    stdout().write(b"Entrez le nom du fichier: ").unwrap();
    stdout().flush().unwrap();
    let mut nom_fichier = String::new();
    stdin().read_line(&mut nom_fichier).unwrap();
    let nom_fichier = nom_fichier.trim();

    // Demander le contenu du fichier
    stdout().write(b"Entrez le contenu du fichier: ").unwrap();
    stdout().flush().unwrap();
    let mut contenu = String::new();
    stdin().read_line(&mut contenu).unwrap();

    // Ecrire le contenu dans le fichier
    std::fs::write(nom_fichier, contenu).unwrap();
    println!("Contenu du fichier ecrit avec succes");
}
