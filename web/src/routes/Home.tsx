import { useQuery } from "@tanstack/react-query";
import { api } from "../api";

export default function Home() {
  const { data, isLoading, isError } = useQuery({
    queryKey: ["health"],
    queryFn: async () => (await api().GET("/health")).data,
  });

  return (
    <main>
      <h1>Palet Vendéen</h1>
      <p>
        {isLoading
          ? "Chargement…"
          : isError
            ? "Erreur de connexion à l'API"
            : `API : ${data?.statut ?? "inconnu"}`}
      </p>
    </main>
  );
}
