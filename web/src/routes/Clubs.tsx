import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../api';
import { Layout } from '../components/layout/Layout';
import { ClubCard } from '../components/domain';
import { Club } from '../types';

const Clubs: React.FC = () => {
  const { data: clubs, isLoading } = useQuery<Club[]>({
    queryKey: ['clubs'],
    queryFn: async () => {
      const response = await api().GET<Club[]>('/api/clubs?sort=name&order=asc');
      return response.data;
    },
  });

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

  // Grouper les clubs par division
  const division1Clubs = clubs?.filter((club) => club.division === 'D1') || [];
  const division2Clubs = clubs?.filter((club) => club.division === 'D2') || [];
  const otherClubs = clubs?.filter((club) => club.division !== 'D1' && club.division !== 'D2') || [];

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
              CLUBS
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Annuaire des clubs de Palet Vendéen
            </p>
          </div>
        </section>

        {/* Division 1 */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                DIVISION 1
              </h2>
              <span className="text-body-s text-primary-300">
                {division1Clubs.length} clubs
              </span>
            </div>

            {division1Clubs.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {division1Clubs.map((club) => (
                  <ClubCard key={club.id} club={club}  />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun club en Division 1
              </p>
            )}
          </div>
        </section>

        {/* Division 2 */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                DIVISION 2
              </h2>
              <span className="text-body-s text-primary-300">
                {division2Clubs.length} clubs
              </span>
            </div>

            {division2Clubs.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {division2Clubs.map((club) => (
                  <ClubCard key={club.id} club={club}  />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun club en Division 2
              </p>
            )}
          </div>
        </section>

        {/* Autres Divisions */}
        {otherClubs.length > 0 && (
          <section className="px-4 py-6 lg:px-8 lg:py-8">
            <div className="max-w-7xl mx-auto">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                  AUTRES DIVISIONS
                </h2>
                <span className="text-body-s text-primary-300">
                  {otherClubs.length} clubs
                </span>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {otherClubs.map((club) => (
                  <ClubCard key={club.id} club={club}  />
                ))}
              </div>
            </div>
          </section>
        )}

        {/* Stats */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              STATISTIQUES
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">Total Clubs</p>
                <p className="text-heading-m font-bold text-gold-700">{clubs?.length || 0}</p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">D1</p>
                <p className="text-heading-m font-bold text-gold-700">{division1Clubs.length}</p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">D2</p>
                <p className="text-heading-m font-bold text-gold-700">{division2Clubs.length}</p>
              </div>
              <div className="bg-primary-900 rounded-lg p-4 text-center border border-primary-700/50">
                <p className="text-body-xs text-primary-300 mb-1">Autres</p>
                <p className="text-heading-m font-bold text-gold-700">{otherClubs.length}</p>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Clubs;
