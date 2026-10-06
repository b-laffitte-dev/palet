import { serve } from "@hono/node-server";
import { OpenAPIHono } from "@hono/zod-openapi";
import { cors } from "hono/cors";
import { logger } from "hono/logger";
import { health } from "./routes/health.js";
import { clients } from "./routes/clients.js";

const app = new OpenAPIHono();

app.use(logger());
app.use(cors());

app.route("/", health);
app.route("/clubs", clients);

app.doc("/api-docs/openapi.json", {
  openapi: "3.1.0",
  info: { title: "API Palet Vendéen", version: "0.1.0" },
});

serve({ fetch: app.fetch, port: 3000 }, (info) => {
  console.log(`API en écoute sur http://localhost:${info.port}`);
});

export default app;
