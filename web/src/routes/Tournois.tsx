import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../api';
import { Layout } from '../components/layout/Layout';
import { TournamentCard } from '../components/domain';
import { Tournament } from '../types';

const Tournois: React.FC = () => {
  const { data: tournaments, isLoading } = useQuery<Tournament[]>({
    queryKey: ['tournaments'],
    queryFn: async () => {
      const response = await api().GET<Tournament[]>('/api/tournaments?limit=20&sort=startDate');
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

  // Grouper les tournois par status
  const upcomingTournaments = tournaments?.filter(
    (t) => t.status === 'registration' || t.status === 'pending'
  ) || [];
  const inProgressTournaments = tournaments?.filter(
    (t) => t.status === 'in_progress'
  ) || [];
  const completedTournaments = tournaments?.filter(
    (t) => t.status === 'completed'
  ) || [];

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
              TOURNOIS
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Tous les tournois et coupes du Palet Vendéen
            </p>
          </div>
        </section>

        {/* Upcoming Tournaments */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                TOURNOIS À VENIR
              </h2>
              <span className="text-body-s text-primary-300">
                {upcomingTournaments.length} tournois
              </span>
            </div>

            {upcomingTournaments.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {upcomingTournaments.map((tournament) => (
                  <TournamentCard key={tournament.id} tournament={tournament} />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun tournoi à venir
              </p>
            )}
          </div>
        </section>

        {/* In Progress Tournaments */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                EN COURS
              </h2>
              <span className="text-body-s text-primary-300">
                {inProgressTournaments.length} tournois
              </span>
            </div>

            {inProgressTournaments.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {inProgressTournaments.map((tournament) => (
                  <TournamentCard key={tournament.id} tournament={tournament} />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun tournoi en cours
              </p>
            )}
          </div>
        </section>

        {/* Completed Tournaments */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                TOURNOIS TERMINÉS
              </h2>
              <span className="text-body-s text-primary-300">
                {completedTournaments.length} tournois
              </span>
            </div>

            {completedTournaments.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {completedTournaments.slice(0, 6).map((tournament) => (
                  <TournamentCard key={tournament.id} tournament={tournament} />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun tournoi terminé
              </p>
            )}
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Tournois;
