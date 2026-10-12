import { Hono } from "hono";
import { sql } from "drizzle-orm";
import { createDb } from "../db/client.js";
import { tournois, federations, clubs, equipes, equipesTournois } from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const tournoisRouter = new Hono();

// GET /tournois - Lister tous les tournois
tournoisRouter.get("/tournois", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;
    const federationId = query.federationId;
    const statut = query.statut as string;
    const ville = query.ville;

    // Construction de la condition WHERE
    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(tournois.nom, `%${search}%`),
          like(tournois.slug, `%${search}%`),
          like(tournois.lieu, `%${search}%`),
          like(tournois.ville, `%${search}%`),
        ),
      );
    }

    if (federationId) {
      whereCondition = and(
        whereCondition,
        eq(tournois.federationId, federationId),
      );
    }

    if (statut) {
      whereCondition = and(
        whereCondition,
        eq(tournois.statut, statut as any),
      );
    }

    if (ville) {
      whereCondition = and(
        whereCondition,
        like(tournois.ville, `%${ville}%`),
      );
    }

    // Compter le total
    const [totalResult] = await db
      .select({ count: count() })
      .from(tournois)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    // Récupérer les tournois avec leurs fédérations
    const tournoisList = await db
      .select({
        tournoi: tournois,
        federation: {
          id: federations.id,
          nom: federations.nom,
          slug: federations.slug,
        },
      })
      .from(tournois)
      .leftJoin(federations, eq(tournois.federationId, federations.id))
      .where(whereCondition)
      .orderBy(desc(tournois.dateDebut))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: tournoisList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération des tournois:", error);
    return c.json(
      { error: "Erreur serveur lors de la récupération des tournois" },
      500,
    );
  }
});

// GET /tournois/:id - Obtenir un tournoi par ID
tournoisRouter.get("/tournois/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [tournoi] = await db
      .select({
        tournoi: tournois,
        federation: {
          id: federations.id,
          nom: federations.nom,
          slug: federations.slug,
          logoUrl: federations.logoUrl,
        },
      })
      .from(tournois)
      .leftJoin(federations, eq(tournois.federationId, federations.id))
      .where(eq(tournois.id, id))
      .execute();

    if (!tournoi) {
      return c.json({ error: "Tournoi non trouvé" }, 404);
    }

    // Récupérer les equipes inscrites
    const equipesInscrites = await db
      .select({
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
        dateInscription: equipesTournois.dateInscription,
        statut: equipesTournois.statut,
        paiementEffectue: equipesTournois.paiementEffectue,
      })
      .from(equipesTournois)
      .leftJoin(equipes, eq(equipesTournois.equipeId, equipes.id))
      .leftJoin(clubs, eq(equipes.clubId, clubs.id))
      .where(eq(equipesTournois.tournoiId, id))
      .execute();

    return c.json({
      ...tournoi,
      equipesInscrites,
      nombreEquipesInscrites: equipesInscrites.length,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération du tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de la récupération du tournoi" },
      500,
    );
  }
});

// POST /tournois - Créer un nouveau tournoi
tournoisRouter.post("/tournois", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation basique
    if (!body.nom || !body.slug || !body.federationId || !body.dateDebut || !body.dateFin || !body.lieu || !body.ville) {
      return c.json(
        { error: "Les champs nom, slug, federationId, dateDebut, dateFin, lieu et ville sont requis" },
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
      return c.json(
        { error: "La fédération spécifiée n'existe pas" },
        400,
      );
    }

    // Vérifier que le slug est unique
    const [existingTournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.slug, body.slug))
      .execute();

    if (existingTournoi) {
      return c.json(
        { error: "Un tournoi avec ce slug existe déjà" },
        409,
      );
    }

    // Créer le tournoi
    const newTournoi = {
      id: uuidv4(),
      federationId: body.federationId,
      nom: body.nom,
      slug: body.slug,
      description: body.description || null,
      statut: body.statut || "planifie",
      dateDebut: new Date(body.dateDebut),
      dateFin: new Date(body.dateFin),
      lieu: body.lieu,
      adresse: body.adresse || null,
      ville: body.ville,
      codePostal: body.codePostal || null,
      nombreEquipesMax: body.nombreEquipesMax || 16,
      nombreEquipesInscrites: 0,
      fraisInscription: body.fraisInscription || 0,
      type: body.type || "tournoi",
      reglement: body.reglement || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdTournoi] = await db
      .insert(tournois)
      .values(newTournoi)
      .returning()
      .execute();

    return c.json(createdTournoi, 201);
  } catch (error) {
    console.error("Erreur lors de la création du tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de la création du tournoi" },
      500,
    );
  }
});

