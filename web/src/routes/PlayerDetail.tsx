import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, UserAvatar } from '../components';
import { PlayerCard, ClubCard } from '../components/domain';
import { Player, Club, Match, Trophy, Badge as BadgeType } from '../types';
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

const mockPlayer: Player = {
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
    bySeason: {
      '2025-2026': {
        matches: 30,
        wins: 22,
        draws: 4,
        losses: 4,
        totalPoints: 330,
        averagePoints: 11,
        pointsPerManche: 3.6,
        winRate: 73.33,
        bestScore: 15,
        bestStreak: 6,
        rank: 1,
        division: 'D1',
      },
      '2024-2025': {
        matches: 40,
        wins: 28,
        draws: 6,
        losses: 6,
        totalPoints: 420,
        averagePoints: 10.5,
        pointsPerManche: 3.5,
        winRate: 70,
        bestScore: 14,
        bestStreak: 8,
        rank: 2,
        division: 'D1',
      },
    },
  },
  trophies: {
    total: 12,
    list: [
      {
        id: 'trophy-001',
        name: 'Champion de Vendée',
        competition: 'Championnat D1',
        category: 'champion',
        year: '2025',
        date: new Date('2025-06-15'),
        type: 'champion',
      },
      {
        id: 'trophy-002',
        name: 'Vainqueur Coupe de France',
        competition: 'Coupe de France',
        category: 'winner',
        year: '2024',
        date: new Date('2024-11-20'),
        type: 'winner',
      },
      {
        id: 'trophy-003',
        name: 'Finaliste Coupe de France',
        competition: 'Coupe de France',
        category: 'finalist',
        year: '2023',
        date: new Date('2023-11-18'),
        type: 'finalist',
      },
    ],
  },
  badges: {
    total: 8,
    categories: [
      {
        id: 'badge-cat-001',
        name: 'Performance',
        badges: [
          {
            id: 'badge-001',
            name: '10 victoires consécutives',
            icon: null,
            description: '10 victoires d\'affilée en championnat',
            rarity: 'gold',
            date: new Date('2025-04-15'),
          },
          {
            id: 'badge-002',
            name: 'Meilleur buteur',
            icon: null,
            description: 'Meilleur buteur de la saison',
            rarity: 'gold',
            date: new Date('2025-06-15'),
          },
        ],
      },
      {
        id: 'badge-cat-002',
        name: 'Participation',
        badges: [
          {
            id: 'badge-003',
            name: '5 ans de fidélité',
            icon: null,
            description: '5 ans au club',
            rarity: 'platinum',
            date: new Date('2025-01-15'),
          },
        ],
      },
    ],
  },
  records: {
    total: 3,
    list: [],
  },
  currentTeams: [],
  pastTeams: [],
  frequentTeammates: [],
  matchHistory: {
    wins: 105,
    losses: 25,
    draws: 20,
    total: 150,
    byCompetition: {
      'Championnat D1': { wins: 35, losses: 8, draws: 7, total: 50 },
      'Coupe de France': { wins: 7, losses: 1, draws: 2, total: 10 },
    },
  },
  competitionStats: [],
  recentActivities: [
    {
      id: 'activity-001',
      type: 'match',
      title: 'Victoire contre Team B',
      message: '15-10 dans le Championnat D1',
      userId: 'player-001',
      timestamp: new Date('2026-10-08T14:00:00'),
      data: { score: '15-10', competition: 'Championnat D1' },
    },
    {
      id: 'activity-002',
      type: 'achievement',
      title: 'Nouveau record',
      message: '15 points en une manche',
      userId: 'player-001',
      timestamp: new Date('2026-10-07T11:00:00'),
      data: { record: '15 points' },
    },
    {
      id: 'activity-003',
      type: 'tournament',
      title: 'Inscription au tournoi',
      message: 'Inscription confirmée pour la Coupe de France 2026',
      userId: 'player-001',
      timestamp: new Date('2026-10-06T09:00:00'),
      data: { tournament: 'Coupe de France 2026' },
    },
  ],
  preferences: {
    language: 'fr',
    timezone: 'Europe/Paris',
    distanceUnit: 'metric',
    notifications: {
      email: { results: true, news: true, reminders: true },
      push: { results: true, matches: true },
    },
  },
  social: {
    facebook: 'https://facebook.com/jean.martin.palet',
    twitter: 'https://twitter.com/jeanmartin85',
    instagram: 'https://instagram.com/jeanmartin_palet',
  },
  lastActiveAt: new Date('2026-10-09'),
  createdAt: new Date('2020-01-15'),
  updatedAt: new Date('2026-10-09'),
};

