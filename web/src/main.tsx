import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { createBrowserRouter, RouterProvider, Navigate, Link, Outlet } from 'react-router-dom';
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

// Auth wrapper component - must be inside Router context
function AuthWrapper() {
  return (
    <AuthProvider>
      <Outlet />
    </AuthProvider>
  );
}

// 404 component
function NotFoundPage() {
  return (
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
  );
}

// Router configuration
const router = createBrowserRouter([
  {
    path: '/',
    element: <AuthWrapper />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: 'championnats',
        element: <Championnats />,
      },
      {
        path: 'championnats/:championshipId',
        element: <ChampionshipDetail />,
      },
      {
        path: 'tournois',
        element: <Tournois />,
      },
      {
        path: 'tournois/:tournamentId',
        element: <TournamentDetail />,
      },
      {
        path: 'clubs',
        element: <Clubs />,
      },
      {
        path: 'clubs/:clubId',
        element: <ClubDetail />,
      },
      {
        path: 'joueurs',
        element: <Joueurs />,
      },
      {
        path: 'joueurs/:playerId',
        element: <PlayerDetail />,
      },
      {
        path: 'actualites',
        element: <Actualites />,
      },
      {
        path: 'federation',
        element: <Federation />,
      },
      {
        path: 'matchs/:matchId',
        element: <MatchDetail />,
      },
      {
        path: 'teams/:teamId',
        element: <TeamDetail />,
      },
      {
        path: 'connexion',
        element: <Connexion />,
      },
      {
        path: 'inscription',
        element: <Inscription />,
      },
      {
        path: 'verification-email',
        element: <VerificationEmail />,
      },
      {
        path: '*',
        element: <NotFoundPage />,
      },
    ],
  },

]);

// Main app component
function App() {
  return (
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
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