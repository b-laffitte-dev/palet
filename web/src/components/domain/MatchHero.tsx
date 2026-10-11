import React from "react";
import { Match } from "../../types";
import { Badge } from "../common/Badge";
import { Icon } from "../common/Icon";

export interface MatchHeroProps {
  match: Match;
  className?: string;
}

/**
 * MatchHero - Featured live match display (Top 14 style)
 * Used on homepage to showcase the main live match
 * Fully responsive for all screen sizes
 */
export const MatchHero: React.FC<MatchHeroProps> = ({ match, className = "" }) => {
  const getStatusVariant = () => {
    if (match.status === "in_progress") return "warning" as const;
    if (match.status === "completed") return "success" as const;
    return "secondary" as const;
  };

  const getStatusText = () => {
    if (match.status === "in_progress") {
      return `LIVE — Manche ${match.currentManche || 1}`;
    }
    if (match.status === "completed") {
      return "Terminé";
    }
    if (match.status === "scheduled") {
      return "À venir";
    }
    return match.status;
  };

  return (
    <section
      className={`match-hero-responsive w-full bg-brand-green-900 rounded-lg overflow-hidden ${className}`}
      aria-labelledby="match-hero-title"
    >
      {/* Header */}
      <div className="px-4 sm:px-6 py-3 sm:py-4 bg-brand-green-900">
        <h2
          id="match-hero-title"
          className="text-heading-sm sm:text-heading-m font-bold text-brand-orange-pure uppercase tracking-wide truncate"
        >
          {match.competition.name} — {match.type}
        </h2>
        <p className="text-body-xs sm:text-body-sm text-brand-orange-pure/80 mt-1 truncate">
          {match.date
            ? new Date(match.date).toLocaleDateString("fr-FR", {
                weekday: "long",
                year: "numeric",
                month: "long",
                day: "numeric",
              })
            : "Date à confirmer"}
          {match.time && " · " + match.time}
          {match.location && " · " + match.location}
        </p>
      </div>

      {/* Main content - Flex layout for desktop, Column for mobile */}
      <div className="px-4 sm:px-6 py-4 sm:py-8 flex flex-col lg:flex-row items-center justify-between gap-4 sm:gap-6 lg:gap-8 min-h-[200px] sm:min-h-[240px] lg:min-h-[280px]">
        {/* Team 1 */}
        <div className="flex-1 flex flex-col items-center gap-2 sm:gap-3 lg:gap-4">
          <div className="relative">
            <div className="w-24 h-24 sm:w-28 sm:h-28 lg:w-40 lg:h-40 rounded-full bg-brand-green-900 border-2 sm:border-4 border-brand-orange-pure flex flex-col items-center justify-center transition-all duration-200">
              {match.team1?.logo ? (
                <img
                  src={match.team1.logo}
                  alt={match.team1.name}
                  className="w-full h-full rounded-full object-cover"
                />
              ) : (
                <>
                  <span className="text-heading-xs sm:text-heading-sm lg:text-heading-xl font-black text-brand-orange-pure">
                    {match.team1?.name?.charAt(0) || "?"}
                  </span>
                  {match.team1?.name && (
                    <span className="text-caption sm:text-body-xs font-bold text-brand-orange-pure/80 text-center">
                      {match.team1.name.split(" ")[0] || "Team"}
                    </span>
                  )}
                </>
              )}
            </div>
          </div>
          <span className="text-body-sm sm:text-body-m lg:text-heading-l font-bold text-white truncate max-w-full text-center">
            {match.team1?.name || "Équipe 1"}
          </span>
          {match.team1?.seed && (
            <span className="text-body-xs sm:text-body-sm text-brand-orange-pure/80">
              Tête de série: {match.team1.seed}
            </span>
          )}
        </div>

        {/* Score - Center section */}
        <div className="flex flex-col items-center gap-2 my-4 lg:my-0">
          {/* Match date/time for mobile */}
          <div className="lg:hidden flex items-center gap-2">
            <Badge variant={getStatusVariant()}>{getStatusText()}</Badge>
          </div>

          <div className="flex items-baseline gap-1 sm:gap-2 bg-brand-orange-pure px-3 sm:px-4 py-2 sm:py-3 rounded-lg sm:rounded-xl shadow-lg">
            <span
              className="text-heading-xl sm:text-display-sm lg:text-display-m font-black text-brand-green-900"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team1Score || 0}
            </span>
            <span className="text-body-sm sm:text-heading-s lg:text-heading-l font-bold text-brand-green-900">
              —
            </span>
            <span
              className="text-heading-xl sm:text-display-sm lg:text-display-m font-black text-brand-green-900"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team2Score || 0}
            </span>
          </div>

          {/* Status and info */}
          <div className="hidden lg:flex items-center gap-2 bg-brand-green-900/80 px-4 py-2 rounded-full">
            {match.status === "in_progress" && (
              <>
                <span className="w-2 h-2 rounded-full bg-red-600 animate-pulse"></span>
                <span className="text-body-xs sm:text-body-sm font-bold text-red-500 uppercase">
                  LIVE
                </span>
              </>
            )}
            <Badge variant={getStatusVariant()} className="text-xs">
              {getStatusText()}
            </Badge>
          </div>

          {/* Manche info */}
          {match.status === "in_progress" && match.currentManche && (
            <div className="flex items-center gap-1 mt-2 lg:mt-0">
              <Icon name="Target" size="xs" className="text-brand-orange-pure/80" />
              <span className="text-caption sm:text-body-xs text-brand-orange-pure/80">
                Manche {match.currentManche}
                {match.stats?.totalManches ? ` / ${match.stats.totalManches}` : ""}
                {match.bestOf ? ` (Meilleur sur ${match.bestOf})` : ""}
              </span>
            </div>
          )}
        </div>

        {/* Team 2 */}
        <div className="flex-1 flex flex-col items-center gap-2 sm:gap-3 lg:gap-4">
          <div className="relative">
            <div className="w-24 h-24 sm:w-28 sm:h-28 lg:w-40 lg:h-40 rounded-full bg-brand-green-900 border-2 sm:border-4 border-brand-orange-pure flex flex-col items-center justify-center transition-all duration-200">
              {match.team2?.logo ? (
                <img
                  src={match.team2.logo}
                  alt={match.team2.name}
                  className="w-full h-full rounded-full object-cover"
                />
              ) : (
                <>
                  <span className="text-heading-xs sm:text-heading-sm lg:text-heading-xl font-black text-brand-orange-pure">
                    {match.team2?.name?.charAt(0) || "?"}
                  </span>
                  {match.team2?.name && (
                    <span className="text-caption sm:text-body-xs font-bold text-brand-orange-pure/80 text-center">
                      {match.team2.name.split(" ")[0] || "Team"}
                    </span>
                  )}
                </>
              )}
            </div>
          </div>
          <span className="text-body-sm sm:text-body-m lg:text-heading-l font-bold text-white truncate max-w-full text-center">
            {match.team2?.name || "Équipe 2"}
          </span>
          {match.team2?.seed && (
            <span className="text-body-xs sm:text-body-sm text-brand-orange-pure/80">
              Tête de série: {match.team2.seed}
            </span>
          )}
        </div>
      </div>

      {/* Status for mobile */}
      {match.status !== "in_progress" && (
        <div className="lg:hidden px-4 pb-4">
          <Badge variant={getStatusVariant()} className="w-full justify-center text-xs">
            {getStatusText()}
          </Badge>
        </div>
      )}

      {/* Additional info */}
      <div className="px-4 sm:px-6 pb-4 lg:hidden">
        <div className="flex flex-wrap justify-center gap-2 text-body-xs sm:text-body-sm text-brand-orange-pure/80">
          {match.currentManche && (
            <span>
              Manche {match.currentManche}
              {match.bestOf ? `/${match.bestOf}` : ""}
              {match.stats?.totalManches ? `/${match.stats.totalManches}` : ""}
            </span>
          )}
          {match.location && <span>· {match.location}</span>}
          {match.time && <span>· {match.time}</span>}
        </div>
      </div>
    </section>
  );
};

export default MatchHero;
