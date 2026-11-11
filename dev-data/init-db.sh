#!/bin/sh
# Seed de la base au premier démarrage du conteneur PostgreSQL.
#
# L'image postgres exécute (source) les scripts de /docker-entrypoint-initdb.d
# uniquement quand le volume de données est vide (première initialisation).
# Voir le service `db` dans docker-compose.yml.
#
# Ordre : schema d'abord, puis les données (data.sql contient tous les INSERT,
# ordonnés selon les FK).

set -e

echo "init-db: chargement du schéma (schema.sql)..."
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
    --file=/dev-data/schema.sql || exit 1

echo "init-db: chargement des données (data.sql)..."
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
    --file=/dev-data/data.sql || exit 1

echo "init-db: seed terminé."
