# adsb
Simulateur offensif ADS-B développé en Rust

# ADS-B Ghost Injector

## Description
ADS-B Ghost Injector est un outil offensif de simulation ADS-B développé en Rust.

Le projet permet de simuler des attaques contre le protocole ADS-B utilisé dans l’aviation civile afin d’étudier les vulnérabilités liées à l’absence d’authentification et de chiffrement.

L’outil génère du trafic aérien simulé et permet la création d’anomalies telles que :
- faux avions
- spoofing de position
- vitesses impossibles
- altitudes incohérentes
- flood de trafic ADS-B

Les données générées sont exportées au format CSV pour analyse.

## Fonctionnalités
- Génération de trafic ADS-B simulé
- Création de faux avions
- Simulation d’attaques ADS-B
- Spoofing de données aéronautiques
- Flood de trafic
- Export CSV
- Architecture modulaire en Rust

## Technologies utilisées
- Rust
- Cargo
- CSV crate
- Serde
- Rand
- Chrono
- Clap

## Structure du projet
src/
├── main.rs
├── aircraft.rs
├── generator.rs
├── attacks.rs
├── export.rs
└── server.rs

## Installation

### Cloner le repository

```bash
git clone https://github.com/Soukaynarafik/adsb.git
```
## Utilisation
Le projet utilise une interface en ligne de commande (CLI) permettant de basculer entre le mode de visualisation en direct et les différents modes d'attaque.

### 1. Lancer le radar Web interactif (Visualisation en direct)
Pour démarrer le serveur Web asynchrone et observer le déplacement des aéronefs ainsi que les anomalies en temps réel sur la carte :

Bash
cargo run -- live
Une fois lancé, accédez à l'interface radar directement depuis votre navigateur à l'adresse : http://localhost:8080

### 2. Simuler une attaque de type "Flood" (Injection massive de pistes fantômes)
Pour générer un trafic saturé par de fausses cibles et exporter les vecteurs de suivi dans un fichier CSV :

Bash
cargo run -- flood --intensity 50

## Objectif pédagogique
Ce projet a été réalisé dans un cadre pédagogique afin d’étudier :
- les vulnérabilités du protocole ADS-B
- les techniques de spoofing
- la simulation d’attaques cyber
- la génération de trafic malveillant

## Workflow Git
Le projet suit un workflow Git basé sur :
- Issues
- Branches feature
- Pull Requests
- Code Reviews
- Merge vers dev puis main

## Avertissement légal
Ce projet est destiné exclusivement à des fins éducatives et de recherche.

Aucune émission radio réelle n’est effectuée.
Aucune interaction avec des systèmes aéronautiques réels n’est réalisée.

L’utilisation de cet outil en dehors d’un environnement de test contrôlé est interdite.

## Auteurs
- RAFIK Soukayna
- LAURANSOT Vanessa

## Repository GitHub
https://github.com/Soukaynarafik/adsb

## Licence

Projet pédagogique.
