-- Schéma initial : fédérations, clubs, joueurs
CREATE TYPE statut_tournoi AS ENUM ('planifie', 'en_cours', 'termine', 'annule');

CREATE TABLE federations (
    id          uuid PRIMARY KEY,
    nom         text NOT NULL,
    slug        text NOT NULL UNIQUE,
    description text,
    cree_le     timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE clubs (
    id          uuid PRIMARY KEY,
    federation_id uuid NOT NULL REFERENCES federations (id),
    nom         text NOT NULL,
    slug        text NOT NULL UNIQUE,
    ville       text,
    cree_le     timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE joueurs (
    id          uuid PRIMARY KEY,
    club_id     uuid NOT NULL REFERENCES clubs (id),
    nom         text NOT NULL,
    prenom      text NOT NULL,
    licence     text UNIQUE,
    cree_le     timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX idx_clubs_federation ON clubs (federation_id);
CREATE INDEX idx_joueurs_club ON joueurs (club_id);
