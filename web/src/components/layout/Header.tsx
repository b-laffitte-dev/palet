import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import { Icon } from '../common/Icon';

export interface HeaderProps {
  showCTA?: boolean;
}

/**
 * Header - Main navigation and branding component
 * Displays logo, navigation links, and call-to-action button
 */
export function Header({ showCTA = true }: HeaderProps) {
  const location = useLocation();
  const { isAuthenticated } = useAuth();

  // Navigation links
  const navLinks = [
    { to: '/', label: 'Accueil' },
    { to: '/championnats', label: 'Championnats' },
    { to: '/tournois', label: 'Tournois' },
    { to: '/clubs', label: 'Clubs' },
    { to: '/joueurs', label: 'Joueurs' },
    { to: '/federation', label: 'Fédération' },
    { to: '/actualites', label: 'Actualités' },
  ];

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/';
    }
    return location.pathname.startsWith(path);
  };

  return (
    <header className="h-[80px] bg-white border-b border-border-primary sticky top-[40px] z-[var(--z-fixed)]">
      <div className="w-full max-w-[var(--container-max)] mx-auto px-[var(--container-padding)] flex items-center justify-between gap-[var(--spacing-4)]">
        {/* Left Section - Logo and Tagline */}
        <div className="flex flex-col gap-[var(--spacing-1)]">
          <Link to="/" className="text-decoration-none">
            <div className="flex items-center gap-[var(--spacing-2)] text-primary-700">
              <Icon name="Target" size="l" />
              <span className="text-[var(--text-heading-m)] font-[var(--font-weight-bold)] tracking-[var(--letter-spacing-tight)]">
                Palet Vendéen
              </span>
            </div>
          </Link>
          <div className="flex items-center gap-[var(--spacing-2)] text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-text-tertiary tracking-[var(--letter-spacing-wide)] uppercase">
            <span>LE SITE OFFICIEL</span>
            <span className="text-primary-500">—</span>
            <span>CHAMPIONNATS, CLUBS & TOURNOIS</span>
          </div>
        </div>

        {/* Center Section - Navigation */}
        <nav className="flex-1 flex justify-center" aria-label="Navigation principale">
          <ul className="flex items-center gap-[var(--spacing-6)] list-none m-0 p-0">
            {navLinks.map((link) => (
              <li key={link.to} className="relative">
                <Link
                  to={link.to}
                  className={`
                    flex items-center px-[var(--spacing-2)] py-[var(--spacing-1)] 
                    text-body-sm font-medium text-text-secondary 
                    border-b-2 border-transparent 
                    hover:text-text-primary hover:border-primary-500
                    transition-colors transition-border
                    ${isActive(link.to) ? 'text-primary-700 border-primary-700 font-semibold' : ''}
                  `}
                >
                  {link.label}
                </Link>
              </li>
            ))}
          </ul>
        </nav>

        {/* Right Section - CTA */}
        {showCTA && !isAuthenticated && (
          <div className="flex items-center">
            <Link 
              to="/inscription" 
              className="
                flex items-center gap-[var(--spacing-2)] 
                px-[var(--spacing-4)] py-[var(--spacing-2)] 
                bg-primary-600 text-white 
                text-body-sm font-semibold 
                rounded-[var(--radius-m)] 
                border-none cursor-pointer
                tracking-[var(--letter-spacing-wide)]
                hover:bg-primary-700 active:bg-primary-800
                transition-colors transition-transform hover:-translate-y-px active:translate-y-0
              "
            >
              S'INSCRIRE
            </Link>
          </div>
        )}
      </div>
    </header>
  );
}

export default Header;
