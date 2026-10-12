import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { equipes, clubs, joueurs, tournois, equipesTournois, matchs } from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { sql } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const equipesRouter = new Hono();

// GET /equipes - List all teams
equipesRouter.get("/equipes", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;
    const clubId = query.clubId;
    const categorie = query.categorie;

    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(equipes.nom, `%${search}%`),
          like(equipes.slug, `%${search}%`),
        ),
      );
    }

    if (clubId) {
      whereCondition = and(
        whereCondition,
        eq(equipes.clubId, clubId),
      );
    }

    if (categorie) {
      whereCondition = and(
        whereCondition,
        eq(equipes.categorie, categorie as any),
      );
    }

    const [totalResult] = await db
      .select({ count: count() })
      .from(equipes)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    const equipesList = await db
      .select({
        equipe: equipes,
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
        capitaine: {
          id: joueurs.id,
          nom: joueurs.nom,
          prenom: joueurs.prenom,
          photoUrl: joueurs.photoUrl,
        },
      })
      .from(equipes)
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .leftJoin(joueurs, eq(equipes.capitaineId, joueurs.id))
      .where(whereCondition)
      .orderBy(desc(equipes.creeLe))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: equipesList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation des equipes:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation des equipes" },
      500,
    );
  }
});

// GET /equipes/:id - Get single team by ID
equipesRouter.get("/equipes/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [equipe] = await db
      .select({
        equipe: equipes,
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
          email: clubs.email,
          telephone: clubs.telephone,
        },
        capitaine: {
          id: joueurs.id,
          nom: joueurs.nom,
          prenom: joueurs.prenom,
          photoUrl: joueurs.photoUrl,
          email: joueurs.email,
          telephone: joueurs.telephone,
        },
      })
      .from(equipes)
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .leftJoin(joueurs, eq(equipes.capitaineId, joueurs.id))
      .where(eq(equipes.id, id))
      .execute();

    if (!equipe) {
      return c.json({ error: "Equipe non trouvee" }, 404);
    }

    // Get team tournaments
    const tournoisList = await db
      .select({
        tournoi: tournois,
        dateInscription: equipesTournois.dateInscription,
        paiementEffectue: equipesTournois.paiementEffectue,
        statut: equipesTournois.statut,
      })
      .from(equipesTournois)
      .leftJoin(tournois, eq(equipesTournois.tournoiId, tournois.id))
      .where(eq(equipesTournois.equipeId, id))
      .execute();

    // Get team matches
    const matchsList = await db
      .select({
        match: matchs,
        adversaire: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
        },
      })
      .from(matchs)
      .leftJoin(equipes, or(
        eq(matchs.equipeExterieurId, equipes.id),
        eq(matchs.equipeDomicileId, equipes.id),
      ))
      .where(or(
        eq(matchs.equipeDomicileId, id),
        eq(matchs.equipeExterieurId, id),
      ))
      .orderBy(desc(matchs.dateHeure))
      .execute();

    return c.json({
      ...equipe,
      tournois: tournoisList,
      matchs: matchsList,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation de l'equipe:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation de l'equipe" },
      500,
    );
  }
});

// POST /equipes - Create new team
equipesRouter.post("/equipes", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation
    if (!body.nom || !body.slug || !body.clubId) {
      return c.json(
        { error: "Les champs nom, slug et clubId sont requis" },
        400,
      );
    }

    // Verify club exists
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

    // Verify slug is unique
    const [existingEquipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.slug, body.slug))
      .execute();

    if (existingEquipe) {
      return c.json(
        { error: "Une equipe avec ce slug existe deja" },
        409,
      );
    }

    // Verify capitaine exists if provided
    if (body.capitaineId) {
      const [capitaine] = await db
        .select()
        .from(joueurs)
        .where(eq(joueurs.id, body.capitaineId))
        .execute();

      if (!capitaine) {
        return c.json(
          { error: "Le capitaine specifie n'existe pas" },
          400,
        );
      }
    }

    const newEquipe = {
      id: uuidv4(),
      clubId: body.clubId,
      nom: body.nom,
      slug: body.slug,
      capitaineId: body.capitaineId || null,
      nombreJoueurs: body.nombreJoueurs || 0,
      categorie: body.categorie || "senior",
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [created] = await db
      .insert(equipes)
      .values(newEquipe)
      .returning()
      .execute();

    return c.json(created, 201);
  } catch (error) {
    console.error("Erreur lors de la creation de l'equipe:", error);
    return c.json(
      { error: "Erreur serveur lors de la creation de l'equipe" },
      500,
    );
  }
});

