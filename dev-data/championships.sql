--
-- PostgreSQL database dump
--

\restrict Qlx40vrxcEySdcf9TodTv9IplMZ4i46ibzPgiAdDAIkc6vqi4MdX1fxDNpR39Ba

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
-- Data for Name: championships; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.championships (id, name, code, season, description, league, divisions, total_journees, journee_duration, classification_primary, classification_secondary, classification_tertiary, status, current_journee, current_phase, organizer_id, created_at, updated_at) VALUES ('0ce2a49c-23de-4fb0-8438-4580f7259e14', 'Championnat de Vendée D1 2026-2027', 'CV-D1-2627', '2026-2027', 'Championnat départemental de palet vendéen, division 1 — saison 2026-2027. Matchs en 13 points.', 'Vendée', '{D1}', 14, '15 jours', 'points', 'diff', 'points_for', 'in_progress', 7, NULL, '3fa85f64-5717-4562-b3fc-2c963f66afa6', '2026-11-08 11:30:30.601835+00', '2026-11-08 11:30:30.709597+00');
INSERT INTO public.championships (id, name, code, season, description, league, divisions, total_journees, journee_duration, classification_primary, classification_secondary, classification_tertiary, status, current_journee, current_phase, organizer_id, created_at, updated_at) VALUES ('dacf3cbf-7122-43c1-85bb-8949e71878b7', 'Championnat des Deux-Sèvres 2026-2027', 'CDSP-2627', '2026-2027', 'Championnat départemental de palet vendéen des Deux-Sèvres (79), divisions D1 à D3. Matchs en 13 points.', 'Deux-Sèvres', '{D1,D2,D3}', 14, '15 jours', 'points', 'diff', 'points_for', 'in_progress', 1, NULL, '3fa85f64-5717-4562-b3fc-2c963f66afa6', '2026-11-08 16:00:47.118162+00', '2026-11-08 16:00:47.155214+00');


--
-- PostgreSQL database dump complete
--

\unrestrict Qlx40vrxcEySdcf9TodTv9IplMZ4i46ibzPgiAdDAIkc6vqi4MdX1fxDNpR39Ba

