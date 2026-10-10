import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, UserAvatar } from '../components';
import { PlayerCard, ClubCard, MatchHero, LiveScores } from '../components/domain';
import { Match, Player, Team } from '../types';
import { api } from '../api';

// Mock data for development
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
  club: {
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
    social: { facebook: null, twitter: null, instagram: null },
    createdAt: new Date('2020-01-15'),
    updatedAt: new Date('2026-10-01'),
  },
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
};

const mockPlayer2: Player = {
  ...mockPlayer,
  id: 'player-002',
  firstName: 'Pierre',
  lastName: 'Durand',
  fullName: 'Pierre Durand',
  alias: 'Le Rocket',
  club: {
    ...(mockPlayer.club as any),
    id: 'club-001',
    name: 'Club des Essarts',
    code: 'ESS',
  } as any,
  clubId: 'club-001',
  clubNumber: 'ESS-002',
  licenseNumber: 'PV-2026-002',
};

const mockTeam1Players: Player[] = [mockPlayer, mockPlayer2];
const mockTeam2Players: Player[] = [
  {
    ...mockPlayer,
    id: 'player-003',
    firstName: 'Marie',
    lastName: 'Dupont',
    fullName: 'Marie Dupont',
    alias: 'La Précise',
    club: {
      id: 'club-002',
      name: 'Palet du Marais Champagnelais',
      nickname: 'Les Maraisiens',
      code: 'MAR',
      logo: '/images/clubs/marais.png',
      address: '456 Rue de la Plaque, 85170 Le Poiré-sur-Vie',
      city: 'Le Poiré-sur-Vie',
      postalCode: '85170',
      league: 'Vendée',
      division: 'D1',
      status: 'active',
      playersCount: 20,
      teamsCount: 6,
      contactEmail: 'contact@palet-marais.fr',
      contactPhone: '+33 2 51 65 43 22',
      website: 'https://palet-marais.fr',
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date('2018-03-20'),
      updatedAt: new Date('2026-10-01'),
    },
    clubId: 'club-002',
    clubNumber: 'MAR-001',
    licenseNumber: 'PV-2026-003',
    createdAt: new Date('2021-01-01'),
    updatedAt: new Date('2026-10-09'),
  },
  {
    ...mockPlayer,
    id: 'player-004',
    firstName: 'Thomas',
    lastName: 'Lemoine',
    fullName: 'Thomas Lemoine',
    alias: null,
    club: {
      id: 'club-002',
      name: 'Palet du Marais Champagnelais',
      nickname: 'Les Maraisiens',
      code: 'MAR',
      logo: '/images/clubs/marais.png',
      address: '456 Rue de la Plaque, 85170 Le Poiré-sur-Vie',
      city: 'Le Poiré-sur-Vie',
      postalCode: '85170',
      league: 'Vendée',
      division: 'D1',
      status: 'active',
      playersCount: 20,
      teamsCount: 6,
      contactEmail: 'contact@palet-marais.fr',
      contactPhone: '+33 2 51 65 43 22',
      website: 'https://palet-marais.fr',
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date('2018-03-20'),
      updatedAt: new Date('2026-10-01'),
    },
    clubId: 'club-002',
    clubNumber: 'MAR-002',
    licenseNumber: 'PV-2026-004',
  },
];

