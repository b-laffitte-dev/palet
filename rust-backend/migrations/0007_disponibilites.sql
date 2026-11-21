-- Vague 3 : disponibilités des joueurs pour un match (Module 8 SportEasy, E3).
-- Un joueur déclare oui / non / peut-être (+ commentaire libre) pour chaque match.

CREATE TABLE public.match_disponibilites (
    match_id uuid NOT NULL REFERENCES public.matches(id) ON DELETE CASCADE,
    player_id uuid NOT NULL REFERENCES public.players(id) ON DELETE CASCADE,
    status text NOT NULL CHECK (status IN ('disponible', 'indisponible', 'incertain')),
    commentaire text,
    updated_at timestamp with time zone NOT NULL,
    PRIMARY KEY (match_id, player_id)
);

CREATE INDEX match_disponibilites_match_idx ON public.match_disponibilites (match_id);
CREATE INDEX match_disponibilites_player_idx ON public.match_disponibilites (player_id);
