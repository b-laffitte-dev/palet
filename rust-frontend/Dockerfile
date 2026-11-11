# syntax=docker/dockerfile:1
# ── Build stage ──────────────────────────────────────────────────────────────
# slim-trixie (Debian 13, glibc ≥ 2.38) : requis sur aarch64 où wasm-bindgen
# 0.2.100 n'existe qu'en build gnu (le build musl aarch64 n'est pas publié).
FROM rust:1.90-slim-trixie AS builder

# wasm-pack via binaire précompilé (évite un cargo install de ~2 min à chaque build).
# wasm-bindgen-cli est requis par wasm-pack — binaire précompilé aussi.
ARG WASM_PACK_VERSION=0.13.1
ARG WASM_BINDGEN_VERSION=0.2.100
RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && ARCH="$(uname -m)" \
    && case "$ARCH" in \
         x86_64)  WP_ARCH="x86_64-unknown-linux-musl";  WB_ARCH="x86_64-unknown-linux-musl" ;; \
         aarch64) WP_ARCH="aarch64-unknown-linux-musl"; WB_ARCH="aarch64-unknown-linux-gnu" ;; \
         *) echo "arch non supportée: $ARCH" >&2; exit 1 ;; \
       esac \
    && curl -fsSL "https://github.com/rustwasm/wasm-pack/releases/download/v${WASM_PACK_VERSION}/wasm-pack-v${WASM_PACK_VERSION}-${WP_ARCH}.tar.gz" \
       | tar xz -C /usr/local/bin --strip-components=1 \
    && curl -fsSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${WB_ARCH}.tar.gz" \
       | tar xz -C /usr/local/bin --strip-components=1 \
    && rustup target add wasm32-unknown-unknown

WORKDIR /app

# ── Cache des dépendances ────────────────────────────────────────────────────
COPY Cargo.toml Cargo.lock ./
COPY rust-backend/Cargo.toml rust-backend/Cargo.toml
COPY rust-frontend/Cargo.toml rust-frontend/Cargo.toml
COPY rust-shared/Cargo.toml   rust-shared/Cargo.toml

RUN mkdir -p rust-backend/src rust-frontend/src rust-shared/src \
    && echo 'fn main() {}'                     > rust-backend/src/main.rs \
    && touch rust-backend/src/lib.rs \
    && touch rust-frontend/src/lib.rs \
    && touch rust-shared/src/lib.rs \
    && cargo fetch --locked \
    && rm -rf rust-backend/src rust-frontend/src rust-shared/src

# ── Vrai code source ─────────────────────────────────────────────────────────
# .dockerignore exclut déjà target/, .git/, docs/, etc.
COPY . .

# BuildKit cache mounts pour le target et le registre.
# On ne monte en cache QUE registry/cache (les .crate téléchargés) et
# registry/index : registry/src (sources extraites par cargo) reste dans la
# layer du conteneur, car un build interrompu y laisse un marqueur .cargo-ok
# corrompu qui fait échouer tous les builds suivants avec
# "failed to open ... .cargo-ok: File exists (os error 17)".
# Le binaire produit reste dans le cache mount : on le copie explicitement
# hors du mount (dans /www) à la fin du RUN, sinon le COPY --from=builder
# du stage final ne le trouverait pas.
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cd rust-frontend && wasm-pack build --release --target web --out-name palet_frontend --out-dir ./pkg

# ── Docroot préparé DANS le builder ─────────────────────────────────────────
#  - pré-compression gzip ici → les .gz partagent la MÊME layer que les
#    originaux (un RUN gzip dans le runtime aurait créé une layer qui
#    doublerait la taille des assets dans l'image) ;
#  - un seul COPY du docroot dans le runtime → 1 layer au lieu de 3 ;
#  - droits posés ici → plus de RUN chmod dans le runtime.
RUN mkdir -p /www \
    && cp -r rust-frontend/pkg /www/pkg \
    && cp rust-frontend/index.html /www/index.html \
    && cp -r rust-frontend/public/. /www/ \
    && find /www -type f \( -name '*.html' -o -name '*.css' -o -name '*.js' -o -name '*.wasm' -o -name '*.json' -o -name '*.svg' \) \
       -exec gzip -9 -k {} \; \
    && chmod -R a+rX /www

# ── Sources nginx (binaire + conf de base) ───────────────────────────────────
FROM nginx:1.27-alpine AS nginxsrc

# ── Runtime stage : alpine + nginx minimal ───────────────────────────────────
# nginx:alpine complet pèse ~79 Mo (modules dynamiques, pages par défaut, etc.).
# Ici on ne garde que le binaire nginx (1,2 Mo, musl) + ses 4 libs dynamiques
# (pcre2, libssl, libcrypto, zlib) + la conf de base : ~20 Mo au total.
FROM alpine:3.21

RUN apk add --no-cache ca-certificates pcre2 libssl3 libcrypto3 zlib \
    && addgroup -S nginx && adduser -S nginx -G nginx \
    && mkdir -p /var/lib/nginx /var/log/nginx /usr/share/nginx/html \
    && mkdir -p /var/cache/nginx/client_temp /var/cache/nginx/proxy_temp \
       /var/cache/nginx/fastcgi_temp /var/cache/nginx/uwsgi_temp /var/cache/nginx/scgi_temp \
    && chown -R nginx:nginx /var/lib/nginx /var/log/nginx /usr/share/nginx/html /var/cache/nginx

COPY --from=nginxsrc /usr/sbin/nginx /usr/sbin/nginx
COPY --from=nginxsrc /etc/nginx /etc/nginx

# Docroot complet (pré-compressé) en UNE layer + conf du projet.
COPY --from=builder /www /usr/share/nginx/html
COPY rust-frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
