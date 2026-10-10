import React from 'react';
import { ChampionshipTeam } from '../../types';
import { Badge } from '../common/Badge';
import { Icon } from '../common/Icon';

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
 */
export const ClassificationTable: React.FC<ClassificationTableProps> = ({
  teams,
  competition,
  journee,
  season = '2024-2025',
  className = '',
  showAllLink = true,
  onAllClick
}) => {
  const getRowClassName = (status: ChampionshipTeam['status']) => {
    const base = 'hover:bg-primary-800/30 transition-colors';
    if (status === 'qualified') return `${base} bg-primary-800/20`;
    if (status === 'barrage') return `${base} bg-amber-900/20`;
    if (status === 'relegated') return `${base} bg-red-900/20`;
    return base;
  };

  const getStatusBadge = (status: ChampionshipTeam['status']) => {
    if (status === 'qualified') return <Badge variant="success" className="text-xs">Finale</Badge>;
    if (status === 'barrage') return <Badge variant="warning" className="text-xs">Barrage</Badge>;
    if (status === 'relegated') return <Badge variant="danger" className="text-xs">Descente D2</Badge>;
    return null;
  };

  // Form indicators
  const getFormIndicator = (status: 'win' | 'draw' | 'loss') => {
    if (status === 'win') return <span className="w-4 h-4 rounded-sm bg-green-600" />;
    if (status === 'loss') return <span className="w-4 h-4 rounded-sm bg-red-600" />;
    if (status === 'draw') return <span className="w-4 h-4 rounded-sm bg-gray-500" />;
    return <span className="w-4 h-4 rounded-sm bg-primary-600" />;
  };

  return (
    <section className={`w-full ${className}`} aria-labelledby="classification-title">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div>
          <h2 
            id="classification-title"
            className="text-heading-l font-bold text-dark-primary uppercase tracking-wide"
          >
            {competition}
          </h2>
          <p className="text-body-m text-dark-secondary">
            Classement après {journee} journée{journee > 1 ? 's' : ''} — {season}
          </p>
        </div>
        {showAllLink && onAllClick && (
          <button 
            onClick={onAllClick}
            className="text-body-s text-gold-pure hover:text-gold-dark flex items-center gap-1"
          >
            Tout le classement
            <Icon name="ArrowRight" size="sm" />
          </button>
        )}
      </div>

      {/* Table */}
      <div className="overflow-x-auto rounded-lg border border-neutral-200 bg-white">
        <table className="w-full text-left">
          {/* Table Header */}
          <thead className="bg-neutral-50">
            <tr>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16">
                POS
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider min-w-[200px]">
                CLUB
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16 text-center">
                MJ
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16 text-center">
                G
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16 text-center">
                P
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16 text-center">
                DIFF
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-16 text-center">
                PTS
              </th>
              <th className="px-4 py-3 text-body-xs font-bold text-dark-tertiary uppercase tracking-wider w-24">
                FORME
              </th>
            </tr>
          </thead>

          {/* Table Body */}
          <tbody>
            {teams.map((team) => (
              <tr 
                key={team.team.id}
                className={getRowClassName(team.status)}
              >
                {/* Position */}
                <td className="px-4 py-3">
                  <span className="text-body-l font-bold text-gold-pure">
                    {team.position}
                  </span>
                </td>

                {/* Club */}
                <td className="px-4 py-3">
                  <div className="flex items-center gap-3">
                    <div>
                      <span className="text-body-m font-medium text-dark-primary block">
                        {team.team.name}
                      </span>
                    </div>
                  </div>
                </td>

                {/* MJ (Matches Joués) */}
                <td className="px-4 py-3 text-center">
                  <span className="text-body-m text-dark-secondary">{team.stats.matches}</span>
                </td>

                {/* G (Gagnés) */}
                <td className="px-4 py-3 text-center">
                  <span className="text-body-m text-green-600">{team.stats.wins}</span>
                </td>

                {/* P (Perdus) */}
                <td className="px-4 py-3 text-center">
                  <span className="text-body-m text-red-600">{team.stats.losses}</span>
                </td>

                {/* DIFF */}
                <td className="px-4 py-3 text-center">
                  <span className={`text-body-m font-medium ${
                    team.stats.diff > 0 ? 'text-green-600' : 'text-red-600'
                  }`}>
                    {team.stats.diff > 0 ? '+' : ''}{team.stats.diff}
                  </span>
                </td>

                {/* PTS */}
                <td className="px-4 py-3 text-center">
                  <span className="text-body-m font-bold text-gold-pure">
                    {team.stats.points}
                  </span>
                </td>

                {/* Form */}
                <td className="px-4 py-3">
                  <div className="flex gap-1 justify-center">
                    {team.form?.slice(0, 5).map((result, i) => (
                      <React.Fragment key={i}>{getFormIndicator(result)}</React.Fragment>
                    ))}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Legend */}
      <div className="mt-4 flex items-center gap-6">
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 rounded-full bg-primary-600" />
          <span className="text-caption text-primary-200">Finale</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 rounded-full bg-gold-700" />
          <span className="text-caption text-primary-200">Barrage</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-4 h-4 rounded-full bg-red-800" />
          <span className="text-caption text-primary-200">Descente D2</span>
        </div>
      </div>
    </section>
  );
};

export default ClassificationTable;
