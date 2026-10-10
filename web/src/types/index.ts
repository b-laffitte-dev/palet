// Core Types for Palet Vendéen Application

// ============================================
// COMMON TYPES
// ============================================

export interface Identifiable {
  id: string;
}

export interface Timestamped {
  createdAt: Date;
  updatedAt: Date;
}

// ============================================
// USER & AUTHENTICATION TYPES
// ============================================

export type UserRole = 'player' | 'club_admin' | 'commission_admin' | 'super_admin' | 'arbitre';

export interface User extends Identifiable, Timestamped {
  email: string;
  username: string | null;
  firstName: string;
  lastName: string;
  
  // Profile
  photo: string | null;
  bio: string | null;
  phone: string | null;
  birthDate: Date | null;
  
  // Palet Info
  licenseNumber: string | null;
  clubId: string | null;
  category: string;
  dominantHand: 'right' | 'left' | 'ambidextrous';
  playStyle: string | null;
  
  // Account
  role: UserRole;
  permissions: Permission[];
  status: 'active' | 'inactive' | 'suspended' | 'banned';
  emailVerified: boolean;
  
  // Preferences
  preferences: UserPreferences;
  
  // Security
  passwordLastChanged: Date | null;
  mfaEnabled: boolean;
  lastLogin: Date | null;
  
  // Social
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
}

export interface UserPreferences {
  language: string;
  timezone: string;
  distanceUnit: 'metric' | 'imperial';
  notifications: {
    email: {
      results: boolean;
      news: boolean;
      reminders: boolean;
    };
    push: {
      results: boolean;
      matches: boolean;
    };
  };
}

// Alias for Player preferences (same structure as UserPreferences)
export type PlayerPreferences = UserPreferences;

// Player Match History interface
export interface PlayerMatchHistory {
  wins: number;
  losses: number;
  draws: number;
  total: number;
  recentMatches?: Match[];
  byCompetition?: Record<string, {
    wins: number;
    losses: number;
    draws: number;
    total: number;
  }>;
  bySeason?: Record<string, {
    wins: number;
    losses: number;
    draws: number;
    total: number;
  }>;
}

// Position Stats (stats by playing position)
export interface PositionStats {
  first: {
    matches: number;
    points: number;
    winRate?: number;
  };
  second: {
    matches: number;
    points: number;
    winRate?: number;
  };
  [key: string]: {
    matches: number;
    points: number;
    winRate?: number;
  };
}

// Season Stats
export interface SeasonStats {
  matches: number;
  wins: number;
  draws: number;
  losses: number;
  totalPoints: number;
  averagePoints: number;
  pointsPerManche: number;
  winRate: number;
  bestScore: number;
  bestStreak: number;
  rank: number;
  division: string;
}

// Team Competition
export interface TeamCompetition {
  id: string;
  competition: Competition;
  competitionId: string;
  season: string;
  status: 'registered' | 'qualified' | 'in_progress' | 'completed' | 'disqualified';
  results: TeamCompetitionResults;
}

export interface TeamCompetitionResults {
  matches: number;
  wins: number;
  draws: number;
  losses: number;
  pointsFor: number;
  pointsAgainst: number;
  position: number | null;
  points: number;
}

// Team Results
export interface TeamResults {
  team: Team;
  competition: Competition;
  matches: Match[];
  stats: TeamCompetitionResults;
}

// Player Results
export interface PlayerResults {
  player: Player;
  competition: Competition;
  matches: Match[];
  stats: {
    matches: number;
    wins: number;
    draws: number;
    losses: number;
    totalPoints: number;
    averagePoints: number;
    bestScore: number;
    position: number | null;
  };
}

export interface Permission {
  id: string;
  name: string;
  code: string;
  category: 'competition' | 'player' | 'club' | 'system' | 'report';
  description: string;
}

export interface AuthToken {
  accessToken: string;
  refreshToken: string;
  expiresIn: number; // seconds
  tokenType: 'Bearer';
}

export interface AuthState {
  user: User | null;
  token: AuthToken | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  requires2FA: boolean;
  twoFactorPending: boolean;
}

// ============================================
// CLUB TYPES
// ============================================

export interface Club extends Identifiable, Timestamped {
  name: string;
  nickname: string | null;
  code: string;
  logo: string | null;
  
  // Location
  address: string | null;
  city: string;
  postalCode: string | null;
  
  // Affiliation
  league: string; // ex: "Vendée", "Deux-Sèvres"
  division: string; // ex: "D1", "D2"
  status: 'active' | 'inactive' | 'suspended';
  
