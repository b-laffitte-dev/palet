-- Baseline : schéma historique du projet (ex dev-data/schema.sql, dump PG 17).
-- Appliqué par sqlx au démarrage du backend ; en dev docker, la base est
-- pré-seedée par dev-data/init-db.sh qui charge cette même migration, et le
-- backend la marque alors comme déjà appliquée (voir db.rs).

CREATE TYPE public.classification_primary AS ENUM (
    'points',
    'win_rate',
    'diff'
);

CREATE TYPE public.classification_secondary AS ENUM (
    'diff',
    'points_for',
    'head_to_head'
);

CREATE TYPE public.classification_tertiary AS ENUM (
    'points_for',
    'points_against'
);

CREATE TYPE public.competition_status AS ENUM (
    'pending',
    'registration',
    'in_progress',
    'completed',
    'cancelled'
);

CREATE TYPE public.match_status AS ENUM (
    'scheduled',
    'in_progress',
    'completed',
    'postponed',
    'cancelled',
    'disputed'
);

CREATE TYPE public.team_status AS ENUM (
    'active',
    'inactive',
    'disbanded'
);

CREATE TABLE public.championship_classifications (
    championship_id uuid NOT NULL,
    division text NOT NULL,
    journee integer NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    teams jsonb NOT NULL
);

CREATE TABLE public.championship_teams (
    championship_id uuid NOT NULL,
    team_id uuid NOT NULL,
    division text NOT NULL,
    "position" integer DEFAULT 0 NOT NULL,
    stats jsonb DEFAULT '{}'::jsonb NOT NULL,
    form jsonb DEFAULT '[]'::jsonb NOT NULL,
    status text DEFAULT 'safe'::text NOT NULL,
    joined_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TABLE public.championships (
    id uuid NOT NULL,
    name text NOT NULL,
    code text,
    season text NOT NULL,
    description text,
    league text NOT NULL,
    divisions text[] NOT NULL,
    total_journees integer NOT NULL,
    journee_duration text,
    classification_primary text NOT NULL,
    classification_secondary text NOT NULL,
    classification_tertiary text NOT NULL,
    status text NOT NULL,
    current_journee integer,
    current_phase text,
    organizer_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE public.clubs (
    id uuid NOT NULL,
    name text NOT NULL,
    nickname text,
    code text NOT NULL,
    logo text,
    address text,
    city text NOT NULL,
    postal_code text,
    league text NOT NULL,
    division text NOT NULL,
    status text NOT NULL,
    players_count integer DEFAULT 0 NOT NULL,
    teams_count integer DEFAULT 0 NOT NULL,
    contact_email text,
    contact_phone text,
    website text,
    social jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE public.divisions (
    id uuid NOT NULL,
    name text NOT NULL,
    league text NOT NULL,
    season text NOT NULL,
    "position" integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    UNIQUE (league, season, name)
);

CREATE TABLE public.matches (
    id uuid NOT NULL,
    competition_id uuid NOT NULL,
    competition_type text NOT NULL,
    journee integer,
    phase text,
    round text,
    team1_id uuid NOT NULL,
    team2_id uuid NOT NULL,
    team1_score integer,
    team2_score integer,
    winner text,
    manches jsonb,
    best_of integer,
    current_manche integer,
    current_scores jsonb,
    status text NOT NULL,
    verified boolean DEFAULT false NOT NULL,
    verified_by uuid,
    verified_at timestamp with time zone,
    date timestamp with time zone NOT NULL,
    "time" text,
    location text,
    duration text,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    started_at timestamp with time zone,
    ended_at timestamp with time zone
);

ALTER TABLE ONLY public.championship_classifications
    ADD CONSTRAINT championship_classifications_pkey PRIMARY KEY (championship_id, division, journee);

ALTER TABLE ONLY public.championship_teams
    ADD CONSTRAINT championship_teams_pkey PRIMARY KEY (championship_id, team_id);

ALTER TABLE ONLY public.championships
    ADD CONSTRAINT championships_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.clubs
    ADD CONSTRAINT clubs_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.divisions
    ADD CONSTRAINT divisions_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.championship_classifications
    ADD CONSTRAINT championship_classifications_championship_id_fkey FOREIGN KEY (championship_id) REFERENCES public.championships(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.championship_teams
    ADD CONSTRAINT championship_teams_championship_id_fkey FOREIGN KEY (championship_id) REFERENCES public.championships(id) ON DELETE CASCADE;
