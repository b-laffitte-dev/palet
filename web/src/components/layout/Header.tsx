import React, { useState, useEffect } from "react";
import { Link, useLocation } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Icon } from "../common/Icon";
import { Logo } from "../common/Logo";
import { MobileNav } from "./MobileNav";
import { LiveIndicator } from "../common/LiveIndicator";

export interface HeaderProps {
  showCTA?: boolean;
  hasLiveMatches?: boolean;
  liveMatchesCount?: number;
}

/**
 * Header - Top 14 inspired header structure
 * Displays logo, live scores ticker, navigation, and call-to-action
 * Fully responsive with mobile hamburger menu
 */
export function Header({ showCTA = true, hasLiveMatches = false, liveMatchesCount = 0 }: HeaderProps) {
  const location = useLocation();
  const { isAuthenticated, user } = useAuth();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isScrolled, setIsScrolled] = useState(false);

  // Navigation links - Top 14 inspired
  const mainNavLinks = [
    { to: "/calendrier-et-resultats", label: "Calendrier et résultats", icon: "Calendar" as const },
    { to: "/classement", label: "Classement", icon: "Trophy" as const },
    { to: "/my-rugby", label: "MyPalet", icon: "Award" as const },
  ];

  const otherNavLinks = [
    { to: "/clubs", label: "Clubs et joueurs" },
    { to: "/videos", label: "Vidéos" },
    { to: "/statistiques", label: "Statistiques" },
    { to: "/actualites", label: "Actualités" },
    { to: "/le-cdsp-79", label: "Le CDSP 79" },
  ];

  const isActive = (path: string) => {
    if (path === "/") {
      return location.pathname === "/";
    }
    return location.pathname.startsWith(path);
  };

  const toggleMobileMenu = () => {
    setIsMobileMenuOpen(!isMobileMenuOpen);
  };

  const closeMobileMenu = () => {
    setIsMobileMenuOpen(false);
  };

  // Scroll effect
  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 10);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  // Close mobile menu on window resize if larger than tablet
  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > 1023 && isMobileMenuOpen) {
        closeMobileMenu();
      }
    };
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [isMobileMenuOpen]);

  return (
    <>
      {/* Main Header */}
      <header
        className={`bg-[var(--bg-dark)] border-b border-border-dark sticky top-[40px] z-[var(--z-fixed)] transition-all duration-200 ${
          isScrolled ? "shadow-lg" : ""
        }`}
      >
        <div className="w-full">
          {/* Top section with logo and live scores */}
          <div className="flex items-center justify-between px-[var(--spacing-4)] py-[var(--spacing-2)] border-b border-border-dark/50">
            {/* Logo Section */}
            <div className="flex items-center gap-[var(--spacing-2)]">
              <Link to="/" className="flex items-center gap-[var(--spacing-2)]">
                <Logo variant="short" size="md" className="lg:hidden" />
                <Logo variant="short" size="lg" className="hidden lg:flex" />
                <div className="flex flex-col">
                  <span className="text-[var(--text-heading-m)] lg:text-[var(--text-heading-l)] font-[var(--font-weight-bold)] tracking-[var(--letter-spacing-tight)] text-white">
                    CDSP 79
                  </span>
                  <span className="hidden lg:block text-[var(--text-body-xs)] text-text-tertiary tracking-[var(--letter-spacing-wide)] uppercase">
                    PALET FONTE
                  </span>
                </div>
              </Link>
            </div>

            {/* Live Scores Indicator */}
            {hasLiveMatches && (
              <div className="hidden lg:flex items-center gap-[var(--spacing-4)]">
                <LiveIndicator count={liveMatchesCount} />
              </div>
            )}

            {/* Right Section - Auth and CTA */}
            <div className="flex items-center gap-[var(--spacing-4)]">
              {isAuthenticated ? (
                <div className="hidden lg:flex items-center gap-[var(--spacing-2)]">
                  <span className="text-[var(--text-body-sm)] text-text-secondary">
                    Bonjour, {user?.firstName}
                  </span>
                  <Link
                    to="/mon-compte"
                    className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-brand-green hover:text-brand-green-400 transition-colors flex items-center gap-[var(--spacing-1)]"
                  >
                    <Icon name="User" size="sm" />
                    Mon compte
                  </Link>
                </div>
              ) : (
                <>
                  {/* Desktop Auth Links */}
                  <div className="hidden lg:flex items-center gap-[var(--spacing-4)]">
                    <Link
                      to="/connexion"
                      className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:text-brand-green transition-colors"
                    >
                      Connexion
                    </Link>
                    {showCTA && (
                      <Link
                        to="/inscription"
                        className="flex items-center gap-[var(--spacing-2)] px-[var(--spacing-4)] py-[var(--spacing-2)] bg-brand-orange text-black text-[var(--text-body-sm)] font-[var(--font-weight-bold)] rounded-[var(--radius-m)] border-none cursor-pointer tracking-[var(--letter-spacing-wide)] hover:bg-brand-orange-400 active:bg-brand-orange-600 transition-all duration-200 shadow-md hover:shadow-lg"
                      >
                        S'inscrire
                      </Link>
                    )}
                  </div>

                  {/* Mobile Auth Link */}
                  <div className="lg:hidden">
                    {showCTA ? (
                      <Link
                        to="/inscription"
                        className="flex items-center gap-[var(--spacing-1)] px-[var(--spacing-3)] py-[var(--spacing-1)] bg-brand-orange text-black text-[var(--text-body-xs)] font-[var(--font-weight-bold)] rounded-[var(--radius-sm)]"
                      >
                        S'inscrire
                      </Link>
                    ) : (
                      <Link
                        to="/connexion"
                        className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:text-brand-green"
                      >
                        Connexion
                      </Link>
                    )}
                  </div>
                </>
              )}

              {/* Mobile Menu Toggle */}
              <button
                onClick={toggleMobileMenu}
                className="lg:hidden flex items-center justify-center p-[var(--spacing-2)] bg-transparent border-none cursor-pointer text-brand-green hover:text-brand-green-400 transition-colors rounded-[var(--radius-m)] hover:bg-brand-green/10"
                aria-label="Menu"
                aria-expanded={isMobileMenuOpen}
                aria-controls="mobile-navigation"
                type="button"
              >
                {isMobileMenuOpen ? <Icon name="X" size="l" /> : <Icon name="Menu" size="l" />}
              </button>
            </div>
          </div>

          {/* Main Navigation - Desktop */}
          <div className="hidden lg:flex items-center justify-between px-[var(--spacing-4)] py-[var(--spacing-2)]">
            <nav className="flex items-center gap-[var(--spacing-6)]" aria-label="Navigation principale">
              {mainNavLinks.map((link) => (
                <Link
                  key={link.to}
                  to={link.to}
                  className={`
                    flex items-center gap-[var(--spacing-1)] px-[var(--spacing-2)] py-[var(--spacing-1)] 
                    text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                    ${isActive(link.to) ? "text-brand-green" : "text-text-secondary hover:text-brand-green"}
                    transition-colors border-b-2 border-transparent hover:border-brand-green
                  `}
                >
                  <Icon name={link.icon} size="sm" />
                  {link.label}
                </Link>
              ))}
            </nav>

            <nav className="flex items-center gap-[var(--spacing-6)]" aria-label="Navigation secondaire">
              {otherNavLinks.map((link) => (
                <Link
                  key={link.to}
                  to={link.to}
                  className={`
                    text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                    ${isActive(link.to) ? "text-brand-green" : "text-text-secondary hover:text-brand-green"}
                    transition-colors
                  `}
                >
                  {link.label}
                </Link>
              ))}
            </nav>

            {/* All Sites Dropdown */}
            <div className="flex items-center">
              <Link
                to="/tous-les-sites"
                className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-text-secondary hover:text-brand-green transition-colors flex items-center gap-[var(--spacing-1)]"
              >
                <Icon name="Globe" size="sm" />
                Tous les sites CDSP
              </Link>
            </div>
          </div>
        </div>
      </header>

      {/* Mobile Navigation Menu */}
      <MobileNav isOpen={isMobileMenuOpen} onClose={closeMobileMenu} />
    </>
  );
}

export default Header;
