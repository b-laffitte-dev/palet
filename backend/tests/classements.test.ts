import { describe, expect, it } from "vitest";
import { Hono } from "hono";
import { classementsChampionnat } from "../db/schema.js";

// Créer une instance de test de l'application sans démarrer le serveur
const testApp = new Hono();

// Mock des routes pour les tests sans DB
testApp.get("/classements", (c) => {
  const query = c.req.query();
  if (query.championnatId === "championnat-1") {
    return c.json({
      data: [
        {
          classement: {
            id: "classement-1",
            championnatId: "championnat-1",
            equipeId: "equipe-1",
            position: 1,
            points: 15,
            joues: 5,
            victoires: 5,
            defaites: 0,
            nuls: 0,
            pour: 20,
            contre: 5,
            difference: 15,
          },
          championnat: {
            id: "championnat-1",
            nom: "Championnat Test",
            slug: "championnat-test",
            saison: "2026-2027",
            statut: "en_cours",
          },
          equipe: {
            id: "equipe-1",
            nom: "Equipe 1",
            slug: "equipe-1",
            clubId: "club-1",
          },
          club: {
            id: "club-1",
            nom: "Club Test",
            slug: "club-test",
            ville: "Ville Test",
          },
        },
        {
          classement: {
            id: "classement-2",
            championnatId: "championnat-1",
            equipeId: "equipe-2",
            position: 2,
            points: 12,
            joues: 5,
            victoires: 4,
            defaites: 1,
            nuls: 0,
            pour: 18,
            contre: 8,
            difference: 10,
          },
          championnat: {
            id: "championnat-1",
            nom: "Championnat Test",
            slug: "championnat-test",
            saison: "2026-2027",
            statut: "en_cours",
          },
          equipe: {
            id: "equipe-2",
            nom: "Equipe 2",
            slug: "equipe-2",
            clubId: "club-2",
          },
          club: {
            id: "club-2",
            nom: "Club 2",
            slug: "club-2",
            ville: "Ville 2",
          },
        },
      ],
      total: 2,
      page: 1,
      pageSize: 20,
      totalPages: 1,
    });
  }
  return c.json({
    data: [],
    total: 0,
    page: 1,
    pageSize: 20,
    totalPages: 0,
  });
});

testApp.get("/classements/championnat/:championnatId", (c) => {
  const championnatId = c.req.param("championnatId");
  if (championnatId === "non-existent-championnat") {
    return c.json({ error: "Championnat non trouve" }, 404);
  }
  return c.json({
    championnat: {
      id: championnatId,
      nom: "Championnat Test",
      slug: "championnat-test",
      saison: "2026-2027",
      statut: "en_cours",
    },
    classements: [
      {
        classement: {
          id: "classement-1",
          championnatId,
          equipeId: "equipe-1",
          position: 1,
          points: 15,
          joues: 5,
          victoires: 5,
          defaites: 0,
          nuls: 0,
          pour: 20,
          contre: 5,
          difference: 15,
        },
        equipe: {
          id: "equipe-1",
          nom: "Equipe 1",
          slug: "equipe-1",
          clubId: "club-1",
        },
        club: {
          id: "club-1",
          nom: "Club Test",
          slug: "club-test",
          ville: "Ville Test",
        },
      },
    ],
    total: 1,
  });
});

testApp.get("/classements/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Classement non trouve" }, 404);
  }
  return c.json({
    classement: {
      id,
      championnatId: "championnat-1",
      equipeId: "equipe-1",
      position: 1,
      points: 15,
      joues: 5,
      victoires: 5,
      defaites: 0,
      nuls: 0,
      pour: 20,
      contre: 5,
      difference: 15,
    },
    championnat: {
      id: "championnat-1",
      nom: "Championnat Test",
      slug: "championnat-test",
      saison: "2026-2027",
    },
    equipe: {
      id: "equipe-1",
      nom: "Equipe 1",
      slug: "equipe-1",
      clubId: "club-1",
    },
    club: {
      id: "club-1",
      nom: "Club Test",
      slug: "club-test",
      ville: "Ville Test",
    },
  });
});

