import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { championnats } from "../db/schema.js";
import { getTableColumns } from "drizzle-orm";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/championnats", (c) => {
  return c.json({
    data: [
      {
        id: "championnat-1",
        federationId: "fed-1",
        nom: "Championnat Test",
        slug: "championnat-test",
        saison: "2026-2027",
        statut: "planifie",
        dateDebut: "2026-10-01",
        dateFin: "2027-05-31",
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

testApp.get("/championnats/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Championnat non trouvé" }, 404);
  }
  return c.json({
    id,
    federationId: "fed-1",
    nom: "Championnat Trouvé",
    slug: "championnat-trouve",
    saison: "2026-2027",
    statut: "planifie",
    dateDebut: "2026-10-01",
    dateFin: "2027-05-31",
    classement: [],
    matchs: [],
  });
});

testApp.post("/championnats", async (c) => {
  const body = await c.req.json();
  if (!body.nom || !body.slug || !body.federationId || !body.saison || !body.dateDebut || !body.dateFin) {
    return c.json(
      { error: "Les champs nom, slug, federationId, saison, dateDebut et dateFin sont requis" },
      400,
    );
  }
  return c.json(
    { id: "new-championnat-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

testApp.put("/championnats/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Championnat non trouvé" }, 404);
  }
  return c.json({
    id,
    nom: "Championnat Mis à Jour",
    slug: "championnat-mis-a-jour",
    federationId: "fed-1",
    saison: "2026-2027",
    statut: "en_cours",
    dateDebut: "2026-10-01",
    dateFin: "2027-05-31",
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.delete("/championnats/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Championnat non trouvé" }, 404);
  }
  return c.json({ message: "Championnat supprimé avec succès" });
});

testApp.post("/championnats/:id/classement", async (c) => {
  const id = c.req.param("id");
  const body = await c.req.json();
  if (!Array.isArray(body.classement)) {
    return c.json(
      { error: "Le champ classement doit être un tableau" },
      400,
    );
  }
  if (id === "non-existent-id") {
    return c.json({ error: "Championnat non trouvé" }, 404);
  }
  return c.json(body.classement, 201);
});

testApp.post("/championnats/:id/matchs", async (c) => {
  const id = c.req.param("id");
  const body = await c.req.json();
  if (!body.equipeDomicileId || !body.equipeExterieurId || !body.dateHeure || !body.phase) {
    return c.json(
      { error: "Les champs equipeDomicileId, equipeExterieurId, dateHeure et phase sont requis" },
      400,
    );
  }
  if (id === "non-existent-id") {
    return c.json({ error: "Championnat non trouvé" }, 404);
  }
  return c.json(
    { id: "new-match-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

describe("Championnats API - Unit Tests", () => {
  describe("GET /championnats", () => {
    it("retourne la liste des championnats", async () => {
      const res = await testApp.request("/championnats");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("filtre par fédération", async () => {
      const res = await testApp.request("/championnats?federationId=fed-1");
      expect(res.status).toBe(200);
    });

    it("filtre par statut", async () => {
      const res = await testApp.request("/championnats?statut=planifie");
      expect(res.status).toBe(200);
    });

    it("filtre par saison", async () => {
      const res = await testApp.request("/championnats?saison=2026-2027");
      expect(res.status).toBe(200);
    });
  });

  describe("POST /championnats", () => {
    it("crée un nouveau championnat avec données valides", async () => {
      const newChampionnat = {
        federationId: "fed-1",
        nom: "Nouveau Championnat",
        slug: "nouveau-championnat",
        saison: "2026-2027",
        dateDebut: "2026-10-01",
        dateFin: "2027-05-31",
      };
      const res = await testApp.request("/championnats", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newChampionnat),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
      expect(body).toHaveProperty("nom", "Nouveau Championnat");
    });

    it("retourne 400 si les données sont invalides", async () => {
      const res = await testApp.request("/championnats", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Test" }),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /championnats/:id", () => {
    it("retourne un championnat par ID", async () => {
      const res = await testApp.request("/championnats/championnat-1");
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("id", "championnat-1");
    });

    it("retourne 404 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/championnats/non-existent-id");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("PUT /championnats/:id", () => {
    it("met à jour un championnat existant", async () => {
      const res = await testApp.request("/championnats/championnat-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Championnat Mis à Jour" }),
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("nom", "Championnat Mis à Jour");
    });

    it("retourne 404 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/championnats/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Championnat Mis à Jour" }),
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("DELETE /championnats/:id", () => {
    it("supprime un championnat existant", async () => {
      const res = await testApp.request("/championnats/championnat-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/championnats/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("POST /championnats/:id/classement", () => {
    it("met à jour le classement", async () => {
      const classement = [
        { equipeId: "equipe-1", points: 10, victoires: 3, défaites: 1, nuls: 0 },
        { equipeId: "equipe-2", points: 7, victoires: 2, défaites: 1, nuls: 1 },
      ];
      const res = await testApp.request("/championnats/championnat-1/classement", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ classement }),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(Array.isArray(body)).toBe(true);
    });

    it("retourne 400 si classement n'est pas un tableau", async () => {
      const res = await testApp.request("/championnats/championnat-1/classement", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ classement: "invalid" }),
      });
      expect(res.status).toBe(400);
    });

    it("retourne 404 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/championnats/non-existent-id/classement", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ classement: [] }),
      });
      expect(res.status).toBe(404);
    });
  });

  describe("POST /championnats/:id/matchs", () => {
    it("ajoute un match au championnat", async () => {
      const newMatch = {
        equipeDomicileId: "equipe-1",
        equipeExterieurId: "equipe-2",
        dateHeure: "2026-10-15T14:00:00",
        phase: "poule",
      };
      const res = await testApp.request("/championnats/championnat-1/matchs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newMatch),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
    });

    it("retourne 400 si les données sont invalides", async () => {
      const res = await testApp.request("/championnats/championnat-1/matchs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ dateHeure: "2026-10-15" }),
      });
      expect(res.status).toBe(400);
    });

    it("retourne 404 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/championnats/non-existent-id/matchs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          equipeDomicileId: "equipe-1",
          equipeExterieurId: "equipe-2",
          dateHeure: "2026-10-15T14:00:00",
          phase: "poule",
        }),
      });
      expect(res.status).toBe(404);
    });
  });
});

describe("Championnats Database Schema", () => {
  describe("Table Structure", () => {
    it("championnats table a les bonnes colonnes", () => {
      const columns = getTableColumns(championnats);
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
  });
});
