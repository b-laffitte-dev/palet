import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { matchs, equipes, clubs, championnats, tournois, statistiquesJoueurs, joueurs } from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const matchsRouter = new Hono();

// GET /matchs - List all matchs
matchsRouter.get("/matchs", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const competitionId = query.competitionId;
    const competitionType = query.competitionType as string;
    const statut = query.statut as string;
    const date = query.date;
    const phase = query.phase;
    const equipeId = query.equipeId;

    let whereCondition = and();

    if (competitionId) {
      whereCondition = and(whereCondition, eq(matchs.competitionId, competitionId));
    }

    if (competitionType) {
      whereCondition = and(whereCondition, eq(matchs.competitionType, competitionType as any));
    }

    if (statut) {
      whereCondition = and(whereCondition, eq(matchs.statut, statut as any));
    }

    if (phase) {
      whereCondition = and(whereCondition, like(matchs.phase, `%${phase}%`));
    }

    if (date) {
      whereCondition = and(whereCondition, like(matchs.dateHeure, `%${date}%`));
    }

    if (equipeId) {
      whereCondition = and(
        whereCondition,
        or(eq(matchs.equipeDomicileId, equipeId), eq(matchs.equipeExterieurId, equipeId)),
      );
    }

    const [totalResult] = await db
      .select({ count: count() })
      .from(matchs)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    const matchsWithDetails = await db
      .select({
        match: matchs,
        equipeDomicile: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          categorie: equipes.categorie,
        },
        equipeExterieur: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          categorie: equipes.categorie,
        },
      })
      .from(matchs)
      .leftJoin(equipes, eq(matchs.equipeDomicileId, equipes.id))
      .leftJoin(equipes, eq(matchs.equipeExterieurId, equipes.id))
      .where(whereCondition)
      .orderBy(desc(matchs.dateHeure))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: matchsWithDetails,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation des matchs:", error);
    return c.json({ error: "Erreur serveur lors de la recuperation des matchs" }, 500);
  }
});

// GET /matchs/:id - Get match by ID
matchsRouter.get("/matchs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [match] = await db.select().from(matchs).where(eq(matchs.id, id)).execute();

    if (!match) {
      return c.json({ error: "Match non trouve" }, 404);
    }

    const [equipeDomicile] = await db.select().from(equipes).where(eq(equipes.id, match.equipeDomicileId)).execute();
    const [equipeExterieur] = await db.select().from(equipes).where(eq(equipes.id, match.equipeExterieurId)).execute();

    let competition = null;
    if (match.competitionType === "championnat") {
      const [championnat] = await db.select().from(championnats).where(eq(championnats.id, match.competitionId)).execute();
      competition = championnat;
    } else if (match.competitionType === "tournoi") {
      const [tournoi] = await db.select().from(tournois).where(eq(tournois.id, match.competitionId)).execute();
      competition = tournoi;
    }

    const stats = await db.select().from(statistiquesJoueurs).where(eq(statistiquesJoueurs.matchId, id)).execute();

    return c.json({ ...match, equipeDomicile, equipeExterieur, competition, statistiques: stats });
  } catch (error) {
    console.error("Erreur lors de la recuperation du match:", error);
    return c.json({ error: "Erreur serveur lors de la recuperation du match" }, 500);
  }
});

// POST /matchs - Create new match
matchsRouter.post("/matchs", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    if (!body.competitionId || !body.competitionType || !body.equipeDomicileId || !body.equipeExterieurId || !body.dateHeure || !body.phase) {
      return c.json({ error: "Les champs competitionId, competitionType, equipeDomicileId, equipeExterieurId, dateHeure et phase sont requis" }, 400);
    }

    const [equipeDomicile] = await db.select().from(equipes).where(eq(equipes.id, body.equipeDomicileId)).execute();
    const [equipeExterieur] = await db.select().from(equipes).where(eq(equipes.id, body.equipeExterieurId)).execute();

    if (!equipeDomicile || !equipeExterieur) {
      return c.json({ error: "Une ou plusieurs equipes n'existent pas" }, 400);
    }

    if (body.equipeDomicileId === body.equipeExterieurId) {
      return c.json({ error: "Les equipes domicile et exterieur doivent etre differentes" }, 400);
    }

    let competitionExists = false;
    if (body.competitionType === "championnat") {
      const [championnat] = await db.select().from(championnats).where(eq(championnats.id, body.competitionId)).execute();
      competitionExists = !!championnat;
    } else if (body.competitionType === "tournoi") {
      const [tournoi] = await db.select().from(tournois).where(eq(tournois.id, body.competitionId)).execute();
      competitionExists = !!tournoi;
    }

    if (!competitionExists) {
      return c.json({ error: "La competition specifiee n'existe pas" }, 400);
    }

    const newMatch = {
      id: uuidv4(),
      competitionId: body.competitionId,
      competitionType: body.competitionType as any,
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
    console.error("Erreur lors de la creation du match:", error);
    return c.json({ error: "Erreur serveur lors de la creation du match" }, 500);
  }
});

