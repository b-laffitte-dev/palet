#!/bin/bash
# Lance l'environnement de développement avec live reload.
#
#   ./scripts/dev.sh          -> front Dioxus en live reload sur http://localhost:8081
#                                (API + DB restent celles du docker compose sur :3000 et :5432)
#   ./scripts/dev.sh full     -> démarre aussi l'API en local (cargo run -p palet-backend)
#
# Prérequis (installés une seule fois) :
#   rustup target add wasm32-unknown-unknown   (déjà dans rust-toolchain)
#   cargo install dioxus-cli --locked --version 0.7.10

set -euo pipefail
cd "$(dirname "$0")/.."

if ! command -v dx >/dev/null 2>&1; then
    echo "dioxus-cli (dx) introuvable. Installation :"
    echo "  cargo install dioxus-cli --locked --version 0.7.10"
    exit 1
fi

if [[ "${1:-}" == "full" ]]; then
    echo "Démarrage de l'API backend en local (port 3000)..."
    cargo run -p palet-backend &
    API_PID=$!
    trap 'kill $API_PID 2>/dev/null || true' EXIT
    sleep 2
fi

echo "Live reload disponible sur http://localhost:8081"
cd rust-frontend
exec dx serve --port 8081