// PUT /tournois/:id - Mettre à jour un tournoi
tournoisRouter.put("/tournois/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Vérifier que le tournoi existe
    const [existingTournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.id, id))
      .execute();

    if (!existingTournoi) {
      return c.json({ error: "Tournoi non trouvé" }, 404);
    }

    // Si le slug est modifié, vérifier qu'il est unique
    if (body.slug && body.slug !== existingTournoi.slug) {
      const [tournoiWithSameSlug] = await db
        .select()
        .from(tournois)
        .where(and(eq(tournois.slug, body.slug), ne(tournois.id, id)))
        .execute();

      if (tournoiWithSameSlug) {
        return c.json(
          { error: "Un tournoi avec ce slug existe déjà" },
          409,
        );
      }
    }

    // Si la fédération est modifiée, vérifier qu'elle existe
    if (body.federationId && body.federationId !== existingTournoi.federationId) {
      const [federation] = await db
        .select()
        .from(federations)
        .where(eq(federations.id, body.federationId))
        .execute();

      if (!federation) {
        return c.json(
          { error: "La fédération spécifiée n'existe pas" },
          400,
        );
      }
    }

    // Mettre à jour le tournoi
    const updatedTournoi = {
      nom: body.nom || existingTournoi.nom,
      slug: body.slug || existingTournoi.slug,
      description: body.description !== undefined ? body.description : existingTournoi.description,
      statut: body.statut || existingTournoi.statut,
      dateDebut: body.dateDebut ? new Date(body.dateDebut) : existingTournoi.dateDebut,
      dateFin: body.dateFin ? new Date(body.dateFin) : existingTournoi.dateFin,
      lieu: body.lieu || existingTournoi.lieu,
      adresse: body.adresse !== undefined ? body.adresse : existingTournoi.adresse,
      ville: body.ville || existingTournoi.ville,
      codePostal: body.codePostal !== undefined ? body.codePostal : existingTournoi.codePostal,
      nombreEquipesMax: body.nombreEquipesMax !== undefined ? body.nombreEquipesMax : existingTournoi.nombreEquipesMax,
      fraisInscription: body.fraisInscription !== undefined ? body.fraisInscription : existingTournoi.fraisInscription,
      type: body.type || existingTournoi.type,
      reglement: body.reglement !== undefined ? body.reglement : existingTournoi.reglement,
      federationId: body.federationId || existingTournoi.federationId,
      misAJourLe: new Date(),
    };

    const [result] = await db
      .update(tournois)
      .set(updatedTournoi)
      .where(eq(tournois.id, id))
      .returning()
      .execute();

    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise à jour du tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de la mise à jour du tournoi" },
      500,
    );
  }
});

// DELETE /tournois/:id - Supprimer un tournoi
tournoisRouter.delete("/tournois/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Vérifier que le tournoi existe
    const [existingTournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.id, id))
      .execute();

    if (!existingTournoi) {
      return c.json({ error: "Tournoi non trouvé" }, 404);
    }

    // Supprimer le tournoi
    await db
      .delete(tournois)
      .where(eq(tournois.id, id))
      .execute();

    return c.json({ message: "Tournoi supprimé avec succès" });
  } catch (error) {
    console.error("Erreur lors de la suppression du tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de la suppression du tournoi" },
      500,
    );
  }
});