// PUT /equipes/:id - Update team
equipesRouter.put("/equipes/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Verify equipe exists
    const [existingEquipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, id))
      .execute();

    if (!existingEquipe) {
      return c.json({ error: "Equipe non trouvee" }, 404);
    }

    // Verify slug is unique (if changing)
    if (body.slug && body.slug !== existingEquipe.slug) {
      const [equipeWithSlug] = await db
        .select()
        .from(equipes)
        .where(eq(equipes.slug, body.slug))
        .execute();

      if (equipeWithSlug) {
        return c.json(
          { error: "Une equipe avec ce slug existe deja" },
          409,
        );
      }
    }

    // Verify club exists if changing
    if (body.clubId && body.clubId !== existingEquipe.clubId) {
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

    // Verify capitaine exists if changing
    if (body.capitaineId && body.capitaineId !== existingEquipe.capitaineId) {
      const [capitaine] = await db
        .select()
        .from(joueurs)
        .where(eq(joueurs.id, body.capitaineId))
        .execute();

      if (!capitaine) {
        return c.json(
          { error: "Le capitaine specifie n'existe pas" },
          400,
        );
      }
    }

    const updatedData = {
      clubId: body.clubId || existingEquipe.clubId,
      nom: body.nom || existingEquipe.nom,
      slug: body.slug || existingEquipe.slug,
      capitaineId: body.capitaineId !== undefined ? body.capitaineId : existingEquipe.capitaineId,
      nombreJoueurs: body.nombreJoueurs !== undefined ? body.nombreJoueurs : existingEquipe.nombreJoueurs,
      categorie: body.categorie || existingEquipe.categorie,
      misAJourLe: new Date(),
    };

    const [updated] = await db
      .update(equipes)
      .set(updatedData)
      .where(eq(equipes.id, id))
      .returning()
      .execute();

    return c.json(updated);
  } catch (error) {
    console.error("Erreur lors de la mise a jour de l'equipe:", error);
    return c.json(
      { error: "Erreur serveur lors de la mise a jour de l'equipe" },
      500,
    );
  }
});

// DELETE /equipes/:id - Delete team
equipesRouter.delete("/equipes/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Verify equipe exists
    const [existingEquipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, id))
      .execute();

    if (!existingEquipe) {
      return c.json({ error: "Equipe non trouvee" }, 404);
    }

    await db
      .delete(equipes)
      .where(eq(equipes.id, id))
      .execute();

    return c.json({ message: "Equipe supprimee avec succes" });
  } catch (error) {
    console.error("Erreur lors de la suppression de l'equipe:", error);
    return c.json(
      { error: "Erreur serveur lors de la suppression de l'equipe" },
      500,
    );
  }
});

// POST /equipes/:id/joueurs - Add player to team
equipesRouter.post("/equipes/:id/joueurs", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    if (!body.joueurId) {
      return c.json(
        { error: "Le champ joueurId est requis" },
        400,
      );
    }

    // Verify equipe exists
    const [equipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, id))
      .execute();

    if (!equipe) {
      return c.json({ error: "Equipe non trouvee" }, 404);
    }

    // Verify joueur exists
    const [joueur] = await db
      .select()
      .from(joueurs)
      .where(eq(joueurs.id, body.joueurId))
      .execute();

    if (!joueur) {
      return c.json(
        { error: "Le joueur specifie n'existe pas" },
        400,
      );
    }

    // Update joueur's club
    await db
      .update(joueurs)
      .set({ clubId: id })
      .where(eq(joueurs.id, body.joueurId))
      .execute();

    // Update equipe's nombreJoueurs
    await db
      .update(equipes)
      .set({ nombreJoueurs: sql`${equipes.nombreJoueurs} + 1` })
      .where(eq(equipes.id, id))
      .execute();

    return c.json({
      message: "Joueur ajoute a l'equipe avec succes",
      joueurId: body.joueurId,
      equipeId: id,
    });
  } catch (error) {
    console.error("Erreur lors de l'ajout du joueur a l'equipe:", error);
    return c.json(
      { error: "Erreur serveur lors de l'ajout du joueur a l'equipe" },
      500,
    );
  }
});

// POST /equipes/:id/tournois/:tournoiId - Register team for tournament
equipesRouter.post("/equipes/:id/tournois/:tournoiId", async (c) => {
  try {
    const db = createDb();
    const equipeId = c.req.param("id");
    const tournoiId = c.req.param("tournoiId");
    const body = await c.req.json();

    // Verify equipe exists
    const [equipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, equipeId))
      .execute();

    if (!equipe) {
      return c.json({ error: "Equipe non trouvee" }, 404);
    }

    // Verify tournoi exists
    const [tournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.id, tournoiId))
      .execute();

    if (!tournoi) {
      return c.json({ error: "Tournoi non trouve" }, 404);
    }

    // Check if already registered
    const [existingRegistration] = await db
      .select()
      .from(equipesTournois)
      .where(
        and(
          eq(equipesTournois.equipeId, equipeId),
          eq(equipesTournois.tournoiId, tournoiId),
        ),
      )
      .execute();

    if (existingRegistration) {
      return c.json(
        { error: "Cette equipe est deja inscrite a ce tournoi" },
        409,
      );
    }

    // Check if tournament is full
    if ((tournoi.nombreEquipesMax || 0) <= (tournoi.nombreEquipesInscrites || 0)) {
      return c.json(
        { error: "Le tournoi a atteint le nombre maximum d'equipes" },
        400,
      );
    }

    const newRegistration = {
      id: uuidv4(),
      tournoiId,
      equipeId,
      dateInscription: new Date(),
      paiementEffectue: body.paiementEffectue || 0,
      statut: body.statut || "inscrite",
      creeLe: new Date(),
    };

    await db
      .insert(equipesTournois)
      .values(newRegistration)
      .execute();

    // Update tournament's nombreEquipesInscrites
    await db
      .update(tournois)
      .set({ nombreEquipesInscrites: sql`${tournois.nombreEquipesInscrites} + 1` })
      .where(eq(tournois.id, tournoiId))
      .execute();

    return c.json({
      message: "Equipe inscrite au tournoi avec succes",
      registration: newRegistration,
    }, 201);
  } catch (error) {
    console.error("Erreur lors de l'inscription de l'equipe au tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de l'inscription de l'equipe au tournoi" },
      500,
    );
  }
});

export default equipesRouter;
