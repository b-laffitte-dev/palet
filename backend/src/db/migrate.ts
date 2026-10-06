import { migrate } from "drizzle-orm/postgres-js/migrator";
import { createDb } from "./client.js";

const db = createDb();

await migrate(db, { migrationsFolder: "./db/migrations" });
console.log("Migrations appliquées");
process.exit(0);
