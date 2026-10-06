#!/usr/bin/env bash
set -euo pipefail

# Génère les types TS du web depuis l'OpenAPI exposée par le backend.
# Usage : backend lancé en local (http://localhost:3000) puis ./scripts/generate-types.sh

BASE_URL="${BASE_URL:-http://localhost:3000}"
OUT="src/schema.d.ts"

curl -fsS "$BASE_URL/api-docs/openapi.json" -o /tmp/openapi.json
cd web
npx openapi-typescript /tmp/openapi.json -o "$OUT"
echo "Types générés dans web/$OUT"
