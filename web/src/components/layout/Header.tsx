import React, { useState, useEffect } from "react";
import { Link, useLocation } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Icon } from "../common/Icon";
import { MobileNav } from "./MobileNav";

export interface HeaderProps {
  showCTA?: boolean;
}

/**
 * Header - Main navigation and branding component
 * Displays logo, navigation links, and call-to-action button
 * Fully responsive with mobile hamburger menu
 */
export function Header({ showCTA = true }: HeaderProps) {
  const location = useLocation();
  const { isAuthenticated } = useAuth();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isScrolled, setIsScrolled] = useState(false);

  // Navigation links
  const navLinks = [
    { to: "/", label: "Accueil" },
    { to: "/championnats", label: "Championnats" },
    { to: "/coupe-79", label: "Coupe 79" },
    { to: "/challenges", label: "Challenges" },
    { to: "/clubs", label: "Clubs" },
    { to: "/joueurs", label: "Joueurs" },
    { to: "/documents", label: "Documents" },
    { to: "/actualites", label: "Actualités" },
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
        className={`h-[80px] bg-[#000000] border-b border-border-dark sticky top-[40px] z-[var(--z-fixed)] shadow-lg transition-all duration-200 ${
          isScrolled ? "bg-opacity-95 backdrop-blur-sm" : ""
        }`}
      >
        <div className="w-full max-w-[var(--container-max)] mx-auto px-[var(--container-padding)] flex items-center justify-between gap-[var(--spacing-4)] h-full">
          {/* Left Section - Logo and Tagline */}
          <div className="flex flex-col gap-[var(--spacing-1)] min-w-0">
            <Link to="/" className="text-decoration-none flex items-center gap-[var(--spacing-2)]">
              <div className="w-12 h-12 lg:w-14 lg:h-14 rounded-full border-4 border-[var(--color-gold)] flex items-center justify-center bg-black transition-all duration-200 hover:scale-105">
                <span className="text-[var(--text-heading-l)] font-[var(--font-weight-black)] text-[var(--color-gold)]">
                  PC
                </span>
              </div>
              <span className="text-[var(--text-heading-m)] lg:text-[var(--text-heading-l)] font-[var(--font-weight-bold)] tracking-[var(--letter-spacing-tight)] text-white truncate">
                CDSP 79
              </span>
            </Link>
            <div className="hidden lg:flex items-center gap-[var(--spacing-2)] text-[var(--text-body-xs)] font-[var(--font-weight-medium)] text-text-tertiary tracking-[var(--letter-spacing-wide)] uppercase">
              <span>COMMISSION DEUX-SÈVRIENNE DU PALET</span>
              <span className="text-gold-pure">—</span>
              <span>CHAMPIONNAT & COUPE 79</span>
            </div>
          </div>

          {/* Center Section - Navigation (Desktop) */}
          <nav className="hidden lg:flex flex-1 justify-center" aria-label="Navigation principale">
            <ul className="flex items-center gap-[var(--spacing-6)] list-none m-0 p-0">
              {navLinks.map((link) => (
                <li key={link.to} className="relative">
                  <Link
                    to={link.to}
                    className={`
                      flex items-center px-[var(--spacing-2)] py-[var(--spacing-1)] 
                      text-body-sm font-medium text-text-secondary 
                      border-b-2 border-transparent 
                      hover:text-gold-pure hover:border-gold-pure
                      transition-colors transition-border
                      ${isActive(link.to) ? "text-gold-pure border-gold-pure font-semibold" : ""}
                    `}
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </nav>

          {/* Mobile Navigation Toggle Button */}
          <button
            onClick={toggleMobileMenu}
            className="lg:hidden flex items-center justify-center p-[var(--spacing-2)] bg-transparent border-none cursor-pointer text-primary hover:text-gold-pure transition-colors rounded-[var(--radius-m)] hover:bg-primary-800/50"
            aria-label="Menu"
            aria-expanded={isMobileMenuOpen}
            aria-controls="mobile-navigation"
            type="button"
          >
            {isMobileMenuOpen ? <Icon name="X" size="l" /> : <Icon name="Menu" size="l" />}
          </button>

          {/* Right Section - CTA (Desktop) */}
          {showCTA && !isAuthenticated && (
            <div className="hidden lg:flex items-center">
              <Link
                to="/inscription"
                className="
                  flex items-center gap-[var(--spacing-2)] 
                  px-[var(--spacing-4)] py-[var(--spacing-2)] 
                  bg-yellow text-black 
                  text-body-sm font-semibold 
                  rounded-[var(--radius-m)] 
                  border-none cursor-pointer
                  tracking-[var(--letter-spacing-wide)]
                  hover:bg-[#ffe082] active:bg-[#ffb300]
                  transition-all duration-200
                  shadow-md hover:shadow-lg hover:-translate-y-px active:translate-y-0
                "
              >
                S'inscrire
              </Link>
            </div>
          )}

          {/* Mobile CTA Button */}
          {showCTA && !isAuthenticated && (
            <div className="lg:hidden flex items-center">
              <Link
                to="/inscription"
                className="
                  flex items-center gap-[var(--spacing-1)] 
                  px-[var(--spacing-3)] py-[var(--spacing-1.5)] 
                  bg-yellow text-black 
                  text-body-xs font-semibold 
                  rounded-[var(--radius-sm)] 
                  border-none cursor-pointer
                  tracking-[var(--letter-spacing-wide)]
                  hover:bg-[#ffe082] active:bg-[#ffb300]
                  transition-all duration-200
                  shadow-sm hover:shadow-md
                "
              >
                S'inscrire
              </Link>
            </div>
          )}
        </div>
      </header>

      {/* Mobile Navigation Menu */}
      <MobileNav isOpen={isMobileMenuOpen} onClose={closeMobileMenu} />
    </>
  );
}

export default Header;