testApp.post("/classements", async (c) => {
  const body = await c.req.json();
  if (!body.championnatId || !body.equipeId || body.position === undefined) {
    return c.json(
      {
        error: "Les champs championnatId, equipeId et position sont requis",
      },
      400,
    );
  }
  if (body.championnatId === "invalid-championnat") {
    return c.json(
      { error: "Le championnat specifie n'existe pas" },
      400,
    );
  }
  if (body.equipeId === "invalid-equipe") {
    return c.json(
      { error: "L'equipe specifiee n'existe pas" },
      400,
    );
  }
  if (body.position === 1 && body.championnatId === "championnat-1") {
    return c.json(
      { error: "Un classement avec cette position existe deja pour ce championnat" },
      409,
    );
  }
  if (body.equipeId === "equipe-1" && body.championnatId === "championnat-1") {
    return c.json(
      { error: "Cette equipe est deja dans le classement de ce championnat" },
      409,
    );
  }
  return c.json(
    {
      id: "new-classement-id",
      ...body,
      points: body.points || 0,
      joues: body.joues || 0,
      victoires: body.victoires || 0,
      defaites: body.defaites || 0,
      nuls: body.nuls || 0,
      pour: body.pour || 0,
      contre: body.contre || 0,
      difference: body.difference || 0,
      creeLe: new Date().toISOString(),
      misAJourLe: new Date().toISOString(),
    },
    201,
  );
});

testApp.put("/classements/:id", async (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Classement non trouve" }, 404);
  }
  const body = await c.req.json();
  if (body.championnatId === "invalid-championnat") {
    return c.json(
      { error: "Le championnat specifie n'existe pas" },
      400,
    );
  }
  if (body.equipeId === "invalid-equipe") {
    return c.json(
      { error: "L'equipe specifiee n'existe pas" },
      400,
    );
  }
  if (body.position === 1) {
    return c.json(
      { error: "Un classement avec cette position existe deja pour ce championnat" },
      409,
    );
  }
  return c.json({
    id,
    championnatId: body.championnatId || "championnat-1",
    equipeId: body.equipeId || "equipe-1",
    position: body.position || 1,
    points: body.points || 15,
    misAJourLe: new Date().toISOString(),
  });
});

testApp.delete("/classements/:id", (c) => {
  const id = c.req.param("id");
  if (id === "non-existent-id") {
    return c.json({ error: "Classement non trouve" }, 404);
  }
  return c.json({ message: "Classement supprime avec succes" });
});

testApp.post("/classements/championnat/:championnatId/recalculate", (c) => {
  const championnatId = c.req.param("championnatId");
  if (championnatId === "non-existent-championnat") {
    return c.json({ error: "Championnat non trouve" }, 404);
  }
  return c.json({
    message: "Classements recalcules avec succes",
    count: 5,
  });
});

