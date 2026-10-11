// Centralized mock data exports for CDSP 79 Application

export * from "./clubs";
export * from "./players";
export * from "./teams";
export * from "./matches";
export * from "./tournaments";
export * from "./championships";

// Re-export specific mocks for convenience
export { mockClubs, mockClub } from "./clubs";
export { mockPlayers, mockPlayer } from "./players";
export { mockTeams, mockTeam, mockClubTeams } from "./teams";
export { mockMatches, mockMatch, mockLiveScores, mockFeaturedMatch } from "./matches";
export { mockTournaments, mockTournament } from "./tournaments";
export { mockChampionships, mockChampionship, mockClassification } from "./championships";

// Combined mock for club detail page - using imported mocks
import { mockPlayers } from "./players";
export const mockClubPlayers = [mockPlayers[1], mockPlayers[2], mockPlayers[3]];
