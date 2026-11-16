-- Vague 0 : tables players / teams / team_players, alignées sur les colonnes
-- lues par player_repository.rs et team_repository.rs (enums en text snake_case).

CREATE TABLE public.players (
    id uuid PRIMARY KEY,
    first_name text NOT NULL,
    last_name text NOT NULL,
    full_name text NOT NULL,
    alias text,
    email text NOT NULL UNIQUE,
    photo text,
    status text NOT NULL DEFAULT 'active',
    birth_date timestamp with time zone NOT NULL,
    age integer NOT NULL,
    birth_place text,
    nationality text NOT NULL DEFAULT 'Française',
    dominant_hand text NOT NULL DEFAULT 'right',
    play_style text,
    phone text,
    address text,
    city text,
    postal_code text,
    club_id uuid NOT NULL REFERENCES public.clubs(id),
    club_number text,
    category text NOT NULL DEFAULT 'senior',
    license_number text NOT NULL DEFAULT '',
    license_date timestamp with time zone NOT NULL,
    license_status text NOT NULL DEFAULT 'valid',
    registration_date timestamp with time zone NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX players_club_id_idx ON public.players (club_id);
CREATE INDEX players_category_idx ON public.players (category);

CREATE TABLE public.teams (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    club_id uuid NOT NULL REFERENCES public.clubs(id),
    captain_id uuid REFERENCES public.players(id) ON DELETE SET NULL,
    logo text,
    stats jsonb NOT NULL DEFAULT '{}'::jsonb,
    status text NOT NULL DEFAULT 'active',
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX teams_club_id_idx ON public.teams (club_id);

-- Effectif d'une équipe (doublette/triplette) : rôle capitaine/membre + poste
-- (pointeur/milieu/tireur) utilisé par la composition de match (module E4).
CREATE TABLE public.team_players (
    team_id uuid NOT NULL REFERENCES public.teams(id) ON DELETE CASCADE,
    player_id uuid NOT NULL REFERENCES public.players(id) ON DELETE CASCADE,
    role text NOT NULL DEFAULT 'member' CHECK (role IN ('captain', 'member')),
    position text,
    joined_at timestamp with time zone NOT NULL,
    PRIMARY KEY (team_id, player_id)
);