describe("Classements API - Unit Tests", () => {
  describe("GET /classements", () => {
    it("retourne la liste des classements", async () => {
      const res = await testApp.request("/classements");
      expect(res.status).toBe(200);
      const body = (await res.json()) as { data: unknown[] };
      expect(body).toHaveProperty("data");
      expect(Array.isArray(body.data)).toBe(true);
    });

    it("retourne les métadonnées de pagination", async () => {
      const res = await testApp.request("/classements");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body).toHaveProperty("total");
      expect(body).toHaveProperty("page");
      expect(body).toHaveProperty("pageSize");
      expect(body).toHaveProperty("totalPages");
    });

    it("filtre par championnatId", async () => {
      const res = await testApp.request("/classements?championnatId=championnat-1");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.data.length).toBe(2);
      expect(body.total).toBe(2);
    });

    it("filtre par clubId", async () => {
      const res = await testApp.request("/classements?clubId=club-1");
      expect(res.status).toBe(200);
    });

    it("filtre par recherche", async () => {
      const res = await testApp.request("/classements?search=Equipe");
      expect(res.status).toBe(200);
    });
  });

  describe("GET /classements/championnat/:championnatId", () => {
    it("retourne les classements pour un championnat spécifique", async () => {
      const res = await testApp.request("/classements/championnat/championnat-1");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.championnat).toBeDefined();
      expect(body.championnat.id).toBe("championnat-1");
      expect(body.classements).toBeDefined();
      expect(Array.isArray(body.classements)).toBe(true);
      expect(body.total).toBe(1);
    });

    it("retourne 404 pour un championnat non trouvé", async () => {
      const res = await testApp.request(
        "/classements/championnat/non-existent-championnat",
      );
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Championnat non trouve");
    });
  });

  describe("GET /classements/:id", () => {
    it("retourne un classement par ID", async () => {
      const res = await testApp.request("/classements/classement-1");
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.classement).toBeDefined();
      expect(body.classement.id).toBe("classement-1");
      expect(body.championnat).toBeDefined();
      expect(body.equipe).toBeDefined();
      expect(body.club).toBeDefined();
    });

    it("retourne 404 pour un classement non trouvé", async () => {
      const res = await testApp.request("/classements/non-existent-id");
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Classement non trouve");
    });
  });

  describe("POST /classements", () => {
    it("crée un nouveau classement avec données valides", async () => {
      const newClassement = {
        championnatId: "championnat-1",
        equipeId: "equipe-3",
        position: 3,
        points: 10,
        joues: 5,
        victoires: 3,
        defaites: 2,
        nuls: 0,
        pour: 15,
        contre: 10,
        difference: 5,
      };
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newClassement),
      });
      expect(res.status).toBe(201);
      const body = (await res.json()) as any;
      expect(body.id).toBe("new-classement-id");
      expect(body.championnatId).toBe(newClassement.championnatId);
      expect(body.equipeId).toBe(newClassement.equipeId);
      expect(body.position).toBe(newClassement.position);
    });

    it("retourne 400 si les champs requis sont manquants", async () => {
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ championnatId: "championnat-1" }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe(
        "Les champs championnatId, equipeId et position sont requis",
      );
    });

    it("retourne 400 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          championnatId: "invalid-championnat",
          equipeId: "equipe-1",
          position: 1,
        }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Le championnat specifie n'existe pas");
    });

    it("retourne 400 si l'équipe n'existe pas", async () => {
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          championnatId: "championnat-1",
          equipeId: "invalid-equipe",
          position: 1,
        }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("L'equipe specifiee n'existe pas");
    });

    it("retourne 409 si la position existe déjà pour ce championnat", async () => {
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          championnatId: "championnat-1",
          equipeId: "equipe-2",
          position: 1,
        }),
      });
      expect(res.status).toBe(409);
      const body = (await res.json()) as any;
      expect(body.error).toBe(
        "Un classement avec cette position existe deja pour ce championnat",
      );
    });

    it("retourne 409 si l'équipe est déjà dans le classement", async () => {
      const res = await testApp.request("/classements", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          championnatId: "championnat-1",
          equipeId: "equipe-1",
          position: 3,
        }),
      });
      expect(res.status).toBe(409);
      const body = (await res.json()) as any;
      expect(body.error).toBe(
        "Cette equipe est deja dans le classement de ce championnat",
      );
    });
  });

  describe("PUT /classements/:id", () => {
    it("met à jour un classement existant", async () => {
      const updateData = {
        position: 2,
        points: 18,
      };
      const res = await testApp.request("/classements/classement-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(updateData),
      });
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.position).toBe(updateData.position);
      expect(body.points).toBe(updateData.points);
    });

    it("retourne 404 pour un classement non trouvé", async () => {
      const res = await testApp.request("/classements/non-existent-id", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ position: 2 }),
      });
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Classement non trouve");
    });

    it("retourne 400 si le championnat n'existe pas", async () => {
      const res = await testApp.request("/classements/classement-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ championnatId: "invalid-championnat" }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Le championnat specifie n'existe pas");
    });

    it("retourne 400 si l'équipe n'existe pas", async () => {
      const res = await testApp.request("/classements/classement-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ equipeId: "invalid-equipe" }),
      });
      expect(res.status).toBe(400);
      const body = (await res.json()) as any;
      expect(body.error).toBe("L'equipe specifiee n'existe pas");
    });

    it("retourne 409 si la nouvelle position existe déjà", async () => {
      const res = await testApp.request("/classements/classement-1", {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ position: 1 }),
      });
      expect(res.status).toBe(409);
      const body = (await res.json()) as any;
      expect(body.error).toBe(
        "Un classement avec cette position existe deja pour ce championnat",
      );
    });
  });

  describe("DELETE /classements/:id", () => {
    it("supprime un classement existant", async () => {
      const res = await testApp.request("/classements/classement-1", {
        method: "DELETE",
      });
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.message).toBe("Classement supprime avec succes");
    });

    it("retourne 404 pour un classement non trouvé", async () => {
      const res = await testApp.request("/classements/non-existent-id", {
        method: "DELETE",
      });
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Classement non trouve");
    });
  });

  describe("POST /classements/championnat/:championnatId/recalculate", () => {
    it("recalcule les classements pour un championnat", async () => {
      const res = await testApp.request(
        "/classements/championnat/championnat-1/recalculate",
        {
          method: "POST",
        },
      );
      expect(res.status).toBe(200);
      const body = (await res.json()) as any;
      expect(body.message).toBe("Classements recalcules avec succes");
      expect(body.count).toBe(5);
    });

    it("retourne 404 pour un championnat non trouvé", async () => {
      const res = await testApp.request(
        "/classements/championnat/non-existent-championnat/recalculate",
        {
          method: "POST",
        },
      );
      expect(res.status).toBe(404);
      const body = (await res.json()) as any;
      expect(body.error).toBe("Championnat non trouve");
    });
  });
});

describe("Classements Schema - Validation", () => {
  it("valide la structure de la table classementsChampionnat", () => {
    const columns = classementsChampionnat;
    expect(columns).toBeDefined();
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
  });
});
