import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAuth } from '../hooks/useAuth';
import { Layout } from '../components/layout/Layout';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';

const Connexion: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { login, isLoading, error, clearError } = useAuth();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [rememberMe, setRememberMe] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    clearError();

    try {
      await login(email, password, rememberMe);
      // Si la connexion réussit, useAuth gère la redirection
      // Sinon, on reste sur la page
    } catch (err) {
      // L'erreur est gérée par useAuth
      console.error('Login error:', err);
    }
  };

  // Si déjà connecté, rediriger
  const { isAuthenticated } = useAuth();
  if (isAuthenticated) {
    navigate('/');
    return null;
  }

  return (
    <Layout showHeader={false}>
      <main className="flex-1 flex items-center justify-center min-h-screen bg-primary-900">
        <div className="w-full max-w-md px-4 py-8">
          <div className="bg-primary-800 rounded-lg p-8 border border-primary-700/50">
            {/* Logo */}
            <div className="text-center mb-8">
              <div className="w-20 h-20 mx-auto mb-4 bg-primary-900 rounded-full border-4 border-gold-700 flex items-center justify-center">
                <Icon name="Target" size="xxl" className="text-gold-700" />
              </div>
              <h1 className="text-heading-l font-bold text-white uppercase tracking-wide">
                PALET VENDÉEN
              </h1>
              <p className="text-body-s text-primary-300 mt-2">
                Fédération des joueurs et clubs
              </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-6">
              <h2 className="text-heading-s font-bold text-gold-700 uppercase tracking-wide text-center mb-6">
                CONNEXION
              </h2>

              {error && (
                <div className="bg-danger-900/50 border border-danger-700 rounded-lg p-3 text-danger-200 text-body-s">
                  <Icon name="AlertCircle" size="sm" className="inline mr-2" />
                  {error}
                </div>
              )}

              <div className="space-y-2">
                <label htmlFor="email" className="text-body-s font-medium text-primary-200">
                  Adresse email
                </label>
                <div className="relative">
                  <Icon
                    name="Mail"
                    size="m"
                    className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300"
                  />
                  <input
                    type="email"
                    id="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="votre@email.com"
                    required
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                  />
                </div>
              </div>

              <div className="space-y-2">
                <label htmlFor="password" className="text-body-s font-medium text-primary-200">
                  Mot de passe
                </label>
                <div className="relative">
                  <Icon
                    name="Lock"
                    size="m"
                    className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300"
                  />
                  <input
                    type="password"
                    id="password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    placeholder="••••••••"
                    required
                    minLength={8}
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                  />
                </div>
              </div>

              <div className="flex items-center justify-between">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={rememberMe}
                    onChange={(e) => setRememberMe(e.target.checked)}
                    className="w-4 h-4 accent-gold-700"
                  />
                  <span className="text-body-s text-primary-200">Se souvenir de moi</span>
                </label>

                <button
                  type="button"
                  className="text-body-s text-gold-700 hover:text-gold-600"
                >
                  Mot de passe oublié ?
                </button>
              </div>

              <Button
                type="submit"
                variant="primary"
                size="m"
                className="w-full"
                loading={isLoading}
              >
                Se connecter
              </Button>

              <div className="text-center pt-4 border-t border-primary-700/50">
                <p className="text-body-s text-primary-300 mb-4">
                  Vous n'avez pas de compte ?
                </p>
                <Button
                  variant="outline"
                  size="m"
                  className="w-full"
                  onClick={() => navigate('/inscription')}
                >
                  S'inscrire
                </Button>
              </div>
            </form>
          </div>

          {/* Footer */}
          <p className="text-center text-caption text-primary-400 mt-8">
            © 2025 Palet Vendéen - Tous droits réservés
          </p>
        </div>
      </main>
    </Layout>
  );
};

export default Connexion;
