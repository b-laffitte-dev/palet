import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { createBrowserRouter, RouterProvider, Navigate, Link } from 'react-router-dom';
import { AuthProvider } from './hooks/useAuth';
import { Layout } from './components/layout/Layout';
import Button from './components/common/Button';
import Home from './routes/Home';
import Championnats from './routes/Championnats';
import Tournois from './routes/Tournois';
import Clubs from './routes/Clubs';
import Joueurs from './routes/Joueurs';
import Actualites from './routes/Actualites';
import Federation from './routes/Federation';
import Connexion from './routes/Connexion';
import Inscription from './routes/Inscription';
import VerificationEmail from './routes/VerificationEmail';
import ClubDetail from './routes/ClubDetail';
import PlayerDetail from './routes/PlayerDetail';
import TournamentDetail from './routes/TournamentDetail';
import ChampionshipDetail from './routes/ChampionshipDetail';
import MatchDetail from './routes/MatchDetail';
import TeamDetail from './routes/TeamDetail';


// Import global styles
import './styles/design-system.css';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      refetchOnWindowFocus: false,
      retry: 2,
    },
  },
});

// Router configuration
const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <Layout>
        <Home />
      </Layout>
    ),
  },
  {
    path: '/championnats',
    element: (
      <Layout>
        <Championnats />
      </Layout>
    ),
  },
  {
    path: '/championnats/:championshipId',
    element: (
      <Layout>
        <ChampionshipDetail />
      </Layout>
    ),
  },
  {
    path: '/tournois',
    element: (
      <Layout>
        <Tournois />
      </Layout>
    ),
  },
  {
    path: '/tournois/:tournamentId',
    element: (
      <Layout>
        <TournamentDetail />
      </Layout>
    ),
  },
  {
    path: '/clubs',
    element: (
      <Layout>
        <Clubs />
      </Layout>
    ),
  },
  {
    path: '/clubs/:clubId',
    element: (
      <Layout>
        <ClubDetail />
      </Layout>
    ),
  },
  {
    path: '/joueurs',
    element: (
      <Layout>
        <Joueurs />
      </Layout>
    ),
  },
  {
    path: '/joueurs/:playerId',
    element: (
      <Layout>
        <PlayerDetail />
      </Layout>
    ),
  },
  {
    path: '/actualites',
    element: (
      <Layout>
        <Actualites />
      </Layout>
    ),
  },
  {
    path: '/federation',
    element: (
      <Layout>
        <Federation />
      </Layout>
    ),
  },
  {
    path: '/matchs/:matchId',
    element: (
      <Layout>
        <MatchDetail />
      </Layout>
    ),
  },
  {
    path: '/teams/:teamId',
    element: (
      <Layout>
        <TeamDetail />
      </Layout>
    ),
  },
  {
    path: '/connexion',
    element: <Connexion />,
  },
  {
    path: '/inscription',
    element: <Inscription />,
  },
  {
    path: '/verification-email',
    element: <VerificationEmail />,
  },
  {
    path: '*',
    element: (
      <Layout>
        <main className="flex-1 px-4 py-6 lg:px-8 lg:py-8">
          <div className="max-w-7xl mx-auto text-center">
            <h1 className="text-heading-xl font-bold text-white uppercase tracking-wide mb-4">
              404 - Page non trouvée
            </h1>
            <p className="text-body-m text-primary-200 mb-6">
              La page que vous cherchez n'existe pas.
            </p>
            <Link to="/">
              <Button variant="primary" size="l">
                Retour à l'accueil
              </Button>
            </Link>
          </div>
        </main>
      </Layout>
    ),
  },

]);

// Main app component
function App() {
  return (
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <AuthProvider>
          <RouterProvider router={router} />
        </AuthProvider>
      </QueryClientProvider>
    </StrictMode>
  );
}

// Render the app
const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error('Failed to find the root element');
}

createRoot(rootElement).render(<App />);