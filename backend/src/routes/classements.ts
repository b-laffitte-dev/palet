import { Hono } from "hono";
import { createDb } from "../db/client.js";
import {
  classementsChampionnat,
  championnats,
  equipes,
  clubs,
  matchs,
} from "../../db/schema.js";
import { eq, and, desc, like, or, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const classementsRouter = new Hono();

// GET /classements - List all championship rankings
classementsRouter.get("/classements", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "20");
    const championnatId = query.championnatId;
    const clubId = query.clubId;
    const search = query.search;

    let whereCondition = and();

    if (championnatId) {
      whereCondition = and(
        whereCondition,
        eq(classementsChampionnat.championnatId, championnatId),
      );
    }

    if (clubId) {
      whereCondition = and(
        whereCondition,
        eq(equipes.clubId, clubId),
      );
    }

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(equipes.nom, `%${search}%`),
          like(clubs.nom, `%${search}%`),
        ),
      );
    }

    const [totalResult] = await db
      .select({ count: count() })
      .from(classementsChampionnat)
      .leftJoin(equipes, eq(classementsChampionnat.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    const classementsList = await db
      .select({
        classement: classementsChampionnat,
        championnat: {
          id: championnats.id,
          nom: championnats.nom,
          slug: championnats.slug,
          saison: championnats.saison,
          statut: championnats.statut,
        },
        equipe: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          categorie: equipes.categorie,
          clubId: equipes.clubId,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(classementsChampionnat)
      .leftJoin(championnats, eq(classementsChampionnat.championnatId, championnats.id))
      .leftJoin(equipes, eq(classementsChampionnat.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(whereCondition)
      .orderBy(classementsChampionnat.position, classementsChampionnat.points)
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: classementsList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation des classements:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation des classements" },
      500,
    );
  }
});

// GET /classements/championnat/:championnatId - Get rankings for a specific championship
classementsRouter.get("/classements/championnat/:championnatId", async (c) => {
  try {
    const db = createDb();
    const championnatId = c.req.param("championnatId");

    // Verify championship exists
    const [championnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, championnatId))
      .execute();

    if (!championnat) {
      return c.json({ error: "Championnat non trouve" }, 404);
    }

    const classementsList = await db
      .select({
        classement: classementsChampionnat,
        equipe: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          categorie: equipes.categorie,
          clubId: equipes.clubId,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(classementsChampionnat)
      .leftJoin(equipes, eq(classementsChampionnat.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(eq(classementsChampionnat.championnatId, championnatId))
      .orderBy(classementsChampionnat.position)
      .execute();

    return c.json({
      championnat: {
        id: championnat.id,
        nom: championnat.nom,
        slug: championnat.slug,
        saison: championnat.saison,
        statut: championnat.statut,
      },
      classements: classementsList,
      total: classementsList.length,
    });
  } catch (error) {
    console.error("Erreur lors de la recuperation du classement:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation du classement" },
      500,
    );
  }
});

// GET /classements/:id - Get single ranking by ID
classementsRouter.get("/classements/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [classement] = await db
      .select({
        classement: classementsChampionnat,
        championnat: {
          id: championnats.id,
          nom: championnats.nom,
          slug: championnats.slug,
          saison: championnats.saison,
        },
        equipe: {
          id: equipes.id,
          nom: equipes.nom,
          slug: equipes.slug,
          clubId: equipes.clubId,
        },
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
        },
      })
      .from(classementsChampionnat)
      .leftJoin(championnats, eq(classementsChampionnat.championnatId, championnats.id))
      .leftJoin(equipes, eq(classementsChampionnat.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(eq(classementsChampionnat.id, id))
      .execute();

    if (!classement) {
      return c.json({ error: "Classement non trouve" }, 404);
    }

    return c.json(classement);
  } catch (error) {
    console.error("Erreur lors de la recuperation du classement:", error);
    return c.json(
      { error: "Erreur serveur lors de la recuperation du classement" },
      500,
    );
  }
});

// POST /classements - Create new ranking entry
classementsRouter.post("/classements", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation
    if (!body.championnatId || !body.equipeId || body.position === undefined) {
      return c.json(
        {
          error: "Les champs championnatId, equipeId et position sont requis",
        },
        400,
      );
    }

    // Verify championship exists
    const [championnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, body.championnatId))
      .execute();

    if (!championnat) {
      return c.json(
        { error: "Le championnat specifie n'existe pas" },
        400,
      );
    }

    // Verify team exists
    const [equipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, body.equipeId))
      .execute();

    if (!equipe) {
      return c.json(
        { error: "L'equipe specifiee n'existe pas" },
        400,
      );
    }

    // Verify position is unique for this championship
    const [existingClassement] = await db
      .select()
      .from(classementsChampionnat)
      .where(
        and(
          eq(classementsChampionnat.championnatId, body.championnatId),
          eq(classementsChampionnat.position, body.position),
        ),
      )
      .execute();

    if (existingClassement) {
      return c.json(
        { error: "Un classement avec cette position existe deja pour ce championnat" },
        409,
      );
    }

    // Verify team is not already in this championship
    const [existingTeamClassement] = await db
      .select()
      .from(classementsChampionnat)
      .where(
        and(
          eq(classementsChampionnat.championnatId, body.championnatId),
          eq(classementsChampionnat.equipeId, body.equipeId),
        ),
      )
      .execute();

    if (existingTeamClassement) {
      return c.json(
        { error: "Cette equipe est deja dans le classement de ce championnat" },
        409,
      );
    }

    const newClassement = {
      id: uuidv4(),
      championnatId: body.championnatId,
      equipeId: body.equipeId,
      position: body.position,
      points: body.points || 0,
      joues: body.joues || 0,
      victoires: body.victoires || 0,
      defaites: body.defaites || 0,
      nuls: body.nuls || 0,
      pour: body.pour || 0,
      contre: body.contre || 0,
      difference: body.difference || 0,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [created] = await db
      .insert(classementsChampionnat)
      .values(newClassement)
      .returning()
      .execute();

    return c.json(created, 201);
  } catch (error) {
    console.error("Erreur lors de la creation du classement:", error);
    return c.json(
      { error: "Erreur serveur lors de la creation du classement" },
      500,
    );
  }
});

