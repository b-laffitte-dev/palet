# syntax=docker/dockerfile:1
# ── busybox statique ─────────────────────────────────────────────────────────
# Variante musl = binaire 100 % statique (la variante par défaut est glibc :
# sans son loader dans un conteneur scratch, elle serait inexécutable).
# Copiée telle quelle dans le stage final pour le healthcheck (wget).
FROM busybox:1.37-musl AS busybox

# ── Build stage ──────────────────────────────────────────────────────────────
# rust:alpine = musl natif → le binaire produit est 100 % statique,
# directement exécutable dans un FROM scratch (runtime).
# Tag flottant "alpine3.21" (et non une version épinglée) : il pointe vers le
# dernier stable publié (vérifié : même digest que 1.99.1-alpine3.21), fidèle
# au rust-toolchain du repo (channel = "stable") et à la CI — sinon le lockfile
# dérive inévitablement hors du MSRV de l'image (ex. sqlx 0.9 exige rustc ≥ 1.94,
# image 1.90 cassée). Les alias "stable-*" n'existent pas sur l'image rust.
FROM rust:alpine3.21 AS builder

# sqlx utilise tls-rustls → pas besoin de openssl-dev.
# build-base = gcc/musl-dev pour le linker ; ca-certificates est copié
# tel quel dans l'image finale pour les appels TLS sortants ;
# curl = requis par le build.rs d'utoipa-swagger-ui (télécharge Swagger UI).
RUN apk add --no-cache build-base ca-certificates curl \
    && addgroup -S app && adduser -S app -G app

WORKDIR /app

# ── Cache des dépendances ────────────────────────────────────────────────────
# On copie d'abord les manifests + des stubs pour que cargo résolve et télécharge
# toutes les dépendances AVANT de copier le vrai code source.
COPY Cargo.toml Cargo.lock ./
COPY rust-backend/Cargo.toml rust-backend/Cargo.toml
COPY rust-frontend/Cargo.toml rust-frontend/Cargo.toml
COPY rust-shared/Cargo.toml   rust-shared/Cargo.toml

RUN mkdir -p rust-backend/src rust-frontend/src rust-shared/src \
    && echo 'fn main() {}' > rust-backend/src/main.rs \
    && touch rust-backend/src/lib.rs \
    && touch rust-frontend/src/lib.rs \
    && touch rust-shared/src/lib.rs \
    && cargo fetch --locked \
    && rm -rf rust-backend/src rust-frontend/src rust-shared/src

# ── Vrai code source ─────────────────────────────────────────────────────────
# .dockerignore exclut déjà target/, .git/, docs/, etc. — le contexte reste léger.
COPY . .

# BuildKit cache mounts : target/ et le registre crates.io sont montés en cache,
# ils ne polluent pas les layers de l'image finale.
# Comme pour le frontend, on ne cache QUE registry/cache et registry/index :
# registry/src (sources extraites) reste dans la layer du conteneur pour
# éviter qu'un build interrompu n'y laisse un .cargo-ok corrompu qui bloquerait
# tous les builds suivants ("File exists (os error 17)").
# On prépare en plus un répertoire /runtime autonome (binaire + config + certs +
# passwd/group + tmp) : le stage final n'aura ainsi qu'UN SEUL COPY → 1 layer.
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cargo build --release --locked --bin palet-backend \
    && strip /app/target/release/palet-backend \
    && mkdir -p /runtime/app /runtime/bin /runtime/etc/ssl/certs /runtime/tmp \
    && cp target/release/palet-backend /runtime/app/palet-backend \
    && cp /etc/ssl/certs/ca-certificates.crt /runtime/etc/ssl/certs/ \
    && cp /etc/passwd /etc/group /runtime/etc/ \
    && chmod 1777 /runtime/tmp

COPY --chown=app:app rust-backend/config /runtime/app/config
COPY --from=busybox /bin/busybox /runtime/bin/busybox

# ── Runtime stage : scratch ──────────────────────────────────────────────────
# Empreinte minimale : UNE SEULE layer contenant uniquement le binaire strippé,
# busybox (healthcheck), les certificats CA et /etc/passwd.
# Le binaire musl est statique : aucune lib système, aucun shell requis.
FROM scratch

COPY --from=builder /runtime/ /

WORKDIR /app
USER app

EXPOSE 3000
ENV RUST_LOG=info
ENV PALET__ENVIRONMENT=production

# wget busybox vers 127.0.0.1 : en scratch il n'y a pas de /etc/hosts,
# "localhost" ne serait pas résoluble — d'où l'IP directe.
HEALTHCHECK --interval=10s --timeout=5s --start-period=10s --retries=5 \
    CMD ["/bin/busybox", "wget", "-q", "-O-", "http://127.0.0.1:3000/health"]

ENTRYPOINT ["/app/palet-backend"]
