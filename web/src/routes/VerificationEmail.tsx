import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Layout } from '../components/layout/Layout';
import { Icon } from '../components/common/Icon';
import Button from '../components/common/Button';

const VerificationEmail: React.FC = () => {
  const navigate = useNavigate();
  const [email, setEmail] = useState<string | null>(null);
  const [isVerified, setIsVerified] = useState(false);
  const [isResending, setIsResending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // En réalité, on devrait récupérer l'email depuis le state de navigation
  // ou depuis le contexte d'authentification
  // Pour l'instant, on simule avec un email stocké
  useEffect(() => {
    // Récupérer l'email depuis le storage ou le state
    const storedEmail = localStorage.getItem('palet_registration_email') ||
      sessionStorage.getItem('palet_registration_email');
    if (storedEmail) {
      setEmail(storedEmail);
    }
  }, []);

  const handleResend = async () => {
    setIsResending(true);
    setError(null);
    
    try {
      // Simuler l'envoi d'un nouvel email de vérification
      // En réalité: await api().POST('/auth/resend-verification', { email });
      setSuccess('Un nouvel email de vérification a été envoyé');
    } catch {
      setError('Erreur lors de l\'envoi de l\'email');
    } finally {
      setIsResending(false);
    }
  };

  // Si on a été redirigé ici après vérification réussie
  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const verified = searchParams.get('verified');
    const token = searchParams.get('token');
    
    if (verified === 'true' && token) {
      setIsVerified(true);
      // Nettoyer les params de l'URL
      window.history.replaceState({}, document.title, window.location.pathname);
    }
  }, []);

  if (isVerified) {
    return (
      <Layout showHeader={false}>
        <main className="flex-1 flex items-center justify-center min-h-screen bg-primary-900">
          <div className="w-full max-w-md px-4 py-8">
            <div className="bg-primary-800 rounded-lg p-8 border border-primary-700/50 text-center">
              <div className="w-20 h-20 mx-auto mb-6 bg-success-900/50 rounded-full flex items-center justify-center border-4 border-success-700">
                <Icon name="Check" size="xxl" className="text-success-700" />
              </div>
              
              <h1 className="text-heading-m font-bold text-white uppercase tracking-wide mb-4">
                VÉRIFICATION RÉUSSIE
              </h1>
              <p className="text-body-m text-primary-200 mb-6">
                Votre adresse email a été vérifiée avec succès.
              </p>
              
              <Button
                variant="primary"
                size="m"
                className="w-full mb-4"
                onClick={() => navigate('/connexion')}
              >
                Se connecter
              </Button>
              
              <Button
                variant="ghost"
                size="m"
                className="w-full"
                onClick={() => navigate('/')}
              >
                Retour à l'accueil
              </Button>
            </div>
          </div>
        </main>
      </Layout>
    );
  }

  return (
    <Layout showHeader={false}>
      <main className="flex-1 flex items-center justify-center min-h-screen bg-primary-900">
        <div className="w-full max-w-md px-4 py-8">
          <div className="bg-primary-800 rounded-lg p-8 border border-primary-700/50 text-center">
            <div className="w-20 h-20 mx-auto mb-6 bg-primary-900 rounded-full flex items-center justify-center border-4 border-primary-700">
              <Icon name="Mail" size="xxl" className="text-primary-300" />
            </div>
            
            <h1 className="text-heading-m font-bold text-white uppercase tracking-wide mb-4">
              VÉRIFIEZ VOTRE EMAIL
            </h1>
            <p className="text-body-m text-primary-200 mb-6">
              Nous avons envoyé un email de vérification à l'adresse :
            </p>
            
            <p className="text-heading-s font-bold text-gold-700 mb-6">
              {email || 'votre@email.com'}
            </p>
            
            <p className="text-body-s text-primary-300 mb-8">
              Cliquez sur le lien dans l'email pour activer votre compte.
              N'oubliez pas de vérifier vos spams.
            </p>

            {error && (
              <div className="bg-danger-900/50 border border-danger-700 rounded-lg p-3 text-danger-200 text-body-s mb-6">
                <Icon name="AlertCircle" size="sm" className="inline mr-2" />
                {error}
              </div>
            )}

            {success && (
              <div className="bg-success-900/50 border border-success-700 rounded-lg p-3 text-success-200 text-body-s mb-6">
                <Icon name="Check" size="sm" className="inline mr-2" />
                {success}
              </div>
            )}

            <Button
              variant="outline"
              size="m"
              className="w-full mb-4"
              onClick={handleResend}
              loading={isResending}
            >
              Renvoyer l'email de vérification
            </Button>
            
            <Button
              variant="ghost"
              size="m"
              className="w-full"
              onClick={() => navigate('/')}
            >
              Retour à l'accueil
            </Button>
            
            <p className="text-caption text-primary-400 mt-8">
              Vous n'avez pas reçu l'email ? Vérifiez vos spams ou{' '}
              <button
                type="button"
                className="text-gold-700 hover:text-gold-600 underline"
                onClick={handleResend}
              >
                renvoyez-le
              </button>
            </p>
          </div>
        </div>
      </main>
    </Layout>
  );
};

export default VerificationEmail;
