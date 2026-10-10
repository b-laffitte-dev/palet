import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Layout } from '../components/layout/Layout';
import { Card, CardHeader, CardContent, CardFooter, Badge, Avatar, Button, Tabs, TabList, TabPanel, Tab, TeamAvatar } from '../components';
import { ClassificationTable, MatchCard, ClubCard, TournamentCard } from '../components/domain';
import { Championship, Team, Match, ChampionshipClassification, ChampionshipTeam } from '../types';
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
    captain: null as any,
    captainId: null,
    players: [],
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
    captain: null as any,
    captainId: null,
    players: [],
    logo: null,
    stats: {
      matches: 20,
      wins: 12,
      draws: 5,
      losses: 3,
      pointsFor: 210,
      pointsAgainst: 180,
      diff: 30,
      points: 41,
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
    captain: null as any,
    captainId: null,
    players: [],
    logo: null,
    stats: {
      matches: 20,
      wins: 10,
      draws: 6,
      losses: 4,
      pointsFor: 195,
      pointsAgainst: 190,
      diff: 5,
      points: 36,
      winRate: 50,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
  {
    id: 'team-004',
    name: 'Équipe 1 - Luçon',
    club: {
      id: 'club-004',
      name: 'Palet de Luçon',
      nickname: 'Les Luçonnais',
      code: 'LUC',
      logo: '/images/clubs/lucon.png',
      address: '321 Rue de la Barre, 85400 Luçon',
      city: 'Luçon',
      postalCode: '85400',
      league: 'Vendée',
      division: 'D1',
      status: 'active',
      playersCount: 15,
      teamsCount: 4,
      contactEmail: 'contact@palet-lucon.fr',
      contactPhone: '+33 2 51 65 43 24',
      website: 'https://palet-lucon.fr',
      social: { facebook: null, twitter: null, instagram: null },
      createdAt: new Date('2021-07-01'),
      updatedAt: new Date('2026-10-01'),
    },
    clubId: 'club-004',
    captain: null as any,
    captainId: null,
    players: [],
    logo: null,
    stats: {
      matches: 20,
      wins: 8,
      draws: 4,
      losses: 8,
      pointsFor: 180,
      pointsAgainst: 200,
      diff: -20,
      points: 28,
      winRate: 40,
    },
    status: 'active',
    competitions: [],
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

const mockChampionship: Championship = {
  id: 'championship-001',
  name: 'Championnat de Vendée D1 2026-2027',
  code: 'D1-2026-2027',
  type: 'championship',
  category: 'doublette',
  material: 'fonte',
  season: '2026-2027',
  description: 'Le championnat départemental de Vendée en Division 1 pour la saison 2026-2027.',
  location: 'Vendée, France',
  divisions: ['D1'],
  format: 'round-robin',
  registrationStart: new Date('2026-08-01'),
  registrationEnd: new Date('2026-08-31'),
  startDate: new Date('2026-09-15'),
  endDate: new Date('2027-05-31'),
  status: 'in_progress',
  currentJournee: 5,
  totalJournees: 18,
  currentPhase: 'Phase Aller',
  maxTeams: 12,
  registeredTeams: mockTeams.length,
  teams: mockTeams,
  pointsType: '13',
  winPoints: 3,
  drawPoints: 1,
  lossPoints: 0,
  autoRegistration: false,
  manualValidation: true,
  isPublic: true,
  isArchived: false,
  stats: {
    totalMatches: 40,
    completedMatches: 20,
    totalPoints: 1800,
    topScorer: null,
    topTeam: mockTeams[0]!,
  },
  organizer: mockTeams[0]!.club!,
  organizerId: 'club-001',
  classificationRules: {
    primary: 'points',
    secondary: 'diff',
    tertiary: 'pointsFor',
  },
  classification: {
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
      {
        position: 3,
        team: mockTeams[2]!,
        stats: mockTeams[2]!.stats,
        form: ['win', 'draw', 'loss', 'win', 'win'],
        status: 'safe',
      },
      {
        position: 4,
        team: mockTeams[3]!,
        stats: mockTeams[3]!.stats,
        form: ['loss', 'draw', 'win', 'loss', 'draw'],
        status: 'relegated',
      },
    ],
  },
  journeeDuration: '1 week',
  createdAt: new Date('2026-07-01'),
  updatedAt: new Date('2026-10-09'),
};

const mockMatches: Match[] = [
  {
    id: 'match-001',
    competitionId: 'championship-001',
    competition: mockChampionship as any,
    type: 'championship',
    journee: 5,
    phase: 'Phase Aller',
    round: 'Round 5',
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
    competition: mockChampionship as any,
    type: 'championship',
    journee: 5,
    phase: 'Phase Aller',
    round: 'Round 5',
    team1: {
      id: 'team-003',
      name: 'Équipe 1 - Herbiers',
      logo: '/images/clubs/herbiers.png',
      players: [],
      rank: 3,
      seed: 3,
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
    team2Score: 11,
    winner: 'team1',
    manches: [
      { number: 1, team1: 7, team2: 5, winner: 'team1' },
      { number: 2, team1: 6, team2: 6, winner: null },
      { number: 3, team1: 6, team2: 4, winner: 'team1' },
    ],
    bestOf: 3,
    currentManche: null,
    currentScores: null,
    status: 'completed',
    verified: true,
    verifiedBy: 'arbitre-001',
    verifiedAt: new Date('2026-10-08T16:30:00'),
    date: new Date('2026-10-08T16:00:00'),
    time: '16:00',
    location: 'Salle de Palet - Les Herbiers',
    duration: '1h15',
    canEdit: false,
    isLive: false,
    isRecent: true,
    stats: {
      totalManches: 3,
      avgPointsPerManche: 5.67,
      bestManche: { number: 3, team1: 6, team2: 4, winner: 'team1' },
      playerStats: [],
    },
    startedAt: new Date('2026-10-08T16:00:00'),
    endedAt: new Date('2026-10-08T16:30:00'),
    createdAt: new Date(),
    updatedAt: new Date(),
  },
];

const ChampionshipDetail: React.FC = () => {
  const { championshipId } = useParams<{ championshipId: string }>();

  const { data: championship, isLoading } = useQuery<Championship>({
    queryKey: ['championship', championshipId],
    queryFn: async () => {
      const response = await api().GET<Championship>(`/api/championships/${championshipId}`);
      return response.data;
    },
    enabled: !!championshipId,
  });

  // For development, use mock data if no real data
  const displayChampionship = championship || mockChampionship;

  if (isLoading && !championship) {
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

  // Calculate progress
  const getJourneeProgress = () => {
    if (!displayChampionship.totalJournees || !displayChampionship.currentJournee) {
      return 0;
    }
    return Math.min(
      ((displayChampionship.currentJournee || 0) / displayChampionship.totalJournees) * 100,
      100
    );
  };

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center gap-4 mb-4">
              <Link to="/championnats">
                <Button variant="ghost" size="sm" leftIcon="ArrowLeft">
                  Retour aux championnats
                </Button>
              </Link>
            </div>

            <div className="flex flex-col md:flex-row items-start gap-6">
              {/* Logo/Icon */}
              <div className="flex-shrink-0">
                <div className="w-24 h-24 bg-primary-800 rounded-full flex items-center justify-center border-2 border-gold-700">
                  <span className="text-4xl text-gold-700">🏅</span>
                </div>
              </div>

              {/* Info */}
              <div className="flex-1">
                <div className="flex items-center gap-4 mb-2">
                  <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
                    {displayChampionship.name}
                  </h1>
                </div>

                <div className="flex items-center gap-4 mb-4">
                  <Badge variant="default">{displayChampionship.season}</Badge>
                  <Badge variant="default">{displayChampionship.divisions[0]}</Badge>
                  <Badge variant="default">{displayChampionship.category}</Badge>
                  <Badge variant="default">{displayChampionship.material}</Badge>
                  {displayChampionship.status === 'pending' && (
                    <Badge variant="default">À venir</Badge>
                  )}
                  {displayChampionship.status === 'in_progress' && (
                    <Badge variant="warning">En cours</Badge>
                  )}
                  {displayChampionship.status === 'completed' && (
                    <Badge variant="success">Terminé</Badge>
                  )}
                  {displayChampionship.status === 'cancelled' && (
                    <Badge variant="danger">Annulé</Badge>
                  )}
                </div>

                <p className="text-body-m text-primary-200 mb-4 max-w-2xl">
                  {displayChampionship.description}
                </p>

                {/* Dates and Location */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Journée actuelle:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayChampionship.currentJournee || 'Non commencé'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Total journées:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayChampionship.totalJournees}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-primary-400">Phase actuelle:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayChampionship.currentPhase}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 md:col-span-3">
                    <span className="text-primary-400">Dates:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayChampionship.startDate?.toLocaleDateString('fr-FR') || 'À définir'} - 
                      {displayChampionship.endDate?.toLocaleDateString('fr-FR') || 'À définir'}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 md:col-span-3">
                    <span className="text-primary-400">Lieu:</span>
                    <span className="text-gold-700 font-semibold text-body-s">
                      {displayChampionship.location || 'Vendée, France'}
                    </span>
                  </div>
                </div>
              </div>

              {/* Code and Organizer */}
              <div className="flex-shrink-0">
                <div className="space-y-4">
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Code Championnat</p>
                    <p className="text-heading-m font-bold text-gold-700">{displayChampionship.code}</p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Organisateur</p>
                    <p className="text-heading-xs font-bold text-gold-700">
                      {(displayChampionship.organizer as any)?.name}
                    </p>
                  </div>
                  <div className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 text-center">
                    <p className="text-body-xs text-primary-400 mb-1">Équipes</p>
                    <p className="text-heading-m font-bold text-gold-700">
                      {displayChampionship.registeredTeams}/{displayChampionship.maxTeams || 'Illimité'}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Progress */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              Progression du Championnat
            </h2>
            
            <div className="mb-6">
              <div className="flex justify-between items-center mb-2">
                <span className="text-body-s text-primary-300">
                  Journée {displayChampionship.currentJournee || 0} / {displayChampionship.totalJournees}
                </span>
                <span className="text-body-s text-gold-700">
                  {getJourneeProgress().toFixed(0)}%
                </span>
              </div>
              <div className="h-6 bg-primary-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-gold-700 to-gold-500 rounded-full"
                  style={{ width: `${getJourneeProgress()}%` }}
                />
              </div>
              <div className="flex justify-between mt-2 text-body-xs text-primary-400">
                <span>Début</span>
                <span>Journée {displayChampionship.currentJournee || 0}</span>
                <span>Fin</span>
              </div>
            </div>

            {/* Stats */}
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-4">
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Matchs Totaux</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayChampionship.stats.totalMatches}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Matchs Termines</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayChampionship.stats.completedMatches}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Points Totaux</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayChampionship.stats.totalPoints}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Points par Manche</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayChampionship.pointsType}</p>
                </CardContent>
              </Card>
              <Card className="bg-primary-900/50 border-primary-700/50">
                <CardContent className="text-center p-4">
                  <p className="text-body-xs text-primary-300 mb-1">Durée Journée</p>
                  <p className="text-heading-m font-bold text-gold-700">{displayChampionship.journeeDuration || '1 semaine'}</p>
                </CardContent>
              </Card>
            </div>
          </div>
        </section>

        {/* Tabs */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <Tabs defaultValue="classement">
              <TabList>
                <Tab value="classement" label="Classement" />
                <Tab value="journee" label="Journée Actuelle" />
                <Tab value="matchs" label="Tous les Matchs" />
                <Tab value="equipes" label="Équipes Participants ({displayChampionship.registeredTeams || 0})" />
                <Tab value="regles" label="Règlement" />
              </TabList>

              {/* Classement */}
              <TabPanel value="classement" className="py-6">
                <ClassificationTable
                  teams={displayChampionship.classification.teams}
                  competition={displayChampionship.name}
                  journee={displayChampionship.currentJournee || 0}
                  season={displayChampionship.season}
                />
              </TabPanel>

              {/* Journée Actuelle */}
              <TabPanel value="journee" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <div className="flex items-center justify-between">
                      <h3 className="text-heading-s font-bold text-white">
                        Journée {displayChampionship.currentJournee || 'Non commencé'} - {displayChampionship.currentPhase}
                      </h3>
                      <Badge variant="warning">
                        {displayChampionship.currentJournee ? 'En cours' : 'Non commencé'}
                      </Badge>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <div className="space-y-4">
                      {displayChampionship.currentJournee && mockMatches.length > 0 ? (
                        mockMatches
                          .filter(m => m.journee === displayChampionship.currentJournee)
                          .map((match) => (
                            <MatchCard
                              key={match.id}
                              match={match}
                              
                              showTournament={false}
                            />
                          ))
                      ) : (
                        <p className="text-body-m text-primary-300 text-center py-4">
                          Aucune journée en cours pour le moment.
                        </p>
                      )}
                    </div>
                  </CardContent>
                </Card>
              </TabPanel>

              {/* Tous les Matchs */}
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

              {/* Équipes Participants */}
              <TabPanel value="equipes" className="py-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  {displayChampionship.teams.map((team) => (
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
                        </div>
                      </CardHeader>
                      <CardContent>
                        <div className="grid grid-cols-2 md:grid-cols-4 gap-2 mb-4">
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
                            <p className="text-body-xs text-primary-300">Points</p>
                            <p className="text-heading-xs font-bold text-gold-700">{team.stats.points}</p>
                          </div>
                        </div>
                        <Link to={`/teams/${team.id}`} className="inline-block">
                          <Button variant="outline" size="sm" fullWidth>
                            Voir l'équipe
                          </Button>
                        </Link>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              </TabPanel>

              {/* Règlement */}
              <TabPanel value="regles" className="py-6">
                <Card className="bg-primary-900/50 border-primary-700/50">
                  <CardHeader>
                    <h3 className="text-heading-s font-bold text-white">Règlement du Championnat</h3>
                  </CardHeader>
                  <CardContent>
                    <div className="prose prose-invert max-w-none text-body-s text-primary-200">
                      <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                        Règles de Classement
                      </h4>
                      <div className="space-y-3 mb-6">
                        <div className="flex items-center gap-2">
                          <span className="text-primary-400">1.</span>
                          <span className="text-white">
                            {displayChampionship.classificationRules.primary === 'points' && 'Points'}
                            {displayChampionship.classificationRules.primary === 'winRate' && 'Taux de victoire'}
                            {displayChampionship.classificationRules.primary === 'diff' && 'Différence de points'}
                          </span>
                        </div>
                        <div className="flex items-center gap-2">
                          <span className="text-primary-400">2.</span>
                          <span className="text-white">
                            {displayChampionship.classificationRules.secondary === 'diff' && 'Différence de points'}
                            {displayChampionship.classificationRules.secondary === 'pointsFor' && 'Points pour'}
                            {displayChampionship.classificationRules.secondary === 'headToHead' && 'Confrontation directe'}
                          </span>
                        </div>
                        <div className="flex items-center gap-2">
                          <span className="text-primary-400">3.</span>
                          <span className="text-white">
                            {displayChampionship.classificationRules.tertiary === 'pointsFor' && 'Points pour'}
                            {displayChampionship.classificationRules.tertiary === 'pointsAgainst' && 'Points contre'}
                          </span>
                        </div>
                      </div>

                      <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                        Points par Match
                      </h4>
                      <div className="space-y-2 mb-6">
                        <p>
                          <span className="text-primary-400">Victoire:</span> {displayChampionship.winPoints} points
                        </p>
                        <p>
                          <span className="text-primary-400">Match nul:</span> {displayChampionship.drawPoints} point
                        </p>
                        <p>
                          <span className="text-primary-400">Défaite:</span> {displayChampionship.lossPoints} point
                        </p>
                      </div>

                      <h4 className="text-heading-xs font-bold text-gold-700 mb-4">
                        Format
                      </h4>
                      <p>
                        {displayChampionship.format === 'round-robin' && 'Tous les participants jouent contre tous les autres participants.'}
                        {displayChampionship.format === 'league' && 'Ligue avec rencontres aller-retour.'}
                        {displayChampionship.format === 'groups' && 'Phase de poules suivie d\'une phase finale.'}
                        {displayChampionship.format === 'knockout' && 'Élimination directe.'}
                      </p>
                    </div>
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

export default ChampionshipDetail;
