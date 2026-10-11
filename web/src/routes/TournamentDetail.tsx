import React from "react";
import { useParams, Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Layout } from "../components/layout/Layout";
import {
  Card,
  CardHeader,
  CardContent,
  CardFooter,
  Badge,
  Avatar,
  Button,
  Tabs,
  TabList,
  TabPanel,
  Tab,
  TeamAvatar,
} from "../components";
import { TournamentCard, MatchCard, ClubCard } from "../components/domain";
import { Tournament, Team, Match, Player } from "../types";
import { api } from "../api";
import { mockTournament, mockTeams, mockMatches } from "../data/mocks";

const TournamentDetail: React.FC = () => {
  const { tournamentId } = useParams<{ tournamentId: string }>();

  const { data: tournament, isLoading } = useQuery<Tournament>({
    queryKey: ["tournament", tournamentId],
    queryFn: async () => {
      const response = await api().GET<Tournament>(`/api/tournaments/${tournamentId}`);
      return response.data;
    },
    enabled: !!tournamentId,
  });

  // For development, use mock data if no real data
  const displayTournament = tournament || mockTournament;

  if (isLoading && !tournament) {
    return (
      <Layout>
        <main className="flex-1 px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <p className="text-body-m text-brand-green-200">Chargement...</p>
          </div>
        </main>
      </Layout>
    );
  }

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center gap-4 mb-4">
              <Link to="/tournois">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux tournois
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Logo/Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-4">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayTournament.name}
                  </h1>
                  <Badge variant="gold">{displayTournament.code}</Badge>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="primary">{displayTournament.category}</Badge>
                  <Badge variant="secondary">{displayTournament.material}</Badge>
                  <Badge variant="default">{displayTournament.season}</Badge>
                  <Badge
                    variant={
                      displayTournament.status === "in_progress"
                        ? "success"
                        : displayTournament.status === "completed"
                          ? "default"
                          : "warning"
                    }
                  >
                    {displayTournament.status.replace("_", " ")}
                  </Badge>
                </div>

                <p className="text-body-m text-brand-green-200 mb-4 max-w-2xl">
                  {displayTournament.description}
                </p>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Lieu:</span>
                    <span className="text-brand-orange-pure">{displayTournament.location}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Format:</span>
                    <span className="text-brand-orange-pure">
                      {displayTournament.format.replace("_", " ")}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Type:</span>
                    <span className="text-brand-orange-pure">{displayTournament.type}</span>
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Équipes inscrites:</span>
                    <span className="text-brand-orange-pure">
                      {displayTournament.registeredTeams}/{displayTournament.maxTeams || "Illimité"}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Frais:</span>
                    <span className="text-brand-orange-pure">
                      {displayTournament.fee ? `${displayTournament.fee}€` : "Gratuit"}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Organisateur:</span>
                    <span className="text-brand-orange-pure">
                      {(displayTournament.organizer as any)?.name || "CDSP 79"}
                    </span>
                  </div>
                </div>
              </div>

              {/* Dates */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-4 border border-brand-green-700/50 text-center">
                  <p className="text-body-xs text-brand-green-400 mb-1">Dates</p>
                  <p className="text-heading-s font-bold text-brand-orange-pure">
                    {displayTournament.startDate &&
                    displayTournament.endDate &&
                    displayTournament.startDate !== displayTournament.endDate
                      ? `${new Date(displayTournament.startDate).toLocaleDateString("fr-FR")} - ${new Date(displayTournament.endDate).toLocaleDateString("fr-FR")}`
                      : new Date(
                          displayTournament.startDate || displayTournament.endDate || Date.now(),
                        ).toLocaleDateString("fr-FR")}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Stats Overview */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-brand-green-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-brand-orange-pure uppercase tracking-wide mb-6">
              Informations
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Type</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTournament.type}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Catégorie</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTournament.category}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Matériel</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTournament.material}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Saison</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTournament.season}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Équipes</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTournament.registeredTeams}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Statut</p>
                  <Badge
                    variant={
                      displayTournament.status === "in_progress"
                        ? "success"
                        : displayTournament.status === "completed"
                          ? "default"
                          : "warning"
                    }
                  >
                    {displayTournament.status.replace("_", " ")}
                  </Badge>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="infos">
              <TabList>
                <Tab value="infos" label="Informations" />
                <Tab value="equipes" label={`Équipes (${displayTournament.registeredTeams})`} />
                <Tab value="matchs" label="Matchs" />
                <Tab value="classement" label="Classement" />
              </TabList>

              {/* Informations */}
              <TabPanel value="infos" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Description</h3>
                    </CardHeader>
                    <CardContent>
                      <p className="text-body-m text-brand-green-200">
                        {displayTournament.description}
                      </p>
                    </CardContent>
                  </Card>

                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Règles et Éligibilité</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">
                            Catégories autorisées:
                          </p>
                          <div className="flex gap-2">
                            {displayTournament.eligibility.categories.map((cat) => (
                              <Badge key={cat} variant="secondary">
                                {cat}
                              </Badge>
                            ))}
                          </div>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">
                            Divisions autorisées:
                          </p>
                          <div className="flex gap-2">
                            {displayTournament.divisions.map((div) => (
                              <Badge key={div} variant="secondary">
                                {div}
                              </Badge>
                            ))}
                          </div>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Licence requise:</p>
                          <Badge
                            variant={
                              displayTournament.eligibility.requireLicense ? "success" : "danger"
                            }
                          >
                            {displayTournament.eligibility.requireLicense ? "Oui" : "Non"}
                          </Badge>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">
                            Max joueurs par club:
                          </p>
                          <p className="text-body-m text-brand-orange-pure">
                            {displayTournament.eligibility.maxPlayersPerClub || "Illimité"}
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>

                <Card className="bg-brand-green-900/50 border-brand-green-700/50 mt-6">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Calendrier</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div>
                        <p className="text-body-xs text-brand-green-300 mb-1">
                          Début des inscriptions:
                        </p>
                        <p className="text-body-m text-brand-orange-pure">
                          {displayTournament.registrationStart
                            ? new Date(displayTournament.registrationStart).toLocaleDateString(
                                "fr-FR",
                              )
                            : "Non spécifié"}
                        </p>
                      </div>
                      <div>
                        <p className="text-body-xs text-brand-green-300 mb-1">
                          Fin des inscriptions:
                        </p>
                        <p className="text-body-m text-brand-orange-pure">
                          {displayTournament.registrationEnd
                            ? new Date(displayTournament.registrationEnd).toLocaleDateString(
                                "fr-FR",
                              )
                            : "Non spécifié"}
                        </p>
                      </div>
                      <div>
                        <p className="text-body-xs text-brand-green-300 mb-1">Début du tournoi:</p>
                        <p className="text-body-m text-brand-orange-pure">
                          {displayTournament.startDate
                            ? new Date(displayTournament.startDate).toLocaleDateString("fr-FR")
                            : "Non spécifié"}
                        </p>
                      </div>
                      <div>
                        <p className="text-body-xs text-brand-green-300 mb-1">Fin du tournoi:</p>
                        <p className="text-body-m text-brand-orange-pure">
                          {displayTournament.endDate
                            ? new Date(displayTournament.endDate).toLocaleDateString("fr-FR")
                            : "Non spécifié"}
                        </p>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {mockTeams.slice(0, 6).map((team) => (
                    <Card key={team.id} className="bg-brand-green-900/50 border-brand-green-700/50">
                      <CardHeader>
                        <div className="flex items-center gap-3">
                          <TeamAvatar
                            team={{
                              id: team.id,
                              name: team.name,
                              logo: team.logo ?? undefined,
                              abbreviation: team.club.code,
                            }}
                            size="m"
                          />
                          <div>
                            <h3 className="text-heading-s font-bold text-white">{team.name}</h3>
                            <p className="text-body-xs text-brand-green-300">{team.club.name}</p>
                          </div>
                        </div>
                      </CardHeader>
                      <CardContent>
                        <div className="grid grid-cols-2 gap-4">
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300">Matchs</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {team.stats.matches}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300">Victoires</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {team.stats.wins}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300">Défaites</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {team.stats.losses}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300">Win Rate</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {(team.stats.winRate * 100).toFixed(0)}%
                            </p>
                          </div>
                        </div>
                        <Link to={`/teams/${team.id}`} className="inline-block mt-4">
                          <Button variant="gold" size="sm" className="w-full">
                            Voir l&apos;équipe
                          </Button>
                        </Link>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </TabPanel>

              {/* Matchs */}
              <TabPanel value="matchs" className="py-6">
                <div className="space-y-4">
                  {mockMatches.map((match) => (
                    <MatchCard key={match.id} match={match} showDetails />
                  ))}
                </div>
              </TabPanel>

              {/* Classement */}
              <TabPanel value="classement" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Classement du Tournoi</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="text-center p-8">
                      <p className="text-body-m text-brand-green-300">
                        Le classement sera affiché ici lors du tournoi.
                      </p>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>
            </Tabs>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default TournamentDetail;
