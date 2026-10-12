import { Hono } from "hono";
import { sql } from "drizzle-orm";
import { createDb } from "../db/client.js";
import {
  championnats,
  federations,
  clubs,
  equipes,
  classementsChampionnat,
  matchs,
} from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const championnatsRouter = new Hono();

// GET /championnats - Lister tous les championnats
championnatsRouter.get("/championnats", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;
    const federationId = query.federationId;
    const statut = query.statut as string;
    const saison = query.saison;

    // Construction de la condition WHERE
    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(championnats.nom, `%${search}%`),
          like(championnats.slug, `%${search}%`),
          like(championnats.saison, `%${search}%`),
        ),
      );
    }

    if (federationId) {
      whereCondition = and(whereCondition, eq(championnats.federationId, federationId));
    }

    if (statut) {
      whereCondition = and(whereCondition, eq(championnats.statut, statut as any));
    }

    if (saison) {
      whereCondition = and(whereCondition, like(championnats.saison, `%${saison}%`));
    }

    // Compter le total
    const [totalResult] = await db
      .select({ count: count() })
      .from(championnats)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    // Récupérer les championnats avec leurs fédérations
    const championnatsList = await db
      .select({
        championnat: championnats,
        federation: {
          id: federations.id,
          nom: federations.nom,
          slug: federations.slug,
        },
      })
      .from(championnats)
      .leftJoin(federations, eq(championnats.federationId, federations.id))
      .where(whereCondition)
      .orderBy(desc(championnats.dateDebut))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: championnatsList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération des championnats:", error);
    return c.json({ error: "Erreur serveur lors de la récupération des championnats" }, 500);
  }
});

// GET /championnats/:id - Obtenir un championnat par ID
championnatsRouter.get("/championnats/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [championnat] = await db
      .select({
        championnat: championnats,
        federation: {
          id: federations.id,
          nom: federations.nom,
          slug: federations.slug,
          logoUrl: federations.logoUrl,
        },
      })
      .from(championnats)
      .leftJoin(federations, eq(championnats.federationId, federations.id))
      .where(eq(championnats.id, id))
      .execute();

    if (!championnat) {
      return c.json({ error: "Championnat non trouvé" }, 404);
    }

    // Récupérer le classement
    const classement = await db
      .select({
        classement: classementsChampionnat,
        equipe: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          categorie: equipes.categorie,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(classementsChampionnat)
      .leftJoin(equipes, eq(classementsChampionnat.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(eq(classementsChampionnat.championnatId, id))
      .orderBy(classementsChampionnat.position)
      .execute();

    // Récupérer les matchs
    const matchsList = await db
      .select({
        match: matchs,
        equipeDomicile: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
        },
        equipeExterieur: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
        },
      })
      .from(matchs)
      .leftJoin(equipes, eq(matchs.equipeDomicileId, equipes.id))
      .leftJoin(equipes, eq(matchs.equipeExterieurId, equipes.id))
      .where(and(eq(matchs.competitionId, id), eq(matchs.competitionType, "championnat")))
      .orderBy(desc(matchs.dateHeure))
      .execute();

    return c.json({
      ...championnat,
      classement,
      matchs: matchsList,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération du championnat:", error);
    return c.json({ error: "Erreur serveur lors de la récupération du championnat" }, 500);
  }
});

// POST /championnats - Créer un nouveau championnat
championnatsRouter.post("/championnats", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation basique
    if (
      !body.nom ||
      !body.slug ||
      !body.federationId ||
      !body.saison ||
      !body.dateDebut ||
      !body.dateFin
    ) {
      return c.json(
        { error: "Les champs nom, slug, federationId, saison, dateDebut et dateFin sont requis" },
        400,
      );
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
    const [existingChampionnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.slug, body.slug))
      .execute();

    if (existingChampionnat) {
      return c.json({ error: "Un championnat avec ce slug existe déjà" }, 409);
    }

    // Créer le championnat
    const newChampionnat = {
      id: uuidv4(),
      federationId: body.federationId,
      nom: body.nom,
      slug: body.slug,
      saison: body.saison,
      description: body.description || null,
      statut: body.statut || "planifie",
      dateDebut: new Date(body.dateDebut),
      dateFin: new Date(body.dateFin),
      nombreEquipes: body.nombreEquipes || 0,
      reglement: body.reglement || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdChampionnat] = await db
      .insert(championnats)
      .values(newChampionnat)
      .returning()
      .execute();

    return c.json(createdChampionnat, 201);
  } catch (error) {
    console.error("Erreur lors de la création du championnat:", error);
    return c.json({ error: "Erreur serveur lors de la création du championnat" }, 500);
  }
});

