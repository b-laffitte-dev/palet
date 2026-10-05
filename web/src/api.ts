import createClient from "openapi-fetch";

type Paths = {
  "/health": {
    get: {
      responses: {
        200: {
          content: {
            "application/json": {
              statut: string;
            };
          };
        };
      };
    };
  };
};

export type Client = ReturnType<typeof createClient<Paths>>;

export function api(): Client {
  return createClient<Paths>({ baseUrl: "/api" });
}
