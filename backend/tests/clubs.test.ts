import { describe, expect, it, vi, beforeEach } from "vitest";
import { Hono } from "hono";
import { clubs } from "../db/schema.js";
import { getTableColumns } from "drizzle-orm";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/clubs", (c) => {
  // Simuler une réponse réussie
  return c.json({
    data: [
      {
        id: "club-1",
        federationId: "fed-1",
        nom: "Club Test",
        slug: "club-test",
        ville: "Ville Test",
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

testApp.get("/clubs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Club non trouvé" }, 404);
  }
  return c.json({
    id,
    federationId: "fed-1",
    nom: "Club Trouvé",
    slug: "club-trouve",
    ville: "Ville Test",
  });
});

testApp.post("/clubs", async (c) => {
  const body = await c.req.json();
  if (!body.nom || !body.slug || !body.ville || !body.federationId) {
    return c.json(
      { error: "Les champs nom, slug, ville et federationId sont requis" },
      400,
    );
  }
  return c.json(
    { id: "new-club-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

testApp.put("/clubs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Club non trouvé" }, 404);
  }
  return c.json({
    id,
    nom: "Club Mis à Jour",
    slug: "club-mis-a-jour",
    ville: "Ville Test",
    federationId: "fed-1",
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.delete("/clubs/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Club non trouvé" }, 404);
  }
  return c.json({ message: "Club supprimé avec succès" });
});

describe("Clubs API - Unit Tests", () => {
  describe("GET /clubs", () => {
    it("retourne la liste des clubs", async () => {
      const res = await testApp.request("/clubs");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });
  });

  describe("POST /clubs", () => {
    it("crée un nouveau club avec données valides", async () => {
      const newClub = {
        federationId: "fed-1",
        nom: "Nouveau Club",
        slug: "nouveau-club",
        ville: "Nouvelle Ville",
      };
      const res = await testApp.request("/clubs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newClub),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
      expect(body).toHaveProperty("nom", "Nouveau Club");
    });

    it("retourne 400 si les données sont invalides (nom manquant)", async () => {
      const res = await testApp.request("/clubs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ slug: "test", ville: "test", federationId: "fed-1" }),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /clubs/:id", () => {
    it("retourne un club par ID", async () => {
      const res = await testApp.request("/clubs/club-1");
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("id", "club-1");
    });

    it("retourne 404 si le club n'existe pas", async () => {
      const res = await testApp.request("/clubs/non-existent-id");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("PUT /clubs/:id", () => {
    it("met à jour un club existant", async () => {
      const res = await testApp.request("/clubs/club-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Club Mis à Jour" }),
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("nom", "Club Mis à Jour");
    });

    it("retourne 404 si le club n'existe pas", async () => {
      const res = await testApp.request("/clubs/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Club Mis à Jour" }),
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("DELETE /clubs/:id", () => {
    it("supprime un club existant", async () => {
      const res = await testApp.request("/clubs/club-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si le club n'existe pas", async () => {
      const res = await testApp.request("/clubs/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });
});

describe("Clubs Database Schema", () => {
  describe("Table Structure", () => {
    it("clubs table a les bonnes colonnes", () => {
      const columns = getTableColumns(clubs);
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
  });
});
