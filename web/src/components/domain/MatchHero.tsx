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
      className={`w-full bg-primary-900 rounded-lg overflow-hidden ${className}`}
      aria-labelledby="match-hero-title"
    >
      {/* Header */}
      <div className="px-6 py-4 bg-gradient-to-r from-primary-800 to-primary-900">
        <h2 
          id="match-hero-title" 
          className="text-heading-m font-bold text-gold-700 uppercase tracking-wide"
        >
          {match.competition.name} — {match.type}
        </h2>
        <p className="text-body-m text-primary-200 mt-1">
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
            <div className="w-40 h-40 rounded-full bg-primary-800 border-4 border-gold-700 flex flex-col items-center justify-center">
              <span className="text-heading-xl font-black text-primary-700">
                {(match.team1.name || 'T1').substring(0, 2).toUpperCase()}
              </span>
            </div>
          </div>
          <span className="text-heading-l font-bold text-white truncate max-w-full">
            {match.team1.name || 'Équipe 1'}
          </span>
        </div>

        {/* Score */}
        <div className="flex flex-col items-center gap-2">
          <div className="flex items-baseline gap-8">
            <span 
              className="text-display-m font-black text-gold-700"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team1Score || 0}
            </span>
            <span className="text-heading-l font-bold text-primary-200">—</span>
            <span 
              className="text-display-m font-black text-gold-700"
              aria-live="polite"
              aria-atomic="true"
            >
              {match.team2Score || 0}
            </span>
          </div>
          
          <div className="flex items-center gap-2 bg-gold-900/50 px-4 py-2 rounded-full">
            {match.status === 'in_progress' && (
              <>
                <span className="w-2 h-2 rounded-full bg-red-600 animate-pulse"></span>
                <span className="text-body-s font-bold text-gold-700 uppercase">
                  LIVE
                </span>
                <span className="text-body-xs text-primary-200">
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
            <div className="w-40 h-40 rounded-full bg-primary-800 border-4 border-gold-700 flex flex-col items-center justify-center">
              <span className="text-heading-xl font-black text-primary-700">
                {(match.team2.name || 'T2').substring(0, 2).toUpperCase()}
              </span>
            </div>
          </div>
          <span className="text-heading-l font-bold text-white truncate max-w-full">
            {match.team2.name || 'Équipe 2'}
          </span>
        </div>
      </div>

      {/* Info footer */}
      <div className="px-6 py-4 bg-primary-800/50">
        <div className="flex items-center gap-2 text-body-m text-primary-200">
          <Icon name="Info" size="sm" />
          <span>Terrain clos municipal — entrée gratuite · buvette et fan zone</span>
        </div>
      </div>
    </section>
  );
};

// Default export for consistency
export default MatchHero;
