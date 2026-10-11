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
  UserAvatar,
} from "../components";
import { PlayerCard, ClubCard } from "../components/domain";
import { Player, Club, Match, Trophy, Badge as BadgeType } from "../types";
import { api } from "../api";
import { mockPlayer, mockClubs } from "../data/mocks";

const PlayerDetail: React.FC = () => {
  const { playerId } = useParams<{ playerId: string }>();

  const { data: player, isLoading } = useQuery<Player>({
    queryKey: ["player", playerId],
    queryFn: async () => {
      const response = await api().GET<Player>(`/api/players/${playerId}`);
      return response.data;
    },
    enabled: !!playerId,
  });

  // For development, use mock data if no real data
  const displayPlayer = player || mockPlayer;
  const displayClub = displayPlayer.club || mockClubs[0];

  if (isLoading && !player) {
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
              <Link to="/joueurs">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux joueurs
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Photo */}
              <div className="flex-shrink-0">
                <img
                  src={displayPlayer.photo || "/images/players/default.png"}
                  alt={displayPlayer.fullName}
                  className="w-32 h-32 rounded-full object-cover border-4 border-brand-orange-700"
                />
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayPlayer.fullName}
                  </h1>
                  {displayPlayer.alias && (
                    <span className="text-heading-m text-brand-green-300">
                      "{displayPlayer.alias}"
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant={displayPlayer.category === "Senior" ? "gold" : "primary"}>
                    {displayPlayer.category}
                  </Badge>
                  <Badge variant={displayPlayer.status === "active" ? "success" : "warning"}>
                    {displayPlayer.status === "active" ? "Actif" : "Inactif"}
                  </Badge>
                  <Badge variant="default">{displayPlayer.nationality}</Badge>
                  <Badge variant="secondary">{displayPlayer.age} ans</Badge>
                </div>

                <p className="text-body-m text-brand-green-200 mb-4 max-w-2xl">
                  {displayPlayer.birthPlace && `${displayPlayer.birthPlace} - `}
                  {displayPlayer.city}, {displayPlayer.postalCode}
                </p>

                {/* Club Info */}
                <div className="flex items-center gap-4 mb-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Club:</span>
                    <Link
                      to={`/clubs/${displayClub.id}`}
                      className="flex items-center gap-2 text-brand-orange-pure hover:underline"
                    >
                      <img
                        src={displayClub.logo || "/images/clubs/default.png"}
                        alt={displayClub.name}
                        className="w-6 h-6 rounded-full"
                      />
                      <span>{displayClub.name}</span>
                    </Link>
                  </div>
                  {displayPlayer.clubNumber && (
                    <Badge variant="secondary">{displayPlayer.clubNumber}</Badge>
                  )}
                </div>

                {/* License Info */}
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Licence:</span>
                    <Badge
                      variant={displayPlayer.licenseStatus === "valid" ? "success" : "warning"}
                    >
                      {displayPlayer.licenseNumber}
                    </Badge>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-brand-green-400">Main:</span>
                    <Badge variant="default">
                      {displayPlayer.dominantHand === "right" ? "Droitier" : "Gaucher"}
                    </Badge>
                  </div>
                  {displayPlayer.playStyle && (
                    <div className="flex items-center gap-2">
                      <span className="text-brand-green-400">Style:</span>
                      <Badge variant="secondary">{displayPlayer.playStyle}</Badge>
                    </div>
                  )}
                </div>
              </div>

              {/* Rank */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-4 border border-brand-green-700/50 text-center">
                  <p className="text-body-xs text-brand-green-400 mb-1">Classement</p>
                  <p className="text-heading-l font-bold text-brand-orange-pure">
                    #{displayPlayer.stats.currentRank}
                  </p>
                  <p className="text-body-xs text-brand-green-400 mt-1">
                    {displayPlayer.stats.currentDivision}
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
              Statistiques Générales
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-4">
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Matchs Joués</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.matchesPlayed}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Victoires</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.wins}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Défaites</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.losses}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Nuls</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.draws}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Win Rate</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.winRate}%
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Points Totaux</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.totalPoints}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Moyenne</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.averagePoints.toFixed(1)}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Meilleur Score</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayPlayer.stats.bestScore}
                  </p>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="stats">
              <TabList>
                <Tab value="stats" label="Statistiques Détaillées" />
                <Tab value="historique" label="Historique" />
                <Tab value="trophees" label={`Trophées (${displayPlayer.trophies.total})`} />
                <Tab value="equipes" label="Équipes" />
              </TabList>

              {/* Stats Détaillées */}
              <TabPanel value="stats" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  {/* Stats par Position */}
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Statistiques par Position
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Premier</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.byPosition.first.matches} matchs
                          </p>
                          <p className="text-body-xs text-brand-green-300">
                            {displayPlayer.stats.byPosition.first.points} points
                          </p>
                          <p className="text-body-xs text-brand-green-300">
                            {displayPlayer.stats.byPosition.first.winRate?.toFixed(0)}% de victoires
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Second</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.byPosition.second.matches} matchs
                          </p>
                          <p className="text-body-xs text-brand-green-300">
                            {displayPlayer.stats.byPosition.second.points} points
                          </p>
                          <p className="text-body-xs text-brand-green-300">
                            {displayPlayer.stats.byPosition.second.winRate?.toFixed(0)}% de
                            victoires
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  {/* Autres Stats */}
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Autres Statistiques</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4">
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Points/Manche</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.pointsPerManche.toFixed(1)}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Meilleure Série</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.bestStreak}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Max Win Streak</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.maxWinStreak}
                          </p>
                        </div>
                        <div className="text-center">
                          <p className="text-body-xs text-brand-green-300 mb-1">Précision</p>
                          <p className="text-heading-s font-bold text-brand-orange-pure">
                            {displayPlayer.stats.accuracy}%
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </div>

                {/* Évolution du Classement */}
                <Card className="bg-brand-green-900/50 border-brand-green-700/50 mt-6">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Évolution du Classement</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="flex items-center justify-between">
                      <div className="text-center">
                        <p className="text-body-xs text-brand-green-300 mb-1">Classement Actuel</p>
                        <p className="text-heading-m font-bold text-brand-orange-pure">
                          #{displayPlayer.stats.currentRank}
                        </p>
                      </div>
                      <div className="text-center">
                        <p className="text-body-xs text-brand-green-300 mb-1">
                          Classement Précédent
                        </p>
                        <p className="text-heading-m font-bold text-brand-orange-pure">
                          #{displayPlayer.stats.previousRank}
                        </p>
                      </div>
                      <div className="text-center">
                        <p className="text-body-xs text-brand-green-300 mb-1">Tendance</p>
                        <Badge
                          variant={
                            displayPlayer.stats.rankTrend === "up"
                              ? "success"
                              : displayPlayer.stats.rankTrend === "down"
                                ? "danger"
                                : "secondary"
                          }
                        >
                          {displayPlayer.stats.rankTrend === "up"
                            ? "↑"
                            : displayPlayer.stats.rankTrend === "down"
                              ? "↓"
                              : "→"}
                        </Badge>
                      </div>
                      <div className="text-center">
                        <p className="text-body-xs text-brand-green-300 mb-1">Points Actuels</p>
                        <p className="text-heading-m font-bold text-brand-orange-pure">
                          {displayPlayer.stats.currentPoints}
                        </p>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Historique */}
              <TabPanel value="historique" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Historique des Matchs</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="text-center p-8">
                      <p className="text-body-m text-brand-green-300">
                        L&apos;historique des matchs sera affiché ici.
                      </p>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Trophées */}
              <TabPanel value="trophees" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">
                      Trophées ({displayPlayer.trophies.total})
                    </h3>
                  </CardHeader>
                  <CardContent>
                    {displayPlayer.trophies.total > 0 ? (
                      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {displayPlayer.trophies.list.map((trophy, index) => (
                          <div
                            key={index}
                            className="text-center p-4 bg-brand-green-800/50 rounded-lg"
                          >
                            <p className="text-body-s text-brand-orange-pure font-bold">
                              {trophy.name || `Trophée #${index + 1}`}
                            </p>
                            <p className="text-body-xs text-brand-green-300">
                              {trophy.year || trophy.season || "Date inconnue"}
                            </p>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <div className="text-center p-8">
                        <p className="text-body-m text-brand-green-300">
                          Aucun trophée pour le moment.
                        </p>
                      </div>
                    )}
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Équipes Actuelles</h3>
                  </CardHeader>
                  <CardContent>
                    {displayPlayer.currentTeams && displayPlayer.currentTeams.length > 0 ? (
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {displayPlayer.currentTeams.map((team) => (
                          <div key={team.id} className="p-4 bg-brand-green-800/50 rounded-lg">
                            <h4 className="text-heading-s text-brand-orange-pure font-bold mb-2">
                              {team.name}
                            </h4>
                            <p className="text-body-xs text-brand-green-300 mb-2">
                              Capitaine: {team.captain?.fullName || "Aucun"}
                            </p>
                            <p className="text-body-xs text-brand-green-300">
                              Joueurs: {team.players.length}
                            </p>
                            <Link to={`/teams/${team.id}`} className="inline-block mt-4">
                              <Button variant="gold" size="sm">
                                Voir l&apos;équipe
                              </Button>
                            </Link>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <div className="text-center p-8">
                        <p className="text-body-m text-brand-green-300">Aucune équipe actuelle.</p>
                      </div>
                    )}
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

export default PlayerDetail;