  // Stats
  playersCount: number;
  teamsCount: number;
  
  // Contact
  contactEmail: string | null;
  contactPhone: string | null;
  website: string | null;
  
  // Social
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
}

// ============================================
// PLAYER TYPES
// ============================================

export interface Player extends Identifiable, Timestamped {
  firstName: string;
  lastName: string;
  fullName: string;
  alias: string | null;
  email: string;
  photo: string | null;
  status: 'active' | 'inactive' | 'suspended' | 'banned';
  
  // Personal info
  birthDate: Date;
  age: number;
  birthPlace: string | null;
  nationality: string;
  dominantHand: 'right' | 'left' | 'ambidextrous';
  playStyle: string | null;
  
  // Contact info
  phone: string | null;
  address: string | null;
  city: string | null;
  postalCode: string | null;
  
  // Club info
  club: Club;
  clubId: string;
  clubNumber: string | null;
  category: string;
  
  // License info
  licenseNumber: string;
  licenseDate: Date;
  licenseStatus: 'valid' | 'expired' | 'pending';
  registrationDate: Date;
  
  // Stats
  stats: PlayerStats;
  
  // Achievements
  trophies: PlayerTrophies;
  badges: PlayerBadges;
  records: PlayerRecords;
  
  // Teams
  currentTeams: PlayerTeam[];
  pastTeams: PastPlayerTeam[];
  frequentTeammates: TeammateRelation[];
  
  // History
  matchHistory: PlayerMatchHistory;
  competitionStats: CompetitionStats[];
  
  // Recent activity
  recentActivities: Activity[];
  
  // Preferences
  preferences: PlayerPreferences;
  
  // Social
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
  
  // Metadata
  lastActiveAt: Date;
}

export interface PlayerStats {
  matchesPlayed: number;
  wins: number;
  draws: number;
  losses: number;
  totalPoints: number;
  averagePoints: number;
  pointsPerManche: number;
  bestScore: number;
  bestStreak: number;
  maxWinStreak: number;
  winRate: number;
  accuracy: number;
  
  currentRank: number;
  currentDivision: string;
  currentPoints: number;
  previousRank: number;
  rankTrend: 'up' | 'down' | 'stable';
  newHighlights: number;
  
  byPosition: PositionStats;
  byCompetition: Record<string, CompetitionStats>;
  bySeason: Record<string, SeasonStats>;
}

export interface PlayerTrophies {
  total: number;
  list: Trophy[];
}

export interface PlayerBadges {
  total: number;
  categories: BadgeCategory[];
}

export interface PlayerRecords {
  total: number;
  list: RecordItem[];
}

// ============================================
// TEAM TYPES
// ============================================

export interface Team extends Identifiable, Timestamped {
  name: string;
  club: Club;
  clubId: string;
  captain: Player | null;
  captainId: string | null;
  players: Player[];
  logo: string | null;
  
  // Stats
  stats: TeamStats;
  
  // Status
  status: 'active' | 'inactive' | 'disbanded';
  
  // Competition info
  competitions: TeamCompetition[];
}

export interface TeamStats {
  matches: number;
  wins: number;
  draws: number;
  losses: number;
  pointsFor: number;
  pointsAgainst: number;
  diff: number;
  points: number;
  winRate: number;
}

export interface PlayerTeam {
  id: string;
  name: string;
  captain: Player | null;
  players: Player[];
  stats: TeamStats;
}

export interface PastPlayerTeam extends PlayerTeam {
  season: string;
}

export interface TeammateRelation {
  player: Player;
  count: number; // number of times played together
  winRate: number;
}

// ============================================
// COMPETITION TYPES
// ============================================

export type CompetitionType = 'championship' | 'tournament' | 'cup' | 'friendly';
export type CompetitionCategory = 'individual' | 'doublette' | 'triplette';
export type CompetitionMaterial = 'fonte' | 'laiton' | 'bois' | 'mixte';

export interface Competition extends Identifiable, Timestamped {
  name: string;
  code: string | null;
  type: CompetitionType;
  category: CompetitionCategory;
  material: CompetitionMaterial;
  season: string;
  
  // Details
  description: string | null;
  location: string | null;
  
  // Structure
  divisions: string[]; // ['D1', 'D2', 'D3']
  format: 'round-robin' | 'league' | 'groups' | 'knockout';
  
  // Dates
  registrationStart: Date | null;
  registrationEnd: Date | null;
  startDate: Date | null;
  endDate: Date | null;
  
  // Status
  status: 'pending' | 'registration' | 'in_progress' | 'completed' | 'cancelled';
  currentJournee: number | null;
  totalJournees: number | null;
  currentPhase: string | null;
  
