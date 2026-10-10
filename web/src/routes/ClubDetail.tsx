import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, UserAvatar, TeamAvatar } from '../components';
import { PlayerCard, ClubCard } from '../components/domain';
import { Club, Player, Team, ChampionshipClassification, ChampionshipTeam } from '../types';
import { api } from '../api';

// Mock data for development
const mockClub: Club = {
  id: 'club-001',
  name: 'Club des Essarts',
  nickname: 'Les Essartiates',
  code: 'ESS',
  logo: '/images/clubs/essarts.png',
  address: '123 Rue du Palet, 85140 Les Essarts',
  city: 'Les Essarts',
  postalCode: '85140',
  league: 'Vendée',
  division: 'D1',
  status: 'active',
  playersCount: 25,
  teamsCount: 8,
  contactEmail: 'contact@palet-essarts.fr',
  contactPhone: '+33 2 51 65 43 21',
  website: 'https://palet-essarts.fr',
  social: {
    facebook: 'https://facebook.com/paletessarts',
    twitter: 'https://twitter.com/paletessarts',
    instagram: 'https://instagram.com/paletessarts',
  },
  createdAt: new Date('2020-01-15'),
  updatedAt: new Date('2026-10-01'),
};

const mockPlayers: Player[] = [
  {
    id: 'player-001',
    firstName: 'Jean',
    lastName: 'Martin',
    fullName: 'Jean Martin',
    alias: 'Le Maestro',
    email: 'jean.martin@palet.fr',
    photo: '/images/players/jean-martin.jpg',
    status: 'active',
    birthDate: new Date('1985-03-15'),
    age: 41,
    birthPlace: 'Les Essarts',
    nationality: 'Français',
    dominantHand: 'right',
    playStyle: 'Précision',
    phone: '+33 6 12 34 56 78',
    address: '123 Rue du Palet, 85140 Les Essarts',
    city: 'Les Essarts',
    postalCode: '85140',
    club: mockClub,
    clubId: 'club-001',
    clubNumber: 'ESS-001',
    category: 'Senior',
    licenseNumber: 'PV-2026-001',
    licenseDate: new Date('2026-01-01'),
    licenseStatus: 'valid',
    registrationDate: new Date('2020-01-15'),
    createdAt: new Date('2020-01-15'),
    updatedAt: new Date('2026-10-09'),
    stats: {
      matchesPlayed: 150,
      wins: 105,
      draws: 20,
      losses: 25,
      totalPoints: 1575,
      averagePoints: 10.5,
      pointsPerManche: 3.5,
      bestScore: 15,
      bestStreak: 15,
      maxWinStreak: 8,
      winRate: 70,
      accuracy: 85,
      currentRank: 1,
      currentDivision: 'D1',
      currentPoints: 45,
      previousRank: 2,
      rankTrend: 'up',
      newHighlights: 5,
      byPosition: {
        first: { matches: 80, points: 840, winRate: 72 },
        second: { matches: 70, points: 735, winRate: 68 },
      },
      byCompetition: {},
      bySeason: {},
    },
    trophies: { total: 12, list: [] },
    badges: { total: 8, categories: [] },
    records: { total: 3, list: [] },
    currentTeams: [],
    pastTeams: [],
    frequentTeammates: [],
    matchHistory: { wins: 105, losses: 25, draws: 20, total: 150 },
    competitionStats: [],
    recentActivities: [],
    preferences: {
      language: 'fr',
      timezone: 'Europe/Paris',
      distanceUnit: 'metric',
      notifications: {
        email: { results: true, news: true, reminders: true },
        push: { results: true, matches: true },
      },
    },
    social: { facebook: null, twitter: null, instagram: null },
    lastActiveAt: new Date('2026-10-08'),
    // createdAt: new Date('2019-03-10'),
    // updatedAt: new Date('2026-10-09'),
  },
  {
    id: 'player-002',
    firstName: 'Pierre',
    lastName: 'Durand',
    fullName: 'Pierre Durand',
    alias: 'Le Rocket',
    email: 'pierre.durand@palet.fr',
    photo: '/images/players/pierre-durand.jpg',
    status: 'active',
    birthDate: new Date('1990-07-22'),
    age: 36,
    birthPlace: 'La Roche-sur-Yon',
    nationality: 'Français',
    dominantHand: 'left',
    playStyle: 'Puissance',
    phone: '+33 6 23 45 67 89',
    address: '456 Rue de la Plaque, 85000 La Roche-sur-Yon',
    city: 'La Roche-sur-Yon',
    postalCode: '85000',
    club: mockClub,
    clubId: 'club-001',
    clubNumber: 'ESS-002',
    category: 'Senior',
    licenseNumber: 'PV-2026-002',
    licenseDate: new Date('2026-01-01'),
    licenseStatus: 'valid',
    registrationDate: new Date('2019-03-10'),
    stats: {
      matchesPlayed: 120,
      wins: 85,
      draws: 15,
      losses: 20,
      totalPoints: 1275,
      averagePoints: 10.625,
      pointsPerManche: 3.53,
      bestScore: 14,
      bestStreak: 12,
      maxWinStreak: 7,
      winRate: 70.83,
      accuracy: 82,
      currentRank: 3,
      currentDivision: 'D1',
      currentPoints: 38,
      previousRank: 5,
      rankTrend: 'up',
      newHighlights: 3,
      byPosition: {
        first: { matches: 60, points: 637, winRate: 71 },
        second: { matches: 60, points: 638, winRate: 70.67 },
      },
      byCompetition: {},
      bySeason: {},
    },
    trophies: { total: 8, list: [] },
    badges: { total: 6, categories: [] },
    records: { total: 2, list: [] },
    currentTeams: [],
    pastTeams: [],
    frequentTeammates: [],
    matchHistory: { wins: 85, losses: 20, draws: 15, total: 120 },
    competitionStats: [],
    recentActivities: [],
    preferences: {
      language: 'fr',
      timezone: 'Europe/Paris',
      distanceUnit: 'metric',
      notifications: {
        email: { results: true, news: true, reminders: true },
        push: { results: true, matches: true },
      },
    },
    social: { facebook: null, twitter: null, instagram: null },
    lastActiveAt: new Date('2026-10-07'),
    createdAt: new Date('2021-02-15'),
    updatedAt: new Date('2026-10-09'),
  },
  {
    id: 'player-003',
    firstName: 'Marie',
    lastName: 'Dupont',
    fullName: 'Marie Dupont',
    alias: 'La Précise',
    email: 'marie.dupont@palet.fr',
    photo: '/images/players/marie-dupont.jpg',
    status: 'active',
    birthDate: new Date('1995-11-03'),
    age: 30,
    birthPlace: 'Cholet',
    nationality: 'Française',
    dominantHand: 'right',
    playStyle: 'Précision',
    phone: '+33 6 34 56 78 90',
    address: '789 Rue du Jeu, 85140 Les Essarts',
    city: 'Les Essarts',
    postalCode: '85140',
    club: mockClub,
    clubId: 'club-001',
    clubNumber: 'ESS-003',
    category: 'Senior',
    licenseNumber: 'PV-2026-003',
    licenseDate: new Date('2026-01-01'),
    licenseStatus: 'valid',
    registrationDate: new Date('2021-02-15'),
    stats: {
      matchesPlayed: 90,
      wins: 65,
      draws: 10,
      losses: 15,
      totalPoints: 975,
      averagePoints: 10.83,
      pointsPerManche: 3.6,
      bestScore: 15,
      bestStreak: 10,
      maxWinStreak: 6,
      winRate: 72.22,
      accuracy: 88,
      currentRank: 2,
      currentDivision: 'D1',
      currentPoints: 42,
      previousRank: 1,
      rankTrend: 'down',
      newHighlights: 4,
      byPosition: {
        first: { matches: 45, points: 487, winRate: 73 },
        second: { matches: 45, points: 488, winRate: 71.43 },
      },
      byCompetition: {},
      bySeason: {},
    },
    trophies: { total: 6, list: [] },
    badges: { total: 5, categories: [] },
    records: { total: 1, list: [] },
    currentTeams: [],
    pastTeams: [],
    frequentTeammates: [],
    matchHistory: { wins: 65, losses: 15, draws: 10, total: 90 },
    competitionStats: [],
    recentActivities: [],
    preferences: {
      language: 'fr',
      timezone: 'Europe/Paris',
      distanceUnit: 'metric',
      notifications: {
        email: { results: true, news: true, reminders: true },
        push: { results: true, matches: true },
      },
    },
    social: { facebook: null, twitter: null, instagram: null },
    lastActiveAt: new Date('2026-10-09'),
    createdAt: new Date('2021-02-15'),
    updatedAt: new Date('2026-10-09'),
  },
];