const mockMatch: Match = {
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
    players: mockTeam1Players,
    rank: 1,
    seed: 1,
  },
  team2: {
    id: 'team-002',
    name: 'Équipe 1 - Marais',
    logo: '/images/clubs/marais.png',
    players: mockTeam2Players,
    rank: 2,
    seed: 2,
  },
  team1Score: 15,
  team2Score: 12,
  winner: 'team1',
  manches: [
    { number: 1, team1: 8, team2: 5, winner: 'team1' },
    { number: 2, team1: 7, team2: 7, winner: null },
    { number: 3, team1: 7, team2: 3, winner: 'team1' },
  ],
  bestOf: 3,
  currentManche: null,
  currentScores: null,
  status: 'completed',
  verified: true,
  verifiedBy: 'Jean Dupont',
  verifiedAt: new Date('2026-10-08T14:30:00'),
  date: new Date('2026-10-08T14:00:00'),
  time: '14:00',
  location: 'Salle de Palet - Les Essarts',
  duration: '1h15',
  canEdit: false,
  isLive: false,
  isRecent: true,
  stats: {
    totalManches: 3,
    avgPointsPerManche: 6.67,
    bestManche: { number: 1, team1: 8, team2: 5, winner: 'team1' },
    playerStats: [
      {
        player: mockPlayer,
        points: 12,
        accuracy: 88,
        bestManche: 8,
      },
      {
        player: mockPlayer2,
        points: 10,
        accuracy: 82,
        bestManche: 7,
      },
      {
        player: mockTeam2Players[0]!,
        points: 9,
        accuracy: 80,
        bestManche: 7,
      },
      {
        player: mockTeam2Players[1]!,
        points: 8,
        accuracy: 78,
        bestManche: 6,
      },
    ],
  },
  startedAt: new Date('2026-10-08T14:00:00'),
  endedAt: new Date('2026-10-08T15:15:00'),
  createdAt: new Date('2026-10-01'),
  updatedAt: new Date('2026-10-08T15:15:00'),
};

