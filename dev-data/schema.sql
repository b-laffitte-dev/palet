--
-- PostgreSQL database dump
--

\restrict w8EJKDJNmjqj3n5rQWMsRN4UmDLevxdS9Lky258isP0XdNLDAWdIj3mSjCRidUJ

-- Dumped from database version 17.11 (Debian 17.11-1.pgdg13+2)
-- Dumped by pg_dump version 17.11 (Debian 17.11-1.pgdg13+2)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: classification_primary; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.classification_primary AS ENUM (
    'points',
    'win_rate',
    'diff'
);


--
-- Name: classification_secondary; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.classification_secondary AS ENUM (
    'diff',
    'points_for',
    'head_to_head'
);


--
-- Name: classification_tertiary; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.classification_tertiary AS ENUM (
    'points_for',
    'points_against'
);


--
-- Name: competition_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.competition_status AS ENUM (
    'pending',
    'registration',
    'in_progress',
    'completed',
    'cancelled'
);


--
-- Name: match_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.match_status AS ENUM (
    'scheduled',
    'in_progress',
    'completed',
    'postponed',
    'cancelled',
    'disputed'
);


--
-- Name: team_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.team_status AS ENUM (
    'active',
    'inactive',
    'disbanded'
);


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: championship_classifications; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.championship_classifications (
    championship_id uuid NOT NULL,
    division text NOT NULL,
    journee integer NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    teams jsonb NOT NULL
);


--
-- Name: championship_teams; Type: TABLE; Schema: public; Owner: -
--

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


--
-- Name: championships; Type: TABLE; Schema: public; Owner: -
--

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


--
-- Name: clubs; Type: TABLE; Schema: public; Owner: -
--

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


--
-- Name: divisions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.divisions (
    id uuid NOT NULL,
    name text NOT NULL,
    league text NOT NULL,
    season text NOT NULL,
    "position" integer DEFAULT 1 NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);


--
-- Name: matches; Type: TABLE; Schema: public; Owner: -
--

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


--
-- Name: championship_classifications championship_classifications_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.championship_classifications
    ADD CONSTRAINT championship_classifications_pkey PRIMARY KEY (championship_id, division, journee);


--
-- Name: championship_teams championship_teams_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.championship_teams
    ADD CONSTRAINT championship_teams_pkey PRIMARY KEY (championship_id, team_id);


--
-- Name: championships championships_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.championships
    ADD CONSTRAINT championships_pkey PRIMARY KEY (id);


--
-- Name: clubs clubs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.clubs
    ADD CONSTRAINT clubs_pkey PRIMARY KEY (id);


--
-- Name: divisions divisions_league_season_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.divisions
    ADD CONSTRAINT divisions_league_season_name_key UNIQUE (league, season, name);


--
-- Name: divisions divisions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.divisions
    ADD CONSTRAINT divisions_pkey PRIMARY KEY (id);


--
-- Name: matches matches_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_pkey PRIMARY KEY (id);


--
-- Name: championship_classifications championship_classifications_championship_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.championship_classifications
    ADD CONSTRAINT championship_classifications_championship_id_fkey FOREIGN KEY (championship_id) REFERENCES public.championships(id) ON DELETE CASCADE;


--
-- Name: championship_teams championship_teams_championship_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.championship_teams
    ADD CONSTRAINT championship_teams_championship_id_fkey FOREIGN KEY (championship_id) REFERENCES public.championships(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict w8EJKDJNmjqj3n5rQWMsRN4UmDLevxdS9Lky258isP0XdNLDAWdIj3mSjCRidUJ

