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
  UserAvatar,
} from "../components";
import { PlayerCard, MatchCard } from "../components/domain";
import { Team, Player, Match } from "../types";
import { api } from "../api";
import { mockTeam, mockPlayers, mockMatches } from "../data/mocks";

const TeamDetail: React.FC = () => {
  const { teamId } = useParams<{ teamId: string }>();

  const { data: team, isLoading } = useQuery<Team>({
    queryKey: ["team", teamId],
    queryFn: async () => {
      const response = await api().GET<Team>(`/api/teams/${teamId}`);
      return response.data;
    },
    enabled: !!teamId,
  });

  // For development, use mock data if no real data
  const displayTeam = team || mockTeam;

  if (isLoading && !team) {
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
              <Link
                to={
                  displayTeam.competitions.length > 0
                    ? `/championnats`
                    : `/clubs/${displayTeam.clubId}`
                }
              >
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Logo/Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-4">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayTeam.name}
                  </h1>
                  <Badge variant="gold">{displayTeam.club.code}</Badge>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="primary">{displayTeam.club.name}</Badge>
                  <Badge variant={displayTeam.status === "active" ? "success" : "warning"}>
                    {displayTeam.status === "active" ? "Active" : "Inactive"}
                  </Badge>
                </div>

                <p className="text-body-m text-brand-green-200 mb-4 max-w-2xl">
                  Équipe de {displayTeam.club.name} évoluant en {displayTeam.club.division}
                </p>

                {/* Captain Info */}
                {displayTeam.captain && (
                  <div className="flex items-center gap-4 mb-4">
                    <span className="text-brand-green-400">Capitaine:</span>
                    <Link
                      to={`/joueurs/${displayTeam.captain.id}`}
                      className="flex items-center gap-2"
                    >
                      <UserAvatar
                        user={{
                          id: displayTeam.captain.id,
                          name: displayTeam.captain.fullName,
                          photo: displayTeam.captain.photo ?? undefined,
                        }}
                        size="m"
                      />
                      <span className="text-brand-orange-pure hover:underline">
                        {displayTeam.captain.fullName}
                      </span>
                    </Link>
                  </div>
                )}
              </div>

              {/* Stats */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-4 border border-brand-green-700/50 text-center min-w-[200px]">
                  <p className="text-body-xs text-brand-green-400 mb-1">Statistiques</p>
                  <div className="grid grid-cols-2 gap-2 mt-2">
                    <div>
                      <p className="text-body-xs text-brand-green-300">Victoires</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayTeam.stats.wins}
                      </p>
                    </div>
                    <div>
                      <p className="text-body-xs text-brand-green-300">Défaites</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayTeam.stats.losses}
                      </p>
                    </div>
                    <div>
                      <p className="text-body-xs text-brand-green-300">Nuls</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayTeam.stats.draws}
                      </p>
                    </div>
                    <div>
                      <p className="text-body-xs text-brand-green-300">Points</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayTeam.stats.points}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Stats Overview */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-brand-green-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-brand-orange-pure uppercase tracking-wide mb-6">
              Statistiques de l&apos;Équipe
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-4">
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Matchs Joués</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.matches}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Victoires</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.wins}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Défaites</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.losses}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Nuls</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.draws}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Points Pour</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.pointsFor}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Points Contre</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.pointsAgainst}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Différence</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayTeam.stats.diff >= 0
                      ? `+${displayTeam.stats.diff}`
                      : displayTeam.stats.diff}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Win Rate</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {(displayTeam.stats.winRate * 100).toFixed(0)}%
                  </p>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="joueurs">
              <TabList>
                <Tab value="joueurs" label={`Joueurs (${displayTeam.players.length})`} />
                <Tab value="matchs" label="Matchs" />
                <Tab
                  value="competitions"
                  label={`Compétitions (${displayTeam.competitions.length})`}
                />
                <Tab value="statistiques" label="Statistiques Détaillées" />
              </TabList>

              {/* Joueurs */}
              <TabPanel value="joueurs" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {displayTeam.players.length > 0 ? (
                    displayTeam.players.map((player) => (
                      <PlayerCard key={player.id} player={player} showClub={false} />
                    ))
                  ) : (
                    <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-brand-green-300">
                          Aucune information sur les joueurs disponible.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>
              </TabPanel>

              {/* Matchs */}
              <TabPanel value="matchs" className="py-6">
                <div className="space-y-4">
                  {mockMatches.slice(0, 5).map((match) => (
                    <MatchCard key={match.id} match={match} showDetails />
                  ))}
                </div>
              </TabPanel>

              {/* Compétitions */}
              <TabPanel value="competitions" className="py-6">
                {displayTeam.competitions.length > 0 ? (
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {displayTeam.competitions.map((competition) => (
                      <Card
                        key={competition.id}
                        className="bg-brand-green-900/50 border-brand-green-700/50"
                      >
                        <CardHeader>
                          <div className="flex items-center justify-between">
                            <div>
                              <h3 className="text-heading-s font-bold text-white">
                                {competition.competition.name}
                              </h3>
                              <p className="text-body-xs text-brand-green-300">
                                {competition.competition.code} - {competition.season}
                              </p>
                            </div>
                            <Badge
                              variant={
                                competition.status === "qualified"
                                  ? "success"
                                  : competition.status === "in_progress"
                                    ? "warning"
                                    : "secondary"
                              }
                            >
                              {competition.status.replace("_", " ")}
                            </Badge>
                          </div>
                        </CardHeader>
                        <CardContent>
                          <div className="grid grid-cols-2 gap-4">
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Matchs</p>
                              <p className="text-heading-s font-bold text-brand-orange-pure">
                                {competition.results.matches}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Victoires</p>
                              <p className="text-heading-s font-bold text-brand-orange-pure">
                                {competition.results.wins}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Position</p>
                              <p className="text-heading-s font-bold text-brand-orange-pure">
                                {competition.results.position || "N/A"}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-brand-green-300">Points</p>
                              <p className="text-heading-s font-bold text-brand-orange-pure">
                                {competition.results.points}
                              </p>
                            </div>
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                ) : (
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardContent className="text-center p-8">
                      <p className="text-body-m text-brand-green-300">
                        Aucune compétition pour cette équipe.
                      </p>
                    </CardContent>
                  </Card>
                )}
              </TabPanel>

              {/* Statistiques Détaillées */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Performance par Position
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Premier</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayTeam.players.filter((p) => p.id === displayTeam.captainId)
                              .length > 0
                              ? "Capitaine"
                              : "Joueur"}
                          </p>
                          <p className="text-body-xs text-brand-green-300 mt-2">
                            {displayTeam.players.length} joueurs
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Second</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayTeam.players.length > 1
                              ? displayTeam.players[1]?.fullName || "Joueur"
                              : "N/A"}
                          </p>
                          <p className="text-body-xs text-brand-green-300 mt-2">
                            {displayTeam.players.length > 1 ? displayTeam.players.length - 1 : 0}{" "}
                            joueurs
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Classement</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">Division:</span>
                          <Badge variant="primary">{displayTeam.club.division}</Badge>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">Ligue:</span>
                          <Badge variant="secondary">{displayTeam.club.league}</Badge>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-brand-green-300">Statut:</span>
                          <Badge variant={displayTeam.status === "active" ? "success" : "warning"}>
                            {displayTeam.status}
                          </Badge>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>

                {/* Club Info */}
                <Card className="bg-brand-green-900/50 border-brand-green-700/50 mt-6">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Club</h3>
                  </CardHeader>
                  <CardContent>
                    <Link to={`/clubs/${displayTeam.clubId}`} className="flex items-center gap-4">
                      <img
                        src={displayTeam.club.logo || "/images/clubs/default.png"}
                        alt={displayTeam.club.name}
                        className="w-16 h-16 rounded-full object-cover"
                      />
                      <div>
                        <h4 className="text-heading-s font-bold text-brand-orange-pure">
                          {displayTeam.club.name}
                        </h4>
                        <p className="text-body-s text-brand-green-300">
                          {displayTeam.club.city} ({displayTeam.club.postalCode})
                        </p>
                        <p className="text-body-xs text-brand-green-400">
                          {displayTeam.club.playersCount} joueurs - {displayTeam.club.teamsCount}{" "}
                          équipes
                        </p>
                      </div>
                    </Link>
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

export default TeamDetail;
