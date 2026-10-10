import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, UserAvatar, TeamAvatar } from '../components';
import { PlayerCard, ClubCard, MatchCard, TournamentCard } from '../components/domain';
import { Team, Player, Match, Club, Tournament } from '../types';
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
    lastActiveAt: new Date('2026-10-09'),
    createdAt: new Date('2020-01-15'),
    updatedAt: new Date('2026-10-09'),
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
    createdAt: new Date('2019-03-10'),
    updatedAt: new Date('2026-10-09'),
  },
];

const mockTeam: Team = {
  id: 'team-001',
  name: 'Équipe 1 - D1',
  club: mockClub,
  clubId: 'club-001',
  captain: mockPlayers[0]!,
  captainId: 'player-001',
  players: mockPlayers,
  logo: null,
  stats: {
    matches: 20,
    wins: 15,
    draws: 3,
    losses: 2,
    pointsFor: 225,
    pointsAgainst: 165,
    diff: 60,
    points: 48,
    winRate: 75,
  },
  status: 'active',
  competitions: [
    {
      id: 'comp-001',
      competition: {
        id: 'championship-001',
        name: 'Championnat de Vendée D1 2026-2027',
        code: 'D1-2026-2027',
        type: 'championship',
        category: 'doublette',
        material: 'fonte',
        season: '2026-2027',
      } as any,
      competitionId: 'championship-001',
      season: '2026-2027',
      status: 'in_progress',
      results: {
        matches: 10,
        wins: 8,
        draws: 1,
        losses: 1,
        pointsFor: 112,
        pointsAgainst: 90,
        position: 1,
        points: 25,
      },
    },
    {
      id: 'comp-002',
      competition: {
        id: 'tournament-001',
        name: 'Coupe de France 2026',
        code: 'CDF-2026',
        type: 'cup',
        category: 'doublette',
        material: 'fonte',
        season: '2026',
      } as any,
      competitionId: 'tournament-001',
      season: '2026',
      status: 'completed',
      results: {
        matches: 5,
        wins: 4,
        draws: 0,
        losses: 1,
        pointsFor: 50,
        pointsAgainst: 35,
        position: 1,
        points: 12,
      },
    },
  ],
  createdAt: new Date('2025-09-01'),
  updatedAt: new Date('2026-10-01'),
};

