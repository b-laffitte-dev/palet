import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { actualites, joueurs, clubs } from "../../db/schema.js";
import { eq, and, desc, like, or, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const actualitesRouter = new Hono();

// GET /actualites - List all news items
actualitesRouter.get("/actualites", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;
    const clubId = query.clubId;
    const auteurId = query.auteurId;
    const categorie = query.categorie;
    const statut = query.statut;

    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(actualites.titre, `%${search}%`),
          like(actualites.contenu, `%${search}%`),
          like(actualites.resume, `%${search}%`),
          like(actualites.categories, `%${search}%`),
        ),
      );
    }

    if (clubId) {
      whereCondition = and(
        whereCondition,
        eq(actualites.clubId, clubId),
      );
    }

    if (auteurId) {
      whereCondition = and(
        whereCondition,
        eq(actualites.auteurId, auteurId),
      );
    }

    if (categorie) {
      whereCondition = and(
        whereCondition,
        like(actualites.categories, `%${categorie}%`),
      );
    }

    if (statut) {
      whereCondition = and(
        whereCondition,
        eq(actualites.statut, statut),
      );
    }

    const [totalResult] = await db
      .select({ count: count() })
      .from(actualites)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    const actualitesList = await db
      .select({
        actualite: actualites,
        auteur: {
          id: joueurs.id,
          nom: joueurs.nom,
          prenom: joueurs.prenom,
          photoUrl: joueurs.photoUrl,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(actualites)
      .leftJoin(joueurs, eq(actualites.auteurId, joueurs.id))
      .leftJoin(clubs, eq(actualites.clubId, clubs.id))
      .where(whereCondition)
      .orderBy(desc(actualites.datePublication))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: actualitesList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation des actualites:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation des actualites" },
      500,
    );
  }
});

// GET /actualites/:id - Get single news item by ID
actualitesRouter.get("/actualites/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [actualite] = await db
      .select({
        actualite: actualites,
        auteur: {
          id: joueurs.id,
          nom: joueurs.nom,
          prenom: joueurs.prenom,
          photoUrl: joueurs.photoUrl,
          email: joueurs.email,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(actualites)
      .leftJoin(joueurs, eq(actualites.auteurId, joueurs.id))
      .leftJoin(clubs, eq(actualites.clubId, clubs.id))
      .where(eq(actualites.id, id))
      .execute();

    if (!actualite) {
      return c.json({ error: "Actualite non trouvee" }, 404);
    }

    return c.json(actualite);
  } catch (error) {
    console.error("Erreur lors de la recuperation de l'actualite:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation de l'actualite" },
      500,
    );
  }
});

// POST /actualites - Create new news item
actualitesRouter.post("/actualites", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation
    if (!body.titre || !body.slug || !body.contenu) {
      return c.json(
        { error: "Les champs titre, slug et contenu sont requis" },
        400,
      );
    }

    // Verify slug is unique
    const [existingActualite] = await db
      .select()
      .from(actualites)
      .where(eq(actualites.slug, body.slug))
      .execute();

    if (existingActualite) {
      return c.json(
        { error: "Une actualite avec ce slug existe deja" },
        409,
      );
    }

    // Verify auteur exists if provided
    if (body.auteurId) {
      const [auteur] = await db
        .select()
        .from(joueurs)
        .where(eq(joueurs.id, body.auteurId))
        .execute();

      if (!auteur) {
        return c.json(
          { error: "L'auteur specifie n'existe pas" },
          400,
        );
      }
    }

    // Verify club exists if provided
    if (body.clubId) {
      const [club] = await db
        .select()
        .from(clubs)
        .where(eq(clubs.id, body.clubId))
        .execute();

      if (!club) {
        return c.json(
          { error: "Le club specifie n'existe pas" },
          400,
        );
      }
    }

    const newActualite = {
      id: uuidv4(),
      titre: body.titre,
      slug: body.slug,
      contenu: body.contenu,
      resume: body.resume || null,
      imageUrl: body.imageUrl || null,
      auteurId: body.auteurId || null,
      clubId: body.clubId || null,
      datePublication: body.datePublication || new Date(),
      dateEvenement: body.dateEvenement || null,
      categories: body.categories || null,
      statut: body.statut || "publie",
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [created] = await db
      .insert(actualites)
      .values(newActualite)
      .returning()
      .execute();

    return c.json(created, 201);
  } catch (error) {
    console.error("Erreur lors de la creation de l'actualite:", error);
    return c.json(
      { error: "Erreur serveur lors de la creation de l'actualite" },
      500,
    );
  }
});

