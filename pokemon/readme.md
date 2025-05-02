# Gestionnaire de Pokémon - README

## Présentation du projet

Ce projet est un système de gestion de Pokémon développé en Rust. Il permet de créer, gérer, entraîner et faire reproduire des Pokémon, tout en offrant diverses fonctionnalités comme le tri et la sauvegarde des données.

## Fonctionnalités

Le programme offre les fonctionnalités suivantes :

1. **Création de Pokémon**
   - Ajout manuel avec nom, type et genre personnalisés
   - Génération aléatoire de noms et de genres
   
2. **Gestion des Pokémon**
   - Affichage de tous les Pokémon du PokeDeck
   - Entraînement collectif pour gagner de l'expérience et des niveaux
   
3. **Reproduction**
   - Création de nouveaux Pokémon par reproduction entre deux Pokémon compatibles
   - Vérification des conditions de reproduction (même type, genres différents, niveau minimum)
   
4. **Organisation**
   - Tri des Pokémon par niveau (décroissant)
   - Tri des Pokémon par type
   
5. **Persistance des données**
   - Sauvegarde du PokeDeck dans un fichier CSV
   - Chargement des Pokémon depuis un fichier CSV

## Structure du code

Le projet est organisé autour de plusieurs structures et énumérations :

- `TypePokemon` : Énumération des types de Pokémon (Feu, Eau, Plante, Electrik, Tenebre)
- `Genre` : Énumération des genres de Pokémon (Male, Femelle)
- `Pokemon` : Structure représentant un Pokémon avec ses attributs
- `PokeDeck` : Structure gérant une collection de Pokémon

## Prérequis

Pour compiler et exécuter ce projet, vous aurez besoin de :

- Rust (édition 2021 ou supérieure)
- Les dépendances suivantes (à ajouter dans votre Cargo.toml) :
  ```toml
  [dependencies]
  rand = "0.8.5"
  uuid = { version = "1.3.3", features = ["v4", "serde"] }
  ```

## Installation et exécution

1. Clonez ce dépôt ou téléchargez les fichiers source
2. Assurez-vous que Rust est installé sur votre système
3. Dans le répertoire du projet, exécutez :
   ```
   cargo build
   cargo run
   ```

## Utilisation

Une fois le programme lancé, vous pourrez naviguer à travers les différentes options du menu :

```
Pokémon Manager - Menu Principal
=================================
1. Ajouter un Pokémon
2. Afficher les Pokémon
3. Entraîner les Pokémon
4. Reproduire des Pokémon
5. Trier les Pokémon par niveau
6. Trier les Pokémon par type
7. Sauvegarder les Pokémon
8. Charger les Pokémon
9. Quitter
Votre choix:
```

### Exemple d'utilisation

1. Ajoutez quelques Pokémon (option 1)
2. Entraînez-les pour augmenter leur niveau (option 3)
3. Lorsque vous avez des Pokémon de niveau 5 ou plus, vous pouvez les faire se reproduire (option 4)
4. Organisez votre collection avec les options de tri (options 5 et 6)
5. Sauvegardez votre collection pour une utilisation ultérieure (option 7)

## Format de fichier de sauvegarde

Les données sont sauvegardées au format CSV avec les champs suivants :
```
ID,Nom,Niveau,Type,Experience,Genre
```

Exemple :
```
12345678-1234-1234-1234-123456789abc,Pikachu,5,Electrik,50,Male
```

## Améliorations possibles

Voici quelques idées pour étendre ce projet :
- Ajouter des capacités (attaques) aux Pokémon
- Implémenter un système de combat
- Créer une interface graphique
- Ajouter plus de types et caractéristiques (statistiques, évolutions)
- Implémenter un système de recherche et de filtrage