  // Participants
  maxTeams: number | null;
  registeredTeams: number;
  teams: Team[];
  
  // Scoring
  pointsType: '13' | '15';
  winPoints: number;
  drawPoints: number;
  lossPoints: number;
  
  // Settings
  autoRegistration: boolean;
  manualValidation: boolean;
  isPublic: boolean;
  isArchived: boolean;
  
  // Stats
  stats: CompetitionStats;
  
  // Organizer
  organizer: Club | User;
  organizerId: string;
}

export interface CompetitionStats {
  totalMatches: number;
  completedMatches: number;
  totalPoints: number;
  topScorer: Player | null;
  topTeam: Team | null;
}

// ============================================
// CHAMPIONSHIP TYPES
// ============================================

export interface Championship extends Competition {
  type: 'championship';
  
  // Championship-specific
  totalJournees: number;
  journeeDuration: string | null; // ex: "1 week"
  
  // Classification rules
  classificationRules: ClassificationRules;
  
  // Standings
  classification: ChampionshipClassification;
}

export interface ClassificationRules {
  primary: 'points' | 'winRate' | 'diff';
  secondary: 'diff' | 'pointsFor' | 'headToHead';
  tertiary: 'pointsFor' | 'pointsAgainst';
}

export interface ChampionshipClassification {
  division: string;
  journee: number;
  updatedAt: Date;
  teams: ChampionshipTeam[];
}

export interface ChampionshipTeam {
  position: number;
  team: Team;
  stats: TeamStats;
  form: ('win' | 'draw' | 'loss')[];
  status: 'qualified' | 'barrage' | 'relegated' | 'safe';
}

// ============================================
// TOURNAMENT TYPES
// ============================================

export interface Tournament extends Competition {
  type: CompetitionType;
  
  // Tournament-specific
  phases: TournamentPhase[];
  brackets: TournamentBracket[] | null;
  
  // Registration
  fee: number | null;
  paymentRequired: boolean;
  paymentMethods: string[]; // ['card', 'paypal', 'transfer', 'later']
  
  // Rules
  rules: string | null;
  eligibility: TournamentEligibility;
}

export interface TournamentPhase {
  id: string;
  name: string;
  type: 'group' | 'knockout' | 'final';
  order: number;
  matches: TournamentMatch[];
  groups: TournamentGroup[] | null;
}

export interface TournamentGroup {
  id: string;
  name: string;
  teams: Team[];
  matches: TournamentMatch[];
}

export interface TournamentMatch {
  id: string;
  team1: Team | null;
  team2: Team | null;
  team1Score: number | null;
  team2Score: number | null;
  winner: Team | null;
  status: 'scheduled' | 'in_progress' | 'completed' | 'postponed' | 'cancelled';
  date: Date | null;
  time: string | null;
  location: string | null;
  round: string;
  manche: number | null;
}

export interface TournamentBracket {
  id: string;
  name: string;
  rounds: BracketRound[];
}

export interface BracketRound {
  id: string;
  name: string;
  order: number;
  matches: TournamentMatch[];
}

export interface TournamentEligibility {
  requireLicense: boolean;
  requireClub: boolean;
  categories: CompetitionCategory[];
  divisions: string[];
  maxPlayersPerClub: number | null;
  minPlayers: number;
  maxPlayers: number;
}

// ============================================
// MATCH TYPES
// ============================================

export interface Match extends Identifiable, Timestamped {
  competitionId: string;
  competition: Competition;
  
  // Match info
  type: CompetitionType;
  journee: number | null;
  phase: string | null;
  round: string | null;
  
  // Teams
  team1: MatchTeam;
  team2: MatchTeam;
  
  // Scores
  team1Score: number | null;
  team2Score: number | null;
  winner: 'team1' | 'team2' | null;
  
  // Manche scores
  manches: MancheScore[] | null;
  bestOf: number | null;
  currentManche: number | null;
  currentScores: CurrentScores | null;
  
  // Status
  status: 'scheduled' | 'in_progress' | 'completed' | 'postponed' | 'cancelled' | 'disputed';
  verified: boolean;
  verifiedBy: string | null;
  verifiedAt: Date | null;
  
  // Details
  date: Date;
  time: string | null;
  location: string | null;
  duration: string | null;
  
  // Metadata
  canEdit: boolean;
  isLive: boolean;
  isRecent: boolean;
  
  // Stats
  stats: MatchStats;
  
  // Timeline
  startedAt: Date | null;
  endedAt: Date | null;
}

