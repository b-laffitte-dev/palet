import { Hono } from "hono";
import { createDb } from "../db/client.js";
import { joueurs, clubs, federations } from "../../db/schema.js";
import { eq, and, desc, like, or, ne, count } from "drizzle-orm";
import { v4 as uuidv4 } from "uuid";

const joueursRouter = new Hono();

// GET /joueurs - Lister tous les joueurs
joueursRouter.get("/joueurs", async (c) => {
  try {
    const db = createDb();
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");
    const search = query.search;
    const clubId = query.clubId;
    const categorie = query.categorie;

    // Construction de la condition WHERE
    let whereCondition = and();

    if (search) {
      whereCondition = and(
        whereCondition,
        or(
          like(joueurs.nom, `%${search}%`),
          like(joueurs.prenom, `%${search}%`),
          like(joueurs.licence, `%${search}%`),
        ),
      );
    }

    if (clubId) {
      whereCondition = and(whereCondition, eq(joueurs.clubId, clubId));
    }

    if (categorie) {
      whereCondition = and(whereCondition, eq(joueurs.categorie, categorie as any));
    }

    // Compter le total
    const [totalResult] = await db
      .select({ count: count() })
      .from(joueurs)
      .where(whereCondition)
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    // Récupérer les joueurs avec leurs clubs
    const joueursList = await db
      .select({
        joueur: joueurs,
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
        },
      })
      .from(joueurs)
      .leftJoin(clubs, eq(joueurs.clubId, clubs.id))
      .where(whereCondition)
      .orderBy(desc(joueurs.creeLe))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: joueursList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération des joueurs:", error);
    return c.json({ error: "Erreur serveur lors de la récupération des joueurs" }, 500);
  }
});

// GET /joueurs/:id - Obtenir un joueur par ID
joueursRouter.get("/joueurs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    const [joueur] = await db
      .select({
        joueur: joueurs,
        club: {
          id: clubs.id,
          nom: clubs.nom,
          slug: clubs.slug,
          ville: clubs.ville,
          logoUrl: clubs.logoUrl,
          federationId: clubs.federationId,
        },
      })
      .from(joueurs)
      .leftJoin(clubs, eq(joueurs.clubId, clubs.id))
      .where(eq(joueurs.id, id))
      .execute();

    if (!joueur) {
      return c.json({ error: "Joueur non trouvé" }, 404);
    }

    return c.json(joueur);
  } catch (error) {
    console.error("Erreur lors de la récupération du joueur:", error);
    return c.json({ error: "Erreur serveur lors de la récupération du joueur" }, 500);
  }
});

// POST /joueurs - Créer un nouveau joueur
joueursRouter.post("/joueurs", async (c) => {
  try {
    const db = createDb();
    const body = await c.req.json();

    // Validation basique
    if (!body.nom || !body.prenom || !body.clubId) {
      return c.json({ error: "Les champs nom, prenom et clubId sont requis" }, 400);
    }

    // Vérifier que le club existe
    const [club] = await db.select().from(clubs).where(eq(clubs.id, body.clubId)).execute();

    if (!club) {
      return c.json({ error: "Le club spécifié n'existe pas" }, 400);
    }

    // Vérifier que la licence est unique (si fournie)
    if (body.licence) {
      const [existingJoueur] = await db
        .select()
        .from(joueurs)
        .where(eq(joueurs.licence, body.licence))
        .execute();

      if (existingJoueur) {
        return c.json({ error: "Un joueur avec cette licence existe déjà" }, 409);
      }
    }

    // Créer le joueur
    const newJoueur = {
      id: uuidv4(),
      clubId: body.clubId,
      nom: body.nom,
      prenom: body.prenom,
      licence: body.licence || null,
      categorie: body.categorie || "senior",
      dateNaissance: body.dateNaissance || null,
      email: body.email || null,
      telephone: body.telephone || null,
      photoUrl: body.photoUrl || null,
      poste: body.poste || null,
      mainDroite: body.mainDroite !== undefined ? body.mainDroite : 1,
      nombreVictoires: body.nombreVictoires || 0,
      nombreDefaites: body.nombreDefaites || 0,
      nombreNuls: body.nombreNuls || 0,
      points: body.points || 0,
      classement: body.classement || null,
      creeLe: new Date(),
      misAJourLe: new Date(),
    };

    const [createdJoueur] = await db.insert(joueurs).values(newJoueur).returning().execute();

    return c.json(createdJoueur, 201);
  } catch (error) {
    console.error("Erreur lors de la création du joueur:", error);
    return c.json({ error: "Erreur serveur lors de la création du joueur" }, 500);
  }
});

