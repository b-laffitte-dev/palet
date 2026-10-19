#!/bin/bash

# Script de vérification du projet Rust Palet Vendéen
# Ce script vérifie la compilation, les tests, le formatage et le linting

set -e

echo "========================================"
echo "Vérification du projet Rust Palet Vendéen"
echo "========================================"

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Fonction pour afficher les erreurs
log_error() {
    echo -e "${RED}[ERREUR]${NC} $1"
    exit 1
}

# Fonction pour afficher les succès
log_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

# Fonction pour afficher les avertissements
log_warning() {
    echo -e "${YELLOW}[AVERTISSEMENT]${NC} $1"
}

# Fonction pour afficher les infos
log_info() {
    echo -e "[INFO] $1"
}

# Vérifier que cargo est disponible
if ! command -v cargo &> /dev/null; then
    log_error "cargo n'est pas installé. Veuillez installer Rust via rustup."
fi

# Vérifier que rustup est disponible
if ! command -v rustup &> /dev/null; then
    log_error "rustup n'est pas installé."
fi

# Vérifier la version de Rust
log_info "Vérification de la version de Rust..."
rustc --version

# Vérifier la toolchain
log_info "Vérification de la toolchain Rust..."
if [ -f rust-toolchain ]; then
    cat rust-toolchain
    rustup show active-toolchain
else
    log_warning "Aucun fichier rust-toolchain trouvé. Utilisation de la toolchain par défaut."
fi

# Vérification du workspace Rust
echo ""
echo "========================================"
echo "Vérification du workspace Rust"
echo "========================================"

# Vérifier le formatage
echo ""
log_info "Vérification du formatage (cargo fmt --check)..."
if cargo fmt --all -- --check 2>&1; then
    log_success "Formatage vérifié avec succès"
else
    log_error "Problèmes de formatage détectés. Exécutez 'cargo fmt --workspace' pour corriger."
fi

# Vérifier le linting avec clippy
echo ""
log_info "Vérification avec clippy (cargo clippy)..."
if cargo clippy --all-targets -- --deny warnings 2>&1; then
    log_success "Clippy vérifié avec succès (aucun avertissement)"
else
    log_error "Problèmes détectés par clippy. Corriguez les avertissements."
fi

# Vérifier la compilation
echo ""
log_info "Vérification de la compilation (cargo check)..."
if cargo check 2>&1; then
    log_success "Compilation vérifiée avec succès"
else
    log_error "Erreurs de compilation détectées."
fi

# Vérifier la compilation en mode release
echo ""
log_info "Vérification de la compilation en mode release (cargo check --release)..."
if cargo check --release 2>&1; then
    log_success "Compilation en mode release vérifiée avec succès"
else
    log_error "Erreurs de compilation en mode release détectées."
fi

# Vérifier les tests unitaires
echo ""
log_info "Exécution des tests unitaires (cargo test)..."
if cargo test 2>&1; then
    log_success "Tous les tests unitaires sont passés"
else
    log_error "Certains tests unitaires ont échoué."
fi

# Vérifier la documentation
echo ""
log_info "Vérification de la documentation (cargo doc --no-deps)..."
if cargo doc --no-deps 2>&1; then
    log_success "Documentation générée avec succès"
else
    log_warning "Problèmes de documentation détectés (non bloquants)."
fi

# Vérification des dépendances
echo ""
log_info "Vérification des dépendances obsolètes..."
cargo outdated --exit-code 1 2>&1 || log_warning "Certaines dépendances sont obsolètes."

# Vérification des vulnérabilités de sécurité
echo ""
log_info "Vérification des vulnérabilités de sécurité (cargo audit)..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit 2>&1; then
        log_success "Aucune vulnérabilité de sécurité détectée"
    else
        log_error "Vulnérabilités de sécurité détectées."
    fi
else
    log_warning "cargo-audit n'est pas installé. Installation: cargo install cargo-audit"
fi

echo ""
echo "========================================"
echo "Vérification terminée avec succès!"
echo "========================================"
