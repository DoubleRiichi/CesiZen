# Supervision CESIZen

Stack d'observabilité décrite au chapitre 4.2 du plan de déploiement/sécurisation.
Elle est **séparée de la stack applicative** (`docker-compose.deploy.yaml`) et peut
être mise à jour ou redémarrée sans toucher à l'application.

| Outil | Pilier | Port local |
|---|---|---|
| Prometheus | Métriques (node_exporter, cAdvisor, sondes blackbox) | 127.0.0.1:9090 |
| Loki + Promtail | Logs des conteneurs | interne |
| Grafana | Tableaux de bord | 127.0.0.1:3001 |
| Alertmanager | Alertes Discord/e-mail | 127.0.0.1:9093 |
| Uptime Kuma | Sonde de disponibilité + page de statut | 127.0.0.1:3002 |

Tous les ports sont liés à `127.0.0.1` : l'accès distant se fait par tunnel SSH
(`ssh -L 3001:127.0.0.1:3001 serveur`) ou en exposant Grafana / Uptime Kuma
derrière le reverse proxy TLS, comme le front.

## Installation (serveur de production)

1. La stack applicative doit tourner (le réseau externe `cesizen-prod_default`
   doit exister) :

   ```bash
   docker compose -f docker-compose.deploy.yaml up -d
   ```

2. Créer le fichier de webhook Discord (non versionné, cf. `.gitignore`) :

   ```bash
   cp monitoring/alertmanager/discord_webhook_url.example monitoring/alertmanager/discord_webhook_url
   # puis y coller l'URL du webhook du salon #alertes
   ```

3. Ajouter au `.env` du serveur :

   ```
   GRAFANA_ADMIN_USER=admin
   GRAFANA_ADMIN_PASSWORD=<mot de passe fort>
   ```

4. Démarrer :

   ```bash
   docker compose -f docker-compose.monitoring.yaml up -d
   ```

5. Vérifier : `http://127.0.0.1:9090/targets` (toutes les cibles **UP**),
   puis se connecter à Grafana sur `http://127.0.0.1:3001`
   (Prometheus et Loki sont déjà provisionnés comme sources de données).

## Tableaux de bord Grafana recommandés

À importer via *Dashboards → Import* (ID du catalogue grafana.com) :

- **1860** — Node Exporter Full (santé du serveur)
- **14282** — cAdvisor (ressources par conteneur)
- **13639** — Logs (Loki) : exploration et volumétrie des journaux

## Alertes configurées (Prometheus → Alertmanager → Discord)

| Alerte | Condition | Sévérité |
|---|---|---|
| APIIndisponible | `/health` KO pendant 1 min | critical |
| FrontIndisponible | front KO pendant 2 min | critical |
| ConteneurEnRedemarrageEnBoucle | > 3 redémarrages en 15 min | critical |
| DisqueBientotPlein | < 15 % libre sur `/` pendant 5 min | warning |
| MemoireSaturee | < 10 % de RAM disponible | warning |
| CibleSupervisionDown | exporteur injoignable 3 min | warning |

**Taux d'erreurs 5xx** : l'API n'exposant pas encore de métriques Prometheus,
cette alerte est portée par **Grafana Alerting** sur la source Loki, avec une
requête LogQL à adapter au format de log de l'API, par exemple :

```
sum(count_over_time({compose_project="cesizen-prod", service="api"} |~ ` 5\d\d ` [5m])) > 10
```

À terme, l'API exposera ses propres métriques (`/metrics`, crate
`axum-prometheus`) pour une alerte 5xx native côté Prometheus.

## Uptime Kuma

Au premier lancement (`127.0.0.1:3002`), créer le compte administrateur puis
deux moniteurs HTTP(S) sur les **URL publiques** :

- `https://<domaine-front>/` — front
- `https://<domaine-api>/health` — API

Activer la vérification d'expiration du certificat TLS et, si besoin, la
notification Discord (même webhook). La *status page* intégrée peut servir de
page de statut publique.

## Lien avec la gestion des incidents

Toute alerte **critical** ouvre immédiatement un ticket GitHub « Anomalie »
(sévérité *Bloquant*) et fait courir les délais de prise en compte du plan de
maintenance. Les seuils sont revus lors des post-mortems pour réduire les faux
positifs.
