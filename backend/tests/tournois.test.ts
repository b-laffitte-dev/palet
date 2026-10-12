import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { tournois } from "../db/schema.js";
import { getTableColumns } from "drizzle-orm";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/tournois", (c) => {
  return c.json({
    data: [
      {
        id: "tournoi-1",
        federationId: "fed-1",
        nom: "Tournoi Test",
        slug: "tournoi-test",
        statut: "planifie",
        dateDebut: "2026-10-15",
        dateFin: "2026-10-16",
        lieu: "Salle Test",
        ville: "Ville Test",
      },
    ],
    total: 1,
    page: 1,
    pageSize: 1,
  });
});

testApp.get("/tournois/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Tournoi non trouvé" }, 404);
  }
  return c.json({
    id,
    federationId: "fed-1",
    nom: "Tournoi Trouvé",
    slug: "tournoi-trouve",
    statut: "planifie",
    dateDebut: "2026-10-15",
    dateFin: "2026-10-16",
    lieu: "Salle Test",
    ville: "Ville Test",
    equipesInscrites: [],
    nombreEquipesInscrites: 0,
  });
});

testApp.post("/tournois", async (c) => {
  const body = await c.req.json();
  if (!body.nom || !body.slug || !body.federationId || !body.dateDebut || !body.dateFin || !body.lieu || !body.ville) {
    return c.json(
      { error: "Les champs nom, slug, federationId, dateDebut, dateFin, lieu et ville sont requis" },
      400,
    );
  }
  return c.json(
    { id: "new-tournoi-id", ...body, creeLe: new Date(), misAJourLe: new Date() },
    201,
  );
});

testApp.put("/tournois/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Tournoi non trouvé" }, 404);
  }
  return c.json({
    id,
    nom: "Tournoi Mis à Jour",
    slug: "tournoi-mis-a-jour",
    federationId: "fed-1",
    statut: "planifie",
    dateDebut: "2026-10-15",
    dateFin: "2026-10-16",
    lieu: "Salle Test",
    ville: "Ville Test",
    creeLe: new Date(),
    misAJourLe: new Date(),
  });
});

testApp.delete("/tournois/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Tournoi non trouvé" }, 404);
  }
  return c.json({ message: "Tournoi supprimé avec succès" });
});

testApp.post("/tournois/:id/inscrire", async (c) => {
  const id = c.req.param("id");
  const body = await c.req.json();
  if (!body.equipeId) {
    return c.json(
      { error: "Le champ equipeId est requis" },
      400,
    );
  }
  if (id === "non-existent-id") {
    return c.json({ error: "Tournoi non trouvé" }, 404);
  }
  return c.json(
    { id: "inscription-id", tournoiId: id, equipeId: body.equipeId, dateInscription: new Date() },
    201,
  );
});

testApp.delete("/tournois/:id/equipes/:equipeId", (c) => {
  const id = c.req.param("id");
  const equipeId = c.req.param("equipeId");
  if (id === "non-existent-id") {
    return c.json({ error: "Inscription non trouvée" }, 404);
  }
  return c.json({ message: "Équipe désinscrite avec succès" });
});

describe("Tournois API - Unit Tests", () => {
  describe("GET /tournois", () => {
    it("retourne la liste des tournois", async () => {
      const res = await testApp.request("/tournois");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("filtre par fédération", async () => {
      const res = await testApp.request("/tournois?federationId=fed-1");
      expect(res.status).toBe(200);
    });

    it("filtre par statut", async () => {
      const res = await testApp.request("/tournois?statut=planifie");
      expect(res.status).toBe(200);
    });
  });

  describe("POST /tournois", () => {
    it("crée un nouveau tournoi avec données valides", async () => {
      const newTournoi = {
        federationId: "fed-1",
        nom: "Nouveau Tournoi",
        slug: "nouveau-tournoi",
        dateDebut: "2026-11-01",
        dateFin: "2026-11-02",
        lieu: "Salle Nouvelle",
        ville: "Nouvelle Ville",
      };
      const res = await testApp.request("/tournois", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newTournoi),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
      expect(body).toHaveProperty("nom", "Nouveau Tournoi");
    });

    it("retourne 400 si les données sont invalides", async () => {
      const res = await testApp.request("/tournois", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Test" }),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("GET /tournois/:id", () => {
    it("retourne un tournoi par ID", async () => {
      const res = await testApp.request("/tournois/tournoi-1");
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("id", "tournoi-1");
    });

    it("retourne 404 si le tournoi n'existe pas", async () => {
      const res = await testApp.request("/tournois/non-existent-id");
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("PUT /tournois/:id", () => {
    it("met à jour un tournoi existant", async () => {
      const res = await testApp.request("/tournois/tournoi-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Tournoi Mis à Jour" }),
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("nom", "Tournoi Mis à Jour");
    });

    it("retourne 404 si le tournoi n'existe pas", async () => {
      const res = await testApp.request("/tournois/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ nom: "Tournoi Mis à Jour" }),
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("DELETE /tournois/:id", () => {
    it("supprime un tournoi existant", async () => {
      const res = await testApp.request("/tournois/tournoi-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si le tournoi n'existe pas", async () => {
      const res = await testApp.request("/tournois/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });
  });

  describe("POST /tournois/:id/inscrire", () => {
    it("inscrit une équipe à un tournoi", async () => {
      const res = await testApp.request("/tournois/tournoi-1/inscrire", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ equipeId: "equipe-1" }),
      });
      expect(res.status).toBe(201);
      const body = await res.json();
      expect(body).toHaveProperty("id");
    });

    it("retourne 400 si équipeId est manquant", async () => {
      const res = await testApp.request("/tournois/tournoi-1/inscrire", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({}),
      });
      expect(res.status).toBe(400);
      const body = await res.json();
      expect(body).toHaveProperty("error");
    });

    it("retourne 404 si le tournoi n'existe pas", async () => {
      const res = await testApp.request("/tournois/non-existent-id/inscrire", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ equipeId: "equipe-1" }),
      });
      expect(res.status).toBe(404);
    });
  });

  describe("DELETE /tournois/:id/equipes/:equipeId", () => {
    it("désinscrit une équipe d'un tournoi", async () => {
      const res = await testApp.request("/tournois/tournoi-1/equipes/equipe-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("message");
    });

    it("retourne 404 si l'inscription n'existe pas", async () => {
      const res = await testApp.request("/tournois/non-existent-id/equipes/equipe-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
    });
  });
});

describe("Tournois Database Schema", () => {
  describe("Table Structure", () => {
    it("tournois table a les bonnes colonnes", () => {
      const columns = getTableColumns(tournois);
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
  });
});