const mockMatches: Match[] = [
  {
    id: 'match-001',
    competitionId: 'championship-001',
    competition: {
      id: 'championship-001',
      name: 'Championnat de Vendée D1 2026-2027',
      code: 'D1-2026-2027',
      type: 'championship',
      category: 'doublette',
      material: 'fonte',
      season: '2026-2027',
    } as any,
    type: 'championship',
    journee: 5,
    phase: 'Phase Aller',
    round: 'Round 5',
    team1: {
      id: 'team-001',
      name: 'Équipe 1 - Essarts',
      logo: '/images/clubs/essarts.png',
      players: mockPlayers,
      rank: 1,
      seed: 1,
    },
    team2: {
      id: 'team-002',
      name: 'Équipe 1 - Marais',
      logo: '/images/clubs/marais.png',
      players: [],
      rank: 2,
      seed: 2,
    },
    team1Score: 13,
    team2Score: 10,
    winner: 'team1',
    manches: [
      { number: 1, team1: 7, team2: 5, winner: 'team1' },
      { number: 2, team1: 6, team2: 5, winner: 'team1' },
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
      avgPointsPerManche: 6.5,
      bestManche: { number: 1, team1: 7, team2: 5, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-08T14:00:00'),
    endedAt: new Date('2026-10-08T14:30:00'),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'match-002',
    competitionId: 'championship-001',
    competition: {
      id: 'championship-001',
      name: 'Championnat de Vendée D1 2026-2027',
      code: 'D1-2026-2027',
      type: 'championship',
      category: 'doublette',
      material: 'fonte',
      season: '2026-2027',
    } as any,
    type: 'championship',
    journee: 4,
    phase: 'Phase Aller',
    round: 'Round 4',
    team1: {
      id: 'team-001',
      name: 'Équipe 1 - Essarts',
      logo: '/images/clubs/essarts.png',
      players: mockPlayers,
      rank: 1,
      seed: 1,
    },
    team2: {
      id: 'team-003',
      name: 'Équipe 1 - Herbiers',
      logo: '/images/clubs/herbiers.png',
      players: [],
      rank: 3,
      seed: 3,
    },
    team1Score: 13,
    team2Score: 12,
    winner: 'team1',
    manches: [
      { number: 1, team1: 7, team2: 5, winner: 'team1' },
      { number: 2, team1: 6, team2: 7, winner: 'team2' },
      { number: 3, team1: 6, team2: 4, winner: 'team1' },
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
      bestManche: { number: 3, team1: 6, team2: 4, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-01T10:00:00'),
    endedAt: new Date('2026-10-01T10:30:00'),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'match-003',
    competitionId: 'championship-001',
    competition: {
      id: 'championship-001',
      name: 'Championnat de Vendée D1 2026-2027',
      code: 'D1-2026-2027',
      type: 'championship',
      category: 'doublette',
      material: 'fonte',
      season: '2026-2027',
    } as any,
    type: 'championship',
    journee: 3,
    phase: 'Phase Aller',
    round: 'Round 3',
    team1: {
      id: 'team-001',
      name: 'Équipe 1 - Essarts',
      logo: '/images/clubs/essarts.png',
      players: mockPlayers,
      rank: 1,
      seed: 1,
    },
    team2: {
      id: 'team-004',
      name: 'Équipe 1 - Luçon',
      logo: '/images/clubs/lucon.png',
      players: [],
      rank: 4,
      seed: 4,
    },
    team1Score: 13,
    team2Score: 8,
    winner: 'team1',
    manches: [
      { number: 1, team1: 8, team2: 3, winner: 'team1' },
      { number: 2, team1: 5, team2: 5, winner: null },
    ],
    bestOf: 3,
    currentManche: null,
    currentScores: null,
    status: 'completed',
    verified: true,
    verifiedBy: 'arbitre-002',
    verifiedAt: new Date('2026-09-24T15:30:00'),
    date: new Date('2026-09-24T15:00:00'),
    time: '15:00',
    location: 'Salle de Palet - Les Essarts',
    duration: '50min',
    canEdit: false,
    isLive: false,
    isRecent: true,
    stats: {
      totalManches: 2,
      avgPointsPerManche: 6.5,
      bestManche: { number: 1, team1: 8, team2: 3, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-09-24T15:00:00'),
    endedAt: new Date('2026-09-24T15:30:00'),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

const TeamDetail: React.FC = () => {
  const { teamId } = useParams<{ teamId: string }>();

  const { data: team, isLoading } = useQuery<Team>({
    queryKey: ['team', teamId],
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
              <Link to={`/clubs/${displayTeam.clubId}`}>
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour au club
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Logo */}
              <div className="flex-shrink-0">
                <TeamAvatar
                  team={{ id: displayTeam.id, name: displayTeam.name, logo: displayTeam.logo ?? undefined, abbreviation: (displayTeam.club as any)?.code }}
                  size="xl"
                  className="border-4 border-gold-700"
                />
                <div className="mt-4 text-center">
                  <ClubCard club={displayTeam.club}  compact={true} />
                </div>
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayTeam.name}
                  </h1>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="default">{(displayTeam.club as any)?.code}</Badge>
                  <Badge variant={displayTeam.status === 'active' ? 'success' : 'warning'}>
                    {displayTeam.status === 'active' ? 'Active' : displayTeam.status}
                  </Badge>
                  <Badge variant="default">
                    {displayTeam.captain?.fullName} (Capitaine)
                  </Badge>
                </div>

                <p className="text-body-m text-primary-200 mb-4 max-w-2xl">
                  Équipe représentant le {(displayTeam.club as any)?.name} en {displayTeam.competitions[0]?.competition?.type === 'championship' ? 'championnat' : 'tournoi'}.{' '}
                  {displayTeam.competitions[0]?.competition?.name}
                </p>

                {/* Stats Summary */}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Matchs Joués</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayTeam.stats.matches}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Victoires</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayTeam.stats.wins}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Taux de Victoire</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayTeam.stats.winRate}%</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-3 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-300 mb-1">Points</p>
                    <p className="text-heading-s font-bold text-gold-700">{displayTeam.stats.points}</p>
                  </div>
                </div>
              </div>

              {/* Additional Info */}
              <div className="flex-shrink-0">
                <div className="space-y-4">
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Joueurs</p>
                    <p className="text-heading-m font-bold text-gold-700">{displayTeam.players.length}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Points Pour</p>
                    <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.pointsFor}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Points Contre</p>
                    <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.pointsAgainst}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Différence</p>
                    <p className="text-heading-m font-bold text-gold-700">
                      +{displayTeam.stats.diff}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Stats Overview */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              Statistiques de l'Équipe
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Matchs</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.matches}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Victoires</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.wins}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Nuls</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.draws}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Défaites</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.losses}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Win Rate</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTeam.stats.winRate}%</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Classement</p>
                  <p className="text-heading-m font-bold text-gold-700">
                    {displayTeam.competitions[0]?.results.position || 'N/A'}
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
                <Tab value="joueurs" label="Joueurs ({displayTeam.players.length})" />
                <Tab value="competitions" label="Compétitions ({displayTeam.competitions.length})" />
                <Tab value="matchs" label="Matchs ({mockMatches.length})" />
                <Tab value="statistiques" label="Statistiques Complètes" />
              </TabList>

              {/* Joueurs */}
              <TabPanel value="joueurs" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {displayTeam.players.map((player) => (
                    <PlayerCard
                      key={player.id}
                      player={player}
                      showClub={false}
                      
                      variant="team"
                    />
                  ))}
                  {displayTeam.players.length === 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-3">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-primary-300">
                          Aucune information sur les joueurs disponibles.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>

                {/* Statistiques Joueurs dans l'équipe */}
                {displayTeam.players.length > 0 && (
                  <Card className="bg-primary-900/50 border-primary-700/50 mt-6">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Statistiques des Joueurs
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="overflow-x-auto">
                        <table className="w-full text-body-s">
                          <thead>
                            <tr className="border-b border-primary-700/50 text-primary-300">
                              <th className="text-left p-3">Joueur</th>
                              <th className="text-left p-3">Matchs</th>
                              <th className="text-left p-3">Victoires</th>
                              <th className="text-left p-3">Points Moyens</th>
                              <th className="text-left p-3">Taux de Victoire</th>
                            </tr>
                          </thead>
                          <tbody>
                            {displayTeam.players.map((player) => (
                              <tr
                                key={player.id}
                                className="border-b border-primary-700/20 hover:bg-primary-800/50"
                              >
                                <td className="p-3">
                                  <div className="flex items-center gap-2">
                                    <UserAvatar
                                      user={{ id: player.id, name: player.fullName, photo: player.photo ?? undefined }}
                                      size="xs"
                                    />
                                    <span className="text-white">{player.fullName}</span>
                                  </div>
                                </td>
                                <td className="p-3 text-primary-300">{player.stats.matchesPlayed}</td>
                                <td className="p-3 text-primary-300">{player.stats.wins}</td>
                                <td className="p-3 text-gold-700 font-bold">
                                  {player.stats.averagePoints.toFixed(1)}
                                </td>
                                <td className="p-3 text-primary-300">{player.stats.winRate}%</td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    </CardContent>
                  </Card>
                )}
              </TabPanel>

              {/* Compétitions */}
              <TabPanel value="competitions" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {displayTeam.competitions.map((competition) => (
                    <Card
                      key={competition.id}
                      className="bg-primary-900/50 border-primary-700/50"
                    >
                      <CardHeader>
                        <div className="flex items-center justify-between">
                          <h3 className="text-heading-s font-bold text-white">
                            {competition.competition?.name}
                          </h3>
                          <Badge variant={competition.status === 'in_progress' ? 'warning' : competition.status === 'completed' ? 'success' : 'default' }>
                            {competition.status === 'registered' && 'Inscrit'}
                            {competition.status === 'qualified' && 'Qualifié'}
                            {competition.status === 'in_progress' && 'En cours'}
                            {competition.status === 'completed' && 'Terminé'}
                            {competition.status === 'disqualified' && 'Disqualifié'}
                          </Badge>
                        </div>
                      </CardHeader>
                      <CardContent>
                        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Matchs</p>
                            <p className="text-heading-xs font-bold text-gold-700">
                              {competition.results.matches}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Victoires</p>
                            <p className="text-heading-xs font-bold text-gold-700">
                              {competition.results.wins}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Points</p>
                            <p className="text-heading-xs font-bold text-gold-700">
                              {competition.results.points}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Position</p>
                            <p className="text-heading-xs font-bold text-gold-700">
                              {competition.results.position || 'N/A'}
                            </p>
                          </div>
                        </div>
                        <p className="text-body-xs text-primary-300 mb-2">
                          Saison: {competition.season}
                        </p>
                        {competition.competition?.type === 'championship' && (
                          <Link to={`/championnats/${competition.competitionId}`}>
                            <Button variant="outline" size="sm" fullWidth>
                              Voir le championnat
                            </Button>
                          </Link>
                        )}
                        {competition.competition?.type === 'tournament' && (
                          <Link to={`/tournois/${competition.competitionId}`}>
                            <Button variant="outline" size="sm" fullWidth>
                              Voir le tournoi
                            </Button>
                          </Link>
                        )}
                      </CardContent>
                    </Card>
                  ))}
                  {displayTeam.competitions.length === 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-2">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-primary-300">
                          Cette équipe ne participe à aucune compétition pour le moment.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>
              </TabPanel>

              {/* Matchs */}
              <TabPanel value="matchs" className="py-6">
                <div className="space-y-4">
                  {mockMatches.map((match) => (
                    <MatchCard
                      key={match.id}
                      match={match}
                      
                      showTournament={true}
                    />
                  ))}
                  {mockMatches.length === 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50">
                      <CardContent className="text-center p-8">
                        <p className="text-body-m text-primary-300">
                          Aucun match programmé pour le moment.
                        </p>
                      </CardContent>
                    </Card>
                  )}
                </div>
              </TabPanel>

              {/* Statistiques Complètes */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {/* Performances Globales */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Performances Globales
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div>
                          <p className="text-body-xs text-primary-300 mb-1">Meilleure Série de Victoires</p>
                          <p className="text-heading-s font-bold text-gold-700">
                            {displayTeam.stats.winRate / 10}
                          </p>
                        </div>
                        <div>
                          <p className="text-body-xs text-primary-300 mb-1">Points Totaux</p>
                          <p className="text-heading-s font-bold text-gold-700">
                            {displayTeam.stats.pointsFor}
                          </p>
                        </div>
                        <div>
                          <p className="text-body-xs text-primary-300 mb-1">Points Encassés</p>
                          <p className="text-heading-s font-bold text-gold-700">
                            {displayTeam.stats.pointsAgainst}
                          </p>
                        </div>
                        <div>
                          <p className="text-body-xs text-primary-300 mb-1">Différence de Points</p>
                          <p className="text-heading-s font-bold text-gold-700">
                            +{displayTeam.stats.diff}
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>

                  {/* Analyse par Compétition */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Analyse par Compétition
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3">
                        {displayTeam.competitions.map((competition) => (
                          <div
                            key={competition.id}
                            className="bg-primary-800/50 rounded p-3 border border-primary-700/30"
                          >
                            <h4 className="text-body-s font-semibold text-gold-700 mb-2">
                              {competition.competition?.name}
                            </h4>
                            <div className="grid grid-cols-2 md:grid-cols-4 gap-2 text-body-xs">
                              <div>
                                <p className="text-primary-300">Matchs: {competition.results.matches}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">V: {competition.results.wins}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">N: {competition.results.draws}</p>
                              </div>
                              <div>
                                <p className="text-primary-300">D: {competition.results.losses}</p>
                              </div>
                              <div className="md:col-span-4">
                                <p className="text-primary-300">
                                  Position: {competition.results.position || 'N/A'}
                                </p>
                              </div>
                            </div>
                          </div>
                        ))}
                        {displayTeam.competitions.length === 0 && (
                          <p className="text-body-s text-primary-300 text-center">
                            Aucune compétition pour analyser les statistiques.
                          </p>
                        )}
                      </div>
                    </CardContent>
                  </Card>

                  {/* Historique des Résultats */}
                  <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-2">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">
                        Historique des Résultats
                      </h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-3">
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-primary-300">5 derniers matchs</span>
                          <div className="flex gap-2">
                            {mockMatches.slice(0, 5).map((match, index) => (
                              <span
                                key={index}
                                className={`px-3 py-1 rounded-full text-xs ${
                                  match.winner === 'team1' && match.team1.players === displayTeam.players
                                    ? 'bg-green-900/50 text-green-400'
                                    : match.winner === 'team2' && match.team2.players === displayTeam.players
                                    ? 'bg-red-900/50 text-red-400'
                                    : 'bg-primary-700/50 text-primary-300'
                                }`}
                              >
                                {match.winner === 'team1' && match.team1.players === displayTeam.players && 'V'}
                                {match.winner === 'team2' && match.team2.players === displayTeam.players && 'D'}
                                {!(match.winner === 'team1' && match.team1.players === displayTeam.players) &&
                                  !(match.winner === 'team2' && match.team2.players === displayTeam.players) &&
                                  'N'}
                              </span>
                            ))}
                          </div>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-body-s text-primary-300">Forme</span>
                          <div className="flex gap-1">
                            {Array.from({ length: 5 }).map((_, index) => {
                              const match = mockMatches[index];
                              if (!match) return null;
                              const isTeam1 = match.team1.players === displayTeam.players;
                              const isTeam2 = match.team2.players === displayTeam.players;
                              
                              if (match.winner === 'team1' && isTeam1) {
                                return <span key={index} className="text-green-400">V</span>;
                              }
                              if (match.winner === 'team2' && isTeam2) {
                                return <span key={index} className="text-green-400">V</span>;
                              }
                              if (match.winner === 'team2' && isTeam1) {
                                return <span key={index} className="text-red-400">D</span>;
                              }
                              if (match.winner === 'team1' && isTeam2) {
                                return <span key={index} className="text-red-400">D</span>;
                              }
                              return <span key={index} className="text-primary-400">N</span>;
                            })}
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

export default TeamDetail;
