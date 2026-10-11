import React from "react";
import { Link } from "react-router-dom";
import { Player } from "../../types";
import { Badge } from "../common/Badge";
import Button from "../common/Button";

export interface PlayerCardProps {
  player: Player;
  accent?: "gold" | "primary" | "amber";
  className?: string;
  showClub?: boolean;
  showActions?: boolean;
  variant?: "default" | "team";
}

/**
 * PlayerCard - Card displaying player information and stats
 * Used in grids on homepage and players page
 * Fully responsive for all screen sizes
 */
export const PlayerCard: React.FC<PlayerCardProps> = ({
  player,
  accent = "primary",
  className = "",
  showClub = true,
  showActions = true,
  variant = "default",
}) => {
  // For team variant, adjust accent based on player stats
  if (variant === "team") {
    accent = player.stats.winRate >= 70 ? "gold" : player.stats.winRate >= 50 ? "primary" : "amber";
  }

  const getAccentColor = () => {
    switch (accent) {
      case "gold":
        return "border-brand-orange-700 bg-gradient-to-br from-brand-orange-900/20 to-brand-orange-800/10";
      case "primary":
        return "border-brand-green-600 bg-gradient-to-br from-brand-green-800/20 to-brand-green-700/10";
      case "amber":
        return "border-amber-700 bg-gradient-to-br from-amber-900/20 to-amber-800/10";
      default:
        return "border-brand-green-700";
    }
  };

  const getAccentBorder = () => {
    switch (accent) {
      case "gold":
        return "border-brand-orange-700";
      case "primary":
        return "border-brand-green-600";
      case "amber":
        return "border-amber-700";
      default:
        return "border-brand-green-700";
    }
  };

  // Get initials from name
  const getInitials = (player: Player) => {
    return `${player.firstName.charAt(0)}${player.lastName.charAt(0)}`.toUpperCase();
  };

  // Compact version for team details
  if (variant === "team") {
    return (
      <Link
        to={`/joueurs/${player.id}`}
        className={`flex items-center gap-3 p-3 sm:p-4 bg-brand-green-900 rounded-lg border border-brand-green-700/50 hover:bg-brand-green-800/50 transition-colors ${className}`}
      >
        <div
          className={`w-10 h-10 sm:w-12 sm:h-12 rounded-full ${getAccentBorder()} border-2 flex items-center justify-center bg-brand-green-800 overflow-hidden flex-shrink-0`}
        >
          {player.photo ? (
            <img
              src={player.photo}
              alt={`${player.firstName} ${player.lastName}`}
              className="w-full h-full object-cover"
              loading="lazy"
            />
          ) : (
            <span className="text-body-xs sm:text-body-sm font-black text-brand-orange-700">
              {getInitials(player)}
            </span>
          )}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-body-xs sm:text-body-sm font-bold text-white truncate">
            {player.firstName} {player.lastName}
          </h3>
          <div className="flex items-center gap-1 sm:gap-2">
            {player.alias && (
              <span className="text-caption sm:text-body-xs text-brand-green-300 truncate">
                ({player.alias})
              </span>
            )}
            {showClub && player.club && (
              <span className="text-caption sm:text-body-xs text-brand-green-300 truncate">
                {player.club.name}
              </span>
            )}
          </div>
        </div>
        <div className="text-center flex-shrink-0">
          <p className="text-caption sm:text-body-xs text-brand-green-300">Victoires</p>
          <p className="text-body-xs sm:text-body-sm font-bold text-brand-orange-700">
            {player.stats.winRate.toFixed(0)}%
          </p>
        </div>
      </Link>
    );
  }

  return (
    <article
      className={`player-card-responsive flex flex-col bg-brand-green-900 rounded-lg overflow-hidden border ${getAccentColor()} ${className}`}
    >
      {/* Accent border on top */}
      <div
        className={`h-1 ${accent === "gold" ? "bg-brand-orange-700" : accent === "primary" ? "bg-brand-green-600" : "bg-amber-700"}`}
      />

      {/* Avatar section */}
      <Link to={`/joueurs/${player.id}`} className="block pt-2">
        <div className="relative mx-auto -mt-12 sm:-mt-14 mb-3 sm:mb-4 hover:scale-105 transition-transform">
          <div
            className={`w-20 h-20 sm:w-24 sm:h-24 rounded-full ${getAccentBorder()} border-2 sm:border-4 flex items-center justify-center bg-brand-green-800 overflow-hidden`}
          >
            {player.photo ? (
              <img
                src={player.photo}
                alt={`${player.firstName} ${player.lastName}`}
                className="w-full h-full object-cover"
                loading="lazy"
              />
            ) : (
              <span className="text-heading-xs sm:text-heading-sm font-black text-brand-orange-700">
                {getInitials(player)}
              </span>
            )}
          </div>
        </div>
      </Link>

      {/* Content */}
      <div className="flex-1 px-3 sm:px-4 pb-3 sm:pb-4 space-y-2">
        {/* Name */}
        <Link to={`/joueurs/${player.id}`} className="block">
          <h3 className="text-body-sm sm:text-heading-s font-bold text-white text-center truncate hover:text-brand-orange-700 transition-colors">
            {player.firstName} {player.lastName}
          </h3>
        </Link>

        {/* Club */}
        {showClub && player.club && (
          <p className="text-body-xs sm:text-body-sm text-brand-green-200 text-center truncate">
            {player.club.name}
          </p>
        )}

        {player.alias && (
          <p className="text-caption sm:text-body-xs text-brand-green-300 text-center truncate">
            "{player.alias}"
          </p>
        )}

        {/* Stats */}
        <div className="pt-2 border-t border-brand-green-700/50">
          <div className="grid grid-cols-2 gap-2 sm:gap-3">
            <div className="text-center">
              <p className="text-caption sm:text-body-xs text-brand-green-300">Victoires</p>
              <p className="text-body-sm sm:text-body-m font-bold text-brand-orange-700">
                {player.stats.winRate.toFixed(0)}%
              </p>
            </div>
            <div className="text-center">
              <p className="text-caption sm:text-body-xs text-brand-green-300">Matchs</p>
              <p className="text-body-sm sm:text-body-m font-bold text-brand-orange-700">
                {player.stats.matchesPlayed}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Actions */}
      {showActions && (
        <div className="px-3 sm:px-4 pb-3 sm:pb-4 pt-2 border-t border-brand-green-700/50">
          <Button
            size="xs"
            fullWidth={true}
            variant="gold"
          >
            Voir le profil
          </Button>
        </div>
      )}
    </article>
  );
};

export default PlayerCard;
