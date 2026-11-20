-- Vague 1 : compétitions "événement" (tournois, coupes, opens) et inscriptions
-- d'équipes. Les matchs de ces compétitions vivent dans `matches` avec
-- competition_type = 'tournament' | 'cup' | 'friendly'.

CREATE TABLE public.competitions (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    code text UNIQUE,
    competition_type text NOT NULL CHECK (competition_type IN ('championship', 'tournament', 'cup', 'friendly')),
    category text NOT NULL DEFAULT 'doublette' CHECK (category IN ('individual', 'doublette', 'triplette')),
    material text NOT NULL DEFAULT 'fonte' CHECK (material IN ('fonte', 'laiton', 'bois', 'mixte')),
    season text NOT NULL,
    description text,
    location text,
    format text NOT NULL DEFAULT 'round_robin' CHECK (format IN ('round_robin', 'league', 'groups', 'knockout')),
    registration_start timestamp with time zone,
    registration_end timestamp with time zone,
    start_date timestamp with time zone,
    end_date timestamp with time zone,
    status text NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'registration', 'in_progress', 'completed', 'cancelled')),
    max_teams integer,
    fee double precision,
    is_public boolean NOT NULL DEFAULT true,
    organizer_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX competitions_type_status_idx ON public.competitions (competition_type, status);
CREATE INDEX competitions_season_idx ON public.competitions (season);

CREATE TABLE public.registrations (
    competition_id uuid NOT NULL REFERENCES public.competitions(id) ON DELETE CASCADE,
    team_id uuid NOT NULL REFERENCES public.teams(id) ON DELETE CASCADE,
    status text NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'waitlist', 'cancelled')),
    registered_by uuid NOT NULL REFERENCES public.users(id),
    paid boolean NOT NULL DEFAULT false,
    created_at timestamp with time zone NOT NULL,
    confirmed_at timestamp with time zone,
    confirmed_by uuid REFERENCES public.users(id),
    PRIMARY KEY (competition_id, team_id)
);
