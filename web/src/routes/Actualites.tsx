import React from 'react';
import { Layout } from '../components/layout/Layout';
import { Icon } from '../components/common/Icon';

const Actualites: React.FC = () => {
  // Mock news data - will be replaced with API call
  const newsItems = [
    {
      id: '1',
      title: 'Résultats de la finale du Championnat Vendée 2025',
      excerpt: 'La Roche-sur-Yon Palet Club remporte la finale contre Clos Fontenois dans un match très serré.',
      date: new Date('2025-06-15'),
      category: 'competitions',
      image: null,
    },
    {
      id: '2',
      title: 'Inscription ouverte pour l\'Open de Luçon',
      excerpt: 'Les inscriptions pour l\'Open National de Luçon sont ouvertes jusqu\'au 5 juillet 2025.',
      date: new Date('2025-06-10'),
      category: 'official',
      image: null,
    },
    {
      id: '3',
      title: 'Nouvelle saison 2025-2026 - Calendrier disponible',
      excerpt: 'Le calendrier des championnats pour la nouvelle saison est disponible. Consultez les dates des journées.',
      date: new Date('2025-06-01'),
      category: 'official',
      image: null,
    },
    {
      id: '4',
      title: 'Règlement mis à jour pour les tournois fédéraux',
      excerpt: 'Un nouveau règlement a été publié pour les tournois sous égide de la CVDP.',
      date: new Date('2025-05-20'),
      category: 'official',
      image: null,
    },
    {
      id: '5',
      title: 'Palmarès des Internationaux de Vendée 2024',
      excerpt: 'Retrouvez tous les résultats et classements des Internationaux de Vendée qui se sont tenus en août 2024.',
      date: new Date('2024-08-20'),
      category: 'competitions',
      image: null,
    },
  ];

  const getCategoryColor = (category: string) => {
    switch (category) {
      case 'official':
        return 'text-gold-700';
      case 'competitions':
        return 'text-primary-400';
      default:
        return 'text-primary-200';
    }
  };

  const getCategoryLabel = (category: string) => {
    switch (category) {
      case 'official':
        return 'OFFICIEL';
      case 'competitions':
        return 'COMPÉTITIONS';
      default:
        return category.toUpperCase();
    }
  };

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
              ACTUALITÉS
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Toutes les nouvelles du Palet Vendéen
            </p>
          </div>
        </section>

        {/* News List */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-4xl mx-auto space-y-6">
            {newsItems.map((item) => (
              <article
                key={item.id}
                className="bg-primary-900 rounded-lg p-6 border border-primary-700/50 hover:border-gold-700/50 transition-colors"
              >
                <div className="flex items-start gap-4">
                  <div className="flex-shrink-0">
                    <div className="w-10 h-10 bg-primary-800 rounded-lg flex items-center justify-center">
                      <Icon name="Newspaper" size="m" className="text-primary-300" />
                    </div>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-4 mb-2">
                      <span
                        className={`text-caption font-bold ${getCategoryColor(
                          item.category
                        )}`}
                      >
                        {getCategoryLabel(item.category)}
                      </span>
                      <span className="text-caption text-primary-300">
                        {item.date.toLocaleDateString('fr-FR', {
                          year: 'numeric',
                          month: 'long',
                          day: 'numeric',
                        })}
                      </span>
                    </div>
                    <h3 className="text-heading-s font-bold text-white mb-2">
                      {item.title}
                    </h3>
                    <p className="text-body-m text-primary-200">{item.excerpt}</p>
                    <div className="mt-4">
                      <button className="text-body-s text-gold-700 hover:text-gold-600 font-medium flex items-center gap-1">
                        Lire la suite
                        <Icon name="ArrowRight" size="xs" />
                      </button>
                    </div>
                  </div>
                </div>
              </article>
            ))}
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Actualites;