// PUT /championnats/:id - Mettre à jour un championnat
championnatsRouter.put("/championnats/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Vérifier que le championnat existe
    const [existingChampionnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, id))
      .execute();

    if (!existingChampionnat) {
      return c.json({ error: "Championnat non trouvé" }, 404);
    }

    // Si le slug est modifié, vérifier qu'il est unique
    if (body.slug && body.slug !== existingChampionnat.slug) {
      const [championnatWithSameSlug] = await db
        .select()
        .from(championnats)
        .where(and(eq(championnats.slug, body.slug), ne(championnats.id, id)))
        .execute();

      if (championnatWithSameSlug) {
        return c.json({ error: "Un championnat avec ce slug existe déjà" }, 409);
      }
    }

    // Si la fédération est modifiée, vérifier qu'elle existe
    if (body.federationId && body.federationId !== existingChampionnat.federationId) {
      const [federation] = await db
        .select()
        .from(federations)
        .where(eq(federations.id, body.federationId))
        .execute();

      if (!federation) {
        return c.json({ error: "La fédération spécifiée n'existe pas" }, 400);
      }
    }

    // Mettre à jour le championnat
    const updatedChampionnat = {
      nom: body.nom || existingChampionnat.nom,
      slug: body.slug || existingChampionnat.slug,
      saison: body.saison || existingChampionnat.saison,
      description:
        body.description !== undefined ? body.description : existingChampionnat.description,
      statut: body.statut || existingChampionnat.statut,
      dateDebut: body.dateDebut ? new Date(body.dateDebut) : existingChampionnat.dateDebut,
      dateFin: body.dateFin ? new Date(body.dateFin) : existingChampionnat.dateFin,
      nombreEquipes:
        body.nombreEquipes !== undefined ? body.nombreEquipes : existingChampionnat.nombreEquipes,
      reglement: body.reglement !== undefined ? body.reglement : existingChampionnat.reglement,
      federationId: body.federationId || existingChampionnat.federationId,
      misAJourLe: new Date(),
    };

    const [result] = await db
      .update(championnats)
      .set(updatedChampionnat)
      .where(eq(championnats.id, id))
      .returning()
      .execute();

    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise à jour du championnat:", error);
    return c.json({ error: "Erreur serveur lors de la mise à jour du championnat" }, 500);
  }
});

// DELETE /championnats/:id - Supprimer un championnat
championnatsRouter.delete("/championnats/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Vérifier que le championnat existe
    const [existingChampionnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, id))
      .execute();

    if (!existingChampionnat) {
      return c.json({ error: "Championnat non trouvé" }, 404);
    }

    // Supprimer le championnat
    await db.delete(championnats).where(eq(championnats.id, id)).execute();

    return c.json({ message: "Championnat supprimé avec succès" });
  } catch (error) {
    console.error("Erreur lors de la suppression du championnat:", error);
    return c.json({ error: "Erreur serveur lors de la suppression du championnat" }, 500);
  }
});

// POST /championnats/:id/classement - Mettre à jour le classement
championnatsRouter.post("/championnats/:id/classement", async (c) => {
  try {
    const db = createDb();
    const championnatId = c.req.param("id");
    const body = await c.req.json();

    // Validation
    if (!Array.isArray(body.classement)) {
      return c.json({ error: "Le champ classement doit être un tableau" }, 400);
    }

    // Vérifier que le championnat existe
    const [championnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, championnatId))
      .execute();

    if (!championnat) {
      return c.json({ error: "Championnat non trouvé" }, 404);
    }

    // Supprimer l'ancien classement
    await db
      .delete(classementsChampionnat)
      .where(eq(classementsChampionnat.championnatId, championnatId))
      .execute();

    // Insérer le nouveau classement
    const newClassement = body.classement.map((item: any, index: number) => ({
      id: uuidv4(),
      championnatId,
      equipeId: item.equipeId,
      position: index + 1,
      points: item.points || 0,
      joues: item.joues || 0,
      victoires: item.victoires || 0,
      défaites: item.defaites || 0,
      nuls: item.nuls || 0,
      pour: item.pour || 0,
      contre: item.contre || 0,
      difference: item.difference || 0,
      creeLe: new Date(),
      misAJourLe: new Date(),
    }));

    const createdClassement = await db
      .insert(classementsChampionnat)
      .values(newClassement)
      .returning()
      .execute();

    return c.json(createdClassement, 201);
  } catch (error) {
    console.error("Erreur lors de la mise à jour du classement:", error);
    return c.json({ error: "Erreur serveur lors de la mise à jour du classement" }, 500);
  }
});

// POST /championnats/:id/matchs - Ajouter un match au championnat
championnatsRouter.post("/championnats/:id/matchs", async (c) => {
  try {
    const db = createDb();
    const championnatId = c.req.param("id");
    const body = await c.req.json();

    // Validation
    if (!body.equipeDomicileId || !body.equipeExterieurId || !body.dateHeure || !body.phase) {
      return c.json(
        { error: "Les champs equipeDomicileId, equipeExterieurId, dateHeure et phase sont requis" },
        400,
      );
    }

    // Vérifier que le championnat existe
    const [championnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, championnatId))
      .execute();

    if (!championnat) {
      return c.json({ error: "Championnat non trouvé" }, 404);
    }

    // Vérifier que les equipes existent
    const [equipeDomicile] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, body.equipeDomicileId))
      .execute();

    const [equipeExterieur] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, body.equipeExterieurId))
      .execute();

    if (!equipeDomicile || !equipeExterieur) {
      return c.json({ error: "Une ou plusieurs equipes n'existent pas" }, 400);
    }

    // Créer le match
    const newMatch = {
      id: uuidv4(),
      competitionId: championnatId,
      competitionType: "championnat" as const,
      tourId: body.tourId || null,
      phase: body.phase,
      numeroTour: body.numeroTour || null,
      numeroMatch: body.numeroMatch || 1,
      equipeDomicileId: body.equipeDomicileId,
      equipeExterieurId: body.equipeExterieurId,
      scoreDomicile: body.scoreDomicile || 0,
      scoreExterieur: body.scoreExterieur || 0,
      statut: body.statut || "planifie",
      dateHeure: new Date(body.dateHeure),
      lieu: body.lieu || null,
      arbitre: body.arbitre || null,
      rapport: body.rapport || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdMatch] = await db.insert(matchs).values(newMatch).returning().execute();

    return c.json(createdMatch, 201);
  } catch (error) {
    console.error("Erreur lors de la création du match:", error);
    return c.json({ error: "Erreur serveur lors de la création du match" }, 500);
  }
});

export default championnatsRouter;