export interface MatchTeam {
  id: string;
  name: string;
  logo: string | null;
  players: Player[];
  rank: number | null;
  seed: number | null;
}

export interface MancheScore {
  number: number;
  team1: number;
  team2: number;
  winner: 'team1' | 'team2' | null;
}

export interface CurrentScores {
  team1: number;
  team2: number;
  manche: number;
}

export interface MatchStats {
  totalManches: number;
  avgPointsPerManche: number;
  bestManche: MancheScore | null;
  playerStats: PlayerMatchStats[];
}

export interface PlayerMatchStats {
  player: Player;
  points: number;
  accuracy: number;
  bestManche: number;
}

// ============================================
// RESULTS & CLASSIFICATION TYPES
// ============================================

export interface ResultsData {
  matches: Match[];
  total: number;
  pages: number;
  currentPage: number;
  groupedByDate: Record<string, DateGroup>;
  byCompetition: Record<string, CompetitionResults>;
  byTeam: Record<string, TeamResults>;
  byPlayer: Record<string, PlayerResults>;
  
  stats: ResultsStats;
  featured: FeaturedResults;
  
  filters: ActiveFilters;
}

export interface DateGroup {
  date: string;
  dayName: string;
  matches: Match[];
  count: number;
}

export interface CompetitionResults {
  competition: Competition;
  matches: Match[];
  standings: Standing[];
  stats: CompetitionStats;
}

export interface Standing {
  position: number;
  team: Team;
  stats: TeamStats;
  form: ('win' | 'draw' | 'loss')[];
}

export interface ResultsStats {
  matchesToday: number;
  matchesTodayTrend: number;
  totalPoints: number;
  avgPoints: number;
  biggestMargin: number;
  biggestMarginMatch: Match | null;
  homeWins: number;
  homeWinsCount: number;
  homeMatches: number;
  resultsDistribution: DistributionData;
  performanceByDay: DayPerformance[];
  scoreTrend: TrendData;
}

export interface FeaturedResults {
  topPerformer: Player | null;
  biggestUpset: Match | null;
  recordBreaker: RecordItem | null;
  perfectGame: Match | null;
}

export interface ActiveFilters {
  type: CompetitionType[];
  category: CompetitionCategory[];
  season: string[];
  division: string[];
  dateRange: [Date | null, Date | null];
  search: string;
}

// ============================================
// REGISTRATION TYPES
// ============================================

export interface Registration extends Identifiable, Timestamped {
  reference: string;
  user: User;
  userId: string;
  tournament: Tournament;
  tournamentId: string;
  team: RegistrationTeam;
  isIndividual: boolean;
  status: 'pending' | 'confirmed' | 'cancelled' | 'waitlisted';
  payment: RegistrationPayment | null;
  paymentStatus: 'pending' | 'paid' | 'waived' | 'failed';
  fee: number;
  email: string;
  phone: string | null;
  address: string | null;
  answers: RegistrationAnswer[];
  confirmedAt: Date | null;
}

export interface RegistrationTeam {
  id: string;
  name: string;
  captain: Player;
  players: Player[];
}

export interface RegistrationPayment {
  id: string;
  method: string;
  amount: number;
  status: 'pending' | 'paid' | 'failed' | 'refunded';
  transactionId: string | null;
  createdAt: Date;
  updatedAt: Date;
}

export interface RegistrationAnswer {
  questionId: string;
  question: string;
  answer: string | number | boolean;
}

// ============================================
// NEWS & MEDIA TYPES
// ============================================

export type NewsCategory = 'official' | 'competitions' | 'clubs' | 'featured' | 'media';
export type NewsType = 'announcement' | 'news' | 'preview' | 'recap' | 'interview' | 'feature' | 'gallery';

export interface NewsArticle extends Identifiable, Timestamped {
  title: string;
  slug: string;
  excerpt: string;
  content: string;
  
  // Category & Type
  category: NewsCategory;
  type: NewsType;
  
  // Media
  image: string | null;
  imageCaption: string | null;
  thumbnail: string | null;
  
  // Author
  author: NewsAuthor;
  authorId: string;
  
  // Metadata
  status: 'draft' | 'published' | 'archived' | 'deleted';
  visibility: 'public' | 'members' | 'admin';
  priority: number;
  
  // Stats
  views: number;
  likes: number;
  shares: number;
  commentsCount: number;
  
  // Classification
  tags: NewsTag[];
  
  // Publishing
  publishedAt: Date;
  scheduledAt: Date | null;
  
  // SEO
  metaTitle: string | null;
  metaDescription: string | null;
  
  // Comments
  comments: NewsComment[];
  
