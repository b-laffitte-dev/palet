import { describe, expect, it } from "vitest";
import * as schema from "../db/schema.js";
import { getTableColumns, getTableName } from "drizzle-orm";

describe("Database Schema", () => {
  describe("Enums", () => {
    it("statutTournoi a les bonnes valeurs", () => {
      expect(schema.statutTournoi.enumValues).toEqual([
        "planifie",
        "en_cours",
        "termine",
        "annule",
      ]);
    });

    it("statutChampionnat a les bonnes valeurs", () => {
      expect(schema.statutChampionnat.enumValues).toEqual([
        "planifie",
        "en_cours",
        "termine",
        "annule",
      ]);
    });

    it("statutMatch a les bonnes valeurs", () => {
      expect(schema.statutMatch.enumValues).toEqual([
        "planifie",
        "en_cours",
        "termine",
        "reporte",
        "annule",
      ]);
    });

    it("typeCompetition a les bonnes valeurs", () => {
      expect(schema.typeCompetition.enumValues).toEqual([
        "tournoi",
        "championnat",
        "amical",
      ]);
    });

    it("categorieJoueur a les bonnes valeurs", () => {
      expect(schema.categorieJoueur.enumValues).toEqual([
        "senior",
        "junior",
        "cadet",
        "minime",
        "veteran",
      ]);
    });
  });

  describe("Tables", () => {
    describe("federations", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.federations);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("description");
        expect(columns).toHaveProperty("logoUrl");
        expect(columns).toHaveProperty("siteWeb");
        expect(columns).toHaveProperty("email");
        expect(columns).toHaveProperty("telephone");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.federations)).toBe("federations");
      });
    });

    describe("clubs", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.clubs);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("federationId");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("ville");
        expect(columns).toHaveProperty("codePostal");
        expect(columns).toHaveProperty("adresse");
        expect(columns).toHaveProperty("logoUrl");
        expect(columns).toHaveProperty("siteWeb");
        expect(columns).toHaveProperty("email");
        expect(columns).toHaveProperty("telephone");
        expect(columns).toHaveProperty("anneeFondation");
        expect(columns).toHaveProperty("nombreLicencies");
        expect(columns).toHaveProperty("couleurPrincipale");
        expect(columns).toHaveProperty("couleurSecondaire");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.clubs)).toBe("clubs");
      });
    });

    describe("joueurs", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.joueurs);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("clubId");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("prenom");
        expect(columns).toHaveProperty("licence");
        expect(columns).toHaveProperty("categorie");
        expect(columns).toHaveProperty("dateNaissance");
        expect(columns).toHaveProperty("email");
        expect(columns).toHaveProperty("telephone");
        expect(columns).toHaveProperty("photoUrl");
        expect(columns).toHaveProperty("poste");
        expect(columns).toHaveProperty("mainDroite");
        expect(columns).toHaveProperty("nombreVictoires");
        expect(columns).toHaveProperty("nombreDefaites");
        expect(columns).toHaveProperty("nombreNuls");
        expect(columns).toHaveProperty("points");
        expect(columns).toHaveProperty("classement");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.joueurs)).toBe("joueurs");
      });
    });

    describe("championnats", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.championnats);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("federationId");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("saison");
        expect(columns).toHaveProperty("description");
        expect(columns).toHaveProperty("statut");
        expect(columns).toHaveProperty("dateDebut");
        expect(columns).toHaveProperty("dateFin");
        expect(columns).toHaveProperty("nombreEquipes");
        expect(columns).toHaveProperty("reglement");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.championnats)).toBe("championnats");
      });
    });

    describe("tournois", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.tournois);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("federationId");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("description");
        expect(columns).toHaveProperty("statut");
        expect(columns).toHaveProperty("dateDebut");
        expect(columns).toHaveProperty("dateFin");
        expect(columns).toHaveProperty("lieu");
        expect(columns).toHaveProperty("adresse");
        expect(columns).toHaveProperty("ville");
        expect(columns).toHaveProperty("codePostal");
        expect(columns).toHaveProperty("nombreEquipesMax");
        expect(columns).toHaveProperty("nombreEquipesInscrites");
        expect(columns).toHaveProperty("fraisInscription");
        expect(columns).toHaveProperty("type");
        expect(columns).toHaveProperty("reglement");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.tournois)).toBe("tournois");
      });
    });

    describe("equipes", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.equipes);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("clubId");
        expect(columns).toHaveProperty("nom");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("capitaineId");
        expect(columns).toHaveProperty("nombreJoueurs");
        expect(columns).toHaveProperty("categorie");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.equipes)).toBe("equipes");
      });
    });

    describe("equipesTournois", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.equipesTournois);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("tournoiId");
        expect(columns).toHaveProperty("equipeId");
        expect(columns).toHaveProperty("dateInscription");
        expect(columns).toHaveProperty("paiementEffectue");
        expect(columns).toHaveProperty("statut");
        expect(columns).toHaveProperty("creeLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.equipesTournois)).toBe("equipes_tournois");
      });
    });

    describe("matchs", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.matchs);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("competitionId");
        expect(columns).toHaveProperty("competitionType");
        expect(columns).toHaveProperty("tourId");
        expect(columns).toHaveProperty("phase");
        expect(columns).toHaveProperty("numeroTour");
        expect(columns).toHaveProperty("numeroMatch");
        expect(columns).toHaveProperty("equipeDomicileId");
        expect(columns).toHaveProperty("equipeExterieurId");
        expect(columns).toHaveProperty("scoreDomicile");
        expect(columns).toHaveProperty("scoreExterieur");
        expect(columns).toHaveProperty("statut");
        expect(columns).toHaveProperty("dateHeure");
        expect(columns).toHaveProperty("lieu");
        expect(columns).toHaveProperty("arbitre");
        expect(columns).toHaveProperty("rapport");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.matchs)).toBe("matchs");
      });
    });

    describe("classementsChampionnat", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.classementsChampionnat);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("championnatId");
        expect(columns).toHaveProperty("equipeId");
        expect(columns).toHaveProperty("position");
        expect(columns).toHaveProperty("points");
        expect(columns).toHaveProperty("joues");
        expect(columns).toHaveProperty("victoires");
        expect(columns).toHaveProperty("defaites");
        expect(columns).toHaveProperty("nuls");
        expect(columns).toHaveProperty("pour");
        expect(columns).toHaveProperty("contre");
        expect(columns).toHaveProperty("difference");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.classementsChampionnat)).toBe(
          "classements_championnat",
        );
      });
    });

    describe("statistiquesJoueurs", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.statistiquesJoueurs);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("joueurId");
        expect(columns).toHaveProperty("matchId");
        expect(columns).toHaveProperty("date");
        expect(columns).toHaveProperty("pointsMarques");
        expect(columns).toHaveProperty("butsMarques");
        expect(columns).toHaveProperty("butsEncaises");
        expect(columns).toHaveProperty("passesDecisives");
        expect(columns).toHaveProperty("interceptions");
        expect(columns).toHaveProperty("fautes");
        expect(columns).toHaveProperty("cartesJaunes");
        expect(columns).toHaveProperty("cartesRouges");
        expect(columns).toHaveProperty("tempsDeJeu");
        expect(columns).toHaveProperty("creeLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.statistiquesJoueurs)).toBe(
          "statistiques_joueurs",
        );
      });
    });

    describe("actualites", () => {
      it("a les colonnes requises", () => {
        const columns = getTableColumns(schema.actualites);
        expect(columns).toHaveProperty("id");
        expect(columns).toHaveProperty("titre");
        expect(columns).toHaveProperty("slug");
        expect(columns).toHaveProperty("contenu");
        expect(columns).toHaveProperty("resume");
        expect(columns).toHaveProperty("imageUrl");
        expect(columns).toHaveProperty("auteurId");
        expect(columns).toHaveProperty("clubId");
        expect(columns).toHaveProperty("datePublication");
        expect(columns).toHaveProperty("dateEvenement");
        expect(columns).toHaveProperty("categories");
        expect(columns).toHaveProperty("statut");
        expect(columns).toHaveProperty("creeLe");
        expect(columns).toHaveProperty("misAJourLe");
      });

      it("a le bon nom de table", () => {
        expect(getTableName(schema.actualites)).toBe("actualites");
      });
    });
  });

  describe("Type Exports", () => {
    it("exporte tous les types attendus", () => {
      expect(schema.federations).toBeDefined();
      expect(schema.clubs).toBeDefined();
      expect(schema.joueurs).toBeDefined();
      expect(schema.championnats).toBeDefined();
      expect(schema.tournois).toBeDefined();
      expect(schema.equipes).toBeDefined();
      expect(schema.matchs).toBeDefined();
      expect(schema.classementsChampionnat).toBeDefined();
      expect(schema.statistiquesJoueurs).toBeDefined();
      expect(schema.actualites).toBeDefined();
    });

    it("Federation type a les bonnes propriétés", () => {
      const fed = {
        id: "test-id",
        nom: "Test Federation",
        slug: "test-federation",
        description: "Test description",
        logoUrl: "http://test.com/logo.png",
        siteWeb: "http://test.com",
        email: "test@test.com",
        telephone: "0123456789",
        creeLe: new Date(),
        misAJourLe: new Date(),
      };
      expect(fed).toBeDefined();
    });

    it("Club type a les bonnes propriétés", () => {
      const club = {
        id: "test-id",
        federationId: "fed-id",
        nom: "Test Club",
        slug: "test-club",
        ville: "Test Ville",
        codePostal: "12345",
        adresse: "Test Adresse",
        logoUrl: "http://test.com/logo.png",
        siteWeb: "http://test.com",
        email: "test@test.com",
        telephone: "0123456789",
        anneeFondation: 2000,
        nombreLicencies: 100,
        couleurPrincipale: "#FF0000",
        couleurSecondaire: "#00FF00",
        creeLe: new Date(),
        misAJourLe: new Date(),
      };
      expect(club).toBeDefined();
    });

    it("Joueur type a les bonnes propriétés", () => {
      const joueur = {
        id: "test-id",
        clubId: "club-id",
        nom: "Test",
        prenom: "Joueur",
        licence: "LIC123",
        categorie: "senior",
        dateNaissance: new Date(),
        email: "test@test.com",
        telephone: "0123456789",
        photoUrl: "http://test.com/photo.png",
        poste: "Attaquant",
        mainDroite: 1,
        nombreVictoires: 10,
        nombreDefaites: 5,
        nombreNuls: 2,
        points: 100,
        classement: 1,
        creeLe: new Date(),
        misAJourLe: new Date(),
      };
      expect(joueur).toBeDefined();
    });
  });
});
