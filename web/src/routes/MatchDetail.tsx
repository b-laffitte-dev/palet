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
import { MatchCard, PlayerCard } from "../components/domain";
import { Match, Team, Player } from "../types";
import { api } from "../api";
import { mockMatch, mockTeams, mockPlayers } from "../data/mocks";

const MatchDetail: React.FC = () => {
  const { matchId } = useParams<{ matchId: string }>();

  const { data: match, isLoading } = useQuery<Match>({
    queryKey: ["match", matchId],
    queryFn: async () => {
      const response = await api().GET<Match>(`/api/matches/${matchId}`);
      return response.data;
    },
    enabled: !!matchId,
  });

  // For development, use mock data if no real data
  const displayMatch = match || mockMatch;

  if (isLoading && !match) {
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

  // Determine match status color
  const getStatusColor = (status: string) => {
    switch (status) {
      case "in_progress":
        return "success";
      case "completed":
        return "default";
      case "scheduled":
        return "warning";
      case "postponed":
      case "cancelled":
        return "danger";
      default:
        return "secondary";
    }
  };

  // Determine winner
  const getWinnerName = () => {
    if (displayMatch.winner === "team1") return displayMatch.team1.name;
    if (displayMatch.winner === "team2") return displayMatch.team2.name;
    if (displayMatch.team1Score && displayMatch.team2Score) {
      if (displayMatch.team1Score > displayMatch.team2Score) return displayMatch.team1.name;
      if (displayMatch.team2Score > displayMatch.team1Score) return displayMatch.team2.name;
    }
    return null;
  };

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center gap-4 mb-4">
              <Link
                to={
                  displayMatch.competition.type === "championship" ? "/championnats" : "/tournois"
                }
              >
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux{" "}
                  {displayMatch.competition.type === "championship" ? "championnats" : "tournois"}
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Match Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-4">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayMatch.competition.name}
                  </h1>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="primary">{displayMatch.competition.code}</Badge>
                  <Badge variant="secondary">{displayMatch.type}</Badge>
                  <Badge variant="default">{displayMatch.competition.category}</Badge>
                  <Badge variant={getStatusColor(displayMatch.status)}>
                    {displayMatch.status.replace("_", " ")}
                  </Badge>
                  {displayMatch.isLive && (
                    <Badge variant="danger" className="animate-pulse">
                      EN DIRECT
                    </Badge>
                  )}
                </div>

                {/* Match Details */}
                <div className="bg-brand-green-900/50 rounded-lg p-6 border border-brand-green-700/50">
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Journée</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.journee || "Non spécifiée"}
                      </p>
                    </div>
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Phase</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.phase || "Non spécifiée"}
                      </p>
                    </div>
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Type</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.competition.category}
                      </p>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Date</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.date
                          ? new Date(displayMatch.date).toLocaleDateString("fr-FR")
                          : "Non spécifiée"}
                      </p>
                    </div>
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Heure</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.time || "Non spécifiée"}
                      </p>
                    </div>
                    <div className="text-center">
                      <p className="text-body-xs text-brand-green-300 mb-1">Lieu</p>
                      <p className="text-heading-s font-bold text-brand-orange-pure">
                        {displayMatch.location || "Non spécifié"}
                      </p>
                    </div>
                  </div>
                </div>
              </div>

              {/* Score */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-6 border border-brand-green-700/50 text-center min-w-[200px]">
                  <p className="text-body-xs text-brand-green-400 mb-2">Score Final</p>
                  <div className="flex items-center justify-center gap-4 mb-4">
                    <div className="text-center">
                      <TeamAvatar
                        team={{
                          id: displayMatch.team1.id,
                          name: displayMatch.team1.name,
                          logo: displayMatch.team1.logo ?? undefined,
                          abbreviation: displayMatch.team1.name.substring(0, 3),
                        }}
                        size="m"
                      />
                      <p className="text-heading-m font-bold text-brand-orange-pure mt-2">
                        {displayMatch.team1Score ?? "-"}
                      </p>
                    </div>
                    <div className="text-heading-xl font-bold text-white">-</div>
                    <div className="text-center">
                      <TeamAvatar
                        team={{
                          id: displayMatch.team2.id,
                          name: displayMatch.team2.name,
                          logo: displayMatch.team2.logo ?? undefined,
                          abbreviation: displayMatch.team2.name.substring(0, 3),
                        }}
                        size="m"
                      />
                      <p className="text-heading-m font-bold text-brand-orange-pure mt-2">
                        {displayMatch.team2Score ?? "-"}
                      </p>
                    </div>
                  </div>
                  {getWinnerName() && displayMatch.status === "completed" && (
                    <Badge variant="gold" className="w-full">
                      Vainqueur: {getWinnerName()}
                    </Badge>
                  )}
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Match Stats */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-brand-green-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-brand-orange-pure uppercase tracking-wide mb-6">
              Détails du Match
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Compétition</p>
                  <p className="text-heading-s font-bold text-brand-orange-pure truncate">
                    {displayMatch.competition.name}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Statut</p>
                  <Badge variant={getStatusColor(displayMatch.status)}>
                    {displayMatch.status.replace("_", " ")}
                  </Badge>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Manches jouées</p>
                  <p className="text-heading-s font-bold text-brand-orange-pure">
                    {displayMatch.currentManche || displayMatch.stats.totalManches || 0}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Moyenne/Manche</p>
                  <p className="text-heading-s font-bold text-brand-orange-pure">
                    {displayMatch.stats.avgPointsPerManche.toFixed(1)}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Vérifié</p>
                  <Badge variant={displayMatch.verified ? "success" : "warning"}>
                    {displayMatch.verified ? "Oui" : "Non"}
                  </Badge>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">En direct</p>
                  <Badge variant={displayMatch.isLive ? "danger" : "secondary"}>
                    {displayMatch.isLive ? "Oui" : "Non"}
                  </Badge>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="resume">
              <TabList>
                <Tab value="resume" label="Résumé" />
                <Tab value="equipes" label="Équipes" />
                <Tab value="joueurs" label="Joueurs" />
                <Tab value="manches" label="Manches" />
                <Tab value="statistiques" label="Statistiques" />
              </TabList>

              {/* Résumé */}
              <TabPanel value="resume" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Informations Générales
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="grid grid-cols-2 gap-4">
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">Type de match</p>
                            <p className="text-body-m text-brand-orange-pure">
                              {displayMatch.type}
                            </p>
                          </div>
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">Catégorie</p>
                            <p className="text-body-m text-brand-orange-pure">
                              {displayMatch.competition.category}
                            </p>
                          </div>
                        </div>
                        <div className="grid grid-cols-2 gap-4">
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">Matériel</p>
                            <p className="text-body-m text-brand-orange-pure">
                              {displayMatch.competition.material}
                            </p>
                          </div>
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">
                              Points par manche
                            </p>
                            <p className="text-body-m text-brand-orange-pure">
                              {displayMatch.competition.pointsType}
                            </p>
                          </div>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Organisateur</p>
                          <p className="text-body-m text-brand-orange-pure">
                            {(displayMatch.competition.organizer as any)?.name || "CDSP 79"}
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Calendrier</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Date</p>
                          <p className="text-body-m text-brand-orange-pure">
                            {displayMatch.date
                              ? new Date(displayMatch.date).toLocaleDateString("fr-FR")
                              : "Non spécifiée"}
                          </p>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Heure</p>
                          <p className="text-body-m text-brand-orange-pure">
                            {displayMatch.time || "Non spécifiée"}
                          </p>
                        </div>
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Lieu</p>
                          <p className="text-body-m text-brand-orange-pure">
                            {displayMatch.location || "Non spécifié"}
                          </p>
                        </div>
                        {displayMatch.startedAt && (
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">Débuté à</p>
                            <p className="text-body-m text-brand-orange-pure">
                              {new Date(displayMatch.startedAt).toLocaleString("fr-FR")}
                            </p>
                          </div>
                        )}
                        {displayMatch.endedAt && (
                          <div>
                            <p className="text-body-xs text-brand-green-300 mb-1">Terminé à</p>
                            <p className="text-body-m text-brand-orange-pure">
                              {new Date(displayMatch.endedAt).toLocaleString("fr-FR")}
                            </p>
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  {/* Équipe 1 */}
                  <Card
                    className={`bg-brand-green-900/50 border-brand-green-700/50 ${getWinnerName() === displayMatch.team1.name ? "ring-2 ring-brand-orange-pure" : ""}`}
                  >
                    <CardHeader>
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <TeamAvatar
                            team={{
                              id: displayMatch.team1.id,
                              name: displayMatch.team1.name,
                              logo: displayMatch.team1.logo ?? undefined,
                              abbreviation: displayMatch.team1.name.substring(0, 3),
                            }}
                            size="m"
                          />
                          <div>
                            <h3 className="text-heading-s font-bold text-white">
                              {displayMatch.team1.name}
                            </h3>
                            <p className="text-body-xs text-brand-green-300">
                              Rang: {displayMatch.team1.rank || "N/A"} | Seed:{" "}
                              {displayMatch.team1.seed || "N/A"}
                            </p>
                          </div>
                        </div>
                        <Badge
                          variant={
                            getWinnerName() === displayMatch.team1.name ? "gold" : "secondary"
                          }
                        >
                          {displayMatch.team1Score ?? "-"} pts
                        </Badge>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Joueurs:</p>
                          {displayMatch.team1.players.length > 0 ? (
                            <div className="flex flex-wrap gap-2">
                              {displayMatch.team1.players.map((p) => (
                                <UserAvatar
                                  key={p.id}
                                  user={{ id: p.id, name: p.fullName, photo: p.photo ?? undefined }}
                                  size="sm"
                                />
                              ))}
                            </div>
                          ) : (
                            <p className="text-body-s text-brand-green-400">
                              Aucun joueur spécifié
                            </p>
                          )}
                        </div>
                      </div>
                    </CardContent>
                    {displayMatch.team1.club && (
                      <CardFooter>
                        <Link
                          to={`/clubs/${(displayMatch.team1 as any).clubId || ""}`}
                          className="text-brand-orange-pure hover:underline text-body-s"
                        >
                          Voir le club
                        </Link>
                      </CardFooter>
                    )}
                  </Card>

                  {/* Équipe 2 */}
                  <Card
                    className={`bg-brand-green-900/50 border-brand-green-700/50 ${getWinnerName() === displayMatch.team2.name ? "ring-2 ring-brand-orange-pure" : ""}`}
                  >
                    <CardHeader>
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-3">
                          <TeamAvatar
                            team={{
                              id: displayMatch.team2.id,
                              name: displayMatch.team2.name,
                              logo: displayMatch.team2.logo ?? undefined,
                              abbreviation: displayMatch.team2.name.substring(0, 3),
                            }}
                            size="m"
                          />
                          <div>
                            <h3 className="text-heading-s font-bold text-white">
                              {displayMatch.team2.name}
                            </h3>
                            <p className="text-body-xs text-brand-green-300">
                              Rang: {displayMatch.team2.rank || "N/A"} | Seed:{" "}
                              {displayMatch.team2.seed || "N/A"}
                            </p>
                          </div>
                        </div>
                        <Badge
                          variant={
                            getWinnerName() === displayMatch.team2.name ? "gold" : "secondary"
                          }
                        >
                          {displayMatch.team2Score ?? "-"} pts
                        </Badge>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div>
                          <p className="text-body-xs text-brand-green-300 mb-1">Joueurs:</p>
                          {displayMatch.team2.players.length > 0 ? (
                            <div className="flex flex-wrap gap-2">
                              {displayMatch.team2.players.map((p) => (
                                <UserAvatar
                                  key={p.id}
                                  user={{ id: p.id, name: p.fullName, photo: p.photo ?? undefined }}
                                  size="sm"
                                />
                              ))}
                            </div>
                          ) : (
                            <p className="text-body-s text-brand-green-400">
                              Aucun joueur spécifié
                            </p>
                          )}
                        </div>
                      </div>
                    </CardContent>
                    {displayMatch.team2.club && (
                      <CardFooter>
                        <Link
                          to={`/clubs/${(displayMatch.team2 as any).clubId || ""}`}
                          className="text-brand-orange-pure hover:underline text-body-s"
                        >
                          Voir le club
                        </Link>
                      </CardFooter>
                    )}
                  </Card>
                </div>
              </TabPanel>

              {/* Joueurs */}
              <TabPanel value="joueurs" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {[...displayMatch.team1.players, ...displayMatch.team2.players].map((player) => (
                    <PlayerCard key={player.id} player={player} showClub={true} />
                  ))}
                </div>
              </TabPanel>

              {/* Manches */}
              <TabPanel value="manches" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Détail des Manches</h3>
                  </CardHeader>
                  <CardContent>
                    {displayMatch.manches && displayMatch.manches.length > 0 ? (
                      <div className="space-y-4">
                        {displayMatch.manches.map((manche, index) => (
                          <div
                            key={index}
                            className="bg-brand-green-800/50 rounded-lg p-4 border border-brand-green-700/30"
                          >
                            <div className="flex items-center justify-between mb-2">
                              <p className="text-body-s text-brand-orange-pure font-bold">
                                Manche {manche.number}
                              </p>
                              <Badge variant={manche.winner ? "success" : "secondary"}>
                                {manche.winner
                                  ? `Vainqueur: ${manche.winner === "team1" ? displayMatch.team1.name : displayMatch.team2.name}`
                                  : "Égalité"}
                              </Badge>
                            </div>
                            <div className="flex items-center justify-center gap-4">
                              <div className="text-center">
                                <p className="text-body-xs text-brand-green-300 mb-1">
                                  {displayMatch.team1.name}
                                </p>
                                <p className="text-heading-m font-bold text-brand-orange-pure">
                                  {manche.team1} pts
                                </p>
                              </div>
                              <div className="text-heading-m font-bold text-white">-</div>
                              <div className="text-center">
                                <p className="text-body-xs text-brand-green-300 mb-1">
                                  {displayMatch.team2.name}
                                </p>
                                <p className="text-heading-m font-bold text-brand-orange-pure">
                                  {manche.team2} pts
                                </p>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <div className="text-center p-8">
                        <p className="text-body-m text-brand-green-300">
                          Aucune manche enregistrée pour ce match.
                        </p>
                      </div>
                    )}
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Statistiques */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Statistiques du Match</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="grid grid-cols-2 gap-4">
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300 mb-1">Total Manches</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {displayMatch.stats.totalManches}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300 mb-1">Moyenne/Manche</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {displayMatch.stats.avgPointsPerManche.toFixed(1)}
                            </p>
                          </div>
                        </div>
                        <div className="grid grid-cols-2 gap-4">
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300 mb-1">
                              Meilleure Manche
                            </p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {displayMatch.stats.bestManche
                                ? `${displayMatch.stats.bestManche.team1 || 0}-${displayMatch.stats.bestManche.team2 || 0}`
                                : "N/A"}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-brand-green-300 mb-1">Statut</p>
                            <Badge variant={displayMatch.verified ? "success" : "warning"}>
                              {displayMatch.verified ? "Vérifié" : "Non vérifié"}
                            </Badge>
                          </div>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Statistiques par Joueur
                      </h3>
                    </CardHeader>
                    <CardContent>
                      {displayMatch.stats.playerStats &&
                      displayMatch.stats.playerStats.length > 0 ? (
                        <div className="space-y-4">
                          {displayMatch.stats.playerStats.map((playerStat) => (
                            <div
                              key={playerStat.player.id}
                              className="bg-brand-green-800/50 rounded-lg p-4 border border-brand-green-700/30"
                            >
                              <div className="flex items-center justify-between">
                                <div className="flex items-center gap-3">
                                  <UserAvatar
                                    user={{
                                      id: playerStat.player.id,
                                      name: playerStat.player.fullName,
                                      photo: playerStat.player.photo ?? undefined,
                                    }}
                                    size="sm"
                                  />
                                  <div>
                                    <p className="text-body-s text-white font-bold">
                                      {playerStat.player.fullName}
                                    </p>
                                    <p className="text-body-xs text-brand-green-300">
                                      {(playerStat.player.club as any)?.name}
                                    </p>
                                  </div>
                                </div>
                                <div className="text-right">
                                  <p className="text-body-xs text-brand-green-300">Points</p>
                                  <p className="text-heading-s font-bold text-brand-orange-pure">
                                    {playerStat.points}
                                  </p>
                                </div>
                              </div>
                              <div className="grid grid-cols-2 gap-4 mt-3">
                                <div className="text-center">
                                  <p className="text-body-xs text-brand-green-300">Précision</p>
                                  <p className="text-body-s text-brand-orange-pure">
                                    {playerStat.accuracy.toFixed(0)}%
                                  </p>
                                </div>
                                <div className="text-center">
                                  <p className="text-body-xs text-brand-green-300">
                                    Meilleure Manche
                                  </p>
                                  <p className="text-body-s text-brand-orange-pure">
                                    {playerStat.bestManche}
                                  </p>
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-center p-8">
                          <p className="text-body-m text-brand-green-300">
                            Aucune statistique par joueur disponible.
                          </p>
                        </div>
                      )}
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

export default MatchDetail;