// PUT /matchs/:id - Update match
matchsRouter.put("/matchs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    const [existingMatch] = await db.select().from(matchs).where(eq(matchs.id, id)).execute();

    if (!existingMatch) {
      return c.json({ error: "Match non trouve" }, 404);
    }

    let statut = body.statut || existingMatch.statut;
    if (body.scoreDomicile !== undefined || body.scoreExterieur !== undefined) {
      const scoreDomicile = body.scoreDomicile !== undefined ? body.scoreDomicile : existingMatch.scoreDomicile;
      const scoreExterieur = body.scoreExterieur !== undefined ? body.scoreExterieur : existingMatch.scoreExterieur;
      if (scoreDomicile > 0 || scoreExterieur > 0) {
        statut = "termine";
      }
    }

    const updatedMatch = {
      tourId: body.tourId !== undefined ? body.tourId : existingMatch.tourId,
      phase: body.phase || existingMatch.phase,
      numeroTour: body.numeroTour !== undefined ? body.numeroTour : existingMatch.numeroTour,
      numeroMatch: body.numeroMatch !== undefined ? body.numeroMatch : existingMatch.numeroMatch,
      equipeDomicileId: body.equipeDomicileId !== undefined ? body.equipeDomicileId : existingMatch.equipeDomicileId,
      equipeExterieurId: body.equipeExterieurId !== undefined ? body.equipeExterieurId : existingMatch.equipeExterieurId,
      scoreDomicile: body.scoreDomicile !== undefined ? body.scoreDomicile : existingMatch.scoreDomicile,
      scoreExterieur: body.scoreExterieur !== undefined ? body.scoreExterieur : existingMatch.scoreExterieur,
      statut: statut,
      dateHeure: body.dateHeure ? new Date(body.dateHeure) : existingMatch.dateHeure,
      lieu: body.lieu !== undefined ? body.lieu : existingMatch.lieu,
      arbitre: body.arbitre !== undefined ? body.arbitre : existingMatch.arbitre,
      rapport: body.rapport !== undefined ? body.rapport : existingMatch.rapport,
      misAJourLe: new Date(),
    };

    const [result] = await db.update(matchs).set(updatedMatch).where(eq(matchs.id, id)).returning().execute();
    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise a jour du match:", error);
    return c.json({ error: "Erreur serveur lors de la mise a jour du match" }, 500);
  }
});

// DELETE /matchs/:id - Delete match
matchsRouter.delete("/matchs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [existingMatch] = await db.select().from(matchs).where(eq(matchs.id, id)).execute();
    if (!existingMatch) {
      return c.json({ error: "Match non trouve" }, 404);
    }

    await db.delete(matchs).where(eq(matchs.id, id)).execute();
    return c.json({ message: "Match supprime avec succes" });
  } catch (error) {
    console.error("Erreur lors de la suppression du match:", error);
    return c.json({ error: "Erreur serveur lors de la suppression du match" }, 500);
  }
});

// POST /matchs/:id/statistiques - Add statistics for a match
matchsRouter.post("/matchs/:id/statistiques", async (c) => {
  try {
    const db = createDb();
    const matchId = c.req.param("id");
    const body = await c.req.json();

    if (!body.joueurId) {
      return c.json({ error: "Le champ joueurId est requis" }, 400);
    }

    const [match] = await db.select().from(matchs).where(eq(matchs.id, matchId)).execute();
    if (!match) {
      return c.json({ error: "Match non trouve" }, 404);
    }

    const [joueur] = await db.select().from(joueurs).where(eq(joueurs.id, body.joueurId)).execute();
    if (!joueur) {
      return c.json({ error: "Le joueur specifie n'existe pas" }, 400);
    }

    const [existingStat] = await db
      .select()
      .from(statistiquesJoueurs)
      .where(and(eq(statistiquesJoueurs.joueurId, body.joueurId), eq(statistiquesJoueurs.matchId, matchId)))
      .execute();

    const statData = {
      id: existingStat?.id || uuidv4(),
      joueurId: body.joueurId,
      matchId: matchId,
      date: new Date(),
      pointsMarques: body.pointsMarques || 0,
      butsMarques: body.butsMarques || 0,
      butsEncaises: body.butsEncaises || 0,
      passesDecisives: body.passesDecisives || 0,
      interceptions: body.interceptions || 0,
      fautes: body.fautes || 0,
      cartesJaunes: body.cartesJaunes || 0,
      cartesRouges: body.cartesRouges || 0,
      tempsDeJeu: body.tempsDeJeu || null,
      creeLe: existingStat ? existingStat.creeLe : new Date(),
    };

    let result;
    if (existingStat) {
      [result] = await db.update(statistiquesJoueurs).set(statData).where(eq(statistiquesJoueurs.id, existingStat.id)).returning().execute();
    } else {
      [result] = await db.insert(statistiquesJoueurs).values(statData).returning().execute();
    }

    return c.json(result, existingStat ? 200 : 201);
  } catch (error) {
    console.error("Erreur lors de l'ajout des statistiques:", error);
    return c.json({ error: "Erreur serveur lors de l'ajout des statistiques" }, 500);
  }
});

export default matchsRouter;
