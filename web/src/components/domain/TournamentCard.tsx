import React from "react";
import { Link } from "react-router-dom";
import { Tournament } from "../../types";
import Button from "../common/Button";
import { Badge } from "../common/Badge";
import { Icon } from "../common/Icon";

export interface TournamentCardProps {
  tournament: Tournament;
  className?: string;
  showActions?: boolean;
}

/**
 * TournamentCard - Card displaying tournament information
 * Used in grids on homepage and tournaments page
 * Fully responsive for all screen sizes
 */
export const TournamentCard: React.FC<TournamentCardProps> = ({
  tournament,
  className = "",
  showActions = true,
}) => {
  const getTypeColor = (type: Tournament["type"]) => {
    switch (type) {
      case "tournament":
        return "border-gold-700 bg-gradient-to-br from-gold-900/20 to-gold-800/10";
      case "cup":
        return "border-primary-600 bg-gradient-to-br from-primary-800/20 to-primary-700/10";
      default:
        return "border-primary-700";
    }
  };

  const getTypeLabel = (type: Tournament["type"]) => {
    switch (type) {
      case "tournament":
        return "TOURNOI";
      case "cup":
        return "COUPE";
      default:
        return type.toUpperCase();
    }
  };

  const getStatusBadge = (status: Tournament["status"]) => {
    switch (status) {
      case "in_progress":
        return (
          <Badge variant="warning" className="text-xs">
            En cours
          </Badge>
        );
      case "completed":
        return (
          <Badge variant="success" className="text-xs">
            Terminé
          </Badge>
        );
      case "pending":
      case "registration":
        return (
          <Badge variant="info" className="text-xs">
            Inscription
          </Badge>
        );
      case "cancelled":
        return (
          <Badge variant="danger" className="text-xs">
            Annulé
          </Badge>
        );
      default:
        return (
          <Badge variant="secondary" className="text-xs">
            À venir
          </Badge>
        );
    }
  };

  // Format date range
  const formatDate = (start: Date | null, end: Date | null) => {
    if (!start) return "Date à confirmer";
    if (!end) {
      return start.toLocaleDateString("fr-FR", {
        weekday: "long",
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    }

    const startDate = new Date(start);
    const endDate = new Date(end);

    if (
      startDate.getMonth() === endDate.getMonth() &&
      startDate.getFullYear() === endDate.getFullYear()
    ) {
      return `${startDate.getDate()}-${endDate.getDate()} ${startDate.toLocaleDateString("fr-FR", { month: "long", year: "numeric" })}`;
    }
    return startDate.toLocaleDateString("fr-FR");
  };

  // Calculate progress percentage
  const getProgress = () => {
    if (!tournament.maxTeams) return 0;
    return Math.round((tournament.registeredTeams / tournament.maxTeams) * 100);
  };

  return (
    <article
      className={`tournament-card-responsive flex flex-col bg-primary-900 rounded-lg overflow-hidden border ${getTypeColor(tournament.type)} ${className}`}
    >
      {/* Header */}
      <Link to={`/tournois/${tournament.id}`} className="block">
        <div className={`px-3 sm:px-4 py-2 sm:py-3 ${"bg-primary-800/20"}`}>
          <div className="flex items-center gap-2">
            <Badge variant="primary" className="text-xs">
              {getTypeLabel(tournament.type)}
            </Badge>
            {tournament.format && (
              <Badge variant="secondary" className="text-xs">
                {tournament.format === "knockout"
                  ? "Élim. directe"
                  : tournament.format === "round-robin"
                    ? "Poules"
                    : tournament.format}
              </Badge>
            )}
          </div>
          <h3 className="text-body-sm sm:text-heading-sm font-bold text-white truncate mt-2 hover:text-gold-700 transition-colors">
            {tournament.name}
          </h3>
        </div>
      </Link>

      {/* Content */}
      <div className="flex-1 p-3 sm:p-4 space-y-3">
        {/* Category and Material */}
        <div className="flex flex-wrap gap-1">
          {tournament.category && (
            <Badge variant="gold" className="text-xs">
              {tournament.category === "doublette"
                ? "Doublette"
                : tournament.category === "triplette"
                  ? "Triplette"
                  : tournament.category === "individual"
                    ? "Individuel"
                    : tournament.category}
            </Badge>
          )}
          {tournament.material && (
            <Badge variant="primary" className="text-xs">
              {tournament.material === "fonte"
                ? "Fonte"
                : tournament.material === "bois"
                  ? "Bois"
                  : tournament.material}
            </Badge>
          )}
        </div>

        {/* Info row */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-body-xs sm:text-body-sm">
          <div className="flex items-center gap-1 text-primary-200">
            <Icon name="MapPin" size="xs" className="text-gold-pure/80" />
            <span>{tournament.location}</span>
          </div>
          <div className="flex items-center gap-1 text-primary-200">
            <Icon name="Calendar" size="xs" className="text-gold-pure/80" />
            <span>{formatDate(tournament.startDate, tournament.endDate)}</span>
          </div>
        </div>

        {/* Progress and stats */}
        <div className="space-y-2 pt-2 border-t border-primary-700/50">
          {/* Registration progress */}
          {tournament.maxTeams && tournament.status === "registration" && (
            <div>
              <div className="flex justify-between text-caption sm:text-body-xs text-primary-300 mb-1">
                <span>Inscriptions</span>
                <span>
                  {tournament.registeredTeams}/{tournament.maxTeams}
                </span>
              </div>
              <div className="h-2 bg-primary-800 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gold-700 rounded-full transition-all duration-500"
                  style={{ width: `${getProgress()}%` }}
                />
              </div>
            </div>
          )}

          {/* Team count */}
          {tournament.maxTeams && tournament.status !== "registration" && (
            <div className="flex items-center gap-2 text-primary-200">
              <Icon name="Users" size="xs" className="text-gold-pure/80" />
              <span className="text-body-xs sm:text-body-sm">
                {tournament.registeredTeams} équipes inscrites
              </span>
            </div>
          )}

          {/* Season */}
          <div className="flex items-center gap-1 text-primary-300">
            <Icon name="Trophy" size="xs" className="text-gold-pure/80" />
            <span className="text-body-xs sm:text-body-sm">{tournament.season}</span>
          </div>
        </div>
      </div>

      {/* Footer with status and actions */}
      <div className="flex items-center justify-between gap-2 px-3 sm:px-4 py-2 sm:py-3 bg-primary-800/50 border-t border-primary-700/50">
        <div>{getStatusBadge(tournament.status)}</div>
        {showActions && (
          <Button size="xs" variant="gold" className="text-gold-pure hover:text-gold-dark">
            Voir plus
          </Button>
        )}
      </div>
    </article>
  );
};

export default TournamentCard;
