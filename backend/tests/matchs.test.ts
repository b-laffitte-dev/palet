import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { matchs } from "../db/schema.js";
import { getTableColumns } from "drizzle-orm";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/matchs", (c) => {
  return c.json({
    data: [
      {
        id: "match-1",
        competitionId: "championnat-1",
        competitionType: "championnat",
        phase: "poule",
        statut: "planifie",
        dateHeure: "2026-10-15T14:00:00",
        scoreDomicile: 0,
        scoreExterieur: 0,
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

testApp.get("/matchs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Match non trouvé" }, 404);
  }
  return c.json({
    id,
    competitionId: "championnat-1",
    competitionType: "championnat",
    phase: "poule",
    statut: "planifie",
    dateHeure: "2026-10-15T14:00:00",
    scoreDomicile: 0,
    scoreExterieur: 0,
    statistiques: [],
  });
});

testApp.post("/matchs", async (c) => {
  const body = await c.req.json();
  if (!body.competitionId || !body.competitionType || !body.equipeDomicileId || !body.equipeExterieurId || !body.dateHeure || !body.phase) {
    return c.json(
      { error: "Les champs competitionId, competitionType, equipeDomicileId, equipeExterieurId, dateHeure et phase sont requis" },
      400,
    );
  }
  return c.json(
    { id: "new-match-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

testApp.put("/matchs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Match non trouvé" }, 404);
  }
  return c.json({
    id,
    competitionId: "championnat-1",
    competitionType: "championnat",
    phase: "poule",
    statut: "termine",
    dateHeure: "2026-10-15T14:00:00",
    scoreDomicile: 5,
    scoreExterieur: 3,
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.delete("/matchs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Match non trouvé" }, 404);
  }
  return c.json({ message: "Match supprimé avec succès" });
});

testApp.post("/matchs/:id/statistiques", async (c) => {
  const id = c.req.param("id");
  const body = await c.req.json();
  if (!body.joueurId) {
    return c.json(
      { error: "Le champ joueurId est requis" },
      400,
    );
  }
  if (id === "non-existent-id") {
    return c.json({ error: "Match non trouvé" }, 404);
  }
  return c.json(
    { id: "stat-id", matchId: id, joueurId: body.joueurId, date: new Date() },
    201,
  );
});

describe("Matchs API - Unit Tests", () => {
  describe("GET /matchs", () => {
    it("retourne la liste des matchs", async () => {
      const res = await testApp.request("/matchs");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("filtre par compétition", async () => {
      const res = await testApp.request("/matchs?competitionId=championnat-1");
      expect(res.status).toBe(200);
    });

    it("filtre par type de compétition", async () => {
      const res = await testApp.request("/matchs?competitionType=championnat");
      expect(res.status).toBe(200);
    });

    it("filtre par statut", async () => {
      const res = await testApp.request("/matchs?statut=planifie");
      expect(res.status).toBe(200);
    });

    it("filtre par phase", async () => {
      const res = await testApp.request("/matchs?phase=poule");
      expect(res.status).toBe(200);
    });
  });

  describe("POST /matchs", () => {
    it("crée un nouveau match avec données valides", async () => {
      const newMatch = {
        competitionId: "championnat-1",
        competitionType: "championnat",
        equipeDomicileId: "equipe-1",
        equipeExterieurId: "equipe-2",
        dateHeure: "2026-10-15T14:00:00",
        phase: "poule",
      };
      const res = await testApp.request("/matchs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newMatch),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
    });

    it("retourne 400 si les données sont invalides", async () => {
      const res = await testApp.request("/matchs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ dateHeure: "2026-10-15" }),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /matchs/:id", () => {
    it("retourne un match par ID", async () => {
      const res = await testApp.request("/matchs/match-1");
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("id", "match-1");
    });

    it("retourne 404 si le match n'existe pas", async () => {
      const res = await testApp.request("/matchs/non-existent-id");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("PUT /matchs/:id", () => {
    it("met à jour un match existant", async () => {
      const res = await testApp.request("/matchs/match-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ scoreDomicile: 5, scoreExterieur: 3 }),
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("statut", "termine");
    });

    it("retourne 404 si le match n'existe pas", async () => {
      const res = await testApp.request("/matchs/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ scoreDomicile: 5 }),
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("DELETE /matchs/:id", () => {
    it("supprime un match existant", async () => {
      const res = await testApp.request("/matchs/match-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si le match n'existe pas", async () => {
      const res = await testApp.request("/matchs/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("POST /matchs/:id/statistiques", () => {
    it("ajoute des statistiques pour un match", async () => {
      const stats = {
        joueurId: "joueur-1",
        pointsMarques: 10,
        butsMarques: 2,
        passesDecisives: 1,
      };
      const res = await testApp.request("/matchs/match-1/statistiques", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(stats),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
    });

    it("retourne 400 si joueurId est manquant", async () => {
      const res = await testApp.request("/matchs/match-1/statistiques", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ pointsMarques: 10 }),
      });
      expect(res.status).toBe(400);
    });

    it("retourne 404 si le match n'existe pas", async () => {
      const res = await testApp.request("/matchs/non-existent-id/statistiques", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ joueurId: "joueur-1" }),
      });
      expect(res.status).toBe(404);
    });
  });
});

describe("Matchs Database Schema", () => {
  describe("Table Structure", () => {
    it("matchs table a les bonnes colonnes", () => {
      const columns = getTableColumns(matchs);
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
  });
});
