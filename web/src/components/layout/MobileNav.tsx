import React, { useState, useEffect } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Icon } from "../common/Icon";
import Button from "../common/Button";

export interface MobileNavProps {
  isOpen: boolean;
  onClose: () => void;
}

export function MobileNav({ isOpen, onClose }: MobileNavProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const { isAuthenticated, user, logout } = useAuth();

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
        className="mobile-nav-overlay active"
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Mobile Navigation Menu */}
      <nav
        className="mobile-nav active"
        role="navigation"
        aria-label="Navigation mobile"
      >
        {/* Close Button */}
        <button
          className="mobile-nav-close"
          onClick={onClose}
          aria-label="Fermer le menu"
          type="button"
        >
          &times;
        </button>

        {/* Navigation Links */}
        <ul role="menubar">
          {navLinks.map((link) => (
            <li key={link.to} role="none">
              <Link
                to={link.to}
                onClick={() => handleNavigation(link.to)}
                className={`mobile-nav-link ${isActive(link.to) ? "active" : ""}`}
                role="menuitem"
              >
                <span>{link.label}</span>
                {isActive(link.to) && (
                  <Icon name="ChevronRight" size="sm" className="mobile-nav-link-icon" />
                )}
              </Link>
            </li>
          ))}

          {/* Authentication Section */}
          {isAuthenticated ? (
            <>
              <li role="none">
                <div className="mobile-nav-divider" role="separator" />
              </li>
              <li role="none">
                <Link
                  to={`/joueurs/${user?.id}`}
                  onClick={onClose}
                  className="mobile-nav-link"
                  role="menuitem"
                >
                  <Icon name="User" size="sm" className="mobile-nav-link-icon left" />
                  <span>Mon profil</span>
                </Link>
              </li>
              <li role="none">
                <button
                  onClick={handleLogout}
                  className="mobile-nav-link mobile-nav-link-danger"
                  type="button"
                >
                  <Icon name="LogOut" size="sm" className="mobile-nav-link-icon left" />
                  <span>Déconnexion</span>
                </button>
              </li>
            </>
          ) : (
            <>
              <li role="none">
                <div className="mobile-nav-divider" role="separator" />
              </li>
              <li role="none">
                <Link
                  to="/connexion"
                  onClick={onClose}
                  className="mobile-nav-link"
                  role="menuitem"
                >
                  <Icon name="LogIn" size="sm" className="mobile-nav-link-icon left" />
                  <span>Connexion</span>
                </Link>
              </li>
              <li role="none">
                <Link
                  to="/inscription"
                  onClick={onClose}
                  className="mobile-nav-link"
                  role="menuitem"
                >
                  <Icon name="UserPlus" size="sm" className="mobile-nav-link-icon left" />
                  <span>Inscription</span>
                </Link>
              </li>
            </>
          )}
        </ul>

        {/* Bottom Section */}
        <div className="mobile-nav-bottom">
          <Link to="/" onClick={onClose} className="mobile-nav-logo" aria-label="Accueil">
            <div className="mobile-nav-logo-circle">
              <span>PC</span>
            </div>
            <span className="mobile-nav-logo-text">CDSP 79</span>
          </Link>
        </div>
      </nav>
    </>
  );
}

export default MobileNav;
