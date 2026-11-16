-- Vague 0 : comptes utilisateurs et authentification (JWT access + refresh tokens).
-- Les rôles suivent palet_shared::UserRole (serde snake_case).

CREATE TABLE public.users (
    id uuid PRIMARY KEY,
    email text NOT NULL UNIQUE,
    username text UNIQUE,
    password_hash text NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    role text NOT NULL DEFAULT 'player' CHECK (role IN ('player', 'club_admin', 'commission_admin', 'super_admin', 'arbitre')),
    club_id uuid REFERENCES public.clubs(id) ON DELETE SET NULL,
    license_number text,
    category text NOT NULL DEFAULT 'senior',
    status text NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'suspended', 'banned')),
    email_verified boolean NOT NULL DEFAULT false,
    last_login timestamp with time zone,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX users_club_id_idx ON public.users (club_id);

-- Refresh tokens opaques (bearer), révocables ; un seul actif par session.
CREATE TABLE public.refresh_tokens (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    token text NOT NULL UNIQUE,
    expires_at timestamp with time zone NOT NULL,
    revoked_at timestamp with time zone,
    created_at timestamp with time zone NOT NULL
);

CREATE INDEX refresh_tokens_user_id_idx ON public.refresh_tokens (user_id);
