import { createRoute, OpenAPIHono, z } from "@hono/zod-openapi";

const healthRoute = createRoute({
  method: "get",
  path: "/health",
  responses: {
    200: {
      content: {
        "application/json": {
          schema: z.object({ statut: z.string() }),
        },
      },
      description: "État de l'API",
    },
  },
});

export const health = new OpenAPIHono();
health.openapi(healthRoute, (c) => c.json({ statut: "ok" }));
