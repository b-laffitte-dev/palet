import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { clubs, federations } from "../../db/schema.js";
import { eq, and, desc, like, or, ne } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const clubsRouter = new Hono();

// GET /clubs - Lister tous les clubs
clubsRouter.get("/clubs", async (c) => {
  try {
    const db = createDb();
    const clubsList = await db.select().from(clubs).orderBy(desc(clubs.creeLe)).execute();

    return c.json({
      data: clubsList,
      total: clubsList.length,
      page: 1,
      pageSize: clubsList.length,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération des clubs:", error);
    return c.json({ error: "Erreur serveur lors de la récupération des clubs" }, 500);
  }
});

// GET /clubs/:id - Obtenir un club par ID
clubsRouter.get("/clubs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [club] = await db.select().from(clubs).where(eq(clubs.id, id)).execute();

    if (!club) {
      return c.json({ error: "Club non trouvé" }, 404);
    }

    return c.json(club);
  } catch (error) {
    console.error("Erreur lors de la récupération du club:", error);
    return c.json({ error: "Erreur serveur lors de la récupération du club" }, 500);
  }
});

// POST /clubs - Créer un nouveau club
clubsRouter.post("/clubs", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation basique
    if (!body.nom || !body.slug || !body.ville || !body.federationId) {
      return c.json({ error: "Les champs nom, slug, ville et federationId sont requis" }, 400);
    }

    // Vérifier que la fédération existe
    const [federation] = await db
      .select()
      .from(federations)
      .where(eq(federations.id, body.federationId))
      .execute();

    if (!federation) {
      return c.json({ error: "La fédération spécifiée n'existe pas" }, 400);
    }

    // Vérifier que le slug est unique
    const [existingClub] = await db.select().from(clubs).where(eq(clubs.slug, body.slug)).execute();

    if (existingClub) {
      return c.json({ error: "Un club avec ce slug existe déjà" }, 409);
    }

    // Créer le club
    const newClub = {
      id: uuidv4(),
      federationId: body.federationId,
      nom: body.nom,
      slug: body.slug,
      ville: body.ville,
      codePostal: body.codePostal || null,
      adresse: body.adresse || null,
      logoUrl: body.logoUrl || null,
      siteWeb: body.siteWeb || null,
      email: body.email || null,
      telephone: body.telephone || null,
      anneeFondation: body.anneeFondation || null,
      nombreLicencies: body.nombreLicencies || 0,
      couleurPrincipale: body.couleurPrincipale || null,
      couleurSecondaire: body.couleurSecondaire || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdClub] = await db.insert(clubs).values(newClub).returning().execute();

    return c.json(createdClub, 201);
  } catch (error) {
    console.error("Erreur lors de la création du club:", error);
    return c.json({ error: "Erreur serveur lors de la création du club" }, 500);
  }
});

// PUT /clubs/:id - Mettre à jour un club
clubsRouter.put("/clubs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Vérifier que le club existe
    const [existingClub] = await db.select().from(clubs).where(eq(clubs.id, id)).execute();

    if (!existingClub) {
      return c.json({ error: "Club non trouvé" }, 404);
    }

    // Si le slug est modifié, vérifier qu'il est unique
    if (body.slug && body.slug !== existingClub.slug) {
      const [clubWithSameSlug] = await db
        .select()
        .from(clubs)
        .where(and(eq(clubs.slug, body.slug), ne(clubs.id, id)))
        .execute();

      if (clubWithSameSlug) {
        return c.json({ error: "Un club avec ce slug existe déjà" }, 409);
      }
    }

    // Si la fédération est modifiée, vérifier qu'elle existe
    if (body.federationId && body.federationId !== existingClub.federationId) {
      const [federation] = await db
        .select()
        .from(federations)
        .where(eq(federations.id, body.federationId))
        .execute();

      if (!federation) {
        return c.json({ error: "La fédération spécifiée n'existe pas" }, 400);
      }
    }

    // Mettre à jour le club
    const updatedClub = {
      nom: body.nom || existingClub.nom,
      slug: body.slug || existingClub.slug,
      ville: body.ville || existingClub.ville,
      codePostal: body.codePostal !== undefined ? body.codePostal : existingClub.codePostal,
      adresse: body.adresse !== undefined ? body.adresse : existingClub.adresse,
      logoUrl: body.logoUrl !== undefined ? body.logoUrl : existingClub.logoUrl,
      siteWeb: body.siteWeb !== undefined ? body.siteWeb : existingClub.siteWeb,
      email: body.email !== undefined ? body.email : existingClub.email,
      telephone: body.telephone !== undefined ? body.telephone : existingClub.telephone,
      federationId: body.federationId || existingClub.federationId,
      anneeFondation:
        body.anneeFondation !== undefined ? body.anneeFondation : existingClub.anneeFondation,
      nombreLicencies:
        body.nombreLicencies !== undefined ? body.nombreLicencies : existingClub.nombreLicencies,
      couleurPrincipale:
        body.couleurPrincipale !== undefined
          ? body.couleurPrincipale
          : existingClub.couleurPrincipale,
      couleurSecondaire:
        body.couleurSecondaire !== undefined
          ? body.couleurSecondaire
          : existingClub.couleurSecondaire,
      misAJourLe: new Date(),
    };

    const [result] = await db
      .update(clubs)
      .set(updatedClub)
      .where(eq(clubs.id, id))
      .returning()
      .execute();

    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise à jour du club:", error);
    return c.json({ error: "Erreur serveur lors de la mise à jour du club" }, 500);
  }
});

// DELETE /clubs/:id - Supprimer un club
clubsRouter.delete("/clubs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Vérifier que le club existe
    const [existingClub] = await db.select().from(clubs).where(eq(clubs.id, id)).execute();

    if (!existingClub) {
      return c.json({ error: "Club non trouvé" }, 404);
    }

    // Supprimer le club
    await db.delete(clubs).where(eq(clubs.id, id)).execute();

    return c.json({ message: "Club supprimé avec succès" });
  } catch (error) {
    console.error("Erreur lors de la suppression du club:", error);
    return c.json({ error: "Erreur serveur lors de la suppression du club" }, 500);
  }
});

export default clubsRouter;