// PUT /classements/:id - Update ranking entry
classementsRouter.put("/classements/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Verify classement exists
    const [existingClassement] = await db
      .select()
      .from(classementsChampionnat)
      .where(eq(classementsChampionnat.id, id))
      .execute();

    if (!existingClassement) {
      return c.json({ error: "Classement non trouve" }, 404);
    }

    // Verify championship exists if changing
    if (body.championnatId && body.championnatId !== existingClassement.championnatId) {
      const [championnat] = await db
        .select()
        .from(championnats)
        .where(eq(championnats.id, body.championnatId))
        .execute();

      if (!championnat) {
        return c.json(
          { error: "Le championnat specifie n'existe pas" },
          400,
        );
      }
    }

    // Verify team exists if changing
    if (body.equipeId && body.equipeId !== existingClassement.equipeId) {
      const [equipe] = await db
        .select()
        .from(equipes)
        .where(eq(equipes.id, body.equipeId))
        .execute();

      if (!equipe) {
        return c.json(
          { error: "L'equipe specifiee n'existe pas" },
          400,
        );
      }
    }

    // Verify new position is unique for this championship
    if (body.position && body.position !== existingClassement.position) {
      const championnatId = body.championnatId || existingClassement.championnatId;
      const [classementWithPosition] = await db
        .select()
        .from(classementsChampionnat)
        .where(
          and(
            eq(classementsChampionnat.championnatId, championnatId),
            eq(classementsChampionnat.position, body.position),
          ),
        )
        .execute();

      if (classementWithPosition) {
        return c.json(
          { error: "Un classement avec cette position existe deja pour ce championnat" },
          409,
        );
      }
    }

    const updatedData = {
      championnatId: body.championnatId || existingClassement.championnatId,
      equipeId: body.equipeId || existingClassement.equipeId,
      position: body.position !== undefined ? body.position : existingClassement.position,
      points: body.points !== undefined ? body.points : existingClassement.points,
      joues: body.joues !== undefined ? body.joues : existingClassement.joues,
      victoires: body.victoires !== undefined ? body.victoires : existingClassement.victoires,
      defaites: body.defaites !== undefined ? body.defaites : existingClassement.defaites,
      nuls: body.nuls !== undefined ? body.nuls : existingClassement.nuls,
      pour: body.pour !== undefined ? body.pour : existingClassement.pour,
      contre: body.contre !== undefined ? body.contre : existingClassement.contre,
      difference: body.difference !== undefined ? body.difference : existingClassement.difference,
      misAJourLe: new Date(),
    };

    const [updated] = await db
      .update(classementsChampionnat)
      .set(updatedData)
      .where(eq(classementsChampionnat.id, id))
      .returning()
      .execute();

    return c.json(updated);
  } catch (error) {
    console.error("Erreur lors de la mise a jour du classement:", error);
    return c.json(
      { error: "Erreur serveur lors de la mise a jour du classement" },
      500,
    );
  }
});

