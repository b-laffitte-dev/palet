import React from 'react';
import { Link } from 'react-router-dom';
import { Tournament } from '../../types';
import Button from '../common/Button';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';

export interface TournamentCardProps {
  tournament: Tournament;
  className?: string;
  showActions?: boolean;
}

/**
 * TournamentCard - Card displaying tournament information
 * Used in grids on homepage and tournaments page
 */
export const TournamentCard: React.FC<TournamentCardProps> = ({ 
  tournament, 
  className = '',
  showActions = true
}) => {
  const getTypeColor = (type: Tournament['type']) => {
    switch (type) {
      case 'tournament':
        return 'border-gold-700 bg-gradient-to-br from-gold-900/20 to-gold-800/10';
      case 'cup':
        return 'border-primary-600 bg-gradient-to-br from-primary-800/20 to-primary-700/10';
      default:
        return 'border-primary-700';
    }
  };

  const getTypeLabel = (type: Tournament['type']) => {
    switch (type) {
      case 'tournament':
        return 'TOURNOI';
      case 'cup':
        return 'COUPE';
      default:
        return type.toUpperCase();
    }
  };

  const getStatusBadge = (status: Tournament['status']) => {
    switch (status) {
      case 'in_progress':
        return <Badge variant="warning">En cours</Badge>;
      case 'completed':
        return <Badge variant="success">Terminé</Badge>;
      case 'pending':
      case 'registration':
        return <Badge variant="info">Inscription</Badge>;
      case 'cancelled':
        return <Badge variant="danger">Annulé</Badge>;
      default:
        return <Badge variant="secondary">À venir</Badge>;
    }
  };

  // Format date range
  const formatDate = (start: Date | null, end: Date | null) => {
    if (!start) return 'Date à confirmer';
    if (!end) {
      return start.toLocaleDateString('fr-FR', { 
        weekday: 'long', 
        year: 'numeric', 
        month: 'long', 
        day: 'numeric' 
      });
    }
    
    const startDate = new Date(start);
    const endDate = new Date(end);
    
    if (startDate.getMonth() === endDate.getMonth() && startDate.getFullYear() === endDate.getFullYear()) {
      return `${startDate.getDate()}-${endDate.getDate()} ${startDate.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })}`;
    }
    return startDate.toLocaleDateString('fr-FR');
  };

  return (
    <article 
      className={`flex flex-col bg-primary-900 rounded-lg overflow-hidden border ${getTypeColor(tournament.type)} ${className}`}
    >
      {/* Header */}
      <Link to={`/tournois/${tournament.id}`} className="block">
        <div className={`px-4 py-3 ${'bg-primary-800/20'}`}>
          <Badge variant="primary" className="mb-2 text-xs">
            {getTypeLabel(tournament.type)}
          </Badge>
          <h3 className="text-heading-s font-bold text-white truncate hover:text-gold-700 transition-colors">
            {tournament.name}
          </h3>
        </div>
      </Link>

      {/* Body */}
      <div className="flex-1 p-4 space-y-3">
        {/* Status indicator */}
        <div className="flex justify-between items-start">
          {getStatusBadge(tournament.status)}
        </div>

        {/* Date */}
        <div className="flex items-center gap-2 text-body-s">
          <Icon name="Calendar" size="sm" className="text-primary-300" />
          <span className="text-primary-200">
            {formatDate(tournament.startDate, tournament.endDate)}
          </span>
        </div>

        {/* Location */}
        <div className="flex items-center gap-2 text-body-s">
          <Icon name="MapPin" size="sm" className="text-primary-300" />
          <span className="text-primary-200 truncate">{tournament.location || 'Lieu à confirmer'}</span>
        </div>

        {/* Players info */}
        <div className="flex items-center gap-2 text-body-s">
          <Icon name="Users" size="sm" className="text-primary-300" />
          <span className="text-primary-200">
            {tournament.registeredTeams || tournament.maxTeams || 0} équipes
          </span>
        </div>
      </div>

      {/* Footer Actions */}
      {showActions && (
        <div className="px-4 py-3 bg-primary-800/50 border-t border-primary-700/50">
          {tournament.status === 'registration' ? (
            <Button variant="secondary" size="sm" className="w-full">
              <Icon name="Plus" size="sm" />
              S'inscrire
            </Button>
          ) : tournament.status === 'in_progress' ? (
            <Link to={`/tournois/${tournament.id}`} className="block">
              <Button variant="secondary" size="sm" className="w-full">
                Suivre en direct
              </Button>
            </Link>
          ) : (
            <Link to={`/tournois/${tournament.id}`} className="block">
              <Button variant="ghost" size="sm" className="w-full">
                Voir les résultats
              </Button>
            </Link>
          )}
        </div>
      )}
    </article>
  );
};

export default TournamentCard;
