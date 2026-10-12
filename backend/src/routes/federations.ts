import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { federations, clubs, championnats, tournois } from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const federationsRouter = new Hono();

// GET /federations - List all federations
federationsRouter.get("/federations", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;

    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(like(federations.nom, `%${search}%`), like(federations.slug, `%${search}%`)),
      );
    }

    const [totalResult] = await db
      .select({ count: count() })
      .from(federations)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    const federationsList = await db
      .select({
        federation: federations,
        clubsCount: count(clubs.id),
        championnatsCount: count(championnats.id),
        tournoisCount: count(tournois.id),
      })
      .from(federations)
      .leftJoin(clubs, eq(federations.id, clubs.federationId))
      .leftJoin(championnats, eq(federations.id, championnats.federationId))
      .leftJoin(tournois, eq(federations.id, tournois.federationId))
      .where(whereCondition)
      .groupBy(federations.id)
      .orderBy(desc(federations.creeLe))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: federationsList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation des federations:", error);
    return c.json({ error: "Erreur serveur lors de la recuperation des federations" }, 500);
  }
});

// GET /federations/:id - Get federation by ID
federationsRouter.get("/federations/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [federation] = await db
      .select({
        federation: federations,
        clubs: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
          nombreLicencies: clubs.nombreLicencies,
        },
        championnats: {
          id: championnats.id,
          nom: championnats.nom,
          slug: championnats.slug,
          saison: championnats.saison,
          statut: championnats.statut,
        },
        tournois: {
          id: tournois.id,
          nom: tournois.nom,
          slug: tournois.slug,
          dateDebut: tournois.dateDebut,
          statut: tournois.statut,
        },
      })
      .from(federations)
      .leftJoin(clubs, eq(federations.id, clubs.federationId))
      .leftJoin(championnats, eq(federations.id, championnats.federationId))
      .leftJoin(tournois, eq(federations.id, tournois.federationId))
      .where(eq(federations.id, id))
      .execute();

    if (!federation) {
      return c.json({ error: "Federation non trouvee" }, 404);
    }

    return c.json(federation);
  } catch (error) {
    console.error("Erreur lors de la recuperation de la federation:", error);
    return c.json({ error: "Erreur serveur lors de la recuperation de la federation" }, 500);
  }
});

// POST /federations - Create new federation
federationsRouter.post("/federations", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    if (!body.nom || !body.slug) {
      return c.json({ error: "Les champs nom et slug sont requis" }, 400);
    }

    const [existingFederation] = await db
      .select()
      .from(federations)
      .where(eq(federations.slug, body.slug))
      .execute();

    if (existingFederation) {
      return c.json({ error: "Une federation avec ce slug existe deja" }, 409);
    }

    const newFederation = {
      id: uuidv4(),
      nom: body.nom,
      slug: body.slug,
      description: body.description || null,
      logoUrl: body.logoUrl || null,
      siteWeb: body.siteWeb || null,
      email: body.email || null,
      telephone: body.telephone || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdFederation] = await db
      .insert(federations)
      .values(newFederation)
      .returning()
      .execute();

    return c.json(createdFederation, 201);
  } catch (error) {
    console.error("Erreur lors de la creation de la federation:", error);
    return c.json({ error: "Erreur serveur lors de la creation de la federation" }, 500);
  }
});

// PUT /federations/:id - Update federation
federationsRouter.put("/federations/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    const [existingFederation] = await db
      .select()
      .from(federations)
      .where(eq(federations.id, id))
      .execute();

    if (!existingFederation) {
      return c.json({ error: "Federation non trouvee" }, 404);
    }

    if (body.slug && body.slug !== existingFederation.slug) {
      const [federationWithSlug] = await db
        .select()
        .from(federations)
        .where(eq(federations.slug, body.slug))
        .execute();

      if (federationWithSlug) {
        return c.json({ error: "Une federation avec ce slug existe deja" }, 409);
      }
    }

    const updatedFederation = {
      nom: body.nom || existingFederation.nom,
      slug: body.slug || existingFederation.slug,
      description:
        body.description !== undefined ? body.description : existingFederation.description,
      logoUrl: body.logoUrl !== undefined ? body.logoUrl : existingFederation.logoUrl,
      siteWeb: body.siteWeb !== undefined ? body.siteWeb : existingFederation.siteWeb,
      email: body.email !== undefined ? body.email : existingFederation.email,
      telephone: body.telephone !== undefined ? body.telephone : existingFederation.telephone,
      misAJourLe: new Date(),
    };

    const [result] = await db
      .update(federations)
      .set(updatedFederation)
      .where(eq(federations.id, id))
      .returning()
      .execute();

    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise a jour de la federation:", error);
    return c.json({ error: "Erreur serveur lors de la mise a jour de la federation" }, 500);
  }
});

// DELETE /federations/:id - Delete federation
federationsRouter.delete("/federations/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [existingFederation] = await db
      .select()
      .from(federations)
      .where(eq(federations.id, id))
      .execute();

    if (!existingFederation) {
      return c.json({ error: "Federation non trouvee" }, 404);
    }

    const [hasClubs] = await db
      .select({ count: count() })
      .from(clubs)
      .where(eq(clubs.federationId, id))
      .execute();

    if (hasClubs?.count && hasClubs.count > 0) {
      return c.json(
        { error: "Impossible de supprimer une federation avec des clubs associes" },
        400,
      );
    }

    await db.delete(federations).where(eq(federations.id, id)).execute();

    return c.json({ message: "Federation supprimee avec succes" });
  } catch (error) {
    console.error("Erreur lors de la suppression de la federation:", error);
    return c.json({ error: "Erreur serveur lors de la suppression de la federation" }, 500);
  }
});

export default federationsRouter;
