import React from 'react';
import { Layout } from '../components/layout/Layout';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';

const Federation: React.FC = () => {
  const federations = [
    {
      name: 'FNSMR',
      fullName: 'Fédération Nationale du Sport en Milieu Rural',
      role: 'Fédération Nationale',
      description: 'Organisme national qui régit le sport en milieu rural, dont le palet fonte.',
      website: 'https://www.sportrural.fr',
      contact: 'contact@fnsmr.fr',
    },
    {
      name: 'CRSMR NA',
      fullName: 'Comité Régional du Sport en Milieu Rural Nouvelle-Aquitaine',
      role: 'Comité Régional',
      description: 'Coordinate les activités sportives en milieu rural en Nouvelle-Aquitaine.',
      website: null,
      contact: null,
    },
    {
      name: 'CDSMR 79',
      fullName: 'Comité Départemental du Sport en Milieu Rural des Deux-Sèvres',
      role: 'Comité Départemental',
      description: 'Coordinate les activités sportives en milieu rural dans les Deux-Sèvres.',
      website: null,
      contact: null,
    },
    {
      name: 'CDSP 79',
      fullName: 'Commission Deux-Sévrienne du Palet',
      role: 'Commission Spécialisée',
      description: 'Organise les championnats et tournois de palet fonte dans les Deux-Sèvres.',
      website: 'https://www.le-palet.com/cdsp',
      contact: 'cdsp79@sportrural.fr',
    },
  ];

  const documents = [
    {
      name: 'Règlement Championnat Palet Fonte 79 2026-2027',
      type: 'pdf',
      date: new Date('2026-09-01'),
      size: '2.4 MB',
    },
    {
      name: 'Règlement Coupe 79 2026-2027',
      type: 'pdf',
      date: new Date('2026-09-01'),
      size: '1.8 MB',
    },
    {
      name: 'Calendrier Championnat CDSP 79 2026-2027',
      type: 'pdf',
      date: new Date('2026-09-09'),
      size: '1.5 MB',
    },
    {
      name: 'Feuille de Match CDSP 79',
      type: 'pdf',
      date: new Date('2026-09-01'),
      size: '0.8 MB',
    },
    {
      name: 'Liste des Clubs CDSP 79 2026-2027',
      type: 'pdf',
      date: new Date('2026-09-01'),
      size: '0.5 MB',
    },
  ];

  const getIconForType = (type: string) => {
    switch (type) {
      case 'pdf':
        return 'FileText';
      default:
        return 'File';
    }
  };

  return (
    <Layout>
      <main className="flex-1">
        {/* Header */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide">
              FÉDÉRATION
            </h1>
            <p className="text-body-m text-primary-200 mt-2">
              Informations officielles et documents fédéraux
            </p>
          </div>
        </section>

        {/* Federation Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              ORGANISMES OFFICIELS
            </h2>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {federations.map((fed) => (
                <div
                  key={fed.name}
                  className="bg-primary-900 rounded-lg p-6 border border-primary-700/50"
                >
                  <h3 className="text-heading-s font-bold text-gold-700 mb-2">
                    {fed.name}
                  </h3>
                  <p className="text-caption text-primary-300 mb-3">
                    {fed.fullName}
                  </p>
                  <p className="text-body-s text-primary-200 mb-4">{fed.description}</p>
                  <p className="text-caption text-primary-400 mb-4">{fed.role}</p>
                  <div className="space-y-2">
                    {fed.website && (
                      <Button variant="ghost" size="sm" className="w-full justify-start">
                        <Icon name="Globe" size="xs" className="mr-2" />
                        Site web
                      </Button>
                    )}
                    {fed.contact && (
                      <Button variant="ghost" size="sm" className="w-full justify-start">
                        <Icon name="Mail" size="xs" className="mr-2" />
                        {fed.contact}
                      </Button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Documents Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8 bg-primary-800/30">
          <div className="max-w-7xl mx-auto">
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide">
                DOCUMENTS OFFICIELS
              </h2>
              <Button variant="secondary" size="sm">
                <Icon name="Download" size="xs" className="mr-2" />
                Tout télécharger
              </Button>
            </div>

            <div className="bg-primary-900 rounded-lg overflow-hidden border border-primary-700/50">
              <table className="w-full">
                <thead className="bg-primary-800">
                  <tr>
                    <th className="text-left p-4 text-caption font-bold text-primary-300 uppercase">
                      Document
                    </th>
                    <th className="text-left p-4 text-caption font-bold text-primary-300 uppercase">
                      Type
                    </th>
                    <th className="text-left p-4 text-caption font-bold text-primary-300 uppercase">
                      Date
                    </th>
                    <th className="text-right p-4 text-caption font-bold text-primary-300 uppercase">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {documents.map((doc) => (
                    <tr
                      key={doc.name}
                      className="border-t border-primary-700/50 hover:bg-primary-800/50"
                    >
                      <td className="p-4">
                        <div className="flex items-center gap-3">
                          <Icon name={getIconForType(doc.type) as any} size="m" className="text-primary-300" />
                          <span className="text-body-m text-white">{doc.name}</span>
                        </div>
                      </td>
                      <td className="p-4">
                        <span className="text-body-s text-primary-200">{doc.type.toUpperCase()}</span>
                      </td>
                      <td className="p-4">
                        <span className="text-body-s text-primary-200">
                          {doc.date.toLocaleDateString('fr-FR')}
                        </span>
                      </td>
                      <td className="p-4 text-right">
                        <Button variant="ghost" size="sm">
                          <Icon name="Download" size="xs" className="mr-2" />
                          Télécharger
                        </Button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </section>

        {/* Links Section */}
        <section className="px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto">
            <h2 className="text-heading-l font-bold text-gold-700 uppercase tracking-wide mb-6">
              LIENS UTILES
            </h2>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {[
                {
                  title: 'Site officiel FNSMR',
                  url: 'https://www.sportrural.fr',
                  icon: 'Globe',
                },
                {
                  title: 'CDSP 79 sur le-palet.com',
                  url: 'https://www.le-palet.com/cdsp',
                  icon: 'BookOpen',
                },
                {
                  title: 'CRSMR Nouvelle-Aquitaine',
                  url: 'https://www.sportrural-nouvelleaquitaine.fr',
                  icon: 'Globe',
                },
              ].map((link, index) => (
                <a
                  key={index}
                  href={link.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="bg-primary-900 rounded-lg p-4 border border-primary-700/50 hover:border-gold-700/50 transition-colors group"
                >
                  <div className="flex items-center gap-3">
                    <Icon name={link.icon as any} size="m" className="text-gold-700 group-hover:text-gold-600" />
                    <span className="text-body-m text-white group-hover:text-gold-700">{link.title}</span>
                    <Icon name="ExternalLink" size="xs" className="text-primary-300 ml-auto" />
                  </div>
                </a>
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
};

export default Federation;
