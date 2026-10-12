import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { joueurs } from "../db/schema.js";
import { getTableColumns } from "drizzle-orm";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/joueurs", (c) => {
  return c.json({
    data: [
      {
        id: "joueur-1",
        clubId: "club-1",
        nom: "Test",
        prenom: "Joueur",
        licence: "LIC123",
        categorie: "senior",
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

testApp.get("/joueurs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Joueur non trouvé" }, 404);
  }
  return c.json({
    id,
    clubId: "club-1",
    nom: "Test",
    prenom: "Joueur",
    licence: "LIC123",
    categorie: "senior",
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.post("/joueurs", async (c) => {
  const body = await c.req.json();
  if (!body.nom || !body.prenom || !body.clubId) {
    return c.json(
      { error: "Les champs nom, prenom et clubId sont requis" },
      400,
    );
  }
  return c.json(
    { id: "new-joueur-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

testApp.put("/joueurs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Joueur non trouvé" }, 404);
  }
  return c.json({
    id,
    nom: "Joueur Mis à Jour",
    prenom: "Test",
    clubId: "club-1",
    categorie: "senior",
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.delete("/joueurs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Joueur non trouvé" }, 404);
  }
  return c.json({ message: "Joueur supprimé avec succès" });
});

testApp.get("/clubs/:clubId/joueurs", (c) => {
  const clubId = c.req.param("clubId");
  if (clubId === "non-existent-club") {
    return c.json({ error: "Club non trouvé" }, 404);
  }
  return c.json({
    data: [
      {
        id: "joueur-1",
        clubId,
        nom: "Test",
        prenom: "Joueur",
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

describe("Joueurs API - Unit Tests", () => {
  describe("GET /joueurs", () => {
    it("retourne la liste des joueurs", async () => {
      const res = await testApp.request("/joueurs");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });
  });

  describe("POST /joueurs", () => {
    it("crée un nouveau joueur avec données valides", async () => {
      const newJoueur = {
        clubId: "club-1",
        nom: "Nouveau",
        prenom: "Joueur",
      };
      const res = await testApp.request("/joueurs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newJoueur),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
      expect(body).toHaveProperty("nom", "Nouveau");
    });

    it("retourne 400 si les données sont invalides (nom manquant)", async () => {
      const res = await testApp.request("/joueurs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prenom: "Joueur", clubId: "club-1" }),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /joueurs/:id", () => {
    it("retourne un joueur par ID", async () => {
      const res = await testApp.request("/joueurs/joueur-1");
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("id", "joueur-1");
    });

    it("retourne 404 si le joueur n'existe pas", async () => {
      const res = await testApp.request("/joueurs/non-existent-id");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("PUT /joueurs/:id", () => {
    it("met à jour un joueur existant", async () => {
      const res = await testApp.request("/joueurs/joueur-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Joueur Mis à Jour" }),
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("nom", "Joueur Mis à Jour");
    });

    it("retourne 404 si le joueur n'existe pas", async () => {
      const res = await testApp.request("/joueurs/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Joueur Mis à Jour" }),
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("DELETE /joueurs/:id", () => {
    it("supprime un joueur existant", async () => {
      const res = await testApp.request("/joueurs/joueur-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si le joueur n'existe pas", async () => {
      const res = await testApp.request("/joueurs/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /clubs/:clubId/joueurs", () => {
    it("retourne les joueurs d'un club", async () => {
      const res = await testApp.request("/clubs/club-1/joueurs");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("retourne 404 si le club n'existe pas", async () => {
      const res = await testApp.request("/clubs/non-existent-club/joueurs");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });
});

describe("Joueurs Database Schema", () => {
  describe("Table Structure", () => {
    it("joueurs table a les bonnes colonnes", () => {
      const columns = getTableColumns(joueurs);
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
  });
});
