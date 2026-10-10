import React from 'react';
import { Match } from '../../types';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';

export interface MatchHeroProps {
  match: Match;
  className?: string;
}

/**
 * MatchHero - Featured live match display (Top 14 style)
 * Used on homepage to showcase the main live match
 */
export const MatchHero: React.FC<MatchHeroProps> = ({ match, className = '' }) => {
  const getStatusVariant = () => {
    if (match.status === 'in_progress') return 'warning' as const;
    if (match.status === 'completed') return 'success' as const;
    return 'secondary' as const;
  };

  const getStatusText = () => {
    if (match.status === 'in_progress') {
      return `LIVE — Manche ${match.currentManche || 1}`;
    }
    if (match.status === 'completed') {
      return 'Terminé';
    }
    if (match.status === 'scheduled') {
      return 'À venir';
    }
    return match.status;
  };

  return (
    <section 
      className={`w-full bg-green-dark rounded-lg overflow-hidden ${className}`}
      aria-labelledby="match-hero-title"
    >
      {/* Header */}
      <div className="px-6 py-4 bg-green-dark">
        <h2 
          id="match-hero-title" 
          className="text-heading-m font-bold text-gold-pure uppercase tracking-wide"
        >
          {match.competition.name} — {match.type}
        </h2>
        <p className="text-body-m text-gold-pure/80 mt-1">
          {new Date(match.date).toLocaleDateString('fr-FR', {
            weekday: 'long',
            year: 'numeric',
            month: 'long',
            day: 'numeric'
          })} · {match.time} · {match.location}
        </p>
      </div>

      {/* Main content */}
      <div className="px-6 py-8 flex items-center justify-between gap-8 min-h-[280px]">
        {/* Team 1 */}
        <div className="flex-1 flex flex-col items-center gap-4">
          <div className="relative">
            <div className="w-40 h-40 rounded-full bg-green-dark border-4 border-gold-pure flex flex-col items-center justify-center">
              <span className="text-heading-xl font-black text-gold-pure">
                PC
              </span>
              <span className="text-body-xs font-bold text-gold-pure/80">PALET CLUB</span>
            </div>
          </div>
          <span className="text-heading-l font-bold text-white truncate max-w-full">
            {match.team1.name || 'Équipe 1'}
          </span>
          <span className="text-body-sm text-gold-pure/80">
            Vainqueur poule A
          </span>
        </div>

        {/* Score */}
        <div className="flex flex-col items-center gap-2">
          <div className="flex items-baseline gap-2 bg-gold-pure px-4 py-2 rounded-lg">
            <span 
              className="text-display-m font-black text-green-dark"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team1Score || 0}
            </span>
            <span className="text-heading-l font-bold text-green-dark">—</span>
            <span 
              className="text-display-m font-black text-green-dark"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team2Score || 0}
            </span>
          </div>
          
          <div className="flex items-center gap-2 bg-green-darker px-4 py-2 rounded-full">
            {match.status === 'in_progress' && (
              <>
                <span className="w-2 h-2 rounded-full bg-red-600 animate-pulse"></span>
                <span className="text-body-s font-bold text-red-500 uppercase">
                  LIVE
                </span>
                <span className="text-body-xs text-gold-pure">
                  Manche {match.currentManche || 1}
                </span>
              </>
            )}
            {match.status !== 'in_progress' && (
              <Badge variant={getStatusVariant()}>
                {getStatusText()}
              </Badge>
            )}
          </div>
        </div>

        {/* Team 2 */}
        <div className="flex-1 flex flex-col items-center gap-4">
          <div className="relative">
            <div className="w-40 h-40 rounded-full bg-green-dark border-4 border-gold-pure flex flex-col items-center justify-center">
              <span className="text-heading-xl font-black text-gold-pure">
                FC
              </span>
              <span className="text-body-xs font-bold text-gold-pure/80">FONTENAY</span>
            </div>
          </div>
          <span className="text-heading-l font-bold text-white truncate max-w-full">
            {match.team2.name || 'Équipe 2'}
          </span>
          <span className="text-body-sm text-gold-pure/80">
            Vainqueur poule B
          </span>
        </div>
      </div>

      {/* Info footer */}
      <div className="px-6 py-4 bg-green-darker">
        <div className="flex items-center gap-2 text-body-m text-gold-pure/80 justify-center">
          <span>★ Terrain clos municipal — entrée gratuite · buvette et fan zone</span>
        </div>
      </div>
    </section>
  );
};

// Default export for consistency
export default MatchHero;