// DELETE /classements/:id - Delete ranking entry
classementsRouter.delete("/classements/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Verify classement exists
    const [existingClassement] = await db
      .select()
      .from(classementsChampionnat)
      .where(eq(classementsChampionnat.id, id))
      .execute();

    if (!existingClassement) {
      return c.json({ error: "Classement non trouve" }, 404);
    }

    await db
      .delete(classementsChampionnat)
      .where(eq(classementsChampionnat.id, id))
      .execute();

    return c.json({ message: "Classement supprime avec succes" });
  } catch (error) {
    console.error("Erreur lors de la suppression du classement:", error);
    return c.json(
      { error: "Erreur serveur lors de la suppression du classement" },
      500,
    );
  }
});

// POST /classements/championnat/:championnatId/recalculate - Recalculate all rankings for a championship
classementsRouter.post("/classements/championnat/:championnatId/recalculate", async (c) => {
  try {
    const db = createDb();
    const championnatId = c.req.param("championnatId");

    // Verify championship exists
    const [championnat] = await db
      .select()
      .from(championnats)
      .where(eq(championnats.id, championnatId))
      .execute();

    if (!championnat) {
      return c.json({ error: "Championnat non trouve" }, 404);
    }

    // Get all matches for this championship
    const matchsList = await db
      .select()
      .from(matchs)
      .where(
        and(
          eq(matchs.competitionId, championnatId),
          eq(matchs.competitionType, "championnat"),
        ),
      )
      .execute();

    // Calculate statistics for each team
    const teamStats: Record<string, any> = {};

    for (const match of matchsList) {
      if (!match.equipeDomicileId || !match.equipeExterieurId) continue;

      // Initialize team stats if not exists
      if (!teamStats[match.equipeDomicileId]) {
        teamStats[match.equipeDomicileId] = {
          equipeId: match.equipeDomicileId,
          joues: 0,
          victoires: 0,
          defaites: 0,
          nuls: 0,
          pour: 0,
          contre: 0,
          points: 0,
        };
      }
      if (!teamStats[match.equipeExterieurId]) {
        teamStats[match.equipeExterieurId] = {
          equipeId: match.equipeExterieurId,
          joues: 0,
          victoires: 0,
          defaites: 0,
          nuls: 0,
          pour: 0,
          contre: 0,
          points: 0,
        };
      }

      const scoreDomicile = match.scoreDomicile || 0;
      const scoreExterieur = match.scoreExterieur || 0;

      teamStats[match.equipeDomicileId].joues += 1;
      teamStats[match.equipeExterieurId].joues += 1;
      teamStats[match.equipeDomicileId].pour += scoreDomicile;
      teamStats[match.equipeDomicileId].contre += scoreExterieur;
      teamStats[match.equipeExterieurId].pour += scoreExterieur;
      teamStats[match.equipeExterieurId].contre += scoreDomicile;

      if (scoreDomicile > scoreExterieur) {
        teamStats[match.equipeDomicileId].victoires += 1;
        teamStats[match.equipeExterieurId].defaites += 1;
        teamStats[match.equipeDomicileId].points += 3;
      } else if (scoreDomicile < scoreExterieur) {
        teamStats[match.equipeExterieurId].victoires += 1;
        teamStats[match.equipeDomicileId].defaites += 1;
        teamStats[match.equipeExterieurId].points += 3;
      } else {
        teamStats[match.equipeDomicileId].nuls += 1;
        teamStats[match.equipeExterieurId].nuls += 1;
        teamStats[match.equipeDomicileId].points += 1;
        teamStats[match.equipeExterieurId].points += 1;
      }
    }

    // Calculate difference and sort by points
    const sortedTeams = Object.values(teamStats)
      .map((team: any) => ({
        ...team,
        difference: team.pour - team.contre,
      }))
      .sort((a: any, b: any) => {
        // Sort by points (descending), then by difference (descending)
        if (b.points !== a.points) return b.points - a.points;
        if (b.difference !== a.difference) return b.difference - a.difference;
        return 0;
      });

    // Delete existing rankings for this championship
    await db
      .delete(classementsChampionnat)
      .where(eq(classementsChampionnat.championnatId, championnatId))
      .execute();

    // Create new rankings
    const newClassements = sortedTeams.map((team: any, index: number) => ({
      id: uuidv4(),
      championnatId,
      equipeId: team.equipeId,
      position: index + 1,
      points: team.points,
      joues: team.joues,
      victoires: team.victoires,
      defaites: team.defaites,
      nuls: team.nuls,
      pour: team.pour,
      contre: team.contre,
      difference: team.difference,
      creeLe: new Date(),
      misAJourLe: new Date(),
    }));

    if (newClassements.length > 0) {
      await db
        .insert(classementsChampionnat)
        .values(newClassements)
        .execute();
    }

    return c.json({
      message: "Classements recalcules avec succes",
      count: newClassements.length,
    });
  } catch (error) {
    console.error("Erreur lors du recalcul des classements:", error);
    return c.json(
      { error: "Erreur serveur lors du recalcul des classements" },
      500,
    );
  }
});

export default classementsRouter;