  // Related
  relatedArticles: string[] | NewsArticle[];
}

export interface NewsAuthor {
  id: string;
  name: string;
  avatar: string | null;
  role: string;
  bio: string | null;
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
}

export interface NewsTag {
  id: string;
  name: string;
  slug: string;
}

export interface NewsComment extends Identifiable, Timestamped {
  content: string;
  author: NewsCommentAuthor;
  likes: number;
  isLiked: boolean;
  replies: NewsComment[];
}

export interface NewsCommentAuthor {
  id: string;
  name: string;
  avatar: string | null;
}

export type MediaCategory = 'competition' | 'player' | 'club' | 'event' | 'behind-the-scenes';

export interface MediaItem extends Identifiable, Timestamped {
  title: string;
  description: string | null;
  type: 'photo' | 'video';
  
  // Media URLs
  url: string;
  thumbnail: string;
  
  // Metadata
  category: MediaCategory;
  tags: string[];
  
  // Stats
  views: number;
  likes: number;
  
  // Details
  date: Date;
  uploadedBy: NewsAuthor;
  
  // For videos
  duration: number | null; // in seconds
  
  // For photos
  width: number | null;
  height: number | null;
}

// ============================================
// NOTIFICATION TYPES
// ============================================

export interface Notification extends Identifiable, Timestamped {
  type: 'match' | 'tournament' | 'club' | 'system' | 'message';
  title: string;
  message: string;
  data: NotificationData | null;
  userId: string;
  isRead: boolean;
  readAt: Date | null;
  
  // For push notifications
  sent: boolean;
  sentAt: Date | null;
  
  // Priority
  priority: 'low' | 'normal' | 'high' | 'urgent';
}

export interface NotificationData {
  // Generic data for the notification
  id: string;
  type: string;
  [key: string]: any;
}

// ============================================
// STATISTICS & CHART TYPES
// ============================================

export interface StatItem {
  label: string;
  value: string | number;
  icon?: string;
  color?: string;
  trend?: 'up' | 'down' | 'stable';
}

export interface ChartData {
  labels: string[];
  datasets: ChartDataset[];
}

export interface ChartDataset {
  label: string;
  data: (number | null)[];
  backgroundColor?: string | string[];
  borderColor?: string | string[];
  borderWidth?: number;
}

export interface DistributionData {
  labels: string[];
  values: number[];
  colors: string[];
}

export interface DayPerformance {
  day: string;
  matches: number;
  wins: number;
  winRate: number;
}

export interface TrendData {
  labels: string[];
  values: number[];
}

// ============================================
// MISC TYPES
// ============================================

export interface Badge {
  id: string;
  name: string;
  icon: string | null;
  description: string;
  rarity: 'bronze' | 'silver' | 'gold' | 'platinum';
  date: Date;
}

export interface BadgeCategory {
  id: string;
  name: string;
  badges: Badge[];
}

export interface Trophy {
  id: string;
  name: string;
  competition: string;
  category: string;
  year: string;
  date: Date;
  type: 'champion' | 'winner' | 'finalist' | 'semifinalist';
}

export interface RecordItem {
  id: string;
  type: string;
  value: string | number;
  tournament: string;
  date: Date;
  player?: Player;
  team?: Team;
  verified: boolean;
}

export interface Activity {
  id: string;
  type: 'match' | 'tournament' | 'achievement' | 'registration' | 'system';
  title: string;
  message: string;
  userId: string | null;
  timestamp: Date;
  data: any;
}

export interface FormField {
  id: string;
  label: string;
  type: 'text' | 'number' | 'select' | 'checkbox' | 'radio' | 'date' | 'email' | 'textarea';
  value: any;
  options?: { value: string; label: string }[];
  required?: boolean;
  placeholder?: string;
  helpText?: string;
  error?: string;
}

export interface Pagination {
  current: number;
  total: number;
  pageSize: number;
  totalItems: number;
  hasNext: boolean;
  hasPrevious: boolean;
}

export interface FilterOption {
  value: string;
  label: string;
  count?: number;
}

export interface SortOption {
  field: string;
  direction: 'asc' | 'desc' | 'none';
}

// ============================================
// API RESPONSE TYPES
// ============================================

export interface ApiResponse<T> {
  data: T;
  message?: string;
  success: boolean;
}

export interface ApiError {
  message: string;
  code: string;
  statusCode: number;
  details?: Record<string, any>;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: Pagination;
}

export interface HealthCheck {
  statut: string;
  version: string;
  timestamp: Date;
  database: 'connected' | 'disconnected';
  cache: 'connected' | 'disconnected';
}