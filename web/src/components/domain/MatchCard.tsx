import React from 'react';
import { Link } from 'react-router-dom';
import { Match } from '../../types';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';

export interface MatchCardProps {
  match: Match;
  compact?: boolean;
  className?: string;
  onClick?: () => void;
  showActions?: boolean;
  showTournament?: boolean;
}

/**
 * MatchCard - Card displaying match information
 * Used in lists of matches, can be compact or full
 */
export const MatchCard: React.FC<MatchCardProps> = ({
  match,
  compact = false,
  className = '',
  onClick,
  showActions = true,
  showTournament = true
}) => {
  const getStatusText = () => {
    if (match.status === 'in_progress') {
      return `Manche ${match.currentManche || 1} · en cours`;
    }
    if (match.status === 'completed') {
      return 'Terminé';
    }
    if (match.status === 'scheduled') {
      return `À venir · ${match.time || ''}`;
    }
    return match.status;
  };

  const getStatusVariant = () => {
    if (match.status === 'in_progress') return 'warning' as const;
    if (match.status === 'completed') return 'success' as const;
    return 'secondary' as const;
  };

  const getStatusColor = () => {
    if (match.status === 'in_progress') return 'border-gold-700 bg-gold-900/10';
    if (match.status === 'completed') return 'border-green-700 bg-green-900/10';
    return 'border-primary-700';
  };

  if (compact) {
    return (
      <Link 
        to={`/matchs/${match.id}`}
        className={`flex items-center gap-4 p-3 rounded-lg border ${getStatusColor()} ${className} ${onClick ? 'cursor-pointer hover:bg-primary-800/50 transition-colors' : ''}`}
        onClick={onClick}
        aria-live="polite"
      >
        <div className="flex-1 flex items-center gap-2 min-w-0">
          <div className="min-w-0">
            <p className="text-body-m font-medium text-white truncate">
              {match.team1.name || 'Équipe 1'}
            </p>
            <p className="text-body-m font-medium text-white truncate">
              {match.team2.name || 'Équipe 2'}
            </p>
          </div>
        </div>

        <div className="flex-shrink-0 text-center">
          <div className="flex items-baseline gap-2">
            <span className="text-body-l font-bold text-gold-700">{match.team1Score || 0}</span>
            <span className="text-body-m text-primary-200">—</span>
            <span className="text-body-l font-bold text-gold-700">{match.team2Score || 0}</span>
          </div>
          <Badge variant={getStatusVariant()} className="text-xs mt-1">
            {getStatusText()}
          </Badge>
        </div>
      </Link>
    );
  }

  return (
    <Link 
      to={`/matchs/${match.id}`}
      className={`rounded-lg border ${getStatusColor()} overflow-hidden ${className} ${onClick ? 'cursor-pointer hover:shadow-lg transition-all' : ''}`}
      onClick={onClick}
    >
      {/* Header */}
      {showTournament && (
        <div className="px-4 py-2 bg-primary-800/50">
          <p className="text-body-xs font-medium text-primary-200 uppercase truncate">
            {match.competition.name}
          </p>
        </div>
      )}

      {/* Body */}
      <div className="p-4">
        <div className="flex items-center justify-between gap-4">
          {/* Team 1 */}
          <div className="flex-1 flex items-center gap-3 min-w-0">
            {match.team1.logo && (
              <img
                src={match.team1.logo}
                alt={match.team1.name}
                className="w-8 h-8 rounded-full object-contain"
              />
            )}
            <div className="min-w-0">
              <p className="text-body-m font-medium text-white truncate">
                {match.team1.name || 'Équipe 1'}
              </p>
            </div>
          </div>

          {/* Score */}
          <div className="flex-shrink-0 text-center">
            <div className="flex items-baseline gap-2">
              <span 
                className={`text-heading-m font-bold ${
                  match.status === 'in_progress' ? 'text-gold-700' : 'text-white'
                }`}
              >
                {match.team1Score || 0}
              </span>
              <span className="text-body-l text-primary-200">—</span>
              <span 
                className={`text-heading-m font-bold ${
                  match.status === 'in_progress' ? 'text-gold-700' : 'text-white'
                }`}
              >
                {match.team2Score || 0}
              </span>
            </div>
            <Badge 
              variant={getStatusVariant()}
              className="text-xs mt-2"
            >
              {getStatusText()}
            </Badge>
          </div>

          {/* Team 2 */}
          <div className="flex-1 flex items-center gap-3 min-w-0 justify-end">
            {match.team2.logo && (
              <img
                src={match.team2.logo}
                alt={match.team2.name}
                className="w-8 h-8 rounded-full object-contain"
              />
            )}
            <div className="min-w-0 text-right">
              <p className="text-body-m font-medium text-white truncate">
                {match.team2.name || 'Équipe 2'}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-2 bg-primary-800/30 border-t border-primary-700/50">
        <p className="text-caption text-primary-200 truncate">
          <Icon name="MapPin" size="xs" className="inline mr-1" />
          {match.location || 'Lieu non précisé'}
          {match.date && (
            <span className="mx-2">·</span>
          )}
          {match.date && (
            <span>{new Date(match.date).toLocaleDateString('fr-FR')}</span>
          )}
        </p>
      </div>
    </Link>
  );
};

export default MatchCard;
