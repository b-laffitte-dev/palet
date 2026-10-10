import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../api';
import { Layout } from '../components/layout/Layout';
import { ClassificationTable, MatchCard } from '../components/domain';
import { Championship, Match } from '../types';

const Championnats: React.FC = () => {
  const { data: championships, isLoading: isLoadingChampionships } = useQuery<Championship[]>({
    queryKey: ['championships'],
    queryFn: async () => {
      const response = await api().GET<Championship[]>('/api/championships');
      return response.data;
    },
  });

  const { data: matches, isLoading: isLoadingMatches } = useQuery<Match[]>({
    queryKey: ['matches', 'recent'],
    queryFn: async () => {
      const response = await api().GET<Match[]>('/api/matches?limit=10&sort=date&order=desc');
      return response.data;
    },
  });

  if (isLoadingChampionships || isLoadingMatches) {
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
              CHAMPIONNATS
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Classement et résultats des championnats en cours
            </p>
          </div>
        </section>

        {/* Current Championships */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              CHAMPIONNATS EN COURS
            </h2>

            {championships && championships.length > 0 ? (
              <div className="space-y-8">
                {championships
                  .filter(champ => champ.status === 'in_progress')
                  .map((championship) => (
                    <div key={championship.id} className="space-y-4">
                      <div className="flex items-center gap-4 mb-4">
                        <h3 className="text-heading-m font-bold text-white">
                          {championship.name}
                        </h3>
                        <span className="text-body-s text-primary-300">
                          {championship.season} - Journées {championship.currentJournee || 0}/{championship.totalJournees || 0}
                        </span>
                      </div>

                      {championship.classification && (
                        <ClassificationTable
                          teams={championship.classification.teams}
                          competition={`${championship.name} — ${championship.divisions?.[0] || 'DIVISION 1'}`}
                          journee={championship.classification.journee}
                          season={championship.season}
                          showAllLink={false}
                        />
                      )}
                    </div>
                  ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun championnat en cours
              </p>
            )}
          </div>
        </section>

        {/* Recent Matches */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              DERNIERS MATCHS
            </h2>

            {matches && matches.length > 0 ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {matches.slice(0, 6).map((match) => (
                  <MatchCard key={match.id} match={match} />
                ))}
              </div>
            ) : (
              <p className="text-body-m text-primary-300">
                Aucun match récent
              </p>
            )}
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Championnats;
