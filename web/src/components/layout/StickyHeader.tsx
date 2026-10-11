import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Logo } from "../common/Logo";

/**
 * StickyHeader - Mimics Top 14 LNR structure
 * Sticky header that appears on scroll, containing navigation and branding
 */
export function StickyHeader() {
  const [isSticky, setIsSticky] = useState(false);
  const { isAuthenticated, user } = useAuth();

  // Navigation links - similar to Top 14 structure
  const navLinks = [
    { to: "/", label: "Accueil" },
    { to: "/calendrier-et-resultats", label: "Calendrier et résultats" },
    { to: "/classement", label: "Classement" },
    { to: "/clubs", label: "Clubs et joueurs" },
    { to: "/videos", label: "Vidéos" },
    { to: "/statistiques", label: "Statistiques" },
    { to: "/actualites", label: "Actualités" },
    { to: "/le-cdsp-79", label: "Le CDSP 79" },
  ];

  useEffect(() => {
    const handleScroll = () => {
      setIsSticky(window.scrollY > 150);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  if (!isSticky) return null;

  return (
    <header className="fixed top-0 left-0 right-0 z-[var(--z-sticky)] bg-[var(--bg-dark)]/95 backdrop-blur-sm border-b border-border-dark shadow-lg transition-all duration-300">
      <div className="container mx-auto px-[var(--container-padding)] py-[var(--spacing-2)]">
        <div className="flex items-center justify-between gap-[var(--spacing-4)]">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-[var(--spacing-2)] flex-shrink-0">
            <Logo variant="short" size="md" />
          </Link>

          {/* Navigation */}
          <nav className="hidden lg:flex flex-1 justify-center">
            <ul className="flex items-center gap-[var(--spacing-4)] list-none m-0 p-0">
              {navLinks.map((link) => (
                <li key={link.to}>
                  <Link
                    to={link.to}
                    className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:text-brand-green transition-colors whitespace-nowrap"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </nav>

          {/* Auth Section */}
          <div className="flex items-center gap-[var(--spacing-4)]">
            {isAuthenticated ? (
              <div className="flex items-center gap-[var(--spacing-2)]">
                <span className="text-[var(--text-body-sm)] text-text-secondary">
                  Bonjour, {user?.firstName}
                </span>
                <Link
                  to="/mon-compte"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-green hover:text-brand-green-400 transition-colors"
                >
                  Mon compte
                </Link>
              </div>
            ) : (
              <>
                <Link
                  to="/connexion"
                  className="hidden sm:flex text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:text-brand-green transition-colors"
                >
                  Connexion
                </Link>
                <Link
                  to="/inscription"
                  className="flex items-center gap-[var(--spacing-2)] px-[var(--spacing-3)] py-[var(--spacing-1)] bg-brand-orange text-black text-[var(--text-body-sm)] font-[var(--font-weight-bold)] rounded-[var(--radius-sm)] hover:bg-brand-orange-400 transition-colors"
                >
                  S'inscrire
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}

export default StickyHeader;