// POST /tournois/:id/inscrire - Inscrire une équipe à un tournoi
tournoisRouter.post("/tournois/:id/inscrire", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Validation
    if (!body.equipeId) {
      return c.json(
        { error: "Le champ equipeId est requis" },
        400,
      );
    }

    // Vérifier que le tournoi existe
    const [tournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.id, id))
      .execute();

    if (!tournoi) {
      return c.json({ error: "Tournoi non trouvé" }, 404);
    }

    // Vérifier que l'équipe existe
    const [equipe] = await db
      .select()
      .from(equipes)
      .where(eq(equipes.id, body.equipeId))
      .execute();

    if (!equipe) {
      return c.json(
        { error: "L'équipe spécifiée n'existe pas" },
        400,
      );
    }

    // Vérifier que le tournoi n'est pas complet
    const [inscriptionsCount] = await db
      .select({ count: count() })
      .from(equipesTournois)
      .where(eq(equipesTournois.tournoiId, id))
      .execute();

    const currentCount = inscriptionsCount?.count || 0;
    const maxCount = tournoi.nombreEquipesMax || 0;

    if (currentCount >= maxCount) {
      return c.json(
        { error: "Le tournoi a atteint le nombre maximum d'equipes" },
        400,
      );
    }

    // Vérifier que l'équipe n'est pas déjà inscrite
    const [existingInscription] = await db
      .select()
      .from(equipesTournois)
      .where(and(eq(equipesTournois.tournoiId, id), eq(equipesTournois.equipeId, body.equipeId)))
      .execute();

    if (existingInscription) {
      return c.json(
        { error: "Cette équipe est déjà inscrite à ce tournoi" },
        409,
      );
    }

    // Inscrire l'équipe
    const inscription = {
      id: uuidv4(),
      tournoiId: id,
      equipeId: body.equipeId,
      dateInscription: new Date(),
      paiementEffectue: body.paiementEffectue || 0,
      statut: body.statut || "inscrite",
      creeLe: new Date(),
    };

    const [createdInscription] = await db
      .insert(equipesTournois)
      .values(inscription)
      .returning()
      .execute();

    // Mettre à jour le nombre d'equipes inscrites
    const currentInscrites = tournoi.nombreEquipesInscrites || 0;
    await db
      .update(tournois)
      .set({
        nombreEquipesInscrites: currentInscrites + 1,
        misAJourLe: new Date(),
      })
      .where(eq(tournois.id, id))
      .execute();

    return c.json(createdInscription, 201);
  } catch (error) {
    console.error("Erreur lors de l'inscription au tournoi:", error);
    return c.json(
      { error: "Erreur serveur lors de l'inscription au tournoi" },
      500,
    );
  }
});

// DELETE /tournois/:id/equipes/:equipeId - Désinscrire une équipe
tournoisRouter.delete("/tournois/:id/equipes/:equipeId", async (c) => {
  try {
    const db = createDb();
    const tournoiId = c.req.param("id");
    const equipeId = c.req.param("equipeId");

    // Vérifier que l'inscription existe
    const [inscription] = await db
      .select()
      .from(equipesTournois)
      .where(and(eq(equipesTournois.tournoiId, tournoiId), eq(equipesTournois.equipeId, equipeId)))
      .execute();

    if (!inscription) {
      return c.json({ error: "Inscription non trouvée" }, 404);
    }

    // Supprimer l'inscription
    await db
      .delete(equipesTournois)
      .where(and(eq(equipesTournois.tournoiId, tournoiId), eq(equipesTournois.equipeId, equipeId)))
      .execute();

    // Mettre à jour le nombre d'equipes inscrites
    const [tournoi] = await db
      .select()
      .from(tournois)
      .where(eq(tournois.id, tournoiId))
      .execute();

    if (tournoi) {
      const currentInscrites = tournoi.nombreEquipesInscrites || 0;
      await db
        .update(tournois)
        .set({
          nombreEquipesInscrites: Math.max(0, currentInscrites - 1),
          misAJourLe: new Date(),
        })
        .where(eq(tournois.id, tournoiId))
        .execute();
    }

    return c.json({ message: "Équipe désinscrite avec succès" });
  } catch (error) {
    console.error("Erreur lors de la désinscription:", error);
    return c.json(
      { error: "Erreur serveur lors de la désinscription" },
      500,
    );
  }
});

export default tournoisRouter;
