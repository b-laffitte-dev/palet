-- Vague 0 : fédérations (FNSMR nationale, commissions départementales CVDP/CDSP…)
-- et rattachement des clubs.

CREATE TABLE public.federations (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    slug text NOT NULL UNIQUE,
    description text,
    created_at timestamp with time zone NOT NULL
);

ALTER TABLE public.clubs
    ADD COLUMN federation_id uuid REFERENCES public.federations(id) ON DELETE SET NULL;
