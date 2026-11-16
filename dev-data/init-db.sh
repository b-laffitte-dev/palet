#!/bin/sh
# Seed de la base au premier démarrage du conteneur PostgreSQL.
#
# L'image postgres exécute (source) les scripts de /docker-entrypoint-initdb.d
# uniquement quand le volume de données est vide (première initialisation).
# Voir le service `db` dans docker-compose.yml.
#
# Ordre : schéma d'abord (migration baseline sqlx, montée dans /migrations),
# puis les données (data.sql contient tous les INSERT, ordonnés selon les FK).
# Le backend, au démarrage, marque cette baseline comme appliquée dans
# _sqlx_migrations puis joue les migrations suivantes (voir db.rs).

set -e

echo "init-db: chargement du schéma (migration baseline 0001)..."
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
    --file=/migrations/0001_baseline.sql || exit 1

echo "init-db: chargement des données (data.sql)..."
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
    --file=/dev-data/data.sql || exit 1

echo "init-db: seed terminé."