// PUT /joueurs/:id - Mettre à jour un joueur
joueursRouter.put("/joueurs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");
    const body = await c.req.json();

    // Vérifier que le joueur existe
    const [existingJoueur] = await db.select().from(joueurs).where(eq(joueurs.id, id)).execute();

    if (!existingJoueur) {
      return c.json({ error: "Joueur non trouvé" }, 404);
    }

    // Si la licence est modifiée, vérifier qu'elle est unique
    if (body.licence && body.licence !== existingJoueur.licence) {
      const [joueurWithSameLicence] = await db
        .select()
        .from(joueurs)
        .where(and(eq(joueurs.licence, body.licence), ne(joueurs.id, id)))
        .execute();

      if (joueurWithSameLicence) {
        return c.json({ error: "Un joueur avec cette licence existe déjà" }, 409);
      }
    }

    // Si le club est modifié, vérifier qu'il existe
    if (body.clubId && body.clubId !== existingJoueur.clubId) {
      const [club] = await db.select().from(clubs).where(eq(clubs.id, body.clubId)).execute();

      if (!club) {
        return c.json({ error: "Le club spécifié n'existe pas" }, 400);
      }
    }

    // Mettre à jour le joueur
    const updatedJoueur = {
      nom: body.nom || existingJoueur.nom,
      prenom: body.prenom || existingJoueur.prenom,
      licence: body.licence !== undefined ? body.licence : existingJoueur.licence,
      clubId: body.clubId || existingJoueur.clubId,
      categorie: body.categorie || existingJoueur.categorie,
      dateNaissance:
        body.dateNaissance !== undefined ? body.dateNaissance : existingJoueur.dateNaissance,
      email: body.email !== undefined ? body.email : existingJoueur.email,
      telephone: body.telephone !== undefined ? body.telephone : existingJoueur.telephone,
      photoUrl: body.photoUrl !== undefined ? body.photoUrl : existingJoueur.photoUrl,
      poste: body.poste !== undefined ? body.poste : existingJoueur.poste,
      mainDroite: body.mainDroite !== undefined ? body.mainDroite : existingJoueur.mainDroite,
      nombreVictoires:
        body.nombreVictoires !== undefined ? body.nombreVictoires : existingJoueur.nombreVictoires,
      nombreDefaites:
        body.nombreDefaites !== undefined ? body.nombreDefaites : existingJoueur.nombreDefaites,
      nombreNuls: body.nombreNuls !== undefined ? body.nombreNuls : existingJoueur.nombreNuls,
      points: body.points !== undefined ? body.points : existingJoueur.points,
      classement: body.classement !== undefined ? body.classement : existingJoueur.classement,
      misAJourLe: new Date(),
    };

    const [result] = await db
      .update(joueurs)
      .set(updatedJoueur)
      .where(eq(joueurs.id, id))
      .returning()
      .execute();

    return c.json(result);
  } catch (error) {
    console.error("Erreur lors de la mise à jour du joueur:", error);
    return c.json({ error: "Erreur serveur lors de la mise à jour du joueur" }, 500);
  }
});

// DELETE /joueurs/:id - Supprimer un joueur
joueursRouter.delete("/joueurs/:id", async (c) => {
  try {
    const db = createDb();
    const id = c.req.param("id");

    // Vérifier que le joueur existe
    const [existingJoueur] = await db.select().from(joueurs).where(eq(joueurs.id, id)).execute();

    if (!existingJoueur) {
      return c.json({ error: "Joueur non trouvé" }, 404);
    }

    // Supprimer le joueur
    await db.delete(joueurs).where(eq(joueurs.id, id)).execute();

    return c.json({ message: "Joueur supprimé avec succès" });
  } catch (error) {
    console.error("Erreur lors de la suppression du joueur:", error);
    return c.json({ error: "Erreur serveur lors de la suppression du joueur" }, 500);
  }
});

// GET /clubs/:clubId/joueurs - Obtenir les joueurs d'un club
joueursRouter.get("/clubs/:clubId/joueurs", async (c) => {
  try {
    const db = createDb();
    const clubId = c.req.param("clubId");
    const query = c.req.query();
    const page = parseInt(query.page || "1");
    const pageSize = parseInt(query.pageSize || "10");

    // Vérifier que le club existe
    const [club] = await db.select().from(clubs).where(eq(clubs.id, clubId)).execute();

    if (!club) {
      return c.json({ error: "Club non trouvé" }, 404);
    }

    // Compter le total
    const [totalResult] = await db
      .select({ count: count() })
      .from(joueurs)
      .where(eq(joueurs.clubId, clubId))
      .execute();

    const total = totalResult?.count || 0;
    const totalPages = Math.ceil(total / pageSize);

    // Récupérer les joueurs
    const joueursList = await db
      .select()
      .from(joueurs)
      .where(eq(joueurs.clubId, clubId))
      .orderBy(desc(joueurs.creeLe))
      .limit(pageSize)
      .offset((page - 1) * pageSize)
      .execute();

    return c.json({
      data: joueursList,
      total,
      page,
      pageSize,
      totalPages,
    });
  } catch (error) {
    console.error("Erreur lors de la récupération des joueurs du club:", error);
    return c.json({ error: "Erreur serveur lors de la récupération des joueurs" }, 500);
  }
});

export default joueursRouter;
