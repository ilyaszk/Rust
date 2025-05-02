use std::io::{self, Write};
use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use thiserror::Error;
use dotenv::dotenv;

// Définition des erreurs personnalisées
#[derive(Error, Debug)]
enum EmailError {
    #[error("Erreur d'entrée/sortie: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Erreur de configuration SMTP: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    
    #[error("Erreur de création du message: {0}")]
    MessageBuildError(#[from] lettre::error::Error),
    
    #[error("Variable d'environnement manquante: {0}")]
    EnvVarError(String),
}

// Structure pour stocker les informations d'un email
struct Email {
    from: String,
    to: String,
    subject: String,
    body: String,
}

impl Email {
    // Méthode pour créer un nouvel email en demandant des informations à l'utilisateur
    fn new_from_input() -> Result<Self, EmailError> {
        let mut to = String::new();
        let mut subject = String::new();
        let mut body = String::new();
        
        print!("Destinataire: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut to)?;
        
        print!("Sujet: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut subject)?;
        
        println!("Corps du message (terminez par une ligne contenant uniquement '.'): ");
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            
            if line.trim() == "." {
                break;
            }
            
            body.push_str(&line);
        }
        
        // Récupération de l'adresse email de l'expéditeur depuis les variables d'environnement
        let from = env::var("EMAIL_FROM").map_err(|_| EmailError::EnvVarError("EMAIL_FROM".to_string()))?;
        
        Ok(Email {
            from,
            to: to.trim().to_string(),
            subject: subject.trim().to_string(),
            body,
        })
    }
    
    // Méthode pour envoyer l'email
    fn send(&self) -> Result<(), EmailError> {
        // Récupération des informations SMTP depuis les variables d'environnement
        let smtp_username = env::var("SMTP_USERNAME").map_err(|_| EmailError::EnvVarError("SMTP_USERNAME".to_string()))?;
        let smtp_password = env::var("SMTP_PASSWORD").map_err(|_| EmailError::EnvVarError("SMTP_PASSWORD".to_string()))?;
        let smtp_server = env::var("SMTP_SERVER").map_err(|_| EmailError::EnvVarError("SMTP_SERVER".to_string()))?;
        
        // Création du message email
        let email = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(&self.subject)
            .body(self.body.clone())?;
            
        // Configuration du transporteur SMTP
        let creds = Credentials::new(smtp_username, smtp_password);
        
        let mailer = SmtpTransport::relay(&smtp_server)?
            .credentials(creds)
            .build();
            
        // Envoi de l'email
        mailer.send(&email)?;
        
        Ok(())
    }
}

fn main() {
    // Charger les variables d'environnement depuis le fichier .env
    dotenv().ok();
    
    println!("=== Envoi d'Email depuis le Terminal ===");
    
    match Email::new_from_input() {
        Ok(email) => {
            println!("Envoi de l'email en cours...");
            
            match email.send() {
                Ok(_) => println!("Email envoyé avec succès!"),
                Err(e) => eprintln!("Erreur lors de l'envoi de l'email: {}", e),
            }
        },
        Err(e) => eprintln!("Erreur lors de la saisie des informations: {}", e),
    }
}