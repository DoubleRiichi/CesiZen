# Changelog

Toutes les évolutions notables du projet CESIZen sont documentées dans ce fichier.

Le format suit la convention [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/)
et le projet applique le [versionnage sémantique](https://semver.org/lang/fr/) (MAJEUR.MINEUR.CORRECTIF).

## [Non publié]

### À venir
- Application mobile (front-end natif consommant l'API existante).
- Modules complémentaires du cahier des charges : questionnaire de diagnostic de stress, exercices de respiration (cohérence cardiaque), activités de détente.
- Chaîne de déploiement continu (CD) vers les environnements de recette et de production.

## [1.0.0] - 2026-04-30

Première version livrée — prototype fonctionnel du jalon 2. Cette version couvre les
deux modules obligatoires (comptes utilisateurs et informations) et le module facultatif
retenu (tracker d'émotions), conformément au cahier des charges du Ministère de la Santé
et de la Prévention.

### Ajouté

#### Architecture et socle technique
- Architecture MVC en trois couches : front-end SvelteKit/TypeScript, API REST Rust/Axum, base PostgreSQL 16.
- API REST comme unique point d'accès aux données, au format JSON, permettant le découplage front/back et l'ajout futur d'un client mobile.
- Organisation modulaire du back-end en six fichiers par module (dto, model, repository, service, handler, router), suivant une approche d'architecture hexagonale.
- Requêtes SQL typées via SQLx, vérifiées à la compilation.
- Système de migrations de base de données versionnées.
- Documentation d'API interactive générée automatiquement (Swagger / OpenAPI) accessible sur `/swagger-ui/`.

#### Module Comptes utilisateurs
- Inscription d'un nouveau compte par un visiteur anonyme.
- Connexion et authentification par token JWT.
- Gestion du profil par l'utilisateur connecté.
- Réinitialisation du mot de passe.
- Gestion des rôles : utilisateur, modérateur, administrateur.
- Création et gestion des comptes utilisateurs et administrateurs depuis le back-office.
- Désactivation et suppression de compte par un administrateur.

#### Module Informations
- Consultation des articles et pages d'information sur la santé mentale (accès public).
- Recherche et filtrage des articles par tags.
- Création, modification et suppression des articles par un administrateur (back-office).

#### Module Tracker d'émotions
- Journal d'émotions personnel pour l'utilisateur connecté.
- Saisie d'une émotion avec intensité (échelle de 1 à 10), notes libres et plage temporelle.
- Modification et suppression de ses propres entrées.
- Visualisation du journal sous forme de calendrier avec indicateurs.
- Filtrage et rapport d'émotions sur une période donnée.
- Configuration de la liste des émotions disponibles par un administrateur.

#### Qualité et tests
- Suite de tests unitaires back-end (`cargo test`) et front-end (Vitest).
- Tests fonctionnels (intégration) couvrant les routes HTTP de bout en bout sur une base de test.
- Tests de non-régression exécutés automatiquement à chaque proposition de modification.
- Cahier de tests formalisé avec identifiants uniques pour les trois modules.

#### Intégration continue et reproductibilité
- Workflow d'intégration continue GitHub Actions déclenché à chaque Pull Request : compilation, application des migrations, analyse statique Clippy (avertissements bloquants), tests unitaires et d'intégration.
- Protection de la branche `master` : aucune fusion possible sans validation complète de la chaîne d'intégration.
- Conteneurisation complète via Docker et Docker Compose, garantissant des environnements identiques entre développement, test et recette.
- Mécanisme de health-check assurant que l'API démarre seulement après la disponibilité de la base.
- Application automatique des migrations au premier lancement du conteneur.

#### Documentation
- Documentation technique : modèle logique des données (MLD), comparatif des solutions techniques et justification du choix retenu, guide d'installation pas à pas (mode Docker et mode développement).
- Documentation de livraison : cahier de tests et procédure de validation avec modèle de procès-verbal de recette.

### Sécurité
- Authentification et autorisation par token JWT.
- Contrôle d'accès par rôle au moyen de guards (`RequireAuth`, `RequireAdmin`) appliqués à chaque route, avec tests unitaires dédiés.
- Vérification de la propriété des ressources : un utilisateur ne peut accéder qu'à ses propres données (hors privilège administrateur).
- Hachage sécurisé des mots de passe avec sel ; aucun mot de passe stocké en clair.
- Validation des entrées utilisateur côté front-end et côté back-end avant écriture en base.
- Fichier de configuration sensible (`.env`) exclu du versionnage via `.gitignore`.

[Non publié]: https://github.com/DoubleRiichi/CesiZen/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/DoubleRiichi/CesiZen/releases/tag/v1.0.0