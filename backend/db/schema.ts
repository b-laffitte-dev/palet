import { pgEnum, pgTable, text, timestamp, uuid } from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm";

export const statutTournoi = pgEnum("statut_tournoi", [
  "planifie",
  "en_cours",
  "termine",
  "annule",
]);

export const federations = pgTable("federations", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  description: text("description"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
});

export const clubs = pgTable("clubs", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  federationId: uuid("federation_id")
    .notNull()
    .references(() => federations.id),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  ville: text("ville"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
});

export const joueurs = pgTable("joueurs", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  clubId: uuid("club_id")
    .notNull()
    .references(() => clubs.id),
  nom: text("nom").notNull(),
  prenom: text("prenom").notNull(),
  licence: text("licence").unique(),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
});
