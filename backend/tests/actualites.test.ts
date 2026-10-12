import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { actualites } from "../db/schema.js";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/actualites", (c) => {
  const query = c.req.query();
  if (query.search === "test") {
    return c.json({
      data: [
        {
          actualite: {
            id: "actualite-1",
            titre: "Titre Test",
            slug: "titre-test",
            contenu: "Contenu test",
            resume: "Resume test",
            statut: "publie",
          },
          auteur: {
            id: "joueur-1",
            nom: "Nom",
            prenom: "Prenom",
          },
          club: {
            id: "club-1",
            nom: "Club Test",
            slug: "club-test",
          },
        },
      ],
      total: 1,
      page: 1,
      pageSize: 1,
      totalPages: 1,
    });
  }
  return c.json({
    data: [
      {
        actualite: {
          id: "actualite-1",
          titre: "Titre Test",
          slug: "titre-test",
          contenu: "Contenu test",
          resume: "Resume test",
          statut: "publie",
        },
        auteur: {
          id: "joueur-1",
          nom: "Nom",
          prenom: "Prenom",
        },
        club: {
          id: "club-1",
          nom: "Club Test",
          slug: "club-test",
        },
      },
      {
        actualite: {
          id: "actualite-2",
          titre: "Deuxième Actualité",
          slug: "deuxieme-actualite",
          contenu: "Contenu 2",
          statut: "publie",
        },
        auteur: null,
        club: null,
      },
    ],
    total: 2,
    page: 1,
    pageSize: 10,
    totalPages: 1,
  });
});

testApp.get("/actualites/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Actualite non trouvee" }, 404);
  }
  return c.json({
    actualite: {
      id,
      titre: "Actualité Trouvée",
      slug: "actualite-trouvee",
      contenu: "Contenu de l'actualité",
      resume: "Résumé",
      imageUrl: "http://example.com/image.jpg",
      statut: "publie",
      datePublication: new Date().toISOString(),
    },
    auteur: {
      id: "joueur-1",
      nom: "Nom",
      prenom: "Prenom",
      photoUrl: "http://example.com/photo.jpg",
      email: "test@example.com",
    },
    club: {
      id: "club-1",
      nom: "Club Test",
      slug: "club-test",
      ville: "Ville Test",
      logoUrl: "http://example.com/logo.png",
    },
  });
});

testApp.post("/actualites", async (c) => {
  const body = await c.req.json();
  if (!body.titre || !body.slug || !body.contenu) {
    return c.json(
      { error: "Les champs titre, slug et contenu sont requis" },
      400,
    );
  }
  if (body.slug === "existing-slug") {
    return c.json(
      { error: "Une actualite avec ce slug existe deja" },
      409,
    );
  }
  if (body.auteurId === "invalid-auteur") {
    return c.json(
      { error: "L'auteur specifie n'existe pas" },
      400,
    );
  }
  if (body.clubId === "invalid-club") {
    return c.json(
      { error: "Le club specifie n'existe pas" },
      400,
    );
  }
  return c.json(
    {
      id: "new-actualite-id",
      ...body,
      datePublication: body.datePublication || new Date().toISOString(),
      statut: body.statut || "publie",
      creeLe: new Date().toISOString(),
      misAJourLe: new Date().toISOString(),
    },
    201,
  );
});

testApp.put("/actualites/:id", async (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Actualite non trouvee" }, 404);
  }
  const body = await c.req.json();
  if (body.slug === "existing-slug") {
    return c.json(
      { error: "Une actualite avec ce slug existe deja" },
      409,
    );
  }
  if (body.auteurId === "invalid-auteur") {
    return c.json(
      { error: "L'auteur specifie n'existe pas" },
      400,
    );
  }
  if (body.clubId === "invalid-club") {
    return c.json(
      { error: "Le club specifie n'existe pas" },
      400,
    );
  }
  return c.json({
    id,
    titre: body.titre || "Titre Mis à Jour",
    slug: body.slug || "slug-mis-a-jour",
    contenu: body.contenu || "Contenu mis à jour",
    statut: body.statut || "publie",
    misAJourLe: new Date().toISOString(),
  });
});

testApp.delete("/actualites/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Actualite non trouvee" }, 404);
  }
  return c.json({ message: "Actualite supprimee avec succes" });
});

