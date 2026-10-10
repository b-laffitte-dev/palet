import React from 'react';
import {
  Layout,
  MatchHero,
  LiveScores,
  ClassificationTable,
  TournamentCard,
  PlayerCard
} from '../components';
import { Tournament, ChampionshipClassification, Match, Player, Competition, Team } from '../types';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';
import { LiveScoreMatch } from '../components/domain/LiveScores';

// Mock competition
const mockCompetition: Competition = {
  id: 'comp-1',
  name: 'FINALE DU CHAMPIONNAT DE VENDÉE',
  code: 'FINAL-24',
  type: 'championship',
  category: 'doublette',
  material: 'fonte',
  season: '2024-2025',
  description: 'Finale du championnat — Division 1',
  location: 'La Roche-sur-Yon',
  divisions: ['D1'],
  format: 'knockout',
  registrationStart: null,
  registrationEnd: null,
  startDate: new Date(),
  endDate: new Date(),
  status: 'in_progress',
  currentJournee: 15,
  totalJournees: 18,
  currentPhase: 'Finale',
  maxTeams: null,
  registeredTeams: 0,
  teams: [],
  pointsType: '15',
  winPoints: 2,
  drawPoints: 1,
  lossPoints: 0,
  autoRegistration: false,
  manualValidation: false,
  isPublic: true,
  isArchived: false,
  stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
  organizer: { id: 'c-cvdp', name: 'CVDP', nickname: null, code: 'CVDP', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'Fédération', status: 'active', playersCount: 0, teamsCount: 0, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
  organizerId: 'c-cvdp',
  createdAt: new Date(),
  updatedAt: new Date()
};

// Mock teams
const mockTeam1: Team = {
  id: 'team-1',
  name: 'La Roche-sur-Yon PC',
  club: { id: 'c-lr', name: 'La Roche-sur-Yon PC', nickname: null, code: 'LR', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 40, teamsCount: 8, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
  clubId: 'c-lr',
  captain: null,
  captainId: null,
  players: [],
  logo: null,
  stats: { matches: 0, wins: 0, draws: 0, losses: 0, pointsFor: 0, pointsAgainst: 0, diff: 0, points: 0, winRate: 0 },
  status: 'active',
  competitions: [],
  createdAt: new Date(),
  updatedAt: new Date()
};

const mockTeam2: Team = {
  id: 'team-2',
  name: 'Clos Fontenois',
  club: { id: 'c-cf', name: 'Clos Fontenois', nickname: null, code: 'CF', logo: null, address: null, city: 'Clos Fontenay', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 35, teamsCount: 7, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
  clubId: 'c-cf',
  captain: null,
  captainId: null,
  players: [],
  logo: null,
  stats: { matches: 0, wins: 0, draws: 0, losses: 0, pointsFor: 0, pointsAgainst: 0, diff: 0, points: 0, winRate: 0 },
  status: 'active',
  competitions: [],
  createdAt: new Date(),
  updatedAt: new Date()
};

// Mock data for development
const mockFeaturedMatch: Match = {
  id: 'match-1',
  competitionId: 'comp-1',
  competition: mockCompetition,
  type: 'championship',
  journee: 15,
  phase: 'Finale',
  round: null,
  team1: { id: 'team-1', name: 'LA ROCHE-SUR-YON', logo: null, players: [], rank: null, seed: null },
  team2: { id: 'team-2', name: 'CLOS FONTENAIS', logo: null, players: [], rank: null, seed: null },
  team1Score: 72,
  team2Score: 68,
  winner: null,
  manches: null,
  bestOf: null,
  currentManche: 24,
  currentScores: null,
  status: 'in_progress',
  verified: false,
  verifiedBy: null,
  verifiedAt: null,
  date: new Date('2025-06-14'),
  time: '15h00',
  location: 'Terrain clos de La Roche-sur-Yon',
  duration: null,
  canEdit: false,
  isLive: true,
  isRecent: false,
  stats: { totalManches: 24, avgPointsPerManche: 11.5, bestManche: null, playerStats: [] },
  startedAt: new Date(),
  endedAt: null,
  createdAt: new Date(),
  updatedAt: new Date()
};

const mockLiveScores = [
  { id: 's-1', team1Name: 'Luçon PC', team2Name: 'Les Sables', team1Score: 45, team2Score: 38, status: 'in_progress' as const, currentManche: 18 },
  { id: 's-2', team1Name: 'Challans', team2Name: 'Montaigu', team1Score: 52, team2Score: 52, status: 'in_progress' as const, currentManche: 21 },
  { id: 's-3', team1Name: 'Saint-Gilles', team2Name: 'Pouzaugues', team1Score: 61, team2Score: 44, status: 'completed' as const, currentManche: 24 },
];

const mockClassification: ChampionshipClassification = {
  division: 'Division 1',
  journee: 12,
  updatedAt: new Date(),
  teams: [
    {
      position: 1,
      team: { id: 't1', name: 'La Roche-sur-Yon PC', players: [], captain: null, captainId: null, club: { id: 'c-lr', name: 'La Roche-sur-Yon PC', nickname: null, code: 'LR', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 40, teamsCount: 8, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() }, clubId: 'c-lr', logo: null, stats: { matches: 12, wins: 10, draws: 0, losses: 2, pointsFor: 400, pointsAgainst: 316, diff: 84, points: 30, winRate: 0.83 }, status: 'active', competitions: [], createdAt: new Date(), updatedAt: new Date() },
      stats: { matches: 12, wins: 10, draws: 0, losses: 2, pointsFor: 400, pointsAgainst: 316, diff: 84, points: 30, winRate: 0.83 },
      form: ['win', 'win', 'win', 'win', 'loss'],
      status: 'qualified' as const
    },
    {
      position: 2,
      team: { id: 't2', name: 'Clos Fontenois', players: [], captain: null, captainId: null, club: { id: 'c-cf', name: 'Clos Fontenois', nickname: null, code: 'CF', logo: null, address: null, city: 'Clos Fontenay', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 35, teamsCount: 7, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() }, clubId: 'c-cf', logo: null, stats: { matches: 12, wins: 9, draws: 0, losses: 3, pointsFor: 380, pointsAgainst: 319, diff: 61, points: 28, winRate: 0.75 }, status: 'active', competitions: [], createdAt: new Date(), updatedAt: new Date() },
      stats: { matches: 12, wins: 9, draws: 0, losses: 3, pointsFor: 380, pointsAgainst: 319, diff: 61, points: 28, winRate: 0.75 },
      form: ['win', 'win', 'loss', 'win', 'win'],
      status: 'qualified' as const
    },
    {
      position: 3,
      team: { id: 't3', name: 'Luçon Palet Club', players: [], captain: null, captainId: null, club: { id: 'c-lu', name: 'Luçon Palet Club', nickname: null, code: 'LU', logo: null, address: null, city: 'Luçon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 30, teamsCount: 6, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() }, clubId: 'c-lu', logo: null, stats: { matches: 12, wins: 8, draws: 0, losses: 4, pointsFor: 360, pointsAgainst: 322, diff: 38, points: 26, winRate: 0.67 }, status: 'active', competitions: [], createdAt: new Date(), updatedAt: new Date() },
      stats: { matches: 12, wins: 8, draws: 0, losses: 4, pointsFor: 360, pointsAgainst: 322, diff: 38, points: 26, winRate: 0.67 },
      form: ['win', 'draw', 'win', 'win', 'loss'],
      status: 'barrage' as const
    },
    {
      position: 4,
      team: { id: 't4', name: 'Les Sables', players: [], captain: null, captainId: null, club: { id: 'c-ls', name: 'Les Sables', nickname: null, code: 'LS', logo: null, address: null, city: 'Les Sables', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 25, teamsCount: 5, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() }, clubId: 'c-ls', logo: null, stats: { matches: 12, wins: 7, draws: 0, losses: 5, pointsFor: 320, pointsAgainst: 300, diff: 20, points: 24, winRate: 0.58 }, status: 'active', competitions: [], createdAt: new Date(), updatedAt: new Date() },
      stats: { matches: 12, wins: 7, draws: 0, losses: 5, pointsFor: 320, pointsAgainst: 300, diff: 20, points: 24, winRate: 0.58 },
      form: ['loss', 'win', 'win', 'draw', 'win'],
      status: 'safe' as const
    },
  ]
};

const mockTournaments: Tournament[] = [
  {
    id: 'tour-1',
    name: 'Open de Luçon',
    code: 'OL25',
    type: 'tournament',
    category: 'individual',
    material: 'fonte',
    season: '2025',
    description: 'Open National',
    location: 'Terrain clos de Luçon (85)',
    divisions: ['D1'],
    format: 'knockout',
    registrationStart: new Date('2025-06-01'),
    registrationEnd: new Date('2025-07-05'),
    startDate: new Date('2025-07-06'),
    endDate: new Date('2025-07-06'),
    status: 'registration',
    currentJournee: null,
    totalJournees: null,
    currentPhase: null,
    maxTeams: 64,
    registeredTeams: 32,
    teams: [],
    pointsType: '13',
    winPoints: 2,
    drawPoints: 1,
    lossPoints: 0,
    autoRegistration: true,
    manualValidation: false,
    isPublic: true,
    isArchived: false,
    stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
    organizer: { id: 'c-lucon', name: 'Luçon Palet Club', nickname: null, code: 'LU', logo: null, address: null, city: 'Luçon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 30, teamsCount: 6, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    organizerId: 'c-lucon',
    createdAt: new Date(),
    updatedAt: new Date(),
    phases: [],
    brackets: null,
    fee: 10,
    paymentRequired: true,
    paymentMethods: ['card', 'paypal'],
    rules: null,
    eligibility: { requireLicense: true, requireClub: true, categories: ['individual'], divisions: ['D1'], maxPlayersPerClub: 2, minPlayers: 1, maxPlayers: 64 }
  },
  {
    id: 'tour-2',
    name: 'Championnat D2 — J13',
    code: 'CD2-J13',
    type: 'tournament',
    category: 'doublette',
    material: 'fonte',
    season: '2024-2025',
    description: 'Championnat Départemental',
    location: 'Saint-Gilles-Croix-de-Vie (85)',
    divisions: ['D2'],
    format: 'league',
    registrationStart: null,
    registrationEnd: null,
    startDate: new Date('2025-06-29'),
    endDate: new Date('2025-06-29'),
    status: 'in_progress',
    currentJournee: 13,
    totalJournees: 18,
    currentPhase: 'League',
    maxTeams: 48,
    registeredTeams: 48,
    teams: [],
    pointsType: '13',
    winPoints: 2,
    drawPoints: 1,
    lossPoints: 0,
    autoRegistration: false,
    manualValidation: true,
    isPublic: true,
    isArchived: false,
    stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
    organizer: { id: 'c-cvdp', name: 'CVDP', nickname: null, code: 'CVDP', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'Fédération', status: 'active', playersCount: 0, teamsCount: 0, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    organizerId: 'c-cvdp',
    createdAt: new Date(),
    updatedAt: new Date(),
    phases: [],
    brackets: null,
    fee: null,
    paymentRequired: false,
    paymentMethods: [],
    rules: null,
    eligibility: { requireLicense: true, requireClub: true, categories: ['doublette'], divisions: ['D2'], maxPlayersPerClub: null, minPlayers: 2, maxPlayers: 48 }
  },
  {
    id: 'tour-3',
    name: 'Internationaux de Vendée',
    code: 'IV25',
    type: 'cup',
    category: 'individual',
    material: 'fonte',
    season: '2025',
    description: 'Événement Fédéral',
    location: 'La Roche-sur-Yon — 4 terrains',
    divisions: ['D1', 'D2'],
    format: 'knockout',
    registrationStart: null,
    registrationEnd: null,
    startDate: new Date('2025-08-15'),
    endDate: new Date('2025-08-17'),
    status: 'completed',
    currentJournee: null,
    totalJournees: null,
    currentPhase: null,
    maxTeams: 128,
    registeredTeams: 128,
    teams: [],
    pointsType: '15',
    winPoints: 2,
    drawPoints: 1,
    lossPoints: 0,
    autoRegistration: false,
    manualValidation: true,
    isPublic: true,
    isArchived: false,
    stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
    organizer: { id: 'c-cvdp', name: 'CVDP', nickname: null, code: 'CVDP', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'Fédération', status: 'active', playersCount: 0, teamsCount: 0, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    organizerId: 'c-cvdp',
    createdAt: new Date(),
    updatedAt: new Date(),
    fee: 20,
    paymentRequired: true,
    paymentMethods: ['card'],
    rules: null,
    eligibility: { requireLicense: true, requireClub: true, categories: ['individual'], divisions: ['D1', 'D2'], maxPlayersPerClub: 3, minPlayers: 1, maxPlayers: 128 },
    phases: [],
    brackets: null
  }
];

const mockPlayers: Player[] = [
  {
    id: 'p1', firstName: 'Jean', lastName: 'Morice', fullName: 'Jean Morice', alias: null, email: 'jean@test.com', photo: null,
    status: 'active', birthDate: new Date('1990-01-01'), age: 35, birthPlace: null, nationality: 'FR',
    dominantHand: 'right', playStyle: null, phone: null, address: null, city: null, postalCode: null,
    club: { id: 'c1', name: 'La Roche-sur-Yon PC', nickname: null, code: 'LR', logo: null, address: null, city: 'La Roche-sur-Yon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 40, teamsCount: 8, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    clubId: 'c1', clubNumber: null, category: 'senior', licenseNumber: 'V001', licenseDate: new Date(), licenseStatus: 'valid', registrationDate: new Date(),
    stats: { matchesPlayed: 20, wins: 14, draws: 2, losses: 4, totalPoints: 412, averagePoints: 20.6, pointsPerManche: 11.4, bestScore: 85, bestStreak: 5, maxWinStreak: 5, winRate: 0.68, accuracy: 0.72, currentRank: 1, currentDivision: 'D1', currentPoints: 30, previousRank: 2, rankTrend: 'up', newHighlights: 3, byPosition: { first: { matches: 0, points: 0 }, second: { matches: 0, points: 0 } }, byCompetition: {}, bySeason: {} },
    trophies: { total: 5, list: [] }, badges: { total: 3, categories: [] }, records: { total: 2, list: [] },
    currentTeams: [], pastTeams: [], frequentTeammates: [], matchHistory: { wins: 14, losses: 4, draws: 2, total: 20, recentMatches: [], byCompetition: {}, bySeason: {} },
    competitionStats: [], recentActivities: [], preferences: { language: 'fr', timezone: 'Europe/Paris', distanceUnit: 'metric', notifications: { email: { results: true, news: true, reminders: true }, push: { results: true, matches: true } } }, social: { facebook: null, twitter: null, instagram: null }, lastActiveAt: new Date(), createdAt: new Date(), updatedAt: new Date()
  },
  {
    id: 'p2', firstName: 'Pierre', lastName: 'Baudry', fullName: 'Pierre Baudry', alias: null, email: 'pierre@test.com', photo: null,
    status: 'active', birthDate: new Date('1992-05-15'), age: 33, birthPlace: null, nationality: 'FR',
    dominantHand: 'right', playStyle: null, phone: null, address: null, city: null, postalCode: null,
    club: { id: 'c2', name: 'Clos Fontenois', nickname: null, code: 'CF', logo: null, address: null, city: 'Clos Fontenay', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 35, teamsCount: 7, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    clubId: 'c2', clubNumber: null, category: 'senior', licenseNumber: 'V002', licenseDate: new Date(), licenseStatus: 'valid', registrationDate: new Date(),
    stats: { matchesPlayed: 22, wins: 16, draws: 1, losses: 5, totalPoints: 445, averagePoints: 20.2, pointsPerManche: 10.8, bestScore: 92, bestStreak: 7, maxWinStreak: 7, winRate: 0.71, accuracy: 0.74, currentRank: 2, currentDivision: 'D1', currentPoints: 28, previousRank: 1, rankTrend: 'down', newHighlights: 2, byPosition: { first: { matches: 0, points: 0 }, second: { matches: 0, points: 0 } }, byCompetition: {}, bySeason: {} },
    trophies: { total: 3, list: [] }, badges: { total: 2, categories: [] }, records: { total: 1, list: [] },
    currentTeams: [], pastTeams: [], frequentTeammates: [], matchHistory: { wins: 16, losses: 5, draws: 1, total: 22, recentMatches: [], byCompetition: {}, bySeason: {} },
    competitionStats: [], recentActivities: [], preferences: { language: 'fr', timezone: 'Europe/Paris', distanceUnit: 'metric', notifications: { email: { results: true, news: true, reminders: true }, push: { results: true, matches: true } } }, social: { facebook: null, twitter: null, instagram: null }, lastActiveAt: new Date(), createdAt: new Date(), updatedAt: new Date()
  },
  {
    id: 'p3', firstName: 'Anaïs', lastName: 'Léger', fullName: 'Anaïs Léger', alias: null, email: 'anais@test.com', photo: null,
    status: 'active', birthDate: new Date('1995-08-20'), age: 30, birthPlace: null, nationality: 'FR',
    dominantHand: 'right', playStyle: null, phone: null, address: null, city: null, postalCode: null,
    club: { id: 'c3', name: 'Luçon Palet Club', nickname: null, code: 'LU', logo: null, address: null, city: 'Luçon', postalCode: null, league: 'Vendée', division: 'D1', status: 'active', playersCount: 30, teamsCount: 6, contactEmail: null, contactPhone: null, website: null, social: { facebook: null, twitter: null, instagram: null }, createdAt: new Date(), updatedAt: new Date() },
    clubId: 'c3', clubNumber: null, category: 'senior', licenseNumber: 'V003', licenseDate: new Date(), licenseStatus: 'valid', registrationDate: new Date(),
    stats: { matchesPlayed: 18, wins: 12, draws: 3, losses: 3, totalPoints: 398, averagePoints: 22.1, pointsPerManche: 11.1, bestScore: 78, bestStreak: 6, maxWinStreak: 6, winRate: 0.71, accuracy: 0.76, currentRank: 3, currentDivision: 'D1', currentPoints: 26, previousRank: 4, rankTrend: 'up', newHighlights: 5, byPosition: { first: { matches: 0, points: 0 }, second: { matches: 0, points: 0 } }, byCompetition: {}, bySeason: {} },
    trophies: { total: 2, list: [] }, badges: { total: 4, categories: [] }, records: { total: 3, list: [] },
    currentTeams: [], pastTeams: [], frequentTeammates: [], matchHistory: { wins: 12, losses: 3, draws: 3, total: 18, recentMatches: [], byCompetition: {}, bySeason: {} },
    competitionStats: [], recentActivities: [], preferences: { language: 'fr', timezone: 'Europe/Paris', distanceUnit: 'metric', notifications: { email: { results: true, news: true, reminders: true }, push: { results: true, matches: true } } }, social: { facebook: null, twitter: null, instagram: null }, lastActiveAt: new Date(), createdAt: new Date(), updatedAt: new Date()
  }
];

/**
 * Home Page - Main landing page for Palet Vendéen application
 * Displays live matches, scores, classification, tournaments, and top players
 */
const Home: React.FC = () => {
  // Use mock data directly - disable API calls until backend is ready
  const featuredMatch = mockFeaturedMatch;
  const liveScores = mockLiveScores;
  const classification = mockClassification;
  const tournaments = mockTournaments;
  const players = mockPlayers;

  return (
    <Layout>
      <main className="flex-1">
        {/* Hero Match Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8" aria-labelledby="featured-match-heading">
          <h1 className="sr-only">Palet Vendéen - Accueil</h1>
          <div className="max-w-7xl mx-auto">
            {featuredMatch && (
              <MatchHero match={featuredMatch} />
            )}
          </div>
        </section>

        {/* Live Scores Section */}
        <section className="px-4 py-4 lg:px-8 lg:py-6" aria-labelledby="live-scores-heading">
          <div className="max-w-7xl mx-auto">
            {liveScores && liveScores.length > 0 && (
              <LiveScores matches={liveScores} journee={12} />
            )}
          </div>
        </section>

        {/* Classification Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8" aria-labelledby="classification-heading">
          <div className="max-w-7xl mx-auto">
            {classification && (
              <ClassificationTable
                teams={classification.teams}
                competition="CHAMPIONNAT — DIVISION 1"
                journee={classification.journee}
                season="2024-2025"
                showAllLink={true}
                onAllClick={() => { /* Navigate to classification page */ }}
              />
            )}
          </div>
        </section>

        {/* Tournaments Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8" aria-labelledby="tournaments-heading">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <div>
                <h2 
                  id="tournaments-heading"
                  className="text-heading-l font-bold text-primary uppercase tracking-wide"
                >
                  TOURNOIS À VENIR
                </h2>
                <p className="text-body-m text-secondary">
                  Open, qualificatifs et championnats
                </p>
              </div>
              <Button variant="ghost" size="sm">
                <Icon name="ArrowRight" size="sm" />
              </Button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {tournaments && tournaments.slice(0, 3).map((tournament) => (
                <TournamentCard 
                  key={tournament.id} 
                  tournament={tournament} 
                />
              ))}
            </div>
          </div>
        </section>

        {/* Best Players Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8" aria-labelledby="players-heading">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <div>
                <h2 
                  id="players-heading"
                  className="text-heading-l font-bold text-primary uppercase tracking-wide"
                >
                  LES MEILLEURS JOUEURS
                </h2>
                <p className="text-body-m text-secondary">
                  Statistiques de la saison
                </p>
              </div>
              <Button variant="ghost" size="sm">
                <Icon name="ArrowRight" size="sm" />
              </Button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {players && players.slice(0, 3).map((player, index) => (
                <PlayerCard 
                  key={player.id} 
                  player={player} 
                  accent={index === 0 ? 'gold' : index === 1 ? 'primary' : 'amber'}
                />
              ))}
            </div>
          </div>
        </section>

        {/* Spaces Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-gold/10" aria-labelledby="spaces-heading">
          <div className="max-w-7xl mx-auto">
            <div className="mb-6">
              <h2 
                id="spaces-heading"
                className="text-heading-l font-bold text-gold-500 uppercase tracking-wide text-center"
              >
                NOS ESPACES
              </h2>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              {[
                { icon: 'Target' as const, title: 'Espace Fédération', description: 'Licences, règles, arbitres' },
                { icon: 'Home' as const, title: 'Espace Club', description: 'Effectifs, résultats' },
                { icon: 'User' as const, title: 'Espace Joueur', description: 'Profil, stats, palmarès' },
                { icon: 'Award' as const, title: 'Espace Supporter', description: 'Pronostics, votes' }
              ].map((space, index) => (
                <div 
                  key={index}
                  className="bg-dark rounded-lg p-4 text-center hover:bg-secondary transition-colors border border-border-secondary/50 cursor-pointer"
                >
                  <div className="w-12 h-12 mx-auto mb-3 bg-secondary rounded-lg flex items-center justify-center">
                    <Icon name={space.icon} size="xl" className="text-gold-500" />
                  </div>
                  <h3 className="text-heading-xs font-bold text-primary mb-1">
                    {space.title}
                  </h3>
                  <p className="text-caption text-tertiary">
                    {space.description}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Home;
