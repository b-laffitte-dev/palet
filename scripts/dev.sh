#!/bin/bash
# Lance l'environnement de développement avec live reload.
#
#   ./scripts/dev.sh          -> front Dioxus en live reload sur http://localhost:8081
#                                (API + DB restent celles du docker compose sur :3000 et :5432)
#   ./scripts/dev.sh full     -> démarre aussi la base PostgreSQL (docker compose, :5432)
#                                et l'API en local (cargo run -p palet-backend, :3000)
#
# En mode full, la DB et l'API lancées par ce script sont arrêtées à la sortie
# (Ctrl+C) ; les données PostgreSQL restent dans le volume docker `pgdata`.
# Si la DB tourne déjà (docker compose up), elle est réutilisée et laissée telle quelle.
#
# Prérequis (installés une seule fois) :
#   rustup target add wasm32-unknown-unknown   (déjà dans rust-toolchain)
#   cargo install dioxus-cli --locked --version 0.7.10
#   docker + plugin compose v2                    (mode full uniquement)

set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v dx >/dev/null 2>&1; then
    echo "dioxus-cli (dx) introuvable. Installation :"
    echo "  cargo install dioxus-cli --locked --version 0.7.10"
    exit 1
fi

if [[ "${1:-}" == "full" ]]; then
    if ! command -v docker >/dev/null 2>&1 || ! docker info >/dev/null 2>&1; then
        echo "docker introuvable ou daemon arrêté : requis pour lancer la base en mode full."
        echo "  Alternative : docker compose up -d db, puis relancez ./scripts/dev.sh full"
        exit 1
    fi

    DB_STARTED=0
    if [[ -z "$(docker compose ps --status running -q db 2>/dev/null)" ]]; then
        echo "Démarrage de PostgreSQL 17 (docker compose, port 5432)..."
        docker compose up -d --wait db
        DB_STARTED=1
    else
        echo "PostgreSQL déjà actif (docker compose) : réutilisé sur le port 5432."
    fi

    echo "Démarrage de l'API backend en local (port 3000)..."
    cargo run -p palet-backend &
    API_PID=$!

    cleanup() {
        kill "$API_PID" 2>/dev/null || true
        if [[ "$DB_STARTED" == "1" ]]; then
            echo "Arrêt de PostgreSQL (données conservées dans le volume pgdata)..."
            docker compose stop db >/dev/null 2>&1 || true
        fi
    }
    trap cleanup EXIT
    trap 'exit 130' INT TERM

    sleep 2
fi

echo "Live reload disponible sur http://localhost:8081"
cd rust-frontend
dx serve --port 8081
