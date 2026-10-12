import { integer, pgEnum, pgTable, text, timestamp, uuid } from "drizzle-orm/pg-core";
import { relations } from "drizzle-orm";
import { sql } from "drizzle-orm";

// Enums
export const statutTournoi = pgEnum("statut_tournoi", [
  "planifie",
  "en_cours",
  "termine",
  "annule",
]);

export const statutChampionnat = pgEnum("statut_championnat", [
  "planifie",
  "en_cours",
  "termine",
  "annule",
]);

export const statutMatch = pgEnum("statut_match", [
  "planifie",
  "en_cours",
  "termine",
  "reporte",
  "annule",
]);

export const typeCompetition = pgEnum("type_competition", [
  "tournoi",
  "championnat",
  "amical",
]);

export const categorieJoueur = pgEnum("categorie_joueur", [
  "senior",
  "junior",
  "cadet",
  "minime",
  "veteran",
]);

// Tables
export const federations = pgTable("federations", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  description: text("description"),
  logoUrl: text("logo_url"),
  siteWeb: text("site_web"),
  email: text("email"),
  telephone: text("telephone"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
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
  ville: text("ville").notNull(),
  codePostal: text("code_postal"),
  adresse: text("adresse"),
  logoUrl: text("logo_url"),
  siteWeb: text("site_web"),
  email: text("email"),
  telephone: text("telephone"),
  anneeFondation: integer("annee_fondation"),
  nombreLicencies: integer("nombre_licencies").default(0),
  couleurPrincipale: text("couleur_principale"),
  couleurSecondaire: text("couleur_secondaire"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
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
  categorie: categorieJoueur("categorie").default("senior"),
  dateNaissance: timestamp("date_naissance", { withTimezone: true }),
  email: text("email"),
  telephone: text("telephone"),
  photoUrl: text("photo_url"),
  poste: text("poste"),
  mainDroite: integer("main_droite").default(1),
  nombreVictoires: integer("nombre_victoires").default(0),
  nombreDefaites: integer("nombre_defaites").default(0),
  nombreNuls: integer("nombre_nuls").default(0),
  points: integer("points").default(0),
  classement: integer("classement"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const championnats = pgTable("championnats", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  federationId: uuid("federation_id")
    .notNull()
    .references(() => federations.id),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  saison: text("saison").notNull(),
  description: text("description"),
  statut: statutChampionnat("statut").default("planifie"),
  dateDebut: timestamp("date_debut", { withTimezone: true }).notNull(),
  dateFin: timestamp("date_fin", { withTimezone: true }).notNull(),
  nombreEquipes: integer("nombre_equipes").default(0),
  reglement: text("reglement"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const tournois = pgTable("tournois", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  federationId: uuid("federation_id")
    .notNull()
    .references(() => federations.id),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  description: text("description"),
  statut: statutTournoi("statut").default("planifie"),
  dateDebut: timestamp("date_debut", { withTimezone: true }).notNull(),
  dateFin: timestamp("date_fin", { withTimezone: true }).notNull(),
  lieu: text("lieu").notNull(),
  adresse: text("adresse"),
  ville: text("ville").notNull(),
  codePostal: text("code_postal"),
  nombreEquipesMax: integer("nombre_equipes_max").default(16),
  nombreEquipesInscrites: integer("nombre_equipes_inscrites").default(0),
  fraisInscription: integer("frais_inscription").default(0),
  type: typeCompetition("type").default("tournoi"),
  reglement: text("reglement"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const equipes = pgTable("equipes", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  clubId: uuid("club_id")
    .notNull()
    .references(() => clubs.id),
  nom: text("nom").notNull(),
  slug: text("slug").notNull().unique(),
  capitaineId: uuid("capitaine_id").references(() => joueurs.id),
  nombreJoueurs: integer("nombre_joueurs").default(0),
  categorie: categorieJoueur("categorie").default("senior"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const equipesTournois = pgTable("equipes_tournois", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  tournoiId: uuid("tournoi_id")
    .notNull()
    .references(() => tournois.id),
  equipeId: uuid("equipe_id")
    .notNull()
    .references(() => equipes.id),
  dateInscription: timestamp("date_inscription", { withTimezone: true }).notNull().defaultNow(),
  paiementEffectue: integer("paiement_effectue").default(0),
  statut: text("statut").default("inscrite"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
});

export const matchs = pgTable("matchs", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  competitionId: uuid("competition_id").notNull(),
  competitionType: typeCompetition("competition_type").notNull(),
  tourId: uuid("tour_id"),
  phase: text("phase").notNull(),
  numeroTour: integer("numero_tour"),
  numeroMatch: integer("numero_match").notNull(),
  equipeDomicileId: uuid("equipe_domicile_id").references(() => equipes.id),
  equipeExterieurId: uuid("equipe_exterieur_id").references(() => equipes.id),
  scoreDomicile: integer("score_domicile").default(0),
  scoreExterieur: integer("score_exterieur").default(0),
  statut: statutMatch("statut").default("planifie"),
  dateHeure: timestamp("date_heure", { withTimezone: true }).notNull(),
  lieu: text("lieu"),
  arbitre: text("arbitre"),
  rapport: text("rapport"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const classementsChampionnat = pgTable("classements_championnat", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  championnatId: uuid("championnat_id")
    .notNull()
    .references(() => championnats.id),
  equipeId: uuid("equipe_id")
    .notNull()
    .references(() => equipes.id),
  position: integer("position").notNull(),
  points: integer("points").default(0),
  joues: integer("joues").default(0),
  victoires: integer("victoires").default(0),
  defaites: integer("defaites").default(0),
  nuls: integer("nuls").default(0),
  pour: integer("pour").default(0),
  contre: integer("contre").default(0),
  difference: integer("difference").default(0),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

export const statistiquesJoueurs = pgTable("statistiques_joueurs", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  joueurId: uuid("joueur_id")
    .notNull()
    .references(() => joueurs.id),
  matchId: uuid("match_id").references(() => matchs.id),
  date: timestamp("date", { withTimezone: true }).notNull().defaultNow(),
  pointsMarques: integer("points_marques").default(0),
  butsMarques: integer("buts_marques").default(0),
  butsEncaises: integer("buts_encaises").default(0),
  passesDecisives: integer("passes_decisives").default(0),
  interceptions: integer("interceptions").default(0),
  fautes: integer("fautes").default(0),
  cartesJaunes: integer("cartes_jaunes").default(0),
  cartesRouges: integer("cartes_rouges").default(0),
  tempsDeJeu: integer("temps_de_jeu"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
});

export const actualites = pgTable("actualites", {
  id: uuid("id")
    .primaryKey()
    .default(sql`gen_random_uuid()`),
  titre: text("titre").notNull(),
  slug: text("slug").notNull().unique(),
  contenu: text("contenu").notNull(),
  resume: text("resume"),
  imageUrl: text("image_url"),
  auteurId: uuid("auteur_id").references(() => joueurs.id),
  clubId: uuid("club_id").references(() => clubs.id),
  datePublication: timestamp("date_publication", { withTimezone: true }).notNull().defaultNow(),
  dateEvenement: timestamp("date_evenement", { withTimezone: true }),
  categories: text("categories"),
  statut: text("statut").default("publie"),
  creeLe: timestamp("cree_le", { withTimezone: true }).notNull().defaultNow(),
  misAJourLe: timestamp("mis_a_jour_le", { withTimezone: true }).notNull().defaultNow(),
});

// Relations
export const federationsRelations = relations(federations, ({ many }) => ({
  clubs: many(clubs),
  championnats: many(championnats),
  tournois: many(tournois),
}));

export const clubsRelations = relations(clubs, ({ one, many }) => ({
  federation: one(federations, { fields: [clubs.federationId], references: [federations.id] }),
  joueurs: many(joueurs),
  equipes: many(equipes),
}));

export const joueursRelations = relations(joueurs, ({ one, many }) => ({
  club: one(clubs, { fields: [joueurs.clubId], references: [clubs.id] }),
  statistiques: many(statistiquesJoueurs),
  actualites: many(actualites),
  equipesCapitaine: many(equipes, { relationName: "capitaine" }),
}));

export const championnatsRelations = relations(championnats, ({ one, many }) => ({
  federation: one(federations, { fields: [championnats.federationId], references: [federations.id] }),
  classements: many(classementsChampionnat),
  matchs: many(matchs, { relationName: "championnat_matchs" }),
}));

export const tournoisRelations = relations(tournois, ({ one, many }) => ({
  federation: one(federations, { fields: [tournois.federationId], references: [federations.id] }),
  equipes: many(equipesTournois),
  matchs: many(matchs, { relationName: "tournoi_matchs" }),
}));

export const equipesRelations = relations(equipes, ({ one, many }) => ({
  club: one(clubs, { fields: [equipes.clubId], references: [clubs.id] }),
  capitaine: one(joueurs, { fields: [equipes.capitaineId], references: [joueurs.id] }),
  tournois: many(equipesTournois),
  matchsDomicile: many(matchs, { relationName: "matchs_domicile" }),
  matchsExterieur: many(matchs, { relationName: "matchs_exterieur" }),
  classements: many(classementsChampionnat),
}));

export const matchsRelations = relations(matchs, ({ one, many }) => ({
  equipeDomicile: one(equipes, { fields: [matchs.equipeDomicileId], references: [equipes.id], relationName: "matchs_domicile" }),
  equipeExterieur: one(equipes, { fields: [matchs.equipeExterieurId], references: [equipes.id], relationName: "matchs_exterieur" }),
  statistiques: many(statistiquesJoueurs),
}));

export const classementsChampionnatRelations = relations(classementsChampionnat, ({ one }) => ({
  championnat: one(championnats, { fields: [classementsChampionnat.championnatId], references: [championnats.id] }),
  equipe: one(equipes, { fields: [classementsChampionnat.equipeId], references: [equipes.id] }),
}));

export const statistiquesJoueursRelations = relations(statistiquesJoueurs, ({ one }) => ({
  joueur: one(joueurs, { fields: [statistiquesJoueurs.joueurId], references: [joueurs.id] }),
  match: one(matchs, { fields: [statistiquesJoueurs.matchId], references: [matchs.id] }),
}));

export const actualitesRelations = relations(actualites, ({ one }) => ({
  auteur: one(joueurs, { fields: [actualites.auteurId], references: [joueurs.id] }),
  club: one(clubs, { fields: [actualites.clubId], references: [clubs.id] }),
}));

// Exports
export type Federation = typeof federations.$inferSelect;
export type NewFederation = typeof federations.$inferInsert;
export type Club = typeof clubs.$inferSelect;
export type NewClub = typeof clubs.$inferInsert;
export type Joueur = typeof joueurs.$inferSelect;
export type NewJoueur = typeof joueurs.$inferInsert;
export type Championnat = typeof championnats.$inferSelect;
export type NewChampionnat = typeof championnats.$inferInsert;
export type Tournoi = typeof tournois.$inferSelect;
export type NewTournoi = typeof tournois.$inferInsert;
export type Equipe = typeof equipes.$inferSelect;
export type NewEquipe = typeof equipes.$inferInsert;
export type Match = typeof matchs.$inferSelect;
export type NewMatch = typeof matchs.$inferInsert;
export type ClassementChampionnat = typeof classementsChampionnat.$inferSelect;
export type NewClassementChampionnat = typeof classementsChampionnat.$inferInsert;
export type StatistiqueJoueur = typeof statistiquesJoueurs.$inferSelect;
export type NewStatistiqueJoueur = typeof statistiquesJoueurs.$inferInsert;
export type Actualite = typeof actualites.$inferSelect;
export type NewActualite = typeof actualites.$inferInsert;