const mockTeams: Team[] = [
  {
    id: 'team-001',
    name: 'Équipe 1 - D1',
    club: mockClub,
    clubId: 'club-001',
    captain: mockPlayers[0]!,
    captainId: 'player-001',
    players: [mockPlayers[0]!, mockPlayers[1]!],
    logo: null,
    stats: {
      matches: 20,
      wins: 15,
      draws: 3,
      losses: 2,
      pointsFor: 180,
      pointsAgainst: 120,
      diff: 60,
      points: 48,
      winRate: 75,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date('2025-09-01'),
    updatedAt: new Date('2026-10-01'),
  },
  {
    id: 'team-002',
    name: 'Équipe 2 - D1',
    club: mockClub,
    clubId: 'club-001',
    captain: mockPlayers[2]!,
    captainId: 'player-003',
    players: [mockPlayers[2]!, mockPlayers[0]!],
    logo: null,
    stats: {
      matches: 20,
      wins: 12,
      draws: 5,
      losses: 3,
      pointsFor: 170,
      pointsAgainst: 140,
      diff: 30,
      points: 41,
      winRate: 60,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date('2025-09-01'),
    updatedAt: new Date('2026-10-01'),
  },
];

const mockClassification: ChampionshipClassification = {
  division: 'D1',
  journee: 5,
  updatedAt: new Date('2026-10-08'),
  teams: [
    {
      position: 1,
      team: mockTeams[0]!,
      stats: mockTeams[0]!.stats,
      form: ['win', 'win', 'win', 'win', 'draw'],
      status: 'qualified',
    },
    {
      position: 2,
      team: mockTeams[1]!,
      stats: mockTeams[1]!.stats,
      form: ['win', 'win', 'draw', 'win', 'loss'],
      status: 'qualified',
    },
  ],
};

const ClubDetail: React.FC = () => {
  const { clubId } = useParams<{ clubId: string }>();

  const { data: club, isLoading } = useQuery<Club>({
    queryKey: ['club', clubId],
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
            <p className="text-body-m text-primary-200">Chargement...</p>
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
                  src={displayClub.logo || '/images/clubs/default.png'}
                  alt={displayClub.name}
                  className="w-24 h-24 rounded-full object-cover border-4 border-gold-700"
                />
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayClub.name}
                  </h1>
                  {displayClub.nickname && (
                    <span className="text-heading-m text-primary-300">
                      ({displayClub.nickname})
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant={displayClub.division === 'D1' ? 'gold' : 'primary'}>
                    {displayClub.division}
                  </Badge>
                  <Badge variant={displayClub.status === 'active' ? 'success' : 'warning'}>
                    {displayClub.status === 'active' ? 'Actif' : 'Inactif'}
                  </Badge>
                  <Badge variant="default">{displayClub.league}</Badge>
                </div>

                <p className="text-body-m text-primary-200 mb-4 max-w-2xl">
                  {displayClub.address}, {displayClub.postalCode} {displayClub.city}
                </p>

                {/* Contact Info */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  {displayClub.contactEmail && (
                    <div className="flex items-center gap-2">
                      <span className="text-primary-400">Email:</span>
                      <a
                        href={`mailto:${displayClub.contactEmail}`}
                        className="text-gold-700 hover:underline text-body-s"
                      >
                        {displayClub.contactEmail}
                      </a>
                    </div>
                  )}
                  {displayClub.contactPhone && (
                    <div className="flex items-center gap-2">
                      <span className="text-primary-400">Téléphone:</span>
                      <a
                        href={`tel:${displayClub.contactPhone}`}
                        className="text-gold-700 hover:underline text-body-s"
                      >
                        {displayClub.contactPhone}
                      </a>
                    </div>
                  )}
                  {displayClub.website && (
                    <div className="flex items-center gap-2">
                      <span className="text-primary-400">Site web:</span>
                      <a
                        href={displayClub.website}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-gold-700 hover:underline text-body-s"
                      >
                        {displayClub.website.replace(/^https?:\/\//, '')}
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
                      className="text-primary-400 hover:text-gold-700"
                    >
                      Facebook
                    </a>
                  )}
                  {displayClub.social.twitter && (
                    <a
                      href={displayClub.social.twitter}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-primary-400 hover:text-gold-700"
                    >
                      Twitter
                    </a>
                  )}
                  {displayClub.social.instagram && (
                    <a
                      href={displayClub.social.instagram}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-primary-400 hover:text-gold-700"
                    >
                      Instagram
                    </a>
                  )}
                </div>
              </div>

              {/* Code */}
              <div className="flex-shrink-0">
                <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                  <p className="text-body-xs text-primary-400 mb-1">Code Club</p>
                  <p className="text-heading-l font-bold text-gold-700">{displayClub.code}</p>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Stats Overview */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              Statistiques du Club
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Joueurs</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayClub.playersCount}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Équipes</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayClub.teamsCount}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Division</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayClub.division}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Ligue</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayClub.league}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Fondé en</p>
                  <p className="text-heading-m font-bold text-gold-700">
                    {new Date(displayClub.createdAt).getFullYear()}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Statut</p>
                  <Badge variant={displayClub.status === 'active' ? 'success' : 'warning'}>
                    {displayClub.status === 'active' ? 'Actif' : 'Inactif'}
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
                <Tab value="effectif" label="Effectif ({displayClub.playersCount})" />
                <Tab value="equipes" label="Équipes ({displayClub.teamsCount})" />
                <Tab value="classement" label="Classement" />
                <Tab value="actualites" label="Actualités" />
              </TabList>

              {/* Effectif */}
              <TabPanel value="effectif" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {mockPlayers.map((player) => (
                    <PlayerCard
                      key={player.id}
                      player={player}
                      showClub={false}
                      
                    />
                  ))}
                </div>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {mockTeams.map((team) => (
                    <Card key={team.id} className="bg-primary-900/50 border-primary-700/50">
                      <CardHeader>
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-3">
                            <TeamAvatar team={{ id: team.id, name: team.name, logo: team.logo ?? undefined, abbreviation: (team.club as any)?.code }} size="m" />
                            <div>
                              <h3 className="text-heading-s font-bold text-white">{team.name}</h3>
                              <p className="text-body-xs text-primary-300">
                                Capitaine: {team.captain?.fullName}
                              </p>
                            </div>
                          </div>
                          <Badge variant="default">{team.stats.wins}V - {team.stats.losses}D</Badge>
                        </div>
                      </CardHeader>
                      <CardContent>
                        <div className="grid grid-cols-3 gap-4">
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300">Matchs</p>
                            <p className="text-heading-s font-bold text-gold-700">{team.stats.matches}</p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300">Victoires</p>
                            <p className="text-heading-s font-bold text-gold-700">{team.stats.wins}</p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300">Win Rate</p>
                            <p className="text-heading-s font-bold text-gold-700">
                              {team.stats.winRate}%
                            </p>
                          </div>
                        </div>
                        <Link to={`/teams/${team.id}`} className="inline-block mt-4">
                          <Button variant="outline" size="sm">
                            Voir l'équipe
                          </Button>
                        </Link>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </TabPanel>

              {/* Classement */}
              <TabPanel value="classement" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">
                      Classement {mockClassification.division} - Journée {mockClassification.journee}
                    </h3>
                  </CardHeader>
                  <CardContent>
                    <div className="overflow-x-auto">
                      <table className="w-full text-body-s">
                        <thead>
                          <tr className="border-b border-primary-700/50 text-primary-300">
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
                              className="border-b border-primary-700/20 hover:bg-primary-800/50"
                            >
                              <td className="p-3">
                                <Badge variant={team.position === 1 ? 'gold' : 'default'}>
                                  {team.position}
                                </Badge>
                              </td>
                              <td className="p-3">
                                <div className="flex items-center gap-2">
                                  <TeamAvatar team={{ id: team.team.id, name: team.team.name, logo: team.team.logo ?? undefined, abbreviation: (team.team.club as any)?.code }} size="xs" />
                                  <span className="text-white">{team.team.name}</span>
                                </div>
                              </td>
                              <td className="p-3 text-primary-300">{team.stats.matches}</td>
                              <td className="p-3 text-primary-300">{team.stats.wins}</td>
                              <td className="p-3 text-primary-300">{team.stats.losses}</td>
                              <td className="p-3 text-gold-700 font-bold">{team.stats.points}</td>
                              <td className="p-3">
                                <div className="flex gap-1">
                                  {team.form.map((result, i) => (
                                    <span
                                      key={i}
                                      className={`px-2 py-1 rounded text-xs ${
                                        result === 'win'
                                          ? 'bg-green-900/50 text-green-400'
                                          : result === 'draw'
                                          ? 'bg-amber-900/50 text-amber-400'
                                          : 'bg-red-900/50 text-red-400'
                                      }`}
                                    >
                                      {result === 'win' ? 'V' : result === 'draw' ? 'N' : 'D'}
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
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardContent className="text-center p-8">
                    <p className="text-body-m text-primary-300">
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
