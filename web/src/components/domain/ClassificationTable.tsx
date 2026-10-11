import React from "react";
import { ChampionshipTeam } from "../../types";
import { Badge } from "../common/Badge";
import { Icon } from "../common/Icon";

export interface ClassificationTableProps {
  teams: ChampionshipTeam[];
  competition: string;
  journee: number;
  season?: string;
  className?: string;
  showAllLink?: boolean;
  onAllClick?: () => void;
}

/**
 * ClassificationTable - Display current championship standings
 * Shows teams with their position, stats, and form
 * Fully responsive with horizontal scrolling on mobile
 */
export const ClassificationTable: React.FC<ClassificationTableProps> = ({
  teams,
  competition,
  journee,
  season = "2026-2027",
  className = "",
  showAllLink = true,
  onAllClick,
}) => {
  const getRowClassName = (status: ChampionshipTeam["status"]) => {
    const base = "hover:bg-brand-green-800/30 transition-colors";
    if (status === "qualified") return `${base} bg-brand-green-800/20`;
    if (status === "barrage") return `${base} bg-amber-900/20`;
    if (status === "relegated") return `${base} bg-red-900/20`;
    return base;
  };

  const getStatusBadge = (status: ChampionshipTeam["status"]) => {
    if (status === "qualified")
      return (
        <Badge variant="success" className="text-xs">
          Finale
        </Badge>
      );
    if (status === "barrage")
      return (
        <Badge variant="warning" className="text-xs">
          Barrage
        </Badge>
      );
    if (status === "relegated")
      return (
        <Badge variant="danger" className="text-xs">
          Descente D2
        </Badge>
      );
    return null;
  };

  // Form indicators
  const getFormIndicator = (status: "win" | "draw" | "loss" | undefined) => {
    if (!status) return <span className="w-4 h-4 rounded-sm bg-brand-green-600" />;
    if (status === "win") return <span className="w-4 h-4 rounded-sm bg-brand-green-600" />;
    if (status === "loss")
      return <span className="w-3 h-3 sm:w-4 sm:h-4 rounded-sm bg-danger-600" />;
    if (status === "draw")
      return <span className="w-3 h-3 sm:w-4 sm:h-4 rounded-sm bg-neutral-500" />;
    return <span className="w-3 h-3 sm:w-4 sm:h-4 rounded-sm bg-brand-green-600" />;
  };

  // Mobile-optimized header columns
  const getMobileHeaders = () => {
    return [
      { key: "position", label: "POS", className: "w-12 text-center" },
      { key: "team", label: "ÉQUIPE", className: "min-w-[120px]" },
      { key: "points", label: "PTS", className: "w-12 text-center" },
    ];
  };

  const getDesktopHeaders = () => {
    return [
      { key: "position", label: "POS", className: "w-12 text-center" },
      { key: "team", label: "CLUB", className: "min-w-[140px] lg:min-w-[200px]" },
      { key: "matches", label: "MJ", className: "w-12 text-center" },
      { key: "wins", label: "G", className: "w-12 text-center" },
      { key: "losses", label: "P", className: "w-12 text-center" },
      { key: "diff", label: "DIFF", className: "w-14 text-center" },
      { key: "points", label: "PTS", className: "w-12 text-center" },
      { key: "form", label: "FORME", className: "w-20" },
    ];
  };

  // Mobile-optimized row
  const getMobileRow = (team: ChampionshipTeam) => {
    return (
      <tr key={team.team.id} className={getRowClassName(team.status)}>
        <td className="px-2 sm:px-3 py-2 sm:py-3 text-center">
          <span className="text-body-sm sm:text-body-m font-bold text-brand-orange-pure">
            {team.position}
          </span>
        </td>
        <td className="px-2 sm:px-3 py-2 sm:py-3 min-w-[120px]">
          <div className="flex items-center gap-2">
            <div className="hidden sm:flex">{getStatusBadge(team.status)}</div>
            <div className="min-w-0">
              <span className="text-body-xs sm:text-body-sm font-medium text-white block truncate">
                {team.team.name}
              </span>
            </div>
          </div>
        </td>
        <td className="px-2 sm:px-3 py-2 sm:py-3 text-center">
          <span className="text-body-sm sm:text-body-m font-bold text-brand-orange-pure">
            {team.stats.points}
          </span>
        </td>
      </tr>
    );
  };

  // Desktop row
  const getDesktopRow = (team: ChampionshipTeam) => {
    return (
      <tr key={team.team.id} className={getRowClassName(team.status)}>
        {/* Position */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span className="text-body-sm sm:text-body-m font-bold text-brand-orange-pure">
            {team.position}
          </span>
        </td>

        {/* Club */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 min-w-[140px] lg:min-w-[200px]">
          <div className="flex items-center gap-2">
            {getStatusBadge(team.status)}
            <div className="min-w-0">
              <span className="text-body-xs sm:text-body-sm font-medium text-white block truncate">
                {team.team.name}
              </span>
            </div>
          </div>
        </td>

        {/* MJ (Matches Joués) */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span className="text-body-xs sm:text-body-sm text-neutral-300">
            {team.stats.matches}
          </span>
        </td>

        {/* G (Gagnés) */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span className="text-body-xs sm:text-body-sm text-brand-green-400">
            {team.stats.wins}
          </span>
        </td>

        {/* P (Perdus) */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span className="text-body-xs sm:text-body-sm text-danger-400">{team.stats.losses}</span>
        </td>

        {/* DIFF */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span
            className={`text-body-xs sm:text-body-sm font-medium ${
              team.stats.diff > 0 ? "text-brand-green-400" : "text-danger-400"
            }`}
          >
            {team.stats.diff > 0 ? "+" : ""}
            {team.stats.diff}
          </span>
        </td>

        {/* PTS */}
        <td className="px-3 sm:px-4 py-2 sm:py-3 text-center whitespace-nowrap">
          <span className="text-body-xs sm:text-body-sm font-bold text-brand-orange-pure">
            {team.stats.points}
          </span>
        </td>

        {/* Form */}
        <td className="px-3 sm:px-4 py-2 sm:py-3">
          <div className="flex gap-0.5 justify-center">
            {team.form?.slice(0, 5).map((result, i) => (
              <React.Fragment key={i}>{getFormIndicator(result)}</React.Fragment>
            ))}
          </div>
        </td>
      </tr>
    );
  };

  // Legend component
  const Legend = () => (
    <div className="mt-3 sm:mt-4 flex flex-wrap items-center gap-3 sm:gap-6 text-caption sm:text-body-xs">
      <div className="flex items-center gap-1 sm:gap-2">
        <span className="w-3 h-3 rounded-full bg-brand-green-600" />
        <span className="text-neutral-300">Qualifié pour la finale</span>
      </div>
      <div className="flex items-center gap-1 sm:gap-2">
        <span className="w-3 h-3 rounded-full bg-brand-orange-700" />
        <span className="text-neutral-300">Barrage</span>
      </div>
      <div className="flex items-center gap-1 sm:gap-2">
        <span className="w-3 h-3 rounded-full bg-danger-700" />
        <span className="text-neutral-300">Relégué en D2</span>
      </div>
    </div>
  );

  return (
    <section className={`w-full ${className}`} aria-labelledby="classification-title">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 mb-3 sm:mb-4">
        <div>
          <h2
            id="classification-title"
            className="text-heading-sm sm:text-heading-l font-bold text-white uppercase tracking-wide"
          >
            {competition}
          </h2>
          <p className="text-body-xs sm:text-body-m text-neutral-300">
            Classement après {journee} journée{journee > 1 ? "s" : ""} — {season}
          </p>
        </div>
        {showAllLink && onAllClick && (
          <button
            onClick={onAllClick}
            className="text-body-xs sm:text-body-sm text-brand-orange-pure hover:text-brand-orange-400 flex items-center gap-1 sm:gap-2 whitespace-nowrap"
          >
            Tout le classement
            <Icon name="ArrowRight" size="xs" />
          </button>
        )}
      </div>

      {/* Table Container with horizontal scrolling */}
      <div className="table-responsive-container rounded-lg border border-neutral-200 bg-white">
        {/* Mobile Table */}
        <table className="w-full lg:hidden">
          <thead className="bg-neutral-50">
            <tr>
              {getMobileHeaders().map((header) => (
                <th
                  key={header.key}
                  className={`px-2 sm:px-3 py-2 sm:py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider ${header.className}`}
                >
                  {header.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>{teams.map((team) => getMobileRow(team))}</tbody>
        </table>

        {/* Desktop Table */}
        <table className="hidden lg:table w-full">
          <thead className="bg-neutral-50">
            <tr>
              {getDesktopHeaders().map((header) => (
                <th
                  key={header.key}
                  className={`px-3 sm:px-4 py-2 sm:py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider whitespace-nowrap ${header.className}`}
                >
                  {header.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>{teams.map((team) => getDesktopRow(team))}</tbody>
        </table>
      </div>

      {/* Legend */}
      <Legend />
    </section>
  );
};

export default ClassificationTable;