// PUT /actualites/:id - Update news item
actualitesRouter.put("/actualites/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Verify actualite exists
    const [existingActualite] = await db
      .select()
      .from(actualites)
      .where(eq(actualites.id, id))
      .execute();

    if (!existingActualite) {
      return c.json({ error: "Actualite non trouvee" }, 404);
    }

    // Verify slug is unique (if changing)
    if (body.slug && body.slug !== existingActualite.slug) {
      const [actualiteWithSlug] = await db
        .select()
        .from(actualites)
        .where(eq(actualites.slug, body.slug))
        .execute();

      if (actualiteWithSlug) {
        return c.json(
          { error: "Une actualite avec ce slug existe deja" },
          409,
        );
      }
    }

    // Verify auteur exists if provided
    if (body.auteurId) {
      const [auteur] = await db
        .select()
        .from(joueurs)
        .where(eq(joueurs.id, body.auteurId))
        .execute();

      if (!auteur) {
        return c.json(
          { error: "L'auteur specifie n'existe pas" },
          400,
        );
      }
    }

    // Verify club exists if provided
    if (body.clubId) {
      const [club] = await db
        .select()
        .from(clubs)
        .where(eq(clubs.id, body.clubId))
        .execute();

      if (!club) {
        return c.json(
          { error: "Le club specifie n'existe pas" },
          400,
        );
      }
    }

    const updatedData = {
      titre: body.titre || existingActualite.titre,
      slug: body.slug || existingActualite.slug,
      contenu: body.contenu || existingActualite.contenu,
      resume: body.resume !== undefined ? body.resume : existingActualite.resume,
      imageUrl: body.imageUrl !== undefined ? body.imageUrl : existingActualite.imageUrl,
      auteurId: body.auteurId !== undefined ? body.auteurId : existingActualite.auteurId,
      clubId: body.clubId !== undefined ? body.clubId : existingActualite.clubId,
      datePublication: body.datePublication || existingActualite.datePublication,
      dateEvenement: body.dateEvenement !== undefined ? body.dateEvenement : existingActualite.dateEvenement,
      categories: body.categories !== undefined ? body.categories : existingActualite.categories,
      statut: body.statut || existingActualite.statut,
      misAJourLe: new Date(),
    };

    const [updated] = await db
      .update(actualites)
      .set(updatedData)
      .where(eq(actualites.id, id))
      .returning()
      .execute();

    return c.json(updated);
  } catch (error) {
    console.error("Erreur lors de la mise a jour de l'actualite:", error);
    return c.json(
      { error: "Erreur serveur lors de la mise a jour de l'actualite" },
      500,
    );
  }
});

// DELETE /actualites/:id - Delete news item
actualitesRouter.delete("/actualites/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Verify actualite exists
    const [existingActualite] = await db
      .select()
      .from(actualites)
      .where(eq(actualites.id, id))
      .execute();

    if (!existingActualite) {
      return c.json({ error: "Actualite non trouvee" }, 404);
    }

    await db
      .delete(actualites)
      .where(eq(actualites.id, id))
      .execute();

    return c.json({ message: "Actualite supprimee avec succes" });
  } catch (error) {
    console.error("Erreur lors de la suppression de l'actualite:", error);
    return c.json(
      { error: "Erreur serveur lors de la suppression de l'actualite" },
      500,
    );
  }
});

export default actualitesRouter;
