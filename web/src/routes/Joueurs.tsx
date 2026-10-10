import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../api';
import { Layout } from '../components/layout/Layout';
import { PlayerCard } from '../components/domain';
import { Player } from '../types';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';

const Joueurs: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'rank' | 'points' | 'winRate' | 'name'>('rank');

  const { data: players, isLoading } = useQuery<Player[]>({
    queryKey: ['players', 'top', 50],
    queryFn: async () => {
      const response = await api().GET<Player[]>('/api/players/top?limit=50');
      return response.data;
    },
  });

  // Filtrer et trier
  const filteredPlayers = players
    ?.filter((player) =>
      player.firstName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      player.lastName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      player.fullName.toLowerCase().includes(searchTerm.toLowerCase()) ||
      (player.club?.name?.toLowerCase().includes(searchTerm.toLowerCase()) ?? false)
    )
    ?.sort((a, b) => {
      switch (sortBy) {
        case 'rank':
          return (a.stats.currentRank || 999) - (b.stats.currentRank || 999);
        case 'points':
          return (b.stats.currentPoints || 0) - (a.stats.currentPoints || 0);
        case 'winRate':
          return (b.stats.winRate || 0) - (a.stats.winRate || 0);
        case 'name':
          return a.lastName.localeCompare(b.lastName);
        default:
          return 0;
      }
    }) || [];

  if (isLoading) {
    return (
      <Layout>
        <main className="flex-1 px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <p className="text-body-m text-primary-200">Chargement...</p>
          </div>
        </main>
      </Layout>
    );
  }

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
              JOUEURS
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Classement et statistiques des joueurs de Palet Vendéen
            </p>
          </div>
        </section>

        {/* Filters */}
        <section className="px-4 py-4 lg:px-8 lg:py-6">
          <div className="max-w-7xl mx-auto">
            <div className="flex flex-col md:flex-row gap-4 items-center">
              <div className="flex-1">
                <div className="relative">
                  <Icon
                    name="Search"
                    size="m"
                    className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300"
                  />
                  <input
                    type="text"
                    placeholder="Rechercher un joueur ou un club..."
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-300 focus:outline-none focus:border-gold-700"
                  />
                </div>
              </div>
              <div className="flex gap-2">
                <Button
                  variant={sortBy === 'rank' ? 'primary' : 'secondary'}
                  size="sm"
                  onClick={() => setSortBy('rank')}
                >
                  Classement
                </Button>
                <Button
                  variant={sortBy === 'points' ? 'primary' : 'secondary'}
                  size="sm"
                  onClick={() => setSortBy('points')}
                >
                  Points
                </Button>
                <Button
                  variant={sortBy === 'winRate' ? 'primary' : 'secondary'}
                  size="sm"
                  onClick={() => setSortBy('winRate')}
                >
                  % Victoires
                </Button>
                <Button
                  variant={sortBy === 'name' ? 'primary' : 'secondary'}
                  size="sm"
                  onClick={() => setSortBy('name')}
                >
                  Nom
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Players Grid */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredPlayers.length > 0 ? (
                filteredPlayers.map((player, index) => (
                  <PlayerCard
                    key={player.id}
                    player={player}
                    accent={index < 3 ? (index === 0 ? 'gold' : index === 1 ? 'primary' : 'amber') : undefined}
                  />
                ))
              ) : (
                <p className="text-body-m text-primary-300 col-span-full">
                  Aucun joueur trouvé
                </p>
              )}
            </div>
          </div>
        </section>

        {/* Stats */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              STATISTIQUES
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">Total Joueurs</p>
                <p className="text-heading-m font-bold text-gold-700">{players?.length || 0}</p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">D1</p>
                <p className="text-heading-m font-bold text-gold-700">
                  {players?.filter((p) => p.stats.currentDivision === 'D1').length || 0}
                </p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">D2</p>
                <p className="text-heading-m font-bold text-gold-700">
                  {players?.filter((p) => p.stats.currentDivision === 'D2').length || 0}
                </p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">Autres</p>
                <p className="text-heading-m font-bold text-gold-700">
                  {players?.filter((p) => p.stats.currentDivision !== 'D1' && p.stats.currentDivision !== 'D2').length || 0}
                </p>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Joueurs;
