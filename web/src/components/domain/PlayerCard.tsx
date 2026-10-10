import React from 'react';
import { Link } from 'react-router-dom';
import { Player } from '../../types';
import { Badge } from '../common/Badge';
import Button from '../common/Button';

export interface PlayerCardProps {
  player: Player;
  accent?: 'gold' | 'primary' | 'amber';
  className?: string;
  showClub?: boolean;
  showActions?: boolean;
  variant?: 'default' | 'team';
}

/**
 * PlayerCard - Card displaying player information and stats
 * Used in grids on homepage and players page
 */
export const PlayerCard: React.FC<PlayerCardProps> = ({
  player,
  accent = 'primary',
  className = '',
  showClub = true,
  showActions = true,
  variant = 'default'
}) => {
  // For team variant, adjust accent based on player stats
  if (variant === 'team') {
    accent = player.stats.winRate >= 70 ? 'gold' : player.stats.winRate >= 50 ? 'primary' : 'amber';
  }

  const getAccentColor = () => {
    switch (accent) {
      case 'gold':
        return 'border-gold-700 bg-gradient-to-br from-gold-900/20 to-gold-800/10';
      case 'primary':
        return 'border-primary-600 bg-gradient-to-br from-primary-800/20 to-primary-700/10';
      case 'amber':
        return 'border-amber-700 bg-gradient-to-br from-amber-900/20 to-amber-800/10';
      default:
        return 'border-primary-700';
    }
  };

  const getAccentBorder = () => {
    switch (accent) {
      case 'gold':
        return 'border-gold-700';
      case 'primary':
        return 'border-primary-600';
      case 'amber':
        return 'border-amber-700';
      default:
        return 'border-primary-700';
    }
  };

  // Get initials from name
  const getInitials = (player: Player) => {
    return `${player.firstName.charAt(0)}${player.lastName.charAt(0)}`.toUpperCase();
  };

  // Compact version for team details
  if (variant === 'team') {
    return (
      <Link 
        to={`/joueurs/${player.id}`}
        className={`flex items-center gap-3 p-3 bg-primary-900 rounded-lg border border-primary-700/50 hover:bg-primary-800/50 transition-colors ${className}`}
      >
        <div className={`w-12 h-12 rounded-full ${getAccentBorder()} border-2 flex items-center justify-center bg-primary-800 overflow-hidden flex-shrink-0`}>
          {player.photo ? (
            <img 
              src={player.photo} 
              alt={`${player.firstName} ${player.lastName}`}
              className="w-full h-full object-cover"
            />
          ) : (
            <span className="text-heading-s font-black text-gold-700">
              {getInitials(player)}
            </span>
          )}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-body-s font-bold text-white truncate">
            {player.firstName} {player.lastName}
          </h3>
          <div className="flex items-center gap-2">
            {player.alias && (
              <span className="text-body-xs text-primary-300">({player.alias})</span>
            )}
            {showClub && player.club && (
              <span className="text-body-xs text-primary-300 truncate">
                {player.club.name}
              </span>
            )}
          </div>
        </div>
        <div className="text-center">
          <p className="text-body-xs text-primary-300">Victoires</p>
          <p className="text-body-s font-bold text-gold-700">{player.stats.winRate.toFixed(0)}%</p>
        </div>
      </Link>
    );
  }

  return (
    <article 
      className={`flex flex-col bg-primary-900 rounded-lg overflow-hidden border ${getAccentColor()} ${className}`}
    >
      {/* Accent border on left */}
      <div className={`h-1 ${accent === 'gold' ? 'bg-gold-700' : accent === 'primary' ? 'bg-primary-600' : 'bg-amber-700'}`} />

      {/* Avatar section */}
      <Link to={`/joueurs/${player.id}`} className="block">
        <div className="relative mx-auto -mt-8 mb-4 hover:scale-105 transition-transform">
          <div className={`w-24 h-24 rounded-full ${getAccentBorder()} border-4 flex items-center justify-center bg-primary-800 overflow-hidden`}>
            {player.photo ? (
              <img 
                src={player.photo} 
                alt={`${player.firstName} ${player.lastName}`}
                className="w-full h-full object-cover"
              />
            ) : (
              <span className="text-heading-l font-black text-gold-700">
                {getInitials(player)}
              </span>
            )}
          </div>
        </div>
      </Link>

      {/* Content */}
      <div className="flex-1 px-4 pb-4 space-y-3">
        {/* Name */}
        <Link to={`/joueurs/${player.id}`} className="block">
          <h3 className="text-heading-s font-bold text-white text-center truncate hover:text-gold-700 transition-colors">
            {player.firstName} {player.lastName}
          </h3>
        </Link>

        {/* Club */}
        {showClub && player.club && (
          <p className="text-body-s text-primary-200 text-center truncate">
            {player.club.name}
          </p>
        )}

        {player.alias && (
          <p className="text-body-xs text-primary-300 text-center truncate">
            "{player.alias}"
          </p>
        )}

        {/* Stats */}
        <div className="pt-2 border-t border-primary-700/50 space-y-2">
          <div className="flex justify-between">
            <span className="text-caption text-primary-200">Points marqués :</span>
            <span className="text-body-m font-bold text-gold-700">{player.stats.totalPoints}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-caption text-primary-200">Moyenne / manche :</span>
            <span className="text-body-m font-bold text-gold-700">{player.stats.pointsPerManche.toFixed(1)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-caption text-primary-200">Victoires :</span>
            <span className="text-body-m font-bold text-gold-700">{player.stats.winRate.toFixed(0)}%</span>
          </div>
        </div>
      </div>

      {/* Footer actions */}
      {showActions && (
        <div className="px-4 py-3 bg-primary-800/50 border-t border-primary-700/50">
          <Link to={`/joueurs/${player.id}`} className="block">
            <Button variant="ghost" size="sm" fullWidth>
              Voir le profil
            </Button>
          </Link>
        </div>
      )}
    </article>
  );
};

export default PlayerCard;
