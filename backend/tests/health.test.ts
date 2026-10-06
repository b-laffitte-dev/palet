import { describe, expect, it } from "vitest";
import app from "../src/index.js";

describe("GET /health", () => {
  it("répond ok", async () => {
    const res = await app.request("/health");
    expect(res.status).toBe(200);
    const body = (await res.json()) as { statut: string };
    expect(body.statut).toBe("ok");
  });

  it("expose l'OpenAPI", async () => {
    const res = await app.request("/api-docs/openapi.json");
    expect(res.status).toBe(200);
    const spec = (await res.json()) as {
      paths: Record<string, unknown>;
    };
    expect(spec.paths).toHaveProperty("/health");
  });
});