const MatchDetail: React.FC = () => {
  const { matchId } = useParams<{ matchId: string }>();

  const { data: match, isLoading } = useQuery<Match>({
    queryKey: ['match', matchId],
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
            <p className="text-body-m text-primary-200">Chargement...</p>
          </div>
        </main>
      </Layout>
    );
  }

  // Calculate team scores per manche
  const getMancheResults = () => {
    if (!displayMatch.manches || displayMatch.manches.length === 0) {
      return [];
    }
    return displayMatch.manches.map((manche) => ({
      manche: manche.number,
      team1: manche.team1,
      team2: manche.team2,
      winner: manche.winner,
    }));
  };

  // Check if match is live
  const isLiveMatch = displayMatch.status === 'in_progress' || displayMatch.isLive;

  // Get winner info
  const getWinnerInfo = () => {
    if (!displayMatch.winner) return null;
    const winner = displayMatch.winner === 'team1' ? displayMatch.team1 : displayMatch.team2;
    return {
      name: winner.name,
      score: displayMatch.winner === 'team1' ? displayMatch.team1Score : displayMatch.team2Score,
    };
  };

  return (
    <Layout>
      <main className="flex-1">
        {/* Header with Match Hero */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center gap-4 mb-4">
              <Link to={displayMatch.competition.type === 'championship' ? '/championnats' : '/tournois'}>
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux {displayMatch.competition.type === 'championship' ? 'championnats' : 'tournois'}
                </Button>
              </Link>
            </div>

            {isLiveMatch ? (
              <MatchHero
                match={displayMatch}
                
              />
            ) : (
              <div className="bg-primary-900/50 border border-primary-700/50 rounded-lg p-6 mb-6">
                <div className="flex flex-col md:flex-row items-center justify-center gap-8">
                  {/* Team 1 */}
                  <div className="text-center">
                    <div className="flex items-center justify-center gap-2 mb-2">
                      {displayMatch.team1.logo && (
                        <img
                          src={displayMatch.team1.logo}
                          alt={displayMatch.team1.name}
                          className="w-16 h-16 rounded-full border-2 border-primary-700"
                        />
                      )}
                      <h2 className="text-heading-l font-bold text-white uppercase tracking-wide">
                        {displayMatch.team1.name}
                      </h2>
                    </div>
                    <p className="text-heading-xl font-bold text-gold-700">
                      {displayMatch.team1Score}
                    </p>
                    {displayMatch.winner === 'team1' && (
                      <Badge variant="success" className="mt-2">
                        Vainqueur
                      </Badge>
                    )}
                  </div>

                  {/* VS */}
                  <div className="text-center">
                    <p className="text-heading-xl font-bold text-primary-300">VS</p>
                    {displayMatch.winner === null && displayMatch.status === 'completed' && (
                      <Badge variant="default" className="mt-2">
                        Match Nul
                      </Badge>
                    )}
                    {displayMatch.status === 'scheduled' && (
                      <Badge variant="info" className="mt-2">
                        À venir
                      </Badge>
                    )}
                  </div>

                  {/* Team 2 */}
                  <div className="text-center">
                    <div className="flex items-center justify-center gap-2 mb-2">
                      {displayMatch.team2.logo && (
                        <img
                          src={displayMatch.team2.logo}
                          alt={displayMatch.team2.name}
                          className="w-16 h-16 rounded-full border-2 border-primary-700"
                        />
                      )}
                      <h2 className="text-heading-l font-bold text-white uppercase tracking-wide">
                        {displayMatch.team2.name}
                      </h2>
                    </div>
                    <p className="text-heading-xl font-bold text-gold-700">
                      {displayMatch.team2Score}
                    </p>
                    {displayMatch.winner === 'team2' && (
                      <Badge variant="success" className="mt-2">
                        Vainqueur
                      </Badge>
                    )}
                  </div>
                </div>

                {/* Match Info */}
                <div className="mt-6 pt-6 border-t border-primary-700/50 flex flex-wrap justify-center gap-4">
                  <div className="text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Compétition</p>
                    <p className="text-body-s font-semibold text-white">{displayMatch.competition.name}</p>
                  </div>
                  {displayMatch.journee && (
                    <div className="text-center">
                      <p className="text-body-xs text-primary-400 mb-1">Journée</p>
                      <p className="text-body-s font-semibold text-gold-700">{displayMatch.journee}</p>
                    </div>
                  )}
                  {displayMatch.phase && (
                    <div className="text-center">
                      <p className="text-body-xs text-primary-400 mb-1">Phase</p>
                      <p className="text-body-s font-semibold text-gold-700">{displayMatch.phase}</p>
                    </div>
                  )}
                  <div className="text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Date</p>
                    <p className="text-body-s font-semibold text-white">
                      {displayMatch.date.toLocaleDateString('fr-FR')}
                    </p>
                  </div>
                  {displayMatch.time && (
                    <div className="text-center">
                      <p className="text-body-xs text-primary-400 mb-1">Heure</p>
                      <p className="text-body-s font-semibold text-white">{displayMatch.time}</p>
                    </div>
                  )}
                  <div className="text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Lieu</p>
                    <p className="text-body-s font-semibold text-white">{displayMatch.location}</p>
                  </div>
                  {displayMatch.duration && (
                    <div className="text-center">
                      <p className="text-body-xs text-primary-400 mb-1">Durée</p>
                      <p className="text-body-s font-semibold text-gold-700">{displayMatch.duration}</p>
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>
        </section>

        {/* Stats Overview */}
        {displayMatch.status === 'completed' && (
          <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
            <div className="max-w-7xl mx-auto">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
                Résultat Final
              </h2>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardContent className="text-center p-4">
                    <p className="text-body-xs text-primary-300 mb-1">Gagnant</p>
                    {displayMatch.winner ? (
                      <p className="text-heading-s font-bold text-gold-700">
                        {displayMatch.winner === 'team1' ? displayMatch.team1.name : displayMatch.team2.name}
                      </p>
                    ) : (
                      <p className="text-heading-s font-bold text-primary-300">Match Nul</p>
                    )}
                  </CardContent>
                </Card>
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardContent className="text-center p-4">
                    <p className="text-body-xs text-primary-300 mb-1">Score Final</p>
                    <p className="text-heading-s font-bold text-gold-700">
                      {displayMatch.team1Score} - {displayMatch.team2Score}
                    </p>
                  </CardContent>
                </Card>
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardContent className="text-center p-4">
                    <p className="text-body-xs text-primary-300 mb-1">Manches Jouées</p>
                    <p className="text-heading-s font-bold text-gold-700">
                      {displayMatch.manches?.length || 0}
                    </p>
                  </CardContent>
                </Card>
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardContent className="text-center p-4">
                    <p className="text-body-xs text-primary-300 mb-1">Meilleure Manche</p>
                    <p className="text-heading-s font-bold text-gold-700">
                      {displayMatch.stats.bestManche?.team1} - {displayMatch.stats.bestManche?.team2}
                    </p>
                  </CardContent>
                </Card>
              </div>

              {/* Verification Info */}
              {displayMatch.verified && (
                <div className="mt-6 flex items-center justify-center gap-4">
                  <Badge variant="success">Match vérifié</Badge>
                  <span className="text-body-s text-primary-300">
                    Vérifié par {displayMatch.verifiedBy} le {displayMatch.verifiedAt?.toLocaleDateString('fr-FR')}
                  </span>
                </div>
              )}
            </div>
          </section>
        )}

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="details">
              <TabList>
                <Tab value="details" label="Détails du Match" />
                <Tab value="manches" label="Manches ({displayMatch.manches?.length || 0})" />
                <Tab value="joueurs" label="Joueurs" />
                <Tab value="statistiques" label="Statistiques" />
              </TabList>

              {/* Détails du Match */}
              <TabPanel value="details" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Informations Complètes</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                      <div>
                        <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                          Équipe à Domicile
                        </h4>
                        <div className="space-y-2">
                          <p>
                            <span className="text-primary-400">Nom:</span> {displayMatch.team1.name}
                          </p>
                          <p>
                            <span className="text-primary-400">Classement:</span> {displayMatch.team1.rank || 'Non classé'}
                          </p>
                          {displayMatch.team1.seed && (
                            <p>
                              <span className="text-primary-400">Seed:</span> {displayMatch.team1.seed}
                            </p>
                          )}
                        </div>

                        <h4 className="text-heading-xs font-bold text-gold-700 mt-6 mb-4">
                          Équipe à l'Extérieur
                        </h4>
                        <div className="space-y-2">
                          <p>
                            <span className="text-primary-400">Nom:</span> {displayMatch.team2.name}
                          </p>
                          <p>
                            <span className="text-primary-400">Classement:</span> {displayMatch.team2.rank || 'Non classé'}
                          </p>
                          {displayMatch.team2.seed && (
                            <p>
                              <span className="text-primary-400">Seed:</span> {displayMatch.team2.seed}
                            </p>
                          )}
                        </div>
                      </div>

                      <div>
                        <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                          Informations de la Compétition
                        </h4>
                        <div className="space-y-2">
                          <p>
                            <span className="text-primary-400">Type:</span>{' '}
                            {displayMatch.competition.type === 'championship' && 'Championnat'}
                            {displayMatch.competition.type === 'tournament' && 'Tournoi'}
                            {displayMatch.competition.type === 'cup' && 'Coupe'}
                            {displayMatch.competition.type === 'friendly' && 'Amical'}
                          </p>
                          <p>
                            <span className="text-primary-400">Catégorie:</span>{' '}
                            {displayMatch.competition.category === 'individual' && 'Individuel'}
                            {displayMatch.competition.category === 'doublette' && 'Doublette'}
                            {displayMatch.competition.category === 'triplette' && 'Triplette'}
                          </p>
                          <p>
                            <span className="text-primary-400">Matériel:</span>{' '}
                            {displayMatch.competition.material === 'fonte' && 'Fonte'}
                            {displayMatch.competition.material === 'laiton' && 'Laiton'}
                            {displayMatch.competition.material === 'bois' && 'Bois'}
                            {displayMatch.competition.material === 'mixte' && 'Mixte'}
                          </p>
                          <p>
                            <span className="text-primary-400">Saison:</span> {displayMatch.competition.season}
                          </p>
                        </div>
                      </div>
                    </div>

                    {/* Timeline */}
                    <div className="mt-6">
                      <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                        Timeline
                      </h4>
                      <div className="space-y-2">
                        {displayMatch.startedAt && (
                          <p>
                            <span className="text-primary-400">Début:</span> {displayMatch.startedAt.toLocaleString('fr-FR')}
                          </p>
                        )}
                        {displayMatch.endedAt && (
                          <p>
                            <span className="text-primary-400">Fin:</span> {displayMatch.endedAt.toLocaleString('fr-FR')}
                          </p>
                        )}
                        {displayMatch.verifiedAt && (
                          <p>
                            <span className="text-primary-400">Vérifié le:</span> {displayMatch.verifiedAt.toLocaleString('fr-FR')}
                          </p>
                        )}
                        {displayMatch.duration && (
                          <p>
                            <span className="text-primary-400">Durée:</span> {displayMatch.duration}
                          </p>
                        )}
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Manches */}
              <TabPanel value="manches" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">
                      Résultats par Manche
                    </h3>
                  </CardHeader>
                  <CardContent>
                    {displayMatch.manches && displayMatch.manches.length > 0 ? (
                      <div className="space-y-4">
                        {displayMatch.manches.map((manche, index) => (
                          <div
                            key={manche.number}
                            className={`bg-primary-800/50 rounded p-4 border border-primary-700/30 ${
                              manche.winner === 'team1'
                                ? 'border-l-4 border-l-gold-700'
                                : manche.winner === 'team2'
                                ? 'border-l-4 border-l-green-500'
                                : 'border-l-4 border-l-primary-500'
                            }`}
                          >
                            <div className="flex items-center justify-between">
                              <div className="flex items-center gap-2">
                                <span className="text-heading-s font-bold text-white">
                                  Manche {manche.number}
                                </span>
                                {manche.winner === 'team1' && (
                                  <Badge variant="success">Gagnée par {displayMatch.team1.name}</Badge>
                                )}
                                {manche.winner === 'team2' && (
                                  <Badge variant="success">Gagnée par {displayMatch.team2.name}</Badge>
                                )}
                                {manche.winner === null && (
                                  <Badge variant="default">Match nul</Badge>
                                )}
                              </div>
                              <div className="text-center">
                                <p className="text-heading-m font-bold text-gold-700">
                                  {manche.team1} - {manche.team2}
                                </p>
                              </div>
                            </div>
                          </div>
                        ))}

                        {/* Summary */}
                        <div className="mt-6 pt-4 border-t border-primary-700/50">
                          <h4 className="text-heading-xs font-bold text-gold-700 mb-3">
                            Résumé
                          </h4>
                          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300 mb-1">Total Manches</p>
                              <p className="text-heading-s font-bold text-gold-700">
                                {displayMatch.manches.length}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300 mb-1">Gagnées par {displayMatch.team1.name}</p>
                              <p className="text-heading-s font-bold text-gold-700">
                                {displayMatch.manches.filter(m => m.winner === 'team1').length}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300 mb-1">Gagnées par {displayMatch.team2.name}</p>
                              <p className="text-heading-s font-bold text-gold-700">
                                {displayMatch.manches.filter(m => m.winner === 'team2').length}
                              </p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300 mb-1">Nulles</p>
                              <p className="text-heading-s font-bold text-gold-700">
                                {displayMatch.manches.filter(m => m.winner === null).length}
                              </p>
                            </div>
                          </div>
                        </div>
                      </div>
                    ) : (
                      <p className="text-body-m text-primary-300 text-center py-4">
                        Aucun détail de manche disponible pour ce match.
                      </p>
                    )}
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Joueurs */}
              <TabPanel value="joueurs" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* Équipe 1 */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <div className="flex items-center gap-3">
                        {displayMatch.team1.logo && (
                          <img
                            src={displayMatch.team1.logo}
                            alt={displayMatch.team1.name}
                            className="w-10 h-10 rounded-full"
                          />
                        )}
                        <h3 className="text-heading-s font-bold text-white">{displayMatch.team1.name}</h3>
                      </div>
                    </CardHeader>
                    <CardContent>
                      {displayMatch.team1.players && displayMatch.team1.players.length > 0 ? (
                        <div className="space-y-3">
                          {displayMatch.team1.players.map((player) => (
                            <div
                              key={player.id}
                              className="flex items-center gap-3 p-2 bg-primary-800/50 rounded"
                            >
                              <UserAvatar
                                user={{ id: player.id, name: player.fullName, photo: player.photo ?? undefined }}
                                size="sm"
                              />
                              <div className="flex-1">
                                <p className="text-body-s font-semibold text-white">{player.fullName}</p>
                                <p className="text-body-xs text-primary-300">
                                  {(player.club as any)?.name} - {player.clubNumber}
                                </p>
                              </div>
                              {player.club && (
                                <Link to={`/clubs/${(player.club as any).id}`} className="flex-shrink-0">
                                  <Button variant="ghost" size="xs">
                                    Voir le club
                                  </Button>
                                </Link>
                              )}
                            </div>
                          ))}
                        </div>
                      ) : (
                        <p className="text-body-s text-primary-300">
                          Aucune information sur les joueurs disponibles.
                        </p>
                      )}
                    </CardContent>
                  </Card>

                  {/* Équipe 2 */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <div className="flex items-center gap-3">
                        {displayMatch.team2.logo && (
                          <img
                            src={displayMatch.team2.logo}
                            alt={displayMatch.team2.name}
                            className="w-10 h-10 rounded-full"
                          />
                        )}
                        <h3 className="text-heading-s font-bold text-white">{displayMatch.team2.name}</h3>
                      </div>
                    </CardHeader>
                    <CardContent>
                      {displayMatch.team2.players && displayMatch.team2.players.length > 0 ? (
                        <div className="space-y-3">
                          {displayMatch.team2.players.map((player) => (
                            <div
                              key={player.id}
                              className="flex items-center gap-3 p-2 bg-primary-800/50 rounded"
                            >
                              <UserAvatar
                                user={{ id: player.id, name: player.fullName, photo: player.photo ?? undefined }}
                                size="sm"
                              />
                              <div className="flex-1">
                                <p className="text-body-s font-semibold text-white">{player.fullName}</p>
                                <p className="text-body-xs text-primary-300">
                                  {(player.club as any)?.name} - {player.clubNumber}
                                </p>
                              </div>
                              {player.club && (
                                <Link to={`/clubs/${(player.club as any).id}`} className="flex-shrink-0">
                                  <Button variant="ghost" size="xs">
                                    Voir le club
                                  </Button>
                                </Link>
                              )}
                            </div>
                          ))}
                        </div>
                      ) : (
                        <p className="text-body-s text-primary-300">
                          Aucune information sur les joueurs disponibles.
                        </p>
                      )}
                    </CardContent>
                  </Card>
                </div>
              </TabPanel>

              {/* Statistiques */}
              <TabPanel value="statistiques" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {/* Statistiques Match */}
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Statistiques du Match</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="space-y-4">
                        <div className="grid grid-cols-2 gap-4">
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Total Manches</p>
                            <p className="text-heading-s font-bold text-gold-700">
                              {displayMatch.manches?.length || 0}
                            </p>
                          </div>
                          <div className="text-center">
                            <p className="text-body-xs text-primary-300 mb-1">Points Moyens/Manche</p>
                            <p className="text-heading-s font-bold text-gold-700">
                              {displayMatch.stats.avgPointsPerManche.toFixed(1)}
                            </p>
                          </div>
                        </div>

                        {displayMatch.stats.bestManche && (
                          <div className="bg-primary-800/50 rounded p-3">
                            <h4 className="text-heading-xs font-bold text-gold-700 mb-2">
                              Meilleure Manche
                            </h4>
                            <p className="text-body-s text-white">
                              Manche {displayMatch.stats.bestManche.number}: {displayMatch.stats.bestManche.team1} - {displayMatch.stats.bestManche.team2}
                            </p>
                            <p className="text-body-xs text-primary-300">
                              Gagnée par: {displayMatch.stats.bestManche.winner === 'team1' ? displayMatch.team1.name : displayMatch.team2.name}
                            </p>
                          </div>
                        )}

                        {displayMatch.duration && (
                          <div>
                            <p className="text-body-xs text-primary-300 mb-1">Durée du match</p>
                            <p className="text-body-s font-semibold text-white">{displayMatch.duration}</p>
                          </div>
                        )}
                      </div>
                    </CardContent>
                  </Card>

                  {/* Statistiques Joueurs */}
                  {displayMatch.stats.playerStats && displayMatch.stats.playerStats.length > 0 && (
                    <Card className="bg-primary-900/50 border-primary-700/50">
                      <CardHeader>
                        <h3 className="text-heading-s font-bold text-white">
                          Statistiques des Joueurs
                        </h3>
                      </CardHeader>
                      <CardContent>
                        <div className="space-y-3">
                          {displayMatch.stats.playerStats.map((playerStat, index) => (
                            <div
                              key={index}
                              className="bg-primary-800/50 rounded p-3"
                            >
                              <div className="flex items-center justify-between">
                                <div className="flex items-center gap-2">
                                  <UserAvatar
                                    user={{ id: playerStat.player.id, name: playerStat.player.fullName, photo: playerStat.player.photo ?? undefined }}
                                    size="xs"
                                  />
                                  <p className="text-body-s font-semibold text-white">
                                    {playerStat.player.fullName}
                                  </p>
                                </div>
                                <div className="text-center">
                                  <p className="text-body-xs text-primary-300">Points</p>
                                  <p className="text-body-s font-bold text-gold-700">
                                    {playerStat.points}
                                  </p>
                                </div>
                                <div className="text-center">
                                  <p className="text-body-xs text-primary-300">Précision</p>
                                  <p className="text-body-s font-bold text-gold-700">
                                    {playerStat.accuracy}%
                                  </p>
                                </div>
                                <div className="text-center">
                                  <p className="text-body-xs text-primary-300">Meilleure Manche</p>
                                  <p className="text-body-s font-bold text-gold-700">
                                    {playerStat.bestManche}
                                  </p>
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      </CardContent>
                    </Card>
                  )}
                </div>

                {/* MEILLEUR JOUEUR */}
                {displayMatch.stats.playerStats && displayMatch.stats.playerStats.length > 0 && (
                  <Card className="bg-primary-900/50 border-primary-700/50 mt-4">
                    <CardHeader>
                      <h3 className="text-heading-s font-bold text-white">Meilleur Joueur du Match</h3>
                    </CardHeader>
                    <CardContent>
                      <div className="text-center">
                        {(() => {
                          const bestPlayer = displayMatch.stats.playerStats.reduce((prev, current) =>
                            (prev.points > current.points ? prev : current)
                          );
                          return (
                            <div className="flex items-center justify-center gap-4">
                              <UserAvatar
                                user={{ id: bestPlayer.player.id, name: bestPlayer.player.fullName, photo: bestPlayer.player.photo ?? undefined }}
                                size="l"
                                className="border-2 border-gold-700"
                              />
                              <div>
                                <p className="text-body-s font-semibold text-white">
                                  {bestPlayer.player.fullName}
                                </p>
                                <p className="text-body-xs text-primary-300">
                                  {(bestPlayer.player.club as any)?.name}
                                </p>
                                <div className="grid grid-cols-3 gap-4 mt-2">
                                  <div className="text-center">
                                    <p className="text-body-xs text-primary-300">Points</p>
                                    <p className="text-heading-xs font-bold text-gold-700">{bestPlayer.points}</p>
                                  </div>
                                  <div className="text-center">
                                    <p className="text-body-xs text-primary-300">Précision</p>
                                    <p className="text-heading-xs font-bold text-gold-700">{bestPlayer.accuracy}%</p>
                                  </div>
                                  <div className="text-center">
                                    <p className="text-body-xs text-primary-300">Meilleure Manche</p>
                                    <p className="text-heading-xs font-bold text-gold-700">{bestPlayer.bestManche}</p>
                                  </div>
                                </div>
                              </div>
                            </div>
                          );
                        })()}
                      </div>
                    </CardContent>
                  </Card>
                )}
              </TabPanel>
            </Tabs>
          </div>
        </section>

        {/* Action Buttons */}
        {displayMatch.status === 'completed' && displayMatch.canEdit && (
          <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
            <div className="max-w-7xl mx-auto">
              <div className="flex justify-center gap-4">
                <Button variant="outline" size="l">
                  Voir le résumé
                </Button>
                <Button variant="primary" size="l">
                  Imprimer le rapport
                </Button>
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  );
};

export default MatchDetail;
