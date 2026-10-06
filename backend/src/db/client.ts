import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import * as schema from "../../db/schema.js";

export function createDb(url?: string) {
  const client = postgres(
    url ?? process.env.DATABASE_URL ?? "postgres://palet:palet@localhost:5432/palet",
  );
  return drizzle(client, { schema });
}

export type Db = ReturnType<typeof createDb>;