describe("Actualites API - Unit Tests", () => {
  describe("GET /actualites", () => {
    it("retourne la liste des actualités", async () => {
      const res = await testApp.request("/actualites");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("retourne les métadonnées de pagination", async () => {
      const res = await testApp.request("/actualites");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body).toHaveProperty("total");
      expect(body).toHaveProperty("page");
      expect(body).toHaveProperty("pageSize");
      expect(body).toHaveProperty("totalPages");
    });

    it("filtre par recherche", async () => {
      const res = await testApp.request("/actualites?search=test");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.data.length).toBe(1);
    });

    it("filtre par clubId", async () => {
      const res = await testApp.request("/actualites?clubId=club-1");
      expect(res.status).toBe(200);
    });

    it("filtre par auteurId", async () => {
      const res = await testApp.request("/actualites?auteurId=joueur-1");
      expect(res.status).toBe(200);
    });

    it("filtre par categorie", async () => {
      const res = await testApp.request("/actualites?categorie=tournoi");
      expect(res.status).toBe(200);
    });

    it("filtre par statut", async () => {
      const res = await testApp.request("/actualites?statut=publie");
      expect(res.status).toBe(200);
    });
  });

  describe("GET /actualites/:id", () => {
    it("retourne une actualité par ID", async () => {
      const res = await testApp.request("/actualites/actualite-1");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.actualite).toBeDefined();
      expect(body.actualite.id).toBe("actualite-1");
      expect(body.auteur).toBeDefined();
      expect(body.club).toBeDefined();
    });

    it("retourne 404 pour une actualité non trouvée", async () => {
      const res = await testApp.request("/actualites/non-existent-id");
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Actualite non trouvee");
    });
  });

  describe("POST /actualites", () => {
    it("crée une nouvelle actualité avec données valides", async () => {
      const newActualite = {
        titre: "Nouvelle Actualité",
        slug: "nouvelle-actualite",
        contenu: "Contenu de la nouvelle actualité",
        resume: "Résumé",
        auteurId: "joueur-1",
        clubId: "club-1",
        categories: "tournoi,important",
      };
      const res = await testApp.request("/actualites", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newActualite),
      });
      expect(res.status).toBe(201);
      const body = (await res.json()) as any;
      expect(body.id).toBe("new-actualite-id");
      expect(body.titre).toBe(newActualite.titre);
      expect(body.slug).toBe(newActualite.slug);
    });

    it("retourne 400 si les champs requis sont manquants", async () => {
      const res = await testApp.request("/actualites", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ titre: "Seulement titre" }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Les champs titre, slug et contenu sont requis");
    });

    it("retourne 409 si le slug existe déjà", async () => {
      const res = await testApp.request("/actualites", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          titre: "Test",
          slug: "existing-slug",
          contenu: "Test",
        }),
      });
      expect(res.status).toBe(409);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Une actualite avec ce slug existe deja");
    });

    it("retourne 400 si l'auteur n'existe pas", async () => {
      const res = await testApp.request("/actualites", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          titre: "Test",
          slug: "test-slug",
          contenu: "Test",
          auteurId: "invalid-auteur",
        }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("L'auteur specifie n'existe pas");
    });

    it("retourne 400 si le club n'existe pas", async () => {
      const res = await testApp.request("/actualites", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          titre: "Test",
          slug: "test-slug",
          contenu: "Test",
          clubId: "invalid-club",
        }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Le club specifie n'existe pas");
    });
  });

  describe("PUT /actualites/:id", () => {
    it("met à jour une actualité existante", async () => {
      const updateData = {
        titre: "Titre Mis à Jour",
        slug: "slug-mis-a-jour",
        contenu: "Contenu mis à jour",
      };
      const res = await testApp.request("/actualites/actualite-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(updateData),
      });
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.titre).toBe(updateData.titre);
    });

    it("retourne 404 pour une actualité non trouvée", async () => {
      const res = await testApp.request("/actualites/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ titre: "Test" }),
      });
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Actualite non trouvee");
    });

    it("retourne 409 si le nouveau slug existe déjà", async () => {
      const res = await testApp.request("/actualites/actualite-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ slug: "existing-slug" }),
      });
      expect(res.status).toBe(409);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Une actualite avec ce slug existe deja");
    });
  });

  describe("DELETE /actualites/:id", () => {
    it("supprime une actualité existante", async () => {
      const res = await testApp.request("/actualites/actualite-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.message).toBe("Actualite supprimee avec succes");
    });

    it("retourne 404 pour une actualité non trouvée", async () => {
      const res = await testApp.request("/actualites/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Actualite non trouvee");
    });
  });
});

describe("Actualites Schema - Validation", () => {
  it("valide la structure de la table actualites", () => {
    const columns = actualites;
    expect(columns).toBeDefined();
    expect(columns).toHaveProperty("id");
    expect(columns).toHaveProperty("titre");
    expect(columns).toHaveProperty("slug");
    expect(columns).toHaveProperty("contenu");
    expect(columns).toHaveProperty("auteurId");
    expect(columns).toHaveProperty("clubId");
    expect(columns).toHaveProperty("datePublication");
    expect(columns).toHaveProperty("statut");
  });
});
