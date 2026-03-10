Commit initial de l'application CesiZEN,
Contient un premier jet de l'API, implémentant l'accès aux tables :
article, feeling, feeling_category, feeling_tracker, tag, user

Chaque table est organisée dans son propre modulue, possédant les mêmes composants :

**dto.rs**        -> défini les structures des requêtes et réponses JSON sérialisées/désérialisées

**handler.rs**    -> défini les fonctions d'entrées des différent endpoints, accepte la requête et appelle la couche service.

**router.rs**     -> défini les routes de l'api et dispatche les requêtes vers les handlers

**model.rs**      -> défini les modèles, représentant les tables de la bdd mais aussi les différentes jointures

**repository.rs** -> défini les requêtes SQL CRUD, utilise les modèles

**service.rs**    -> défini la logique métier propre au module et endpoint, performe la validation des requêtes et de gestion des rôles

l'API est documentée avec Swagger, dont le schéma est défini grâce au traits ToSchema présent dans les DTOs, du côté des handler, et dans docs.rs
.