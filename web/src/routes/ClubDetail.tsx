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
  TeamAvatar,
} from "../components";
import { PlayerCard, ClubCard } from "../components/domain";
import { Club, Player, Team, ChampionshipClassification, ChampionshipTeam } from "../types";
import { api } from "../api";
import { mockClub, mockClubPlayers, mockClubTeams, mockClassification } from "../data/mocks";

const ClubDetail: React.FC = () => {
  const { clubId } = useParams<{ clubId: string }>();

  const { data: club, isLoading } = useQuery<Club>({
    queryKey: ["club", clubId],
    queryFn: async () => {
      const response = await api().GET<Club>(`/api/clubs/${clubId}`);
      return response.data;
    },
    enabled: !!clubId,
  });

  // For development, use mock data if no real data
  const displayClub = club || mockClub;

  if (isLoading && !club) {
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
              <Link to="/clubs">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux clubs
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Logo */}
              <div className="flex-shrink-0">
                <img
                  src={displayClub.logo || "/images/clubs/default.png"}
                  alt={displayClub.name}
                  className="w-24 h-24 rounded-full object-cover border-4 border-brand-orange-700"
                />
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayClub.name}
                  </h1>
                  {displayClub.nickname && (
                    <span className="text-heading-m text-brand-green-300">
                      ({displayClub.nickname})
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant={displayClub.division === "D1" ? "gold" : "primary"}>
                    {displayClub.division}
                  </Badge>
                  <Badge variant={displayClub.status === "active" ? "success" : "warning"}>
                    {displayClub.status === "active" ? "Actif" : "Inactif"}
                  </Badge>
                  <Badge variant="default">{displayClub.league}</Badge>
                </div>

                <p className="text-body-m text-brand-green-200 mb-4 max-w-2xl">
                  {displayClub.address}, {displayClub.postalCode} {displayClub.city}
                </p>

                {/* Contact Info */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  {displayClub.contactEmail && (
                    <div className="flex items-center gap-2">
                      <span className="text-brand-green-400">Email:</span>
                      <a
                        href={`mailto:${displayClub.contactEmail}`}
                        className="text-brand-orange-pure hover:underline text-body-s"
                      >
                        {displayClub.contactEmail}
                      </a>
                    </div>
                  )}
                  {displayClub.contactPhone && (
                    <div className="flex items-center gap-2">
                      <span className="text-brand-green-400">Téléphone:</span>
                      <a
                        href={`tel:${displayClub.contactPhone}`}
                        className="text-brand-orange-pure hover:underline text-body-s"
                      >
                        {displayClub.contactPhone}
                      </a>
                    </div>
                  )}
                  {displayClub.website && (
                    <div className="flex items-center gap-2">
                      <span className="text-brand-green-400">Site web:</span>
                      <a
                        href={displayClub.website}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-brand-orange-pure hover:underline text-body-s"
                      >
                        {displayClub.website.replace(/^https?:\/\//, "")}
                      </a>
                    </div>
                  )}
                </div>

                {/* Social Links */}
                <div className="flex items-center gap-4 mt-4">
                  {displayClub.social.facebook && (
                    <a
                      href={displayClub.social.facebook}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-brand-green-400 hover:text-brand-orange-pure"
                    >
                      Facebook
                    </a>
                  )}
                  {displayClub.social.twitter && (
                    <a
                      href={displayClub.social.twitter}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-brand-green-400 hover:text-brand-orange-pure"
                    >
                      Twitter
                    </a>
                  )}
                  {displayClub.social.instagram && (
                    <a
                      href={displayClub.social.instagram}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-brand-green-400 hover:text-brand-orange-pure"
                    >
                      Instagram
                    </a>
                  )}
                </div>
              </div>

              {/* Code */}
              <div className="flex-shrink-0">
                <div className="bg-brand-green-900 rounded-lg p-4 border border-brand-green-700/50 text-center">
                  <p className="text-body-xs text-brand-green-400 mb-1">Code Club</p>
                  <p className="text-heading-l font-bold text-brand-orange-pure">
                    {displayClub.code}
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
              Statistiques du Club
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Joueurs</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayClub.playersCount}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Équipes</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayClub.teamsCount}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Division</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayClub.division}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Ligue</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {displayClub.league}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Fondé en</p>
                  <p className="text-heading-m font-bold text-brand-orange-pure">
                    {new Date(displayClub.createdAt).getFullYear()}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-brand-green-300 mb-1">Statut</p>
                  <Badge variant={displayClub.status === "active" ? "success" : "warning"}>
                    {displayClub.status === "active" ? "Actif" : "Inactif"}
                  </Badge>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="effectif">
              <TabList>
                <Tab value="effectif" label={`Effectif (${displayClub.playersCount})`} />
                <Tab value="equipes" label={`Équipes (${displayClub.teamsCount})`} />
                <Tab value="classement" label="Classement" />
                <Tab value="actualites" label="Actualités" />
              </TabList>

              {/* Effectif */}
              <TabPanel value="effectif" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {mockClubPlayers.map((player) => (
                    <PlayerCard key={player.id} player={player} showClub={false} />
                  ))}
                </div>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {mockClubTeams.map((team) => (
                    <Card key={team.id} className="bg-brand-green-900/50 border-brand-green-700/50">
                      <CardHeader>
                        <div className="flex items-center justify-between">
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
                              <p className="text-body-xs text-brand-green-300">
                                Capitaine: {team.captain?.fullName}
                              </p>
                            </div>
                          </div>
                          <Badge variant="secondary">
                            {team.stats.wins}V - {team.stats.losses}D
                          </Badge>
                        </div>
                      </CardHeader>
                      <CardContent>
                        <div className="grid grid-cols-3 gap-4">
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
                            <p className="text-body-xs text-brand-green-300">Win Rate</p>
                            <p className="text-heading-s font-bold text-brand-orange-pure">
                              {(team.stats.winRate * 100).toFixed(0)}%
                            </p>
                          </div>
                        </div>
                        <Link to={`/teams/${team.id}`} className="inline-block mt-4">
                          <Button variant="gold" size="sm">
                            Voir l&apos;équipe
                          </Button>
                        </Link>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </TabPanel>

              {/* Classement */}
              <TabPanel value="classement" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">
                      Classement {mockClassification.division} - Journée{" "}
                      {mockClassification.journee}
                    </h3>
                  </CardHeader>
                  <CardContent>
                    <div className="overflow-x-auto">
                      <table className="w-full text-body-s">
                        <thead>
                          <tr className="border-b border-brand-green-700/50 text-brand-green-300">
                            <th className="text-left p-3">Position</th>
                            <th className="text-left p-3">Équipe</th>
                            <th className="text-left p-3">Joués</th>
                            <th className="text-left p-3">Victoires</th>
                            <th className="text-left p-3">Défaites</th>
                            <th className="text-left p-3">Points</th>
                            <th className="text-left p-3">Forme</th>
                          </tr>
                        </thead>
                        <tbody>
                          {mockClassification.teams.map((team, index) => (
                            <tr
                              key={team.team.id}
                              className="border-b border-brand-green-700/20 hover:bg-brand-green-800/50"
                            >
                              <td className="p-3">
                                <Badge variant={team.position === 1 ? "gold" : "secondary"}>
                                  {team.position}
                                </Badge>
                              </td>
                              <td className="p-3">
                                <div className="flex items-center gap-2">
                                  <TeamAvatar
                                    team={{
                                      id: team.team.id,
                                      name: team.team.name,
                                      logo: team.team.logo ?? undefined,
                                      abbreviation: team.team.club.code,
                                    }}
                                    size="xs"
                                  />
                                  <span className="text-white">{team.team.name}</span>
                                </div>
                              </td>
                              <td className="p-3 text-brand-green-300">{team.stats.matches}</td>
                              <td className="p-3 text-brand-green-300">{team.stats.wins}</td>
                              <td className="p-3 text-brand-green-300">{team.stats.losses}</td>
                              <td className="p-3 text-brand-orange-pure font-bold">
                                {team.stats.points}
                              </td>
                              <td className="p-3">
                                <div className="flex gap-1">
                                  {team.form.map((result, i) => (
                                    <span
                                      key={i}
                                      className={`px-2 py-1 rounded text-xs ${
                                        result === "win"
                                          ? "bg-brand-green-900/50 text-brand-green-400"
                                          : result === "draw"
                                            ? "bg-amber-900/50 text-amber-400"
                                            : "bg-danger-900/50 text-danger-400"
                                      }`}
                                    >
                                      {result === "win" ? "V" : result === "draw" ? "N" : "D"}
                                    </span>
                                  ))}
                                </div>
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Actualités */}
              <TabPanel value="actualites" className="py-6">
                <Card className="bg-brand-green-900/50 border-brand-green-700/50">
                  <CardContent className="text-center p-8">
                    <p className="text-body-m text-brand-green-300">
                      Les actualités du club seront affichées ici.
                    </p>
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

export default ClubDetail;
