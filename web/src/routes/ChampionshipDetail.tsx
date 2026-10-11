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
import { ClassificationTable, MatchCard, ClubCard, TournamentCard } from "../components/domain";
import { Championship, Team, Match, ChampionshipClassification, ChampionshipTeam } from "../types";
import { api } from "../api";
import { mockChampionship, mockClassification, mockMatches, mockTeams } from "../data/mocks";

const ChampionshipDetail: React.FC = () => {
  const { championshipId } = useParams<{ championshipId: string }>();

  const { data: championship, isLoading } = useQuery<Championship>({
    queryKey: ["championship", championshipId],
    queryFn: async () => {
      const response = await api().GET<Championship>(`/api/championships/${championshipId}`);
      return response.data;
    },
    enabled: !!championshipId,
  });

  // For development, use mock data if no real data
  const displayChampionship = championship || mockChampionship;

  if (isLoading && !championship) {
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
              <Link to="/championnats">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux championnats
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-4">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayChampionship.name}
                  </h1>
                  <Badge variant="gold">{displayChampionship.code}</Badge>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="primary">{displayChampionship.category}</Badge>
                  <Badge variant="secondary">{displayChampionship.material}</Badge>
                  <Badge variant="default">{displayChampionship.season}</Badge>
                  <Badge
                    variant={
                      displayChampionship.status === "in_progress"
                        ? "success"
                        : displayChampionship.status === "completed"
                          ? "default"
                          : "warning"
                    }
                  >
                    {displayChampionship.status.replace("_", " ")}
                  </Badge>
                </div>

                <p className="text-body-m text-brand-green-200 mb-4 max-w-2xl">
                  {displayChampionship.description}
                </p>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Lieu:</span>
                    <span className="text-brand-orange-pure">{displayChampionship.location}</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Format:</span>
                    <span className="text-brand-orange-pure">
                      {displayChampionship.format.replace("_", " ")}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Type:</span>
                    <span className="text-brand-orange-pure">{displayChampionship.type}</span>
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Équipes inscrites:</span>
                    <span className="text-brand-orange-pure">
                      {displayChampionship.registeredTeams}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Journées:</span>
                    <span className="text-brand-orange-pure">
                      {displayChampionship.currentJournee || 0}/{displayChampionship.totalJournees}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Organisateur:</span>
                    <span className="text-brand-orange-pure">
                      {(displayChampionship.organizer as any)?.name || "CDSP 79"}
                    </span>
                  </div>
                </div>
              </div>

              {/* Current Journee */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-4 border border-brand-green-700/50 text-center">
                  <p className="text-body-xs text-brand-green-400 mb-1">Journée Actuelle</p>
                  <p className="text-heading-l font-bold text-brand-orange-pure">
                    #{displayChampionship.currentJournee || 0}
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
                    {displayChampionship.type}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Catégorie</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayChampionship.category}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Matériel</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayChampionship.material}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Saison</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayChampionship.season}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Équipes</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayChampionship.registeredTeams}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Statut</p>
                  <Badge
                    variant={
                      displayChampionship.status === "in_progress"
                        ? "success"
                        : displayChampionship.status === "completed"
                          ? "default"
                          : "warning"
                    }
                  >
                    {displayChampionship.status.replace("_", " ")}
                  </Badge>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="classement">
              <TabList>
                <Tab value="classement" label="Classement" />
                <Tab value="journees" label="Journées" />
                <Tab value="equipes" label={`Équipes (${displayChampionship.registeredTeams})`} />
                <Tab value="matchs" label="Matchs" />
                <Tab value="statistiques" label="Statistiques" />
              </TabList>

              {/* Classement */}
              <TabPanel value="classement" className="py-6">
                <ClassificationTable
                  teams={displayChampionship.classification?.teams || mockClassification.teams}
                  competition={`${displayChampionship.name} - ${displayChampionship.divisions[0]}`}
                  journee={displayChampionship.currentJournee || mockClassification.journee}
                  season={displayChampionship.season}
                  showAllLink={false}
                  onAllClick={() => {}}
                />
              </TabPanel>

              {/* Journées */}
              <TabPanel value="journees" className="py-6">
                <div className="space-y-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Journée {displayChampionship.currentJournee || mockClassification.journee}
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        {mockMatches.slice(0, 3).map((match) => (
                          <MatchCard key={match.id} match={match} showDetails />
                        ))}
                      </div>
                    </CardContent>
                  </Card>

                  {Array.from({ length: Math.min(3, displayChampionship.totalJournees || 5) }).map(
                    (_, index) => {
                      const journeeNumber = (displayChampionship.currentJournee || 5) - index - 1;
                      if (journeeNumber <= 0) return null;
                      return (
                        <Card
                          key={journeeNumber}
                          className="bg-brand-green-900/50 border-brand-green-700/50"
                        >
                          <CardHeader>
                            <h3 className="text-heading-s font-bold text-white">
                              Journée {journeeNumber}
                            </h3>
                          </CardHeader>
                          <CardContent>
                            <p className="text-body-m text-brand-green-300 text-center">
                              Matchs de la journée {journeeNumber}
                            </p>
                          </CardContent>
                        </Card>
                      );
                    },
                  )}
                </div>
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
                            <p className="text-body-xs text-brand-green-300">Points</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {team.stats.points}
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

              {/* Statistiques */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Statistiques du Championnat
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Matchs joués</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayChampionship.stats.totalMatches}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Matchs terminés</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayChampionship.stats.completedMatches}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Points totaux</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayChampionship.stats.totalPoints}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">
                            Meilleur marqueur
                          </p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {(displayChampionship.stats.topScorer as any)?.fullName || "Non défini"}
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Règles de Classement</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">
                            Critère principal:
                          </span>
                          <Badge variant="gold">
                            {displayChampionship.classificationRules.primary}
                          </Badge>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">
                            Critère secondaire:
                          </span>
                          <Badge variant="secondary">
                            {displayChampionship.classificationRules.secondary}
                          </Badge>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">
                            Critère tertiaire:
                          </span>
                          <Badge variant="secondary">
                            {displayChampionship.classificationRules.tertiary}
                          </Badge>
                        </div>
                        <div className="mt-4 pt-4 border-t border-brand-green-700/50">
                          <p className="text-body-xs text-brand-green-400 mb-2">
                            Système de points:
                          </p>
                          <div className="grid grid-cols-3 gap-2">
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Victoire</p>
                              <Badge variant="success">{displayChampionship.winPoints} pts</Badge>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Nul</p>
                              <Badge variant="secondary">{displayChampionship.drawPoints} pt</Badge>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Défaite</p>
                              <Badge variant="danger">{displayChampionship.lossPoints} pt</Badge>
                            </div>
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </TabPanel>
            </Tabs>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default ChampionshipDetail;
