import React from 'react';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';

export interface LiveScoreMatch {
  id: string;
  team1Name: string;
  team2Name: string;
  team1Score: number;
  team2Score: number;
  status: 'scheduled' | 'in_progress' | 'completed' | 'postponed' | 'cancelled';
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
 */
export const LiveScores: React.FC<LiveScoresProps> = ({ 
  matches, 
  journee = 1, 
  className = '' 
}) => {
  const getStatusText = (match: LiveScoreMatch) => {
    if (match.status === 'in_progress') {
      return `Manche ${match.currentManche} · en cours`;
    }
    if (match.status === 'completed') {
      return 'Terminé';
    }
    if (match.status === 'scheduled') {
      return 'À venir';
    }
    return match.status;
  };

  const getStatusVariant = (match: LiveScoreMatch) => {
    if (match.status === 'in_progress') return 'warning' as const;
    if (match.status === 'completed') return 'success' as const;
    return 'secondary' as const;
  };

  return (
    <section className={`w-full ${className}`} aria-labelledby="live-scores-title">
      <div className="px-6 py-4 bg-primary-800/50">
        <div className="flex items-center gap-2">
          <Icon name="Clock" size="sm" className="text-gold-700" />
          <h2 
            id="live-scores-title"
            className="text-body-m font-bold text-gold-700 uppercase tracking-wide"
          >
            Scores en direct — {journee}e journée
          </h2>
        </div>
      </div>

      <div className="overflow-x-auto">
        <div className="flex gap-4 p-4 min-w-max">
          {matches.map((match) => (
            <div 
              key={match.id}
              className="min-w-[300px] flex-shrink-0 bg-primary-900 rounded-lg p-4 border border-primary-700/50 hover:border-gold-700 transition-colors"
              aria-live="polite"
            >
              <div className="flex items-center justify-between gap-4">
                {/* Teams */}
                <div className="flex-1">
                  <span className="text-body-m font-medium text-white truncate max-w-[100px] block">
                    {match.team1Name}
                  </span>
                </div>

                {/* Score */}
                <div className="flex flex-col items-center">
                  <div className="flex items-baseline gap-2">
                    <span 
                      className={`text-heading-s font-bold ${
                        match.status === 'in_progress' ? 'text-gold-700' : 'text-white'
                      }`}
                    >
                      {match.team1Score}
                    </span>
                    <span className="text-body-l font-bold text-primary-200">—</span>
                    <span 
                      className={`text-heading-s font-bold ${
                        match.status === 'in_progress' ? 'text-gold-700' : 'text-white'
                      }`}
                    >
                      {match.team2Score}
                    </span>
                  </div>
                </div>

                {/* Team 2 */}
                <div className="flex-1">
                  <span className="text-body-m font-medium text-white truncate max-w-[100px] block text-right">
                    {match.team2Name}
                  </span>
                </div>
              </div>

              {/* Status */}
              <div className="mt-3 flex items-center justify-center">
                <Badge 
                  variant={getStatusVariant(match)}
                  className="text-xs"
                >
                  {getStatusText(match)}
                </Badge>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default LiveScores;
