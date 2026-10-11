import React, { useState, useEffect } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Icon } from "../common/Icon";
import { Logo } from "../common/Logo";

export interface MobileNavProps {
  isOpen: boolean;
  onClose: () => void;
}

/**
 * MobileNav - Top 14 inspired mobile navigation menu
 * Full-screen overlay with slide-in animation
 */
export function MobileNav({ isOpen, onClose }: MobileNavProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const { isAuthenticated, user, logout } = useAuth();

  // Navigation links - Top 14 inspired
  const mainNavLinks = [
    { to: "/", label: "Accueil", icon: "Home" as const },
    { to: "/calendrier-et-resultats", label: "Calendrier et résultats", icon: "Calendar" as const },
    { to: "/classement", label: "Classement", icon: "Trophy" as const },
    { to: "/my-rugby", label: "MyPalet", icon: "Award" as const },
  ];

  const otherNavLinks = [
    { to: "/clubs", label: "Clubs et joueurs", icon: "Users" as const },
    { to: "/videos", label: "Vidéos", icon: "PlayCircle" as const },
    { to: "/statistiques", label: "Statistiques", icon: "BarChart3" as const },
    { to: "/actualites", label: "Actualités", icon: "FileText" as const },
    { to: "/le-cdsp-79", label: "Le CDSP 79", icon: "Info" as const },
  ];

  const isActive = (path: string) => {
    if (path === "/") {
      return location.pathname === "/";
    }
    return location.pathname.startsWith(path);
  };

  const handleNavigation = (to: string) => {
    navigate(to);
    onClose();
  };

  const handleLogout = () => {
    logout();
    onClose();
    navigate("/");
  };

  // Close menu on route change
  useEffect(() => {
    onClose();
  }, [location, onClose]);

  // Prevent body scroll when menu is open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden";
    } else {
      document.body.style.overflow = "";
    }
    return () => {
      document.body.style.overflow = "";
    };
  }, [isOpen]);

  // Close on escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <>
      {/* Overlay */}
      <div
        className="fixed inset-0 bg-black/80 backdrop-blur-sm z-[calc(var(--z-modal)-1)] transition-opacity duration-300"
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Mobile Navigation Menu - Slide in from right like Top 14 */}
      <nav
        className="fixed top-0 right-0 bottom-0 w-80 max-w-[90vw] bg-[var(--bg-dark)] shadow-2xl z-[var(--z-modal)] transform transition-transform duration-300 ease-in-out"
        role="navigation"
        aria-label="Navigation mobile"
      >
        {/* Header with close button and logo */}
        <div className="flex items-center justify-between p-[var(--spacing-4)] border-b border-border-dark">
          <Link to="/" onClick={onClose} className="flex items-center gap-[var(--spacing-2)]">
            <Logo variant="short" size="md" />
          </Link>
          <button
            className="w-10 h-10 flex items-center justify-center bg-transparent border-none cursor-pointer text-brand-green hover:text-brand-green-400 rounded-[var(--radius-m)] hover:bg-[var(--bg-secondary)] transition-colors"
            onClick={onClose}
            aria-label="Fermer le menu"
            type="button"
          >
            <Icon name="X" size="l" />
          </button>
        </div>

        {/* Scrollable content */}
        <div className="flex flex-col h-[calc(100%-80px)] overflow-y-auto">
          {/* Main Navigation */}
          <div className="p-[var(--spacing-4)]">
            <h3 className="text-[var(--text-body-xs)] font-[var(--font-weight-bold)] text-text-tertiary uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-3)]">
              Menu principal
            </h3>
            <ul className="space-y-[var(--spacing-1)]">
              {mainNavLinks.map((link) => (
                <li key={link.to}>
                  <Link
                    to={link.to}
                    onClick={() => handleNavigation(link.to)}
                    className={`
                      flex items-center gap-[var(--spacing-3)] 
                      px-[var(--spacing-3)] py-[var(--spacing-2)] 
                      text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                      rounded-[var(--radius-m)] transition-all duration-200
                      ${
                        isActive(link.to)
                          ? "bg-brand-green/10 text-brand-green border border-brand-green/30"
                          : "text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green"
                      }
                    `}
                  >
                    <Icon name={link.icon} size="m" />
                    <span>{link.label}</span>
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Other Navigation */}
          <div className="p-[var(--spacing-4)] border-t border-border-dark">
            <h3 className="text-[var(--text-body-xs)] font-[var(--font-weight-bold)] text-text-tertiary uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-3)]">
              Autres
            </h3>
            <ul className="space-y-[var(--spacing-1)]">
              {otherNavLinks.map((link) => (
                <li key={link.to}>
                  <Link
                    to={link.to}
                    onClick={() => handleNavigation(link.to)}
                    className={`
                      flex items-center gap-[var(--spacing-3)] 
                      px-[var(--spacing-3)] py-[var(--spacing-2)] 
                      text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                      rounded-[var(--radius-m)] transition-all duration-200
                      ${
                        isActive(link.to)
                          ? "bg-brand-green/10 text-brand-green"
                          : "text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green"
                      }
                    `}
                  >
                    <Icon name={link.icon} size="m" />
                    <span>{link.label}</span>
                  </Link>
                </li>
              ))}
            </ul>
          </div>

          {/* Authentication Section */}
          <div className="p-[var(--spacing-4)] border-t border-border-dark mt-auto">
            {isAuthenticated ? (
              <>
                <h3 className="text-[var(--text-body-xs)] font-[var(--font-weight-bold)] text-text-tertiary uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-3)]">
                  Compte
                </h3>
                <ul className="space-y-[var(--spacing-1)]">
                  <li>
                    <Link
                      to={`/joueurs/${user?.id}`}
                      onClick={onClose}
                      className="flex items-center gap-[var(--spacing-3)] px-[var(--spacing-3)] py-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green rounded-[var(--radius-m)] transition-all duration-200"
                    >
                      <Icon name="User" size="m" />
                      <span>Mon profil</span>
                    </Link>
                  </li>
                  <li>
                    <Link
                      to="/mon-compte"
                      onClick={onClose}
                      className="flex items-center gap-[var(--spacing-3)] px-[var(--spacing-3)] py-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green rounded-[var(--radius-m)] transition-all duration-200"
                    >
                      <Icon name="Settings" size="m" />
                      <span>Paramètres</span>
                    </Link>
                  </li>
                  <li>
                    <button
                      onClick={handleLogout}
                      className="flex items-center gap-[var(--spacing-3)] w-full px-[var(--spacing-3)] py-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-orange hover:bg-brand-orange/10 rounded-[var(--radius-m)] transition-all duration-200"
                      type="button"
                    >
                      <Icon name="LogOut" size="m" />
                      <span>Déconnexion</span>
                    </button>
                  </li>
                </ul>
              </>
            ) : (
              <>
                <h3 className="text-[var(--text-body-xs)] font-[var(--font-weight-bold)] text-text-tertiary uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-3)]">
                  Connexion
                </h3>
                <ul className="space-y-[var(--spacing-1)]">
                  <li>
                    <Link
                      to="/connexion"
                      onClick={onClose}
                      className="flex items-center gap-[var(--spacing-3)] px-[var(--spacing-3)] py-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green rounded-[var(--radius-m)] transition-all duration-200"
                    >
                      <Icon name="LogIn" size="m" />
                      <span>Se connecter</span>
                    </Link>
                  </li>
                  <li>
                    <Link
                      to="/inscription"
                      onClick={onClose}
                      className="flex items-center gap-[var(--spacing-3)] px-[var(--spacing-3)] py-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:bg-[var(--bg-secondary)] hover:text-brand-green rounded-[var(--radius-m)] transition-all duration-200"
                    >
                      <Icon name="UserPlus" size="m" />
                      <span>Créer un compte</span>
                    </Link>
                  </li>
                </ul>
              </>
            )}
          </div>

          {/* All Sites Section */}
          <div className="p-[var(--spacing-4)] border-t border-border-dark bg-[var(--bg-secondary)]">
            <h3 className="text-[var(--text-body-xs)] font-[var(--font-weight-bold)] text-text-tertiary uppercase tracking-[var(--letter-spacing-wider)] mb-[var(--spacing-3)]">
              Tous les sites CDSP
            </h3>
            <div className="grid grid-cols-2 gap-[var(--spacing-2)]">
              {[
                { name: "Championnat 79", to: "/championnats" },
                { name: "Coupe 79", to: "/coupe-79" },
                { name: "Challenges", to: "/challenges" },
                { name: "Documents", to: "/documents" },
              ].map((site) => (
                <Link
                  key={site.to}
                  to={site.to}
                  onClick={onClose}
                  className="flex items-center justify-center gap-[var(--spacing-1)] px-[var(--spacing-2)] py-[var(--spacing-2)] bg-[var(--bg-tertiary)] text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-text-secondary hover:bg-[var(--bg-secondary)] rounded-[var(--radius-sm)] transition-all duration-200"
                >
                  <Icon name="Link" size="xs" />
                  <span className="truncate">{site.name}</span>
                </Link>
              ))}
            </div>
          </div>
        </div>
      </nav>
    </>
  );
}

export default MobileNav;
