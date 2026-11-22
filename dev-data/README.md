# dev-data — Extraction SQL de la base de dev

Dump de la base PostgreSQL de développement (conteneur `palet-db-1`,
base `palet`, user `palet`), généré avec `pg_dump` (PostgreSQL 17).

## Contenu

| Fichier | Contenu |
|---|---|
| `data.sql` | Toutes les données en INSERT, ordonnées selon les FK (`--data-only --column-inserts`) |
| `<table>.sql` | Données d'une seule table (restore sélectif) |

Le **schéma** ne vit plus ici : il est géré par les migrations SQLx dans
`rust-backend/migrations/` (`0001_baseline.sql` = schéma historique).

Tables extraites : `championships`, `clubs`, `divisions`,
`championship_teams`, `matches`, `championship_classifications`.

Seeds Vague 1 (générés, pas des dumps) : `players.sql`, `teams.sql` (une équipe
par club, `id = club_id`), `team_players.sql`, `users_seed.sql` (comptes de
démo, mot de passe `Palet2026!`), `news.sql`, `competitions.sql` (tournois).

Ordre de chargement (dépendances FK, utilisé par `init-db.sh`) :
`data.sql` → `players.sql` → `teams.sql` → `team_players.sql` →
`users_seed.sql` → `news.sql` → `competitions.sql`.

> Ces seeds Vague 1 sont appliqués par le **backend** au démarrage
> (`seed_dev_data`, voir `rust-backend/src/db.rs`) uniquement si la table
> `users` est vide — c'est un one-shot : re-seeder from scratch avec
> `docker compose down -v && docker compose up`.

## Ré-extraire depuis la base de dev

```bash
# Données complètes (le schéma se re-extrait via une migration : pg_dump --schema-only
# puis nettoyage, ou édition manuelle de rust-backend/migrations/)
docker exec palet-db-1 pg_dump -U palet -d palet --data-only --column-inserts --no-owner --no-privileges > dev-data/data.sql

# Une table à la fois
for t in championships clubs divisions championship_teams matches championship_classifications; do
  docker exec palet-db-1 pg_dump -U palet -d palet --data-only --column-inserts --no-owner --no-privileges -t "$t" > "dev-data/$t.sql"
done
```

## Restaurer dans une base vide

```bash
docker exec -i palet-db-1 psql -U palet -d <base_cible> -f - < rust-backend/migrations/0001_baseline.sql
docker exec -i palet-db-1 psql -U palet -d <base_cible> -f - < dev-data/data.sql
# ou, hors conteneur :
# psql "postgres://palet:palet@localhost:5432/<base_cible>" -f rust-backend/migrations/0001_baseline.sql -f dev-data/data.sql
```

> Cible : une base **vide** (le dump ne contient pas de `DROP`/`TRUNCATE`).

## Seed automatique (docker compose)

`init-db.sh` est monté dans `/docker-entrypoint-initdb.d` du service `db`
(docker-compose.yml) : à la première initialisation du volume `pgdata`,
PostgreSQL l'exécute et charge la migration baseline
(`rust-backend/migrations/0001_baseline.sql`, montée dans `/migrations`) puis
`data.sql` dans la base `palet`. Re-seeder from scratch :
`docker compose down -v && docker compose up`.
