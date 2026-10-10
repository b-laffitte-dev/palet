import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../hooks/useAuth';
import { Layout } from '../components/layout/Layout';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';

const Inscription: React.FC = () => {
  const navigate = useNavigate();
  const { register, isLoading, error, clearError } = useAuth();

  const [formData, setFormData] = useState({
    firstName: '',
    lastName: '',
    email: '',
    password: '',
    confirmPassword: '',
  });

  const [acceptTerms, setAcceptTerms] = useState(false);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
    clearError();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    clearError();

    if (formData.password !== formData.confirmPassword) {
      // Gérer l'erreur localement
      return;
    }

    if (!acceptTerms) {
      return;
    }

    try {
      const userData = {
        firstName: formData.firstName,
        lastName: formData.lastName,
        email: formData.email,
        password: formData.password,
      };

      await register(userData);
      // Si l'inscription réussit, useAuth redirige vers /verification-email
    } catch (err) {
      console.error('Registration error:', err);
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
        <div className="w-full max-w-2xl px-4 py-8">
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
                Créer votre compte
              </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-6">
              <h2 className="text-heading-s font-bold text-gold-700 uppercase tracking-wide text-center mb-6">
                INSCRIPTION
              </h2>

              {error && (
                <div className="bg-danger-900/50 border border-danger-700 rounded-lg p-3 text-danger-200 text-body-s">
                  <Icon name="AlertCircle" size="sm" className="inline mr-2" />
                  {error}
                </div>
              )}

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <label htmlFor="firstName" className="text-body-s font-medium text-primary-200">
                    Prénom
                  </label>
                  <input
                    type="text"
                    id="firstName"
                    name="firstName"
                    value={formData.firstName}
                    onChange={handleChange}
                    placeholder="Jean"
                    required
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg px-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                  />
                </div>
                <div className="space-y-2">
                  <label htmlFor="lastName" className="text-body-s font-medium text-primary-200">
                    Nom
                  </label>
                  <input
                    type="text"
                    id="lastName"
                    name="lastName"
                    value={formData.lastName}
                    onChange={handleChange}
                    placeholder="Dupont"
                    required
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg px-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                  />
                </div>
              </div>

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
                    name="email"
                    value={formData.email}
                    onChange={handleChange}
                    placeholder="votre@email.com"
                    required
                    className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                  />
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
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
                      name="password"
                      value={formData.password}
                      onChange={handleChange}
                      placeholder="••••••••"
                      required
                      minLength={8}
                      className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                    />
                  </div>
                  <p className="text-caption text-primary-400">
                    Minimum 8 caractères
                  </p>
                </div>
                <div className="space-y-2">
                  <label htmlFor="confirmPassword" className="text-body-s font-medium text-primary-200">
                    Confirmer le mot de passe
                  </label>
                  <div className="relative">
                    <Icon
                      name="Lock"
                      size="m"
                      className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-300"
                    />
                    <input
                      type="password"
                      id="confirmPassword"
                      name="confirmPassword"
                      value={formData.confirmPassword}
                      onChange={handleChange}
                      placeholder="••••••••"
                      required
                      className="w-full bg-primary-900 border border-primary-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder:text-primary-400 focus:outline-none focus:border-gold-700"
                    />
                  </div>
                  {formData.password && formData.confirmPassword && formData.password !== formData.confirmPassword && (
                    <p className="text-caption text-danger-400">
                      <Icon name="AlertCircle" size="xs" className="inline mr-1" />
                      Les mots de passe ne correspondent pas
                    </p>
                  )}
                </div>
              </div>

              <div className="flex items-start gap-3">
                <input
                  type="checkbox"
                  id="acceptTerms"
                  checked={acceptTerms}
                  onChange={(e) => setAcceptTerms(e.target.checked)}
                  className="w-4 h-4 accent-gold-700 mt-1"
                />
                <label htmlFor="acceptTerms" className="text-body-s text-primary-200">
                  J'accepte les{' '}
                  <button type="button" className="text-gold-700 hover:text-gold-600 underline">
                    conditions générales
                  </button>{' '}
                  et la{' '}
                  <button type="button" className="text-gold-700 hover:text-gold-600 underline">
                    politique de confidentialité
                  </button>
                </label>
              </div>

              <Button
                type="submit"
                variant="primary"
                size="m"
                className="w-full"
                loading={isLoading}
                disabled={!acceptTerms || formData.password !== formData.confirmPassword}
              >
                Créer mon compte
              </Button>

              <div className="text-center pt-4 border-t border-primary-700/50">
                <p className="text-body-s text-primary-300 mb-4">
                  Vous avez déjà un compte ?
                </p>
                <Button
                  variant="outline"
                  size="m"
                  className="w-full"
                  onClick={() => navigate('/connexion')}
                >
                  Se connecter
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

export default Inscription;
