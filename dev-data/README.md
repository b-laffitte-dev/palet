# dev-data — Extraction SQL de la base de dev

Dump de la base PostgreSQL de développement (conteneur `palet-db-1`,
base `palet`, user `palet`), généré avec `pg_dump` (PostgreSQL 17).

## Contenu

| Fichier | Contenu |
|---|---|
| `schema.sql` | Schéma complet : tables, types enum, contraintes, index (`--schema-only`) |
| `data.sql` | Toutes les données en INSERT, ordonnées selon les FK (`--data-only --column-inserts`) |
| `<table>.sql` | Données d'une seule table (restore sélectif) |

Tables extraites : `championships`, `clubs`, `divisions`,
`championship_teams`, `matches`, `championship_classifications`.

Ordre de chargement si restore table par table (dépendances FK) :
`championships` → `clubs` → `divisions` → `championship_teams` →
`matches` → `championship_classifications`.

## Ré-extraire depuis la base de dev

```bash
# Schéma + données complètes
docker exec palet-db-1 pg_dump -U palet -d palet --schema-only --no-owner --no-privileges > dev-data/schema.sql
docker exec palet-db-1 pg_dump -U palet -d palet --data-only --column-inserts --no-owner --no-privileges > dev-data/data.sql

# Une table à la fois
for t in championships clubs divisions championship_teams matches championship_classifications; do
  docker exec palet-db-1 pg_dump -U palet -d palet --data-only --column-inserts --no-owner --no-privileges -t "$t" > "dev-data/$t.sql"
done
```

## Restaurer dans une base vide

```bash
docker exec -i palet-db-1 psql -U palet -d <base_cible> -f - < dev-data/schema.sql
docker exec -i palet-db-1 psql -U palet -d <base_cible> -f - < dev-data/data.sql
# ou, hors conteneur :
# psql "postgres://palet:palet@localhost:5432/<base_cible>" -f dev-data/schema.sql -f dev-data/data.sql
```

> Cible : une base **vide** (le dump ne contient pas de `DROP`/`TRUNCATE`).

## Seed automatique (docker compose)

`init-db.sh` est monté dans `/docker-entrypoint-initdb.d` du service `db`
(docker-compose.yml) : à la première initialisation du volume `pgdata`,
PostgreSQL l'exécute et charge `schema.sql` puis `data.sql` dans la base
`palet`. Re-seeder from scratch : `docker compose down -v && docker compose up`.
