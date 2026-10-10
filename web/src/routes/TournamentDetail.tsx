import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, TeamAvatar } from '../components';
import { TournamentCard, MatchCard, ClubCard } from '../components/domain';
import { Tournament, Team, Match, Player } from '../types';
import { api } from '../api';

// Mock data for development
const mockTeams: Team[] = [
  {
    id: 'team-001',
    name: 'Équipe 1 - Essarts',
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
    captain: {
      id: 'player-001',
      firstName: 'Jean',
      lastName: 'Martin',
      fullName: 'Jean Martin',
      alias: null,
      email: '',
      photo: null,
      status: 'active',
      birthDate: new Date(),
      age: 40,
      birthPlace: '',
      nationality: '',
      dominantHand: 'right',
      playStyle: null,
      phone: null,
      address: null,
      city: null,
      postalCode: null,
      club: null as any,
      clubId: '',
      clubNumber: null,
      category: '',
      licenseNumber: '',
      licenseDate: new Date(),
      licenseStatus: 'valid',
      registrationDate: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
      stats: {
        matchesPlayed: 0,
        wins: 0,
        draws: 0,
        losses: 0,
        totalPoints: 0,
        averagePoints: 0,
        pointsPerManche: 0,
        bestScore: 0,
        bestStreak: 0,
        maxWinStreak: 0,
        winRate: 0,
        accuracy: 0,
        currentRank: 0,
        currentDivision: '',
        currentPoints: 0,
        previousRank: 0,
        rankTrend: 'stable',
        newHighlights: 0,
        byPosition: {
          first: { matches: 0, points: 0, winRate: 0 },
          second: { matches: 0, points: 0, winRate: 0 },
        },
        byCompetition: {},
        bySeason: {},
      },
      trophies: { total: 0, list: [] },
      badges: { total: 0, categories: [] },
      records: { total: 0, list: [] },
      currentTeams: [],
      pastTeams: [],
      frequentTeammates: [],
      matchHistory: { wins: 0, losses: 0, draws: 0, total: 0 },
      competitionStats: [],
      recentActivities: [],
      preferences: {
        language: 'fr',
        timezone: 'Europe/Paris',
        distanceUnit: 'metric',
        notifications: { email: { results: false, news: false, reminders: false }, push: { results: false, matches: false } },
      },
      social: { facebook: null, twitter: null, instagram: null },
      lastActiveAt: new Date(),
    },
    captainId: 'player-001',
    players: [],
    logo: null,
    stats: {
      matches: 25,
      wins: 18,
      draws: 4,
      losses: 3,
      pointsFor: 225,
      pointsAgainst: 165,
      diff: 60,
      points: 58,
      winRate: 72,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'team-002',
    name: 'Équipe 1 - Marais',
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
    captain: {
      id: 'player-002',
      firstName: 'Pierre',
      lastName: 'Durand',
      fullName: 'Pierre Durand',
      alias: null,
      email: '',
      photo: null,
      status: 'active',
      birthDate: new Date(),
      age: 35,
      birthPlace: '',
      nationality: '',
      dominantHand: 'left',
      playStyle: null,
      phone: null,
      address: null,
      city: null,
      postalCode: null,
      club: null as any,
      clubId: '',
      clubNumber: null,
      category: '',
      licenseNumber: '',
      licenseDate: new Date(),
      licenseStatus: 'valid',
      registrationDate: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
      stats: {
        matchesPlayed: 0,
        wins: 0,
        draws: 0,
        losses: 0,
        totalPoints: 0,
        averagePoints: 0,
        pointsPerManche: 0,
        bestScore: 0,
        bestStreak: 0,
        maxWinStreak: 0,
        winRate: 0,
        accuracy: 0,
        currentRank: 0,
        currentDivision: '',
        currentPoints: 0,
        previousRank: 0,
        rankTrend: 'stable',
        newHighlights: 0,
        byPosition: {
          first: { matches: 0, points: 0, winRate: 0 },
          second: { matches: 0, points: 0, winRate: 0 },
        },
        byCompetition: {},
        bySeason: {},
      },
      trophies: { total: 0, list: [] },
      badges: { total: 0, categories: [] },
      records: { total: 0, list: [] },
      currentTeams: [],
      pastTeams: [],
      frequentTeammates: [],
      matchHistory: { wins: 0, losses: 0, draws: 0, total: 0 },
      competitionStats: [],
      recentActivities: [],
      preferences: {
        language: 'fr',
        timezone: 'Europe/Paris',
        distanceUnit: 'metric',
        notifications: { email: { results: false, news: false, reminders: false }, push: { results: false, matches: false } },
      },
      social: { facebook: null, twitter: null, instagram: null },
      lastActiveAt: new Date(),
    },
    captainId: 'player-002',
    players: [],
    logo: null,
    stats: {
      matches: 25,
      wins: 15,
      draws: 5,
      losses: 5,
      pointsFor: 210,
      pointsAgainst: 180,
      diff: 30,
      points: 50,
      winRate: 60,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'team-003',
    name: 'Équipe 1 - Herbiers',
    club: {
      id: 'club-003',
      name: 'Palet des Herbiers',
      nickname: 'Les Herbiers',
      code: 'HER',
      logo: '/images/clubs/herbiers.png',
      address: '789 Rue du Jeu, 85500 Les Herbiers',
      city: 'Les Herbiers',
      postalCode: '85500',
      league: 'Vendée',
      division: 'D1',
      status: 'active',
      playersCount: 18,
      teamsCount: 5,
      contactEmail: 'contact@palet-herbiers.fr',
      contactPhone: '+33 2 51 65 43 23',
      website: 'https://palet-herbiers.fr',
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date('2019-05-10'),
      updatedAt: new Date('2026-10-01'),
    },
    clubId: 'club-003',
    captain: {
      id: 'player-003',
      firstName: 'Marie',
      lastName: 'Dupont',
      fullName: 'Marie Dupont',
      alias: null,
      email: '',
      photo: null,
      status: 'active',
      birthDate: new Date(),
      age: 30,
      birthPlace: '',
      nationality: '',
      dominantHand: 'right',
      playStyle: null,
      phone: null,
      address: null,
      city: null,
      postalCode: null,
      club: null as any,
      clubId: '',
      clubNumber: null,
      category: '',
      licenseNumber: '',
      licenseDate: new Date(),
      licenseStatus: 'valid',
      registrationDate: new Date(),
      createdAt: new Date(),
      updatedAt: new Date(),
      stats: {
        matchesPlayed: 0,
        wins: 0,
        draws: 0,
        losses: 0,
        totalPoints: 0,
        averagePoints: 0,
        pointsPerManche: 0,
        bestScore: 0,
        bestStreak: 0,
        maxWinStreak: 0,
        winRate: 0,
        accuracy: 0,
        currentRank: 0,
        currentDivision: '',
        currentPoints: 0,
        previousRank: 0,
        rankTrend: 'stable',
        newHighlights: 0,
        byPosition: {
          first: { matches: 0, points: 0, winRate: 0 },
          second: { matches: 0, points: 0, winRate: 0 },
        },
        byCompetition: {},
        bySeason: {},
      },
      trophies: { total: 0, list: [] },
      badges: { total: 0, categories: [] },
      records: { total: 0, list: [] },
      currentTeams: [],
      pastTeams: [],
      frequentTeammates: [],
      matchHistory: { wins: 0, losses: 0, draws: 0, total: 0 },
      competitionStats: [],
      recentActivities: [],
      preferences: {
        language: 'fr',
        timezone: 'Europe/Paris',
        distanceUnit: 'metric',
        notifications: { email: { results: false, news: false, reminders: false }, push: { results: false, matches: false } },
      },
      social: { facebook: null, twitter: null, instagram: null },
      lastActiveAt: new Date(),
    },
    captainId: 'player-003',
    players: [],
    logo: null,
    stats: {
      matches: 25,
      wins: 12,
      draws: 6,
      losses: 7,
      pointsFor: 195,
      pointsAgainst: 200,
      diff: -5,
      points: 42,
      winRate: 48,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

const mockTournament: Tournament = {
  id: 'tournament-001',
  name: 'Coupe de France 2026',
  code: 'CDF-2026',
  type: 'cup',
  category: 'doublette',
  material: 'fonte',
  season: '2026',
  description: 'La prestigieuse Coupe de France de Palet Vendéen ouverte à toutes les équipes.',
  location: 'Vendée, France',
  divisions: ['D1', 'D2'],
  format: 'knockout',
  registrationStart: new Date('2026-09-01'),
  registrationEnd: new Date('2026-09-30'),
  startDate: new Date('2026-10-15'),
  endDate: new Date('2026-10-20'),
  status: 'registration',
  currentJournee: null,
  totalJournees: null,
  currentPhase: 'Registration',
  maxTeams: 64,
  registeredTeams: mockTeams.length,
  teams: mockTeams,
  pointsType: '15',
  winPoints: 3,
  drawPoints: 1,
  lossPoints: 0,
  autoRegistration: false,
  manualValidation: true,
  isPublic: true,
  isArchived: false,
  stats: {
    totalMatches: 0,
    completedMatches: 0,
    totalPoints: 0,
    topScorer: null,
    topTeam: null,
  },
  organizer: mockTeams[0]!.club!,
  organizerId: 'club-001',
  phases: [
    {
      id: 'phase-001',
      name: 'Phase de Poules',
      type: 'group',
      order: 1,
      matches: [],
      groups: [
        {
          id: 'group-001',
          name: 'Poule A',
          teams: [mockTeams[0]!, mockTeams[1]!],
          matches: [],
        },
        {
          id: 'group-002',
          name: 'Poule B',
          teams: [mockTeams[2]!],
          matches: [],
        },
      ],
    },
    {
      id: 'phase-002',
      name: 'Phase Finale',
      type: 'knockout',
      order: 2,
      matches: [],
      groups: null,
    },
  ],
  brackets: null,
  fee: 50,
  paymentRequired: true,
  paymentMethods: ['card', 'paypal', 'transfer'],
  rules: 'Règlement officiel FNSMR. Points à 15, distance 3,80m pour la fonte.',
  eligibility: {
    requireLicense: true,
    requireClub: true,
    categories: ['doublette'],
    divisions: ['D1', 'D2', 'D3'],
    maxPlayersPerClub: 2,
    minPlayers: 2,
    maxPlayers: 2,
  },
  createdAt: new Date('2026-08-01'),
  updatedAt: new Date('2026-10-09'),
};

const mockMatches: Match[] = [
  {
    id: 'match-001',
    competitionId: 'tournament-001',
    competition: mockTournament as any,
    type: 'cup',
    journee: null,
    phase: 'Phase de Poules',
    round: 'Round 1',
    team1: {
      id: 'team-001',
      name: 'Équipe 1 - Essarts',
      logo: '/images/clubs/essarts.png',
      players: [],
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
    verifiedBy: 'arbitre-001',
    verifiedAt: new Date('2026-10-15T14:30:00'),
    date: new Date('2026-10-15T14:00:00'),
    time: '14:00',
    location: 'Salle de Palet - Les Essarts',
    duration: '1h15',
    canEdit: false,
    isLive: false,
    isRecent: true,
    stats: {
      totalManches: 3,
      avgPointsPerManche: 6.67,
      bestManche: { number: 3, team1: 7, team2: 3, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-15T14:00:00'),
    endedAt: new Date('2026-10-15T14:30:00'),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'match-002',
    competitionId: 'tournament-001',
    competition: mockTournament as any,
    type: 'cup',
    journee: null,
    phase: 'Phase de Poules',
    round: 'Round 1',
    team1: {
      id: 'team-003',
      name: 'Équipe 1 - Herbiers',
      logo: '/images/clubs/herbiers.png',
      players: [],
      rank: 3,
      seed: 3,
    },
    team2: {
      id: 'team-001',
      name: 'Équipe 1 - Essarts',
      logo: '/images/clubs/essarts.png',
      players: [],
      rank: 1,
      seed: 1,
    },
    team1Score: null,
    team2Score: null,
    winner: null,
    manches: null,
    bestOf: 3,
    currentManche: null,
    currentScores: null,
    status: 'scheduled',
    verified: false,
    verifiedBy: null,
    verifiedAt: null,
    date: new Date('2026-10-16T16:00:00'),
    time: '16:00',
    location: 'Salle de Palet - Les Herbiers',
    duration: null,
    canEdit: false,
    isLive: false,
    isRecent: false,
    stats: {
      totalManches: 0,
      avgPointsPerManche: 0,
      bestManche: null,
      playerStats: [],
    },
    startedAt: null,
    endedAt: null,
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

const TournamentDetail: React.FC = () => {
  const { tournamentId } = useParams<{ tournamentId: string }>();

  const { data: tournament, isLoading } = useQuery<Tournament>({
    queryKey: ['tournament', tournamentId],
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
            <p className="text-body-m text-primary-200">Chargement...</p>
          </div>
        </main>
      </Layout>
    );
  }

  // Calculate status colors and labels
  const getStatusBadge = () => {
    switch (displayTournament.status) {
      case 'pending':
        return <Badge variant="default">À venir</Badge>;
      case 'registration':
        return <Badge variant="info">Inscriptions ouvertes</Badge>;
      case 'in_progress':
        return <Badge variant="warning">En cours</Badge>;
      case 'completed':
        return <Badge variant="success">Terminé</Badge>;
      case 'cancelled':
        return <Badge variant="danger">Annulé</Badge>;
      default:
        return <Badge variant="default">{displayTournament.status}</Badge>;
    }
  };

  // Calculate progress
  const getRegistrationProgress = () => {
    const max = displayTournament.maxTeams || 100;
    const registered = displayTournament.registeredTeams || 0;
    return Math.min((registered / max) * 100, 100);
  };

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
              {/* Logo/Icon */}
              <div className="flex-shrink-0">
                <div className="w-24 h-24 bg-primary-800 rounded-full flex items-center justify-center border-2 border-gold-700">
                  <span className="text-4xl text-gold-700">🏆</span>
                </div>
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayTournament.name}
                  </h1>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  {getStatusBadge()}
                  <Badge variant="default">{displayTournament.type}</Badge>
                  <Badge variant="default">{displayTournament.category}</Badge>
                  <Badge variant="default">{displayTournament.material}</Badge>
                  <Badge variant="default">{displayTournament.season}</Badge>
                </div>

                <p className="text-body-m text-primary-200 mb-4 max-w-2xl">
                  {displayTournament.description}
                </p>

                {/* Dates and Location */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Début des inscriptions:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayTournament.registrationStart?.toLocaleDateString('fr-FR') || 'À définir'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Fin des inscriptions:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayTournament.registrationEnd?.toLocaleDateString('fr-FR') || 'À définir'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Dates du tournoi:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayTournament.startDate?.toLocaleDateString('fr-FR') || 'À définir'} - 
                      {displayTournament.endDate?.toLocaleDateString('fr-FR') || 'À définir'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 md:col-span-3">
                    <span className="text-primary-400">Lieu:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayTournament.location || 'À définir'}
                    </span>
                  </div>
                </div>
              </div>

              {/* Code and Organizer */}
              <div className="flex-shrink-0">
                <div className="space-y-4">
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Code Tournoi</p>
                    <p className="text-heading-m font-bold text-gold-700">{displayTournament.code}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Organisateur</p>
                    {displayTournament.organizer && 'name' in displayTournament.organizer && (
                      <p className="text-heading-xs font-bold text-gold-700">
                        {(displayTournament.organizer as any).name}
                      </p>
                    )}
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
              Informations
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Équipes Inscrites</p>
                  <p className="text-heading-m font-bold text-gold-700">
                    {displayTournament.registeredTeams} / {displayTournament.maxTeams || 'Illimité'}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Format</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTournament.format}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Frais d'inscription</p>
                  <p className="text-heading-m font-bold text-gold-700">
                    {displayTournament.fee ? `${displayTournament.fee} €` : 'Gratuit'}
                  </p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Points par manche</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayTournament.pointsType}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50 md:col-span-2">
                <CardContent className="p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Paiement requis</p>
                  <p className="text-heading-s font-bold text-gold-700">
                    {displayTournament.paymentRequired ? 'Oui' : 'Non'}
                  </p>
                  {displayTournament.paymentRequired && (
                    <div className="mt-2">
                      <p className="text-body-xs text-primary-400 mb-1">Méthodes acceptées:</p>
                      <div className="flex flex-wrap gap-1">
                        {displayTournament.paymentMethods?.map((method) => (
                          <Badge key={method} variant="default" size="xs">
                            {method === 'card' && 'Carte bancaire'}
                            {method === 'paypal' && 'PayPal'}
                            {method === 'transfer' && 'Virement'}
                            {method === 'later' && 'Sur place'}
                            {method !== 'card' && method !== 'paypal' && method !== 'transfer' && method !== 'later' && method}
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                </CardContent>
              </Card>
            </div>

            {/* Registration Progress */}
            {displayTournament.status === 'registration' && (
              <div className="mt-6">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-body-s text-primary-300">
                    Progression des inscriptions: {displayTournament.registeredTeams}/{displayTournament.maxTeams || 'Illimité'}
                  </span>
                  <span className="text-body-s text-gold-700">
                    {getRegistrationProgress().toFixed(0)}%
                  </span>
                </div>
                <div className="h-4 bg-primary-700 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-gold-700 to-gold-500 rounded-full"
                    style={{ width: `${getRegistrationProgress()}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        </section>

        {/* Eligibility */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              Critères d'Éligibilité
            </h2>
            <Card className="bg-primary-900/50 border-primary-700/50">
              <CardContent className="p-6">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Licence requise:</span>
                    <Badge variant={displayTournament.eligibility.requireLicense ? 'success' : 'danger'}>
                      {displayTournament.eligibility.requireLicense ? 'Oui' : 'Non'}
                    </Badge>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Club requis:</span>
                    <Badge variant={displayTournament.eligibility.requireClub ? 'success' : 'danger'}>
                      {displayTournament.eligibility.requireClub ? 'Oui' : 'Non'}
                    </Badge>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Catégories:</span>
                    <div className="flex flex-wrap gap-1">
                      {displayTournament.eligibility.categories.map((cat) => (
                        <Badge key={cat} variant="default" size="xs">
                          {cat === 'individual' && 'Individuel'}
                          {cat === 'doublette' && 'Doublette'}
                          {cat === 'triplette' && 'Triplette'}
                          {cat !== 'individual' && cat !== 'doublette' && cat !== 'triplette' && cat}
                        </Badge>
                      ))}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Divisions:</span>
                    <div className="flex flex-wrap gap-1">
                      {displayTournament.eligibility.divisions.map((div) => (
                        <Badge key={div} variant="default" size="xs">{div}</Badge>
                      ))}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Joueurs max par club:</span>
                    <span className="text-gold-700 font-semibold">
                      {displayTournament.eligibility.maxPlayersPerClub || 'Illimité'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Joueurs par équipe:</span>
                    <span className="text-gold-700 font-semibold">
                      {displayTournament.eligibility.minPlayers}-{displayTournament.eligibility.maxPlayers}
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="equipes">
              <TabList>
                <Tab value="equipes" label="Équipes Inscrites ({displayTournament.registeredTeams || 0})" />
                <Tab value="phases" label="Phases du Tournoi" />
                <Tab value="matchs" label="Matchs ({displayTournament.stats.totalMatches || 0})" />
                <Tab value="regles" label="Règlement" />
              </TabList>

              {/* Équipes Inscrites */}
              <TabPanel value="equipes" className="py-6">
                {displayTournament.registeredTeams > 0 ? (
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {mockTeams.map((team) => (
                      <Card key={team.id} className="bg-primary-900/50 border-primary-700/50">
                        <CardHeader>
                          <div className="flex items-center gap-3">
                            <TeamAvatar team={{ id: team.id, name: team.name, logo: team.logo ?? undefined, abbreviation: (team.club as any)?.code }} size="m" />
                            <div className="flex-1">
                              <h3 className="text-heading-s font-bold text-white">{team.name}</h3>
                              <p className="text-body-xs text-primary-300">
                                {(team.club as any)?.name}
                              </p>
                            </div>
                            {displayTournament.status === 'registration' && (
                              <Badge variant="success">Inscrit</Badge>
                            )}
                          </div>
                        </CardHeader>
                        <CardContent>
                          <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300">Matchs</p>
                              <p className="text-heading-xs font-bold text-gold-700">{team.stats.matches}</p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300">Victoires</p>
                              <p className="text-heading-xs font-bold text-gold-700">{team.stats.wins}</p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300">Défaites</p>
                              <p className="text-heading-xs font-bold text-gold-700">{team.stats.losses}</p>
                            </div>
                            <div className="text-center">
                              <p className="text-body-xs text-primary-300">Win Rate</p>
                              <p className="text-heading-xs font-bold text-gold-700">{team.stats.winRate}%</p>
                            </div>
                          </div>
                          <Link to={`/teams/${team.id}`} className="inline-block mt-4">
                            <Button variant="outline" size="sm" fullWidth>
                              Voir l'équipe
                            </Button>
                          </Link>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                ) : (
                  <Card className="bg-primary-900/50 border-primary-700/50">
                    <CardContent className="text-center p-8">
                      <p className="text-body-m text-primary-300">
                        Aucune équipe inscrite pour le moment.
                      </p>
                    </CardContent>
                  </Card>
                )}
              </TabPanel>

              {/* Phases du Tournoi */}
              <TabPanel value="phases" className="py-6">
                <div className="space-y-4">
                  {displayTournament.phases.map((phase) => (
                    <Card
                      key={phase.id}
                      className="bg-primary-900/50 border-primary-700/50"
                    >
                      <CardHeader>
                        <div className="flex items-center justify-between">
                          <h3 className="text-heading-s font-bold text-white">
                            {phase.order}. {phase.name}
                          </h3>
                          <Badge variant="default">
                            {phase.type === 'group' && 'Poules'}
                            {phase.type === 'knockout' && 'Élimination directe'}
                            {phase.type === 'final' && 'Finale'}
                          </Badge>
                        </div>
                      </CardHeader>
                      <CardContent>
                        {phase.type === 'group' && phase.groups && (
                          <div className="space-y-4">
                            {phase.groups.map((group) => (
                              <div
                                key={group.id}
                                className="bg-primary-800/50 rounded p-3 border border-primary-700/30"
                              >
                                <h4 className="text-heading-xs font-bold text-gold-700 mb-2">
                                  {group.name}
                                </h4>
                                <div className="space-y-2">
                                  {group.teams.map((team, index) => (
                                    <div
                                      key={team.id}
                                      className="flex items-center gap-2 text-body-s"
                                    >
                                      <span className="text-primary-400">{index + 1}.</span>
                                      <TeamAvatar team={{ id: team.id, name: team.name, logo: team.logo ?? undefined, abbreviation: (team.club as any)?.code }} size="xs" />
                                      <span className="text-white">{team.name}</span>
                                    </div>
                                  ))}
                                </div>
                              </div>
                            ))}
                          </div>
                        )}
                        {phase.type === 'knockout' && (
                          <div className="space-y-2">
                            <p className="text-body-s text-primary-300">
                              {phase.matches?.length || 0} matchs prévus en élimination directe
                            </p>
                          </div>
                        )}
                        <p className="text-body-xs text-primary-400 mt-4">
                          {phase.matches?.length || 0} matchs prévus dans cette phase
                        </p>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </TabPanel>

              {/* Matchs */}
              <TabPanel value="matchs" className="py-6">
                <div className="space-y-4">
                  {mockMatches.map((match) => (
                    <MatchCard
                      key={match.id}
                      match={match}
                      
                      showTournament={false}
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

              {/* Règlement */}
              <TabPanel value="regles" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Règlement Officiel</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="prose prose-invert max-w-none text-body-s text-primary-200">
                      {displayTournament.rules ? (
                        <p>{displayTournament.rules}</p>
                      ) : (
                        <p>Le règlement complet sera disponible bientôt.</p>
                      )}
                      
                      <div className="mt-6 space-y-4">
                        <div>
                          <h4 className="text-heading-xs font-bold text-gold-700 mb-2">
                            Points par manche
                          </h4>
                          <p className="text-body-s text-primary-300">
                            Les matchs se jouent en {displayTournament.pointsType} points.
                          </p>
                        </div>
                        <div>
                          <h4 className="text-heading-xs font-bold text-gold-700 mb-2">
                            Matériel
                          </h4>
                          <p className="text-body-s text-primary-300">
                            {displayTournament.material === 'fonte' && 'Palets en fonte - Distance: 3,80m'}
                            {displayTournament.material === 'laiton' && 'Palets en laiton - Distance: 2,80m'}
                            {displayTournament.material === 'bois' && 'Palets en bois - Distance variable'}
                            {displayTournament.material === 'mixte' && 'Matériel mixte selon les phases'}
                          </p>
                        </div>
                        <div>
                          <h4 className="text-heading-xs font-bold text-gold-700 mb-2">
                            Format
                          </h4>
                          <p className="text-body-s text-primary-300">
                            {displayTournament.format === 'knockout' && 'Tournoi en élimination directe'}
                            {displayTournament.format === 'round-robin' && 'Tournoi en round-robin (tous contre tous)'}
                            {displayTournament.format === 'groups' && 'Phase de poules suivie d\'une phase finale'}
                            {displayTournament.format === 'league' && 'Ligue avec classement final'}
                          </p>
                        </div>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>
            </Tabs>
          </div>
        </section>

        {/* Action Buttons */}
        {displayTournament.status === 'registration' && (
          <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
            <div className="max-w-7xl mx-auto">
              <div className="flex justify-center gap-4">
                <Button variant="primary" size="l">
                  S'inscrire au tournoi
                </Button>
                <Button variant="outline" size="l">
                  Voir les équipes inscrites
                </Button>
              </div>
            </div>
          </section>
        )}
      </main>
    </Layout>
  );
};

export default TournamentDetail;
