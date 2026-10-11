import React from "react";
import { Badge } from "../common/Badge";
import { Icon } from "../common/Icon";

export interface LiveScoreMatch {
  id: string;
  team1Name: string;
  team2Name: string;
  team1Score: number;
  team2Score: number;
  status: "scheduled" | "in_progress" | "completed" | "postponed" | "cancelled";
  currentManche: number;
}

export interface LiveScoresProps {
  matches: LiveScoreMatch[];
  journee?: number;
  className?: string;
}

/**
 * LiveScores - Scrolling display of current live scores
 * Used on homepage as a ticker
 * Fully responsive with horizontal scrolling on mobile
 */
export const LiveScores: React.FC<LiveScoresProps> = ({ matches, journee = 1, className = "" }) => {
  const getStatusText = (match: LiveScoreMatch) => {
    if (match.status === "in_progress") {
      return `Manche ${match.currentManche} · en cours`;
    }
    if (match.status === "completed") {
      return "Terminé";
    }
    if (match.status === "scheduled") {
      return "À venir";
    }
    return match.status;
  };

  const getStatusVariant = (match: LiveScoreMatch) => {
    if (match.status === "in_progress") return "warning" as const;
    if (match.status === "completed") return "success" as const;
    return "secondary" as const;
  };

  if (matches.length === 0) {
    return (
      <section className={`w-full ${className}`} aria-labelledby="live-scores-title">
        <div className="px-4 sm:px-6 py-3 sm:py-4 bg-[#000000]">
          <div className="flex items-center gap-2">
            <Icon name="Clock" size="sm" className="text-gold-pure" />
            <h2
              id="live-scores-title"
              className="text-body-sm sm:text-body-m font-bold text-gold-pure uppercase tracking-wide"
            >
              Scores en direct — {journee}e journée
            </h2>
          </div>
        </div>
        <div className="bg-[#000000] border-t border-neutral-800">
          <p className="text-body-sm text-tertiary text-center py-6 sm:py-8">
            Aucun match en direct pour le moment
          </p>
        </div>
      </section>
    );
  }

  return (
    <section
      className={`live-scores-responsive w-full ${className}`}
      aria-labelledby="live-scores-title"
    >
      {/* Header */}
      <div className="px-4 sm:px-6 py-3 sm:py-4 bg-[#000000] border-b border-neutral-800">
        <div className="flex items-center gap-2">
          <Icon name="Clock" size="sm" className="text-gold-pure flex-shrink-0" />
          <h2
            id="live-scores-title"
            className="text-body-sm sm:text-body-m font-bold text-gold-pure uppercase tracking-wide truncate"
          >
            Scores en direct — {journee}e journée
          </h2>
          <span className="ml-auto text-body-xs text-gold-pure/80">
            {matches.length} match{matches.length > 1 ? "s" : ""}
          </span>
        </div>
      </div>

      {/* Scores container with horizontal scrolling */}
      <div className="bg-[#000000]">
        <div className="live-scores-grid flex gap-2 sm:gap-3 lg:gap-4 p-2 sm:p-3 lg:p-4 min-w-max">
          {matches.map((match) => (
            <div
              key={match.id}
              className="live-score-card flex-shrink-0 bg-green-dark rounded-lg p-3 sm:p-4 border border-green-light hover:border-gold-pure transition-colors min-w-[160px] sm:min-w-[200px] lg:min-w-[240px]"
              aria-live="polite"
            >
              <div className="flex flex-col gap-2">
                {/* Teams and Score */}
                <div className="flex items-center justify-between gap-2">
                  {/* Team 1 */}
                  <div className="flex-1 min-w-0">
                    <span className="text-body-xs sm:text-body-sm font-medium text-white truncate block max-w-[80px] sm:max-w-[100px] lg:max-w-[120px]">
                      {match.team1Name}
                    </span>
                  </div>

                  {/* Score */}
                  <div className="flex flex-col items-center mx-1">
                    <div className="flex items-baseline gap-1 sm:gap-2">
                      <span
                        className={`text-body-sm sm:text-heading-sm font-bold ${
                          match.status === "in_progress" ? "text-gold-pure" : "text-white"
                        }`}
                      >
                        {match.team1Score}
                      </span>
                      <span className="text-body-xs sm:text-body-sm font-bold text-gold-pure/80">
                        —
                      </span>
                      <span
                        className={`text-body-sm sm:text-heading-sm font-bold ${
                          match.status === "in_progress" ? "text-gold-pure" : "text-white"
                        }`}
                      >
                        {match.team2Score}
                      </span>
                    </div>

                    {/* Status indicator */}
                    {match.status === "in_progress" && (
                      <div className="flex items-center gap-1 mt-1">
                        <span className="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse" />
                      </div>
                    )}
                  </div>

                  {/* Team 2 */}
                  <div className="flex-1 min-w-0">
                    <span className="text-body-xs sm:text-body-sm font-medium text-white truncate block max-w-[80px] sm:max-w-[100px] lg:max-w-[120px] text-right">
                      {match.team2Name}
                    </span>
                  </div>
                </div>

                {/* Status text */}
                <div className="flex justify-center">
                  <span className="text-caption sm:text-body-xs text-gold-pure/80 truncate max-w-full">
                    {getStatusText(match)}
                  </span>
                </div>

                {/* Status badge for larger screens */}
                <div className="hidden sm:flex justify-center">
                  <Badge variant={getStatusVariant(match)} className="text-xs">
                    {match.status === "in_progress" ? "LIVE" : getStatusText(match)}
                  </Badge>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default LiveScores;
