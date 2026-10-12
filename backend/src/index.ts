import { serve } from "@hono/node-server";
import { OpenAPIHono } from "@hono/zod-openapi";
import { cors } from "hono/cors";
import { logger } from "hono/logger";
import { health } from "./routes/health.js";
import clubsRouter from "./routes/clubs.js";
import joueursRouter from "./routes/joueurs.js";
import tournoisRouter from "./routes/tournois.js";
import championnatsRouter from "./routes/championnats.js";
import matchsRouter from "./routes/matchs.js";
import federationsRouter from "./routes/federations.js";
import equipesRouter from "./routes/equipes.js";
import actualitesRouter from "./routes/actualites.js";
import classementsRouter from "./routes/classements.js";

const app = new OpenAPIHono();

app.use(logger());
app.use(cors());

app.route("/", health);
app.route("/", clubsRouter);
app.route("/", joueursRouter);
app.route("/", tournoisRouter);
app.route("/", championnatsRouter);
app.route("/", matchsRouter);
app.route("/", federationsRouter);
app.route("/", equipesRouter);
app.route("/", actualitesRouter);
app.route("/", classementsRouter);

app.doc("/api-docs/openapi.json", {
  openapi: "3.1.0",
  info: { title: "API CDSP 79 - Palet", version: "0.1.0" },
});

serve({ fetch: app.fetch, port: 3000 }, (info) => {
  console.log(`API en ecoute sur http://localhost:${info.port}`);
});

export default app;
