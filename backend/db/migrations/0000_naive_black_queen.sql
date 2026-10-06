CREATE TYPE "public"."statut_tournoi" AS ENUM('planifie', 'en_cours', 'termine', 'annule');--> statement-breakpoint
CREATE TABLE "clubs" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"federation_id" uuid NOT NULL,
	"nom" text NOT NULL,
	"slug" text NOT NULL,
	"ville" text,
	"cree_le" timestamp with time zone DEFAULT now() NOT NULL,
	CONSTRAINT "clubs_slug_unique" UNIQUE("slug")
);
--> statement-breakpoint
CREATE TABLE "federations" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"nom" text NOT NULL,
	"slug" text NOT NULL,
	"description" text,
	"cree_le" timestamp with time zone DEFAULT now() NOT NULL,
	CONSTRAINT "federations_slug_unique" UNIQUE("slug")
);
--> statement-breakpoint
CREATE TABLE "joueurs" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"club_id" uuid NOT NULL,
	"nom" text NOT NULL,
	"prenom" text NOT NULL,
	"licence" text,
	"cree_le" timestamp with time zone DEFAULT now() NOT NULL,
	CONSTRAINT "joueurs_licence_unique" UNIQUE("licence")
);
--> statement-breakpoint
ALTER TABLE "clubs" ADD CONSTRAINT "clubs_federation_id_federations_id_fk" FOREIGN KEY ("federation_id") REFERENCES "public"."federations"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "joueurs" ADD CONSTRAINT "joueurs_club_id_clubs_id_fk" FOREIGN KEY ("club_id") REFERENCES "public"."clubs"("id") ON DELETE no action ON UPDATE no action;