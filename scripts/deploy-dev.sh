#!/bin/bash
# Déploie l'environnement dev sur le serveur depuis les images publiées par la CI
# sur le registry custom (job docker-dev de .github/workflows/rust-ci.yml).
#
#   ./scripts/deploy-dev.sh                          -> déploie le tag "dev" (dernier push CI)
#   IMAGE_TAG=dev-<sha> ./scripts/deploy-dev.sh      -> déploie un tag précis
#
# Prérequis : docker + plugin compose v2, et un clone du dépôt sur le serveur
# (le script s'exécute depuis la racine du dépôt).
#
# Credentials du registry : variables d'environnement DOCKER_REGISTRY,
# DOCKER_LOGIN_USER, DOCKER_LOGIN_PASS, ou fichier .env.deploy à la racine
# du dépôt (ignoré par git, voir .gitignore) :
#   DOCKER_REGISTRY=registry.example.com
#   DOCKER_LOGIN_USER=...
#   DOCKER_LOGIN_PASS=...

set -euo pipefail
cd "$(dirname "$0")/.."

if [[ -f .env.deploy ]]; then
    set -a
    # shellcheck disable=SC1091
    source .env.deploy
    set +a
fi

: "${DOCKER_REGISTRY:?DOCKER_REGISTRY requis (variable env ou .env.deploy)}"
: "${DOCKER_LOGIN_USER:?DOCKER_LOGIN_USER requis (variable env ou .env.deploy)}"
: "${DOCKER_LOGIN_PASS:?DOCKER_LOGIN_PASS requis (variable env ou .env.deploy)}"

IMAGE_TAG="${IMAGE_TAG:-dev}"
REGISTRY_NAMESPACE="${REGISTRY_NAMESPACE:-b-laffitte-dev}"
COMPOSE_FILE="docker-compose.dev.yml"

if ! command -v docker >/dev/null 2>&1 || ! docker info >/dev/null 2>&1; then
    echo "docker introuvable ou daemon arrêté."
    exit 1
fi

echo "Login au registry ${DOCKER_REGISTRY} (user: ${DOCKER_LOGIN_USER})..."
echo "$DOCKER_LOGIN_PASS" | docker login "$DOCKER_REGISTRY" -u "$DOCKER_LOGIN_USER" --password-stdin

echo "Pull des images '${REGISTRY_NAMESPACE}/palet-*:${IMAGE_TAG}'..."
docker compose -f "$COMPOSE_FILE" pull

echo "Redémarrage des services (tag ${IMAGE_TAG})..."
docker compose -f "$COMPOSE_FILE" up -d --wait

echo "Nettoyage des images non utilisées..."
docker image prune -f

echo "OK — environnement dev à jour (tag ${IMAGE_TAG}) :"
docker compose -f "$COMPOSE_FILE" ps
