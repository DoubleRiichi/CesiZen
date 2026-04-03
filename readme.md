# CESIZen

> Application web de gestion du stress et de suivi émotionnel,
> commanditée par le Ministère de la Santé et de la Prévention. (Pas vraiment, il s'agit d'un projet effectué dans le cadre de mon Bachelor)

---

## Présentation

CESIZen est une plateforme web **mobile-first** destinée au grand public,
conçue pour améliorer la santé mentale et prévenir le stress.
Elle permet aux utilisateurs de suivre leurs émotions au quotidien,
consulter des articles informatifs, et accéder à des exercices de bien-être.

L'application respecte une architecture **MVC découplée** :
un backend API RESTful totalement indépendant du frontend,
garantissant modularité, scalabilité et conformité RGPD.

### Fonctionnalités principales

- Gestion des comptes utilisateurs (inscription, connexion, rôles)
-  Consultation et gestion d'articles informatifs
-  Tracker d'émotions avec journal de bord et rapports visuels
-  Exercices de cohérence cardiaque et de respiration
-  Back-office administrateur (modération, configuration)

---

## Architecture
cesizen/ \
-> cesizen-app/        # Frontend — SvelteKit + TypeScript \
-> cesizen_api/        # Backend  — Rust (Axum) + PostgreSQL

### Frontend: `cesizen-app`

| Technologie | Rôle |
|-------------|------|
| **SvelteKit 2** | Framework SSR/SPA |
| **TypeScript** | Typage statique |
| **Vite** | Bundler |
| **Vitest** | Tests unitaires & non-régression |

Interface responsive **mobile-first**, compatible Chrome, Firefox, Edge et Safari.
Communique exclusivement avec le backend via l'API REST.

### Backend: `cesizen_api`

| Technologie | Rôle |
|-------------|------|
| **Rust + Axum** | Serveur HTTP asynchrone |
| **SQLx** | ORM async pour PostgreSQL |
| **PostgreSQL** | Base de données relationnelle |
| **JWT** | Authentification stateless |
| **Swagger UI** | Documentation API interactive |

L'API expose ses routes sous `/article`, `/user`, `/tag`, `/feeling`,
`/feeling_tracker`, etc. La documentation Swagger est accessible
sur `/swagger-ui`.

---

## Guide d'installation

### Prérequis

- [Node.js](https://nodejs.org/) ≥ 18 + [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) (édition stable)
- [PostgreSQL](https://www.postgresql.org/) ≥ 15
- *(Optionnel)* [Docker](https://www.docker.com/) + Docker Compose

---

### 1. Cloner le dépôt
```bash
git clone https://github.com/DoubleRiichi/CesiZen.git
cd cesizen
```

---

### 2. Base de données

Créer une base PostgreSQL et noter vos identifiants.
```bash
psql -U postgres -c "CREATE DATABASE cesizen;"
```

---

### 3. Backend (API Rust)
```bash
cd cesizen_api

# Copier et renseigner le fichier d'environnement
cp .env.example .env
# Éditer .env :
#   DATABASE_URL="postgres://user:password@localhost/cesizen"
#   JWT_SECRET="votre_secret_32_caracteres_minimum"

# Compiler et lancer
cargo run
```

L'API sera disponible sur **http://localhost:8080**
La Swagger UI sur **http://localhost:8080/swagger-ui**

---

### 4. Frontend 
```bash
cd cesizen-app

# Installer les dépendances
pnpm install

# Lancer en développement
pnpm dev
```

L'interface sera disponible sur **http://localhost:5173**

---

### 5. (Optionnel) Lancer avec Docker
```bash
# À la racine du projet
docker compose up --build
```

---

### 6. Lancer les tests

**Backend :**
```bash
cd cesizen_api
cargo test
```

**Frontend :**
```bash
cd cesizen-app
pnpm test
```

---

##  Licence

Projet réalisé dans le cadre du titre **Concepteur Développeur d'Applications** — CESI 2025.