const mockMatches: Match[] = [
  {
    id: 'match-001',
    competitionId: 'comp-001',
    competition: {
      id: 'comp-001',
      name: 'Championnat D1',
      type: 'championship',
      category: 'doublette',
      material: 'fonte',
      season: '2026-2027',
    } as any,
    type: 'championship',
    journee: 5,
    phase: 'Phase 1',
    round: 'Round 1',
    team1: {
      id: 'team-001',
      name: "Jean Martin - Pierre Durand",
      logo: null,
      players: [mockPlayer],
      rank: 1,
      seed: 1,
    },
    team2: {
      id: 'team-002',
      name: "Team B",
      logo: null,
      players: [],
      rank: 2,
      seed: 2,
    },
    team1Score: 15,
    team2Score: 10,
    winner: 'team1',
    manches: [
      { number: 1, team1: 8, team2: 5, winner: 'team1' },
      { number: 2, team1: 7, team2: 5, winner: 'team1' },
    ],
    bestOf: 3,
    currentManche: null,
    currentScores: null,
    status: 'completed',
    verified: true,
    verifiedBy: 'arbitre-001',
    verifiedAt: new Date('2026-10-08T14:30:00'),
    date: new Date('2026-10-08T14:00:00'),
    time: '14:00',
    location: 'Salle de Palet - Les Essarts',
    duration: '1h',
    canEdit: false,
    isLive: false,
    isRecent: true,
    stats: {
      totalManches: 2,
      avgPointsPerManche: 7.5,
      bestManche: { number: 2, team1: 7, team2: 5, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-08T14:00:00'),
    endedAt: new Date('2026-10-08T14:30:00'),
    createdAt: new Date('2026-10-01'),
    updatedAt: new Date('2026-10-08T14:30:00'),
  },
  {
    id: 'match-002',
    competitionId: 'comp-001',
    competition: {
      id: 'comp-001',
      name: 'Championnat D1',
      type: 'championship',
      category: 'doublette',
      material: 'fonte',
      season: '2026-2027',
    } as any,
    type: 'championship',
    journee: 4,
    phase: 'Phase 1',
    round: 'Round 1',
    team1: {
      id: 'team-001',
      name: "Jean Martin - Pierre Durand",
      logo: null,
      players: [mockPlayer],
      rank: 1,
      seed: 1,
    },
    team2: {
      id: 'team-003',
      name: "Team C",
      logo: null,
      players: [],
      rank: 3,
      seed: 3,
    },
    team1Score: 15,
    team2Score: 12,
    winner: 'team1',
    manches: [
      { number: 1, team1: 9, team2: 6, winner: 'team1' },
      { number: 2, team1: 6, team2: 6, winner: null },
      { number: 3, team1: 6, team2: 3, winner: 'team1' },
    ],
    bestOf: 3,
    currentManche: null,
    currentScores: null,
    status: 'completed',
    verified: true,
    verifiedBy: 'arbitre-001',
    verifiedAt: new Date('2026-10-01T10:30:00'),
    date: new Date('2026-10-01T10:00:00'),
    time: '10:00',
    location: 'Salle de Palet - Les Essarts',
    duration: '1h15',
    canEdit: false,
    isLive: false,
    isRecent: true,
    stats: {
      totalManches: 3,
      avgPointsPerManche: 6.67,
      bestManche: { number: 1, team1: 9, team2: 6, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-01T10:00:00'),
    endedAt: new Date('2026-10-01T10:30:00'),
    createdAt: new Date('2026-09-25'),
    updatedAt: new Date('2026-10-01T10:30:00'),
  },
];

const PlayerDetail: React.FC = () => {
  const { playerId } = useParams<{ playerId: string }>();

  const { data: player, isLoading } = useQuery<Player>({
    queryKey: ['player', playerId],
    queryFn: async () => {
      const response = await api().GET<Player>(`/api/players/${playerId}`);
      return response.data;
    },
    enabled: !!playerId,
  });

  // For development, use mock data if no real data
  const displayPlayer = player || mockPlayer;

  if (isLoading && !player) {
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
              <Link to="/joueurs">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux joueurs
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Photo */}
              <div className="flex-shrink-0">
                <UserAvatar
                  user={{
                    id: displayPlayer.id,
                    name: displayPlayer.fullName,
                    photo: displayPlayer.photo ?? undefined,
                  }}
                  size="xl"
                  className="border-4 border-gold-700"
                />
                {displayPlayer.club && (
                  <div className="mt-4 text-center">
                    <ClubCard club={displayPlayer.club} compact={true} />
                  </div>
                )}
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayPlayer.fullName}
                  </h1>
                  {displayPlayer.alias && (
                    <span className="text-heading-m text-primary-300">
                      ({displayPlayer.alias})
                    </span>
                  )}
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="gold">N°{displayPlayer.clubNumber}</Badge>
                  <Badge variant={displayPlayer.licenseStatus === 'valid' ? 'success' : 'warning'}>
                    {displayPlayer.licenseStatus === 'valid' ? 'Licence Valide' : displayPlayer.licenseStatus}
                  </Badge>
                  <Badge variant="default">{displayPlayer.category}</Badge>
                  <Badge variant="default">{displayPlayer.dominantHand === 'right' ? 'Droitier' : displayPlayer.dominantHand === 'left' ? 'Gaucher' : 'Ambidextre'}</Badge>
                </div>

                <p className="text-body-m text-primary-200 mb-4 max-w-2xl">
                  Né le {new Date(displayPlayer.birthDate).toLocaleDateString('fr-FR')} à {displayPlayer.birthPlace} ({displayPlayer.age} ans)
                </p>

                {/* Stats Summary */}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Matchs Joués</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayPlayer.stats.matchesPlayed}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Victoires</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayPlayer.stats.wins}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Taux de Victoire</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayPlayer.stats.winRate}%</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Points Moyens</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayPlayer.stats.averagePoints.toFixed(1)}</p>
                  </div>
                </div>

                {/* Contact & Social */}
                <div className="flex flex-col md:flex-row gap-4">
                  <div className="flex items-center gap-4">
                    {displayPlayer.email && (
                      <div className="flex items-center gap-2">
                        <span className="text-primary-400">Email:</span>
                        <a href={`mailto:${displayPlayer.email}`} className="text-gold-700 hover:underline text-body-s">
                          {displayPlayer.email}
                        </a>
                      </div>
                    )}
                    {displayPlayer.phone && (
                      <div className="flex items-center gap-2">
                        <span className="text-primary-400">Téléphone:</span>
                        <a href={`tel:${displayPlayer.phone}`} className="text-gold-700 hover:underline text-body-s">
                          {displayPlayer.phone}
                        </a>
                      </div>
                    )}
                  </div>
                  <div className="flex items-center gap-4">
                    {displayPlayer.social.facebook && (
                      <a
                        href={displayPlayer.social.facebook}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary-400 hover:text-gold-700"
                      >
                        Facebook
                      </a>
                    )}
                    {displayPlayer.social.twitter && (
                      <a
                        href={displayPlayer.social.twitter}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary-400 hover:text-gold-700"
                      >
                        Twitter
                      </a>
                    )}
                    {displayPlayer.social.instagram && (
                      <a
                        href={displayPlayer.social.instagram}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary-400 hover:text-gold-700"
                      >
                        Instagram
                      </a>
                    )}
                  </div>
                </div>
              </div>

              {/* Rank */}
              <div className="flex-shrink-0">
                <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                  <p className="text-body-xs text-primary-400 mb-1">Classement Actuel</p>
                  <div className="flex items-center justify-center gap-2">
                    <span className="text-heading-xl font-bold text-gold-700">{displayPlayer.stats.currentRank}</span>
                    {displayPlayer.stats.rankTrend === 'up' && (
                      <span className="text-green-400">↑</span>
                    )}
                    {displayPlayer.stats.rankTrend === 'down' && (
                      <span className="text-red-400">↓</span>
                    )}
                  </div>
                  <p className="text-body-xs text-primary-300 mt-1">
                    {displayPlayer.stats.currentDivision}
                  </p>
                  <p className="text-body-xs text-primary-400 mt-2">
                    {displayPlayer.stats.currentPoints} pts
                  </p>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Progress Bar - Win Rate */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              Statistiques Détaillées
            </h2>

            {/* Win Rate Progress */}
            <div className="mb-6">
              <div className="flex justify-between items-center mb-2">
                <span className="text-body-s text-primary-300">Taux de Victoire: {displayPlayer.stats.winRate}%</span>
                <span className="text-body-s text-gold-700">{displayPlayer.stats.wins}V - {displayPlayer.stats.draws}N - {displayPlayer.stats.losses}D</span>
              </div>
              <div className="h-4 bg-primary-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gold-700 rounded-full"
                  style={{ width: `${displayPlayer.stats.winRate}%` }}
                />
              </div>
            </div>

            {/* Accuracy Progress */}
            <div className="mb-6">
              <div className="flex justify-between items-center mb-2">
                <span className="text-body-s text-primary-300">Précision: {displayPlayer.stats.accuracy}%</span>
                <span className="text-body-s text-gold-700">Meilleur score: {displayPlayer.stats.bestScore}</span>
              </div>
              <div className="h-4 bg-primary-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-green-500 rounded-full"
                  style={{ width: `${displayPlayer.stats.accuracy}%` }}
                />
              </div>
            </div>

            {/* Stats Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Meilleure Série</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayPlayer.stats.bestStreak}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Points/Manche</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayPlayer.stats.pointsPerManche.toFixed(1)}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Total Points</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayPlayer.stats.totalPoints}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Matchs Nuls</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayPlayer.stats.draws}</p>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="statistiques">
              <TabList>
                <Tab value="statistiques" label="Statistiques" />
                <Tab value="historique" label="Historique des Matchs" />
                <Tab value="trophees" label="Trophées ({displayPlayer.trophies.total})" />
                <Tab value="equipes" label="Équipes" />
                <Tab value="records" label="Records ({displayPlayer.records.total})" />
              </TabList>

              {/* Statistiques */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {/* Par Compétition */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Par Compétition</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3">
                        {Object.entries(displayPlayer.stats.byCompetition).map(([compName, stats]) => (
                          <div key={compName} className="bg-primary-800/50 rounded p-3">
                            <p className="text-body-s font-semibold text-gold-700">{compName}</p>
                            <div className="grid grid-cols-3 gap-2 mt-2 text-body-xs">
                              <div>
                                <p className="text-primary-300">Points: {stats.totalPoints}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Victoires: {(stats as any).wins || 'N/A'}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Win Rate: {(stats as any).winRate || 'N/A'}%</p>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>

                  {/* Par Saison */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Par Saison</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3">
                        {Object.entries(displayPlayer.stats.bySeason).map(([season, stats]) => (
                          <div key={season} className="bg-primary-800/50 rounded p-3">
                            <p className="text-body-s font-semibold text-gold-700">{season}</p>
                            <div className="grid grid-cols-3 gap-2 mt-2 text-body-xs">
                              <div>
                                <p className="text-primary-300">Classement: {stats.rank}°</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Victoires: {stats.wins}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Points: {stats.totalPoints}</p>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>

                  {/* Par Position */}
                  <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-2">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Par Position</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {Object.entries(displayPlayer.stats.byPosition).map(([position, stats]) => (
                          <div key={position} className="bg-primary-800/50 rounded p-3">
                            <p className="text-body-s font-semibold text-gold-700">
                              Position: {position}
                            </p>
                            <div className="grid grid-cols-3 gap-2 mt-2 text-body-xs">
                              <div>
                                <p className="text-primary-300">Matchs: {stats.matches}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Points: {stats.points}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">Win Rate: {stats.winRate}%</p>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>
                </div>
              </TabPanel>

              {/* Historique des Matchs */}
              <TabPanel value="historique" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">
                      Derniers Matchs ({mockMatches.length})
                    </h3>
                  </CardHeader>
                  <CardContent>
                    <div className="space-y-4">
                      {mockMatches.map((match) => (
                        <div
                          key={match.id}
                          className="bg-primary-800/50 rounded p-4 flex items-center justify-between"
                        >
                          <div className="flex-1">
                            <div className="flex items-center gap-2 mb-1">
                              <span className="text-body-s font-semibold text-white">
                                {match.competition.name}
                              </span>
                              {match.status === 'completed' && (
                                <Badge variant="success">Terminé</Badge>
                              )}
                              {match.status === 'in_progress' && (
                                <Badge variant="warning">En cours</Badge>
                              )}
                              {match.status === 'scheduled' && (
                                <Badge variant="default">À venir</Badge>
                              )}
                            </div>
                            <p className="text-body-xs text-primary-300">
                              Journée {match.journee} - {match.date.toLocaleDateString('fr-FR')}
                            </p>
                            <div className="flex items-center gap-2 mt-2">
                              <span className="text-body-s text-white">{match.team1.name}</span>
                              <span className="text-body-s text-gold-700 font-bold">
                                {match.team1Score} - {match.team2Score}
                              </span>
                              <span className="text-body-s text-white">{match.team2.name}</span>
                            </div>
                          </div>
                          <div className="flex-shrink-0">
                            {match.winner === 'team1' && match.team1.players.some(p => p.id === displayPlayer.id) && (
                              <Badge variant="success">Victoire</Badge>
                            )}
                            {match.winner === 'team2' && match.team2.players.some(p => p.id === displayPlayer.id) && (
                              <Badge variant="danger">Défaite</Badge>
                            )}
                            {match.winner === null && (
                              <Badge variant="default">Match nul</Badge>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Trophées */}
              <TabPanel value="trophees" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {displayPlayer.trophies.list.map((trophy) => (
                    <Card
                      key={trophy.id}
                      className="bg-primary-900/50 border-primary-700/50"
                    >
                      <CardContent className="text-center p-6">
                        <div className="mb-4">
                          <span className="text-4xl">🏆</span>
                        </div>
                        <h3 className="text-heading-s font-bold text-gold-700 mb-2">
                          {trophy.name}
                        </h3>
                        <p className="text-body-xs text-primary-300 mb-1">
                          {trophy.competition}
                        </p>
                        <p className="text-body-xs text-primary-400 mb-1">
                          {trophy.type === 'champion' && 'Champion'}
                          {trophy.type === 'winner' && 'Vainqueur'}
                          {trophy.type === 'finalist' && 'Finaliste'}
                          {trophy.type === 'semifinalist' && 'Demi-finaliste'}
                        </p>
                        <p className="text-body-xs text-primary-300">
                          Saison {trophy.year}
                        </p>
                      </CardContent>
                    </Card>
                  ))}
                  {displayPlayer.trophies.list.length === 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-3">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-primary-300">
                          Aucun trophée pour le moment.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>
              </TabPanel>

              {/* Équipes */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* Équipes actuelles */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Équipes Actuelles ({displayPlayer.currentTeams.length})
                      </h3>
                    </CardHeader>
                    <CardContent>
                      {displayPlayer.currentTeams.length > 0 ? (
                        <div className="space-y-3">
                          {displayPlayer.currentTeams.map((team) => (
                            <div
                              key={team.id}
                              className="bg-primary-800/50 rounded p-3 flex items-center justify-between"
                            >
                              <div>
                                <p className="text-body-s font-semibold text-white">{team.name}</p>
                                <p className="text-body-xs text-primary-300">
                                  {team.stats.wins}V - {team.stats.losses}D
                                </p>
                              </div>
                              <Badge variant="success">Actif</Badge>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <p className="text-body-s text-primary-300">
                          Aucune équipe actuelle.
                        </p>
                      )}
                    </CardContent>
                  </Card>

                  {/* Équipes passées */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Équipes Passées ({displayPlayer.pastTeams.length})
                      </h3>
                    </CardHeader>
                    <CardContent>
                      {displayPlayer.pastTeams.length > 0 ? (
                        <div className="space-y-3">
                          {displayPlayer.pastTeams.map((team) => (
                            <div
                              key={team.id}
                              className="bg-primary-800/50 rounded p-3 flex items-center justify-between"
                            >
                              <div>
                                <p className="text-body-s font-semibold text-white">{team.name}</p>
                                <p className="text-body-xs text-primary-300">
                                  Saison: {team.season}
                                </p>
                                <p className="text-body-xs text-primary-300">
                                  {team.stats.wins}V - {team.stats.losses}D
                                </p>
                              </div>
                              <Badge variant="default">Passé</Badge>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <p className="text-body-s text-primary-300">
                          Aucune équipe passée.
                        </p>
                      )}
                    </CardContent>
                  </Card>
                </div>

                {/* Coéquipiers fréquents */}
                {displayPlayer.frequentTeammates.length > 0 && (
                  <Card className="bg-primary-900/50 border-primary-700/50 mt-4">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Coéquipiers Fréquents
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                        {displayPlayer.frequentTeammates.map((teammate, index) => (
                          <div
                            key={index}
                            className="bg-primary-800/50 rounded p-3 flex items-center gap-3"
                          >
                            <UserAvatar
                              user={{ id: teammate.player.id, name: teammate.player.fullName, photo: teammate.player.photo || undefined }}
                              size="sm"
                            />
                            <div className="flex-1">
                              <p className="text-body-s font-semibold text-white">
                                {teammate.player.fullName}
                              </p>
                              <p className="text-body-xs text-primary-300">
                                {teammate.count} matchs ensemble
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300">Win Rate</p>
                              <p className="text-body-s font-bold text-gold-700">
                                {teammate.winRate}%
                              </p>
                            </div>
                          </div>
                        ))}
                      </div>
                    </CardContent>
                  </Card>
                )}
              </TabPanel>

              {/* Records */}
              <TabPanel value="records" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {displayPlayer.records.list.map((record) => (
                    <Card
                      key={record.id}
                      className="bg-primary-900/50 border-primary-700/50"
                    >
                      <CardContent className="p-4">
                        <div className="flex items-start gap-3">
                          <div className="flex-shrink-0">
                            <span className="text-2xl">🎯</span>
                          </div>
                          <div className="flex-1">
                            <h3 className="text-heading-s font-bold text-gold-700 mb-1">
                              {record.type}
                            </h3>
                            <p className="text-body-s text-white mb-2">{record.value}</p>
                            <p className="text-body-xs text-primary-300 mb-1">
                              {record.tournament}
                            </p>
                            <p className="text-body-xs text-primary-400">
                              {new Date(record.date).toLocaleDateString('fr-FR')}
                            </p>
                          </div>
                          <div className="flex-shrink-0">
                            {record.verified && <Badge variant="success">Vérifié</Badge>}
                          </div>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
                  {displayPlayer.records.list.length === 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-3">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-primary-300">
                          Aucun record pour le moment.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>
              </TabPanel>
            </Tabs>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default PlayerDetail;
