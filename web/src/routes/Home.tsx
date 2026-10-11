import React from "react";
import {
  Layout,
  MatchHero,
  LiveScores,
  ClassificationTable,
  TournamentCard,
  PlayerCard,
  ClubCard,
} from "../components";
import { Tournament, ChampionshipClassification, Match, Player, Team } from "../types";
import { Icon } from "../components/common/Icon";
import Button from "../components/common/Button";
import { Link } from "react-router-dom";

// Simplified mock data with proper types
const mockClassification: ChampionshipClassification = {
  division: "Division 1",
  journee: 15,
  updatedAt: new Date(),
  teams: [
    {
      position: 1,
      team: {
        id: "t1",
        name: "Niort Palet Club",
        players: [],
        captain: null,
        captainId: null,
        club: {
          id: "c1",
          name: "Niort Palet",
          nickname: "NPC",
          code: "NPC",
          logo: null,
          address: null,
          city: "Niort",
          postalCode: "79000",
          league: "Deux-Sèvres",
          division: "D1",
          status: "active",
          playersCount: 40,
          teamsCount: 8,
          contactEmail: null,
          contactPhone: null,
          website: null,
          social: { facebook: null, twitter: null, instagram: null },
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        clubId: "c1",
        logo: null,
        stats: {
          matches: 15,
          wins: 12,
          draws: 1,
          losses: 2,
          pointsFor: 450,
          pointsAgainst: 350,
          diff: 100,
          points: 37,
          winRate: 0.8,
        },
        status: "active",
        competitions: [] as Team["competitions"],
        createdAt: new Date(),
        updatedAt: new Date(),
      },
      stats: {
        matches: 15,
        wins: 12,
        draws: 1,
        losses: 2,
        pointsFor: 450,
        pointsAgainst: 350,
        diff: 100,
        points: 37,
        winRate: 0.8,
      },
      form: ["win", "win", "win", "win", "loss"],
      status: "qualified",
    },
    {
      position: 2,
      team: {
        id: "t2",
        name: "Bressuire Palet",
        players: [],
        captain: null,
        captainId: null,
        club: {
          id: "c2",
          name: "Bressuire Palet",
          nickname: "BP",
          code: "BP",
          logo: null,
          address: null,
          city: "Bressuire",
          postalCode: "79300",
          league: "Deux-Sèvres",
          division: "D1",
          status: "active",
          playersCount: 35,
          teamsCount: 7,
          contactEmail: null,
          contactPhone: null,
          website: null,
          social: { facebook: null, twitter: null, instagram: null },
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        clubId: "c2",
        logo: null,
        stats: {
          matches: 15,
          wins: 10,
          draws: 2,
          losses: 3,
          pointsFor: 420,
          pointsAgainst: 340,
          diff: 80,
          points: 32,
          winRate: 0.67,
        },
        status: "active",
        competitions: [] as Team["competitions"],
        createdAt: new Date(),
        updatedAt: new Date(),
      },
      stats: {
        matches: 15,
        wins: 10,
        draws: 2,
        losses: 3,
        pointsFor: 420,
        pointsAgainst: 340,
        diff: 80,
        points: 32,
        winRate: 0.67,
      },
      form: ["win", "win", "loss", "win", "win"],
      status: "qualified",
    },
    {
      position: 3,
      team: {
        id: "t3",
        name: "Thouars Palet",
        players: [],
        captain: null,
        captainId: null,
        club: {
          id: "c3",
          name: "Thouars Palet",
          nickname: "TP",
          code: "TP",
          logo: null,
          address: null,
          city: "Thouars",
          postalCode: "79100",
          league: "Deux-Sèvres",
          division: "D1",
          status: "active",
          playersCount: 30,
          teamsCount: 6,
          contactEmail: null,
          contactPhone: null,
          website: null,
          social: { facebook: null, twitter: null, instagram: null },
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        clubId: "c3",
        logo: null,
        stats: {
          matches: 15,
          wins: 8,
          draws: 2,
          losses: 5,
          pointsFor: 360,
          pointsAgainst: 322,
          diff: 38,
          points: 26,
          winRate: 0.53,
        },
        status: "active",
        competitions: [] as Team["competitions"],
        createdAt: new Date(),
        updatedAt: new Date(),
      },
      stats: {
        matches: 15,
        wins: 8,
        draws: 2,
        losses: 5,
        pointsFor: 360,
        pointsAgainst: 322,
        diff: 38,
        points: 26,
        winRate: 0.53,
      },
      form: ["win", "draw", "win", "win", "loss"],
      status: "barrage",
    },
    {
      position: 4,
      team: {
        id: "t4",
        name: "Parthenay Palet",
        players: [],
        captain: null,
        captainId: null,
        club: {
          id: "c4",
          name: "Parthenay Palet",
          nickname: null,
          code: "PP",
          logo: null,
          address: null,
          city: "Parthenay",
          postalCode: "79200",
          league: "Deux-Sèvres",
          division: "D1",
          status: "active",
          playersCount: 25,
          teamsCount: 5,
          contactEmail: null,
          contactPhone: null,
          website: null,
          social: { facebook: null, twitter: null, instagram: null },
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        clubId: "c4",
        logo: null,
        stats: {
          matches: 15,
          wins: 7,
          draws: 1,
          losses: 7,
          pointsFor: 320,
          pointsAgainst: 300,
          diff: 20,
          points: 24,
          winRate: 0.47,
        },
        status: "active",
        competitions: [] as Team["competitions"],
        createdAt: new Date(),
        updatedAt: new Date(),
      },
      stats: {
        matches: 15,
        wins: 7,
        draws: 1,
        losses: 7,
        pointsFor: 320,
        pointsAgainst: 300,
        diff: 20,
        points: 24,
        winRate: 0.47,
      },
      form: ["loss", "win", "win", "draw", "win"],
      status: "safe",
    },
    {
      position: 5,
      team: {
        id: "t5",
        name: "Melle Palet",
        players: [],
        captain: null,
        captainId: null,
        club: {
          id: "c5",
          name: "Melle Palet",
          nickname: null,
          code: "MP",
          logo: null,
          address: null,
          city: "Melle",
          postalCode: "79500",
          league: "Deux-Sèvres",
          division: "D1",
          status: "active",
          playersCount: 20,
          teamsCount: 4,
          contactEmail: null,
          contactPhone: null,
          website: null,
          social: { facebook: null, twitter: null, instagram: null },
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        clubId: "c5",
        logo: null,
        stats: {
          matches: 15,
          wins: 6,
          draws: 0,
          losses: 9,
          pointsFor: 280,
          pointsAgainst: 340,
          diff: -60,
          points: 20,
          winRate: 0.4,
        },
        status: "active",
        competitions: [] as Team["competitions"],
        createdAt: new Date(),
        updatedAt: new Date(),
      },
      stats: {
        matches: 15,
        wins: 6,
        draws: 0,
        losses: 9,
        pointsFor: 280,
        pointsAgainst: 340,
        diff: -60,
        points: 20,
        winRate: 0.4,
      },
      form: ["win", "loss", "win", "loss", "loss"],
      status: "relegated",
    },
  ],
};

const mockFeaturedMatch: Match = {
  id: "match-1",
  competitionId: "comp-1",
  competition: {
    id: "comp-1",
    name: "CHAMPIONNAT 79",
    code: "C79",
    type: "championship",
    category: "doublette",
    material: "fonte",
    season: "2026-2027",
    description: "Championnat Départemental",
    location: "Niort",
    divisions: ["D1"],
    format: "league",
    registrationStart: null,
    registrationEnd: null,
    startDate: new Date(),
    endDate: new Date(),
    status: "in_progress",
    currentJournee: 15,
    totalJournees: 18,
    currentPhase: "League",
    maxTeams: null,
    registeredTeams: 0,
    teams: [],
    pointsType: "15",
    winPoints: 2,
    drawPoints: 1,
    lossPoints: 0,
    autoRegistration: false,
    manualValidation: false,
    isPublic: true,
    isArchived: false,
    stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
    organizer: {
      id: "c-cdsp79",
      name: "CDSP 79",
      nickname: null,
      code: "CDSP79",
      logo: null,
      address: null,
      city: "Niort",
      postalCode: "79000",
      league: "Deux-Sèvres",
      division: "Commission",
      status: "active",
      playersCount: 0,
      teamsCount: 0,
      contactEmail: "cdsp79@sportrural.fr",
      contactPhone: null,
      website: null,
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date(),
      updatedAt: new Date(),
    },
    organizerId: "c-cdsp79",
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  type: "championship",
  journee: 15,
  phase: "Journée 15",
  round: null,
  team1: {
    id: "team-1",
    name: "NIORT PALET",
    logo: null,
    players: [],
    rank: 1,
    seed: 1,
  } as Match["team1"],
  team2: {
    id: "team-2",
    name: "BRESSUIRE PALET",
    logo: null,
    players: [],
    rank: 2,
    seed: 2,
  } as Match["team2"],
  team1Score: 72,
  team2Score: 68,
  winner: null,
  manches: null,
  bestOf: null,
  currentManche: 24,
  currentScores: null,
  status: "in_progress",
  verified: false,
  verifiedBy: null,
  verifiedAt: null,
  date: new Date("2026-10-14"),
  time: "15h00",
  location: "Terrain de Niort",
  duration: null,
  canEdit: false,
  isLive: true,
  isRecent: false,
  stats: { totalManches: 24, avgPointsPerManche: 11.5, bestManche: null, playerStats: [] },
  startedAt: new Date(),
  endedAt: null,
  createdAt: new Date(),
  updatedAt: new Date(),
};

const mockLiveScores = [
  { id: "s-1", team1Name: "Niort PC", team2Name: "Bressuire", team1Score: 45, team2Score: 38, status: "in_progress" as const, currentManche: 18 },
  { id: "s-2", team1Name: "Thouars", team2Name: "Parthenay", team1Score: 52, team2Score: 52, status: "in_progress" as const, currentManche: 21 },
  { id: "s-3", team1Name: "Melle", team2Name: "Coulonges", team1Score: 61, team2Score: 44, status: "completed" as const, currentManche: 24 },
];

const mockTournaments: Tournament[] = [
  {
    id: "tour-1",
    name: "Open de Niort",
    code: "ON26",
    type: "tournament",
    category: "individual",
    material: "fonte",
    season: "2026",
    description: "Open Départemental Deux-Sèvres",
    location: "Terrain de Niort (79)",
    divisions: ["D1"],
    format: "knockout",
    registrationStart: new Date("2026-10-01"),
    registrationEnd: new Date("2026-10-15"),
    startDate: new Date("2026-10-16"),
    endDate: new Date("2026-10-16"),
    status: "registration",
    currentJournee: null,
    totalJournees: null,
    currentPhase: null,
    maxTeams: 64,
    registeredTeams: 32,
    teams: [] as Team[],
    pointsType: "13",
    winPoints: 2,
    drawPoints: 1,
    lossPoints: 0,
    autoRegistration: true,
    manualValidation: false,
    isPublic: true,
    isArchived: false,
    stats: { totalMatches: 0, completedMatches: 0, totalPoints: 0, topScorer: null, topTeam: null },
    organizer: {
      id: "c-cdsp79",
      name: "CDSP 79",
      nickname: null,
      code: "CDSP79",
      logo: null,
      address: null,
      city: "Niort",
      postalCode: "79000",
      league: "Deux-Sèvres",
      division: "Commission",
      status: "active",
      playersCount: 0,
      teamsCount: 0,
      contactEmail: "cdsp79@sportrural.fr",
      contactPhone: null,
      website: null,
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date(),
      updatedAt: new Date(),
    },
    organizerId: "c-cdsp79",
    createdAt: new Date(),
    updatedAt: new Date(),
    phases: [],
    brackets: null,
    fee: 10,
    paymentRequired: true,
    paymentMethods: ["card"],
    rules: null,
    eligibility: {
      requireLicense: true,
      requireClub: true,
      categories: ["individual"],
      divisions: ["D1"],
      maxPlayersPerClub: 2,
      minPlayers: 1,
      maxPlayers: 64,
    },
  },
];

const mockPlayers: Player[] = [
  {
    id: "p1",
    firstName: "Jean",
    lastName: "Morice",
    fullName: "Jean Morice",
    alias: null,
    email: "jean@test.com",
    photo: null,
    status: "active",
    birthDate: new Date("1990-01-01"),
    age: 35,
    birthPlace: null,
    nationality: "FR",
    dominantHand: "right",
    playStyle: null,
    phone: null,
    address: null,
    city: null,
    postalCode: null,
    club: {
      id: "c1",
      name: "Niort Palet",
      nickname: "NPC",
      code: "NPC",
      logo: null,
      address: null,
      city: "Niort",
      postalCode: "79000",
      league: "Deux-Sèvres",
      division: "D1",
      status: "active",
      playersCount: 40,
      teamsCount: 8,
      contactEmail: null,
      contactPhone: null,
      website: null,
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date(),
      updatedAt: new Date(),
    },
    clubId: "c1",
    clubNumber: null,
    category: "senior",
    licenseNumber: "79001",
    licenseDate: new Date(),
    licenseStatus: "valid",
    registrationDate: new Date(),
    stats: {
      matchesPlayed: 20,
      wins: 14,
      draws: 2,
      losses: 4,
      totalPoints: 412,
      averagePoints: 20.6,
      pointsPerManche: 11.4,
      bestScore: 85,
      bestStreak: 5,
      maxWinStreak: 5,
      winRate: 0.68,
      accuracy: 0.72,
      currentRank: 1,
      currentDivision: "D1",
      currentPoints: 30,
      previousRank: 2,
      rankTrend: "up",
      newHighlights: 3,
      byPosition: { first: { matches: 0, points: 0 }, second: { matches: 0, points: 0 } },
      byCompetition: {},
      bySeason: {},
    },
    trophies: { total: 5, list: [] },
    badges: { total: 3, categories: [] },
    records: { total: 2, list: [] },
    currentTeams: [],
    pastTeams: [],
    frequentTeammates: [],
    matchHistory: {
      wins: 14,
      losses: 4,
      draws: 2,
      total: 20,
      recentMatches: [],
      byCompetition: {},
      bySeason: {},
    },
    competitionStats: [],
    recentActivities: [],
    preferences: {
      language: "fr",
      timezone: "Europe/Paris",
      distanceUnit: "metric",
      notifications: {
        email: { results: true, news: true, reminders: true },
        push: { results: true, matches: true },
      },
    },
    social: { facebook: null, twitter: null, instagram: null },
    lastActiveAt: new Date(),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

/**
 * Home Page - Top 14 inspired structure for CDSP 79
 * Layout mimics https://top14.lnr.fr/ with featured matches, classifications, news, etc.
 */
const Home: React.FC = () => {
  const classification = mockClassification;
  const featuredMatch = mockFeaturedMatch;
  const liveScores = mockLiveScores;
  const tournaments = mockTournaments;
  const players = mockPlayers;

  return (
    <Layout hasLiveMatches={liveScores.length > 0} liveMatchesCount={liveScores.length}>
      <div className="flex-1">
        {/* Hero Section - Top 14 style */}
        <section className="w-full bg-gradient-brand relative overflow-hidden" aria-labelledby="hero-heading">
          <div className="max-w-[1440px] mx-auto px-[var(--spacing-4)] py-[var(--spacing-8)] lg:px-[var(--spacing-8)] lg:py-[var(--spacing-16)]">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-[var(--spacing-8)] items-center">
              {/* Hero Content */}
              <div className="text-center lg:text-left">
                <h1
                  id="hero-heading"
                  className="text-[var(--text-display-m)] lg:text-[var(--text-display-l)] font-[var(--font-weight-black)] text-white mb-[var(--spacing-4)] leading-tight"
                >
                  CHAMPIONNAT DÉPARTEMENTAL 79
                </h1>
                <p className="text-[var(--text-body-xl)] lg:text-[var(--text-heading-l)] text-brand-green-400 font-[var(--font-weight-semibold)] mb-[var(--spacing-6)]">
                  Division 1 Palet Fonte — Saison 2026/2027
                </p>
                <div className="flex flex-col sm:flex-row gap-[var(--spacing-4)] justify-center lg:justify-start">
                  <Link
                    to="/classement"
                    className="inline-flex items-center justify-center gap-[var(--spacing-2)] px-[var(--spacing-6)] py-[var(--spacing-3)] bg-brand-green text-black text-[var(--text-body-sm)] font-[var(--font-weight-bold)] rounded-[var(--radius-m)] uppercase tracking-[var(--letter-spacing-wide)] hover:bg-brand-green-400 transition-all duration-200 shadow-lg hover:shadow-xl"
                  >
                    <Icon name="Trophy" size="sm" />
                    Classement
                  </Link>
                  <Link
                    to="/calendrier-et-resultats"
                    className="inline-flex items-center justify-center gap-[var(--spacing-2)] px-[var(--spacing-6)] py-[var(--spacing-3)] bg-transparent border-2 border-brand-green text-brand-green text-[var(--text-body-sm)] font-[var(--font-weight-bold)] rounded-[var(--radius-m)] uppercase tracking-[var(--letter-spacing-wide)] hover:bg-brand-green hover:text-black transition-all duration-200"
                  >
                    <Icon name="Calendar" size="sm" />
                    Calendrier
                  </Link>
                </div>
              </div>

              {/* Featured Match Card */}
              <div className="flex justify-center lg:justify-end">
                {featuredMatch && (
                  <div className="max-w-md w-full">
                    <MatchHero match={featuredMatch} />
                  </div>
                )}
              </div>
            </div>
          </div>
          {/* Decorative elements */}
          <div className="absolute bottom-0 left-0 right-0 h-[100px] pointer-events-none">
            <svg viewBox="0 0 1440 100" className="w-full h-full" preserveAspectRatio="none">
              <path d="M0,100 L720,20 L1440,100 L1440,100 L0,100 Z" fill="var(--bg-dark)" />
            </svg>
          </div>
        </section>

        {/* Live Scores Ticker */}
        {liveScores.length > 0 && (
          <section className="bg-[var(--bg-darker)] border-b border-border-dark py-[var(--spacing-2)] sticky top-[120px] z-[var(--z-sticky)]" aria-labelledby="live-scores-heading">
            <div className="max-w-[1440px] mx-auto px-[var(--spacing-4)]">
              <div className="flex items-center gap-[var(--spacing-4)]">
                <h2
                  id="live-scores-heading"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-bold)] text-brand-green uppercase tracking-[var(--letter-spacing-wider)] whitespace-nowrap"
                >
                  <Icon name="Zap" size="sm" className="inline" />
                  Scores en Direct
                </h2>
                <div className="flex-1 overflow-x-auto">
                  <LiveScores matches={liveScores} />
                </div>
                <Link
                  to="/calendrier-et-resultats"
                  className="text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-text-tertiary hover:text-brand-green transition-colors whitespace-nowrap uppercase tracking-wider"
                >
                  TOUS LES MATCHS
                </Link>
              </div>
            </div>
          </section>
        )}

        {/* Main Content */}
        <div className="max-w-[1440px] mx-auto px-[var(--spacing-4)] py-[var(--spacing-8)] lg:px-[var(--spacing-8)] lg:py-[var(--spacing-8)]">
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-[var(--spacing-8)]">
            {/* Left Column - Classification */}
            <div className="lg:col-span-2">
              <section aria-labelledby="classification-heading">
                <div className="flex items-center justify-between mb-[var(--spacing-4)]">
                  <h2
                    id="classification-heading"
                    className="text-[var(--text-heading-l)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)]"
                  >
                    <Icon name="Trophy" size="m" className="inline mr-[var(--spacing-2)]" />
                    Classement
                  </h2>
                  <Link
                    to="/classement"
                    className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-green-400 hover:text-brand-green transition-colors uppercase tracking-wider"
                  >
                    VOIR TOUT
                  </Link>
                </div>
                <ClassificationTable
                  teams={classification.teams}
                  competition="CHAMPIONNAT 79 — DIVISION 1"
                  journee={classification.journee}
                  season="2026-2027"
                  showAllLink={false}
                  onAllClick={() => {}}
                />
              </section>

              {/* Tournaments Section */}
              <section className="mt-[var(--spacing-12)]" aria-labelledby="tournaments-heading">
                <div className="flex items-center justify-between mb-[var(--spacing-4)]">
                  <h2
                    id="tournaments-heading"
                    className="text-[var(--text-heading-l)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)]"
                  >
                    <Icon name="Award" size="m" className="inline mr-[var(--spacing-2)]" />
                    Compétitions
                  </h2>
                  <Link
                    to="/tournois"
                    className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-green-400 hover:text-brand-green transition-colors uppercase tracking-wider"
                  >
                    VOIR TOUT
                  </Link>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-[var(--spacing-4)]">
                  {tournaments.slice(0, 2).map((tournament) => (
                    <TournamentCard key={tournament.id} tournament={tournament} />
                  ))}
                </div>
              </section>

              {/* News Section */}
              <section className="mt-[var(--spacing-12)] bg-brand-green/30 rounded-[var(--radius-xl)] p-[var(--spacing-6)] lg:p-[var(--spacing-8)]" aria-labelledby="news-heading">
                <div className="flex items-center justify-between mb-[var(--spacing-4)]">
                  <h2
                    id="news-heading"
                    className="text-[var(--text-heading-l)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)]"
                  >
                    <Icon name="FileText" size="m" className="inline mr-[var(--spacing-2)]" />
                    Actualités
                  </h2>
                  <Link
                    to="/actualites"
                    className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-green-400 hover:text-brand-green transition-colors uppercase tracking-wider"
                  >
                    TOUS LES ARTICLES
                  </Link>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-[var(--spacing-4)]">
                  {[1, 2, 3].map((item) => (
                    <div
                      key={item}
                      className="bg-[var(--bg-secondary)] rounded-[var(--radius-lg)] overflow-hidden transition-all duration-200 hover:shadow-lg"
                    >
                      <div className="h-40 bg-gradient-to-br from-brand-green to-brand-green-dark flex items-center justify-center">
                        <Icon name="Calendar" size="xl" className="text-brand-orange opacity-50" />
                      </div>
                      <div className="p-[var(--spacing-4)]">
                        <span className="inline-block px-[var(--spacing-2)] py-[var(--spacing-1)] bg-brand-green/10 text-brand-green text-[var(--text-body-xs)] font-[var(--font-weight-semibold)] rounded-[var(--radius-sm)] uppercase tracking-wider mb-[var(--spacing-2)]">
                          Championnat
                        </span>
                        <h3 className="text-[var(--text-heading-sm)] font-[var(--font-weight-bold)] text-white mb-[var(--spacing-2)] hover:text-brand-green transition-colors">
                          Résultats de la journée 15
                        </h3>
                        <p className="text-[var(--text-body-sm)] text-text-tertiary line-clamp-2">
                          Découvrez les résultats complets de la 15ème journée du championnat 79.
                        </p>
                        <div className="mt-[var(--spacing-3)] flex items-center justify-between text-[var(--text-body-xs)] text-text-tertiary">
                          <span>14 Octobre 2026</span>
                          <Button variant="ghost" size="xs" className="text-brand-green-400 hover:text-brand-green">
                            Lire la suite
                          </Button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </section>
            </div>

            {/* Right Column - Stats and Top Players */}
            <div className="space-y-[var(--spacing-8)]">
              {/* Next Matches */}
              <section aria-labelledby="next-matches-heading">
                <div className="flex items-center justify-between mb-[var(--spacing-4)]">
                  <h2
                    id="next-matches-heading"
                    className="text-[var(--text-heading-sm)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)]"
                  >
                    <Icon name="Calendar" size="sm" className="inline mr-[var(--spacing-2)]" />
                    Prochains matchs
                  </h2>
                  <Link
                    to="/calendrier-et-resultats"
                    className="text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-brand-green-400 hover:text-brand-green transition-colors uppercase"
                  >
                    VOIR TOUT
                  </Link>
                </div>
                <div className="space-y-[var(--spacing-2)]">
                  {[1, 2, 3].map((item) => (
                    <div
                      key={item}
                      className="bg-[var(--bg-secondary)] rounded-[var(--radius-lg)] p-[var(--spacing-3)] flex items-center gap-[var(--spacing-3)] hover:bg-[var(--bg-tertiary)] transition-colors"
                    >
                      <div className="w-8 h-8 bg-brand-green rounded-full flex items-center justify-center flex-shrink-0">
                        <Icon name="Calendar" size="xs" className="text-black" />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="text-[var(--text-body-xs)] text-text-tertiary mb-[2px]">
                          Journée {15 + item}
                        </div>
                        <div className="text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] text-white truncate">
                          Niort PC vs Bressuire
                        </div>
                      </div>
                      <div className="text-right">
                        <div className="text-[var(--text-body-xs)] text-text-tertiary">
                          Dimanche
                        </div>
                        <div className="text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] text-brand-green-400">
                          15h00
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </section>

              {/* Top Players */}
              <section aria-labelledby="top-players-heading">
                <div className="flex items-center justify-between mb-[var(--spacing-4)]">
                  <h2
                    id="top-players-heading"
                    className="text-[var(--text-heading-sm)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)]"
                  >
                    <Icon name="Star" size="sm" className="inline mr-[var(--spacing-2)]" />
                    Meilleurs joueurs
                  </h2>
                  <Link
                    to="/joueurs"
                    className="text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-brand-green-400 hover:text-brand-green transition-colors uppercase"
                  >
                    VOIR TOUT
                  </Link>
                </div>
                <div className="space-y-[var(--spacing-3)]">
                  {players.slice(0, 3).map((player, index) => (
                    <PlayerCard
                      key={player.id}
                      player={player}
                      accent={index === 0 ? "gold" : index === 1 ? "primary" : "amber"}
                    />
                  ))}
                </div>
              </section>

              {/* Stats Summary */}
              <section className="bg-gradient-to-br from-brand-green to-brand-green-dark rounded-[var(--radius-xl)] p-[var(--spacing-6)]" aria-labelledby="stats-heading">
                <h2
                  id="stats-heading"
                  className="text-[var(--text-heading-sm)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-4)]"
                >
                  <Icon name="BarChart3" size="sm" className="inline mr-[var(--spacing-2)]" />
                  Statistiques
                </h2>
                <div className="grid grid-cols-2 gap-[var(--spacing-3)]">
                  {[
                    { label: "Matchs joués", value: "180", icon: "Calendar" as const },
                    { label: "Buts marqués", value: "4 500", icon: "Target" as const },
                    { label: "Joueurs actifs", value: "250", icon: "Users" as const },
                    { label: "Clubs engagés", value: "12", icon: "Home" as const },
                  ].map((stat, index) => (
                    <div
                      key={index}
                      className="text-center p-[var(--spacing-3)] bg-[rgba(255,255,255,0.05)] rounded-[var(--radius-lg)]"
                    >
                      <Icon name={stat.icon} size="l" className="text-brand-orange mx-auto mb-[var(--spacing-2)]" />
                      <div className="text-[var(--text-display-xs)] font-[var(--font-weight-bold)] text-white mb-[2px]">
                        {stat.value}
                      </div>
                      <div className="text-[var(--text-body-xs)] text-text-tertiary uppercase tracking-wider">
                        {stat.label}
                      </div>
                    </div>
                  ))}
                </div>
              </section>
            </div>
          </div>
        </div>

        {/* Partners Section */}
        <section className="bg-brand-green/20 py-[var(--spacing-8)] lg:py-[var(--spacing-12)] mt-[var(--spacing-8)]" aria-labelledby="partners-heading">
          <div className="max-w-[1440px] mx-auto px-[var(--spacing-4)] lg:px-[var(--spacing-8)]">
            <h2
              id="partners-heading"
              className="text-[var(--text-heading-l)] font-[var(--font-weight-bold)] text-white uppercase tracking-[var(--letter-spacing-wide)] text-center mb-[var(--spacing-6)]"
            >
              <Icon name="Handshake" size="m" className="inline mr-[var(--spacing-2)]" />
              Partenaires
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-[var(--spacing-4)]">
              {[
                { name: "Conseil Départemental 79" },
                { name: "Fédération Sport Rural" },
                { name: "FFTB" },
                { name: "Crédit Agricole" },
                { name: "Région Nouvelle-Aquitaine" },
                { name: "Ville de Niort" },
              ].map((partner, index) => (
                <div
                  key={index}
                  className="bg-[var(--bg-secondary)] rounded-[var(--radius-lg)] p-[var(--spacing-4)] flex items-center justify-center h-20 hover:bg-[var(--bg-tertiary)] transition-colors"
                >
                  <span className="text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] text-white text-center">
                    {partner.name}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </section>
      </div>
    </Layout>
  );
};

export default Home;
