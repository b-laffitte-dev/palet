import React from 'react';
import { Link } from 'react-router-dom';
import { Club } from '../../types';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';
import Button from '../common/Button';

export interface ClubCardProps {
  club: Club;
  showActions?: boolean;
  className?: string;
  compact?: boolean;
}

/**
 * ClubCard - Card displaying club information
 * Used in grids on clubs page and potentially on homepage
 */
export const ClubCard: React.FC<ClubCardProps> = ({
  club,
  showActions = true,
  className = '',
  compact = false
}) => {
  // Get division badge
  const getDivisionBadge = (division: string) => {
    switch (division.toLowerCase()) {
      case 'division 1':
      case 'div. 1':
      case 'd1':
        return <Badge variant="gold" className="text-xs">DIVISION 1</Badge>;
      case 'division 2':
      case 'div. 2':
      case 'd2':
        return <Badge variant="primary" className="text-xs">DIVISION 2</Badge>;
      case 'division 3':
      case 'div. 3':
      case 'd3':
        return <Badge variant="secondary" className="text-xs">DIVISION 3</Badge>;
      default:
        return <Badge variant="subtle" className="text-xs">{division.toUpperCase()}</Badge>;
    }
  };

  // Compact version for detail pages
  if (compact) {
    return (
      <Link 
        to={`/clubs/${club.id}`}
        className={`flex items-center gap-3 p-3 bg-primary-900 rounded-lg border border-primary-700/50 hover:bg-primary-800/50 transition-colors ${className}`}
      >
        <div className="w-12 h-12 rounded-full bg-primary-800 border-2 border-gold-700 flex items-center justify-center flex-shrink-0">
          {club.logo ? (
            <img 
              src={club.logo} 
              alt={`Logo ${club.name}`}
              className="w-10 h-10 object-contain"
            />
          ) : (
            <span className="text-heading-s font-black text-gold-700">
              {club.code || club.name.substring(0, 2)}
            </span>
          )}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-body-s font-bold text-white truncate">
            {club.name}
          </h3>
          <p className="text-body-xs text-primary-200 truncate">
            {club.city} - {club.division}
          </p>
        </div>
        {getDivisionBadge(club.division)}
      </Link>
    );
  }

  return (
    <article 
      className={`flex flex-col bg-primary-900 rounded-lg overflow-hidden border border-primary-700/50 ${className}`}
    >
      {/* Header with logo and name */}
      <Link to={`/clubs/${club.id}`} className="block">
        <div className="flex items-center gap-4 p-4 bg-gradient-to-r from-primary-800 to-primary-700 hover:bg-primary-700/80 transition-colors">
          <div className="w-16 h-16 rounded-full bg-primary-800 border-2 border-gold-700 flex items-center justify-center flex-shrink-0">
            {club.logo ? (
              <img 
                src={club.logo} 
                alt={`Logo ${club.name}`}
                className="w-12 h-12 object-contain"
              />
            ) : (
              <span className="text-heading-m font-black text-gold-700">
                {club.code || club.name.substring(0, 2)}
              </span>
            )}
          </div>
          <div className="flex-1 min-w-0">
            <h3 className="text-heading-s font-bold text-white truncate">
              {club.name}
            </h3>
            <p className="text-body-s text-primary-200 truncate">
              {club.city}
            </p>
          </div>
          <div className="flex-shrink-0">
            {getDivisionBadge(club.division)}
          </div>
        </div>
      </Link>

      {/* Body with stats */}
      <div className="flex-1 p-4 space-y-3">
        {/* Stats row */}
        <div className="grid grid-cols-3 gap-4 text-center">
          <div>
            <p className="text-caption text-primary-200">Joueurs</p>
            <p className="text-body-l font-bold text-gold-700">{club.playersCount}</p>
          </div>
          <div>
            <p className="text-caption text-primary-200">Équipes</p>
            <p className="text-body-l font-bold text-gold-700">{club.teamsCount}</p>
          </div>
          <div>
            <p className="text-caption text-primary-200">Ligue</p>
            <p className="text-body-l font-bold text-gold-700">{club.league}</p>
          </div>
        </div>

        {/* Contact info */}
        <div className="pt-2 border-t border-primary-700/50 space-y-1">
          {club.contactEmail && (
            <p className="text-body-xs text-primary-200 truncate">
              <Icon name="Mail" size="xs" className="inline mr-1" />
              {club.contactEmail}
            </p>
          )}
          {club.contactPhone && (
            <p className="text-body-xs text-primary-200 truncate">
              <Icon name="Phone" size="xs" className="inline mr-1" />
              {club.contactPhone}
            </p>
          )}
          {club.website && (
            <p className="text-body-xs text-primary-200 truncate">
              <Icon name="Globe" size="xs" className="inline mr-1" />
              {club.website}
            </p>
          )}
        </div>
      </div>

      {/* Footer actions */}
      {showActions && (
        <div className="px-4 py-3 bg-primary-800/50 border-t border-primary-700/50 flex gap-2">
          <Link to={`/clubs/${club.id}`} className="flex-1">
            <Button variant="ghost" size="sm" fullWidth>
              <Icon name="Eye" size="sm" />
              Voir
            </Button>
          </Link>
          <Button variant="secondary" size="sm" className="flex-1">
            <Icon name="Users" size="sm" />
            Effectif
          </Button>
        </div>
      )}
    </article>
  );
};

export default ClubCard;
