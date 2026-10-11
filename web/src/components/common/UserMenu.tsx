import { useState, useRef, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { Icon } from "./Icon";
import { User } from "../../types";

export interface UserMenuProps {
  user: User;
  onLogout?: () => void;
}

export function UserMenu({ user, onLogout }: UserMenuProps) {
  const navigate = useNavigate();
  const { logout } = useAuth();
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [isOpen]);

  // Close menu on escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        setIsOpen(false);
      }
    };
    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, []);

  const toggleMenu = () => setIsOpen(!isOpen);
  const closeMenu = () => setIsOpen(false);

  const handleLogout = () => {
    if (onLogout) {
      onLogout();
    } else {
      logout();
    }
    closeMenu();
    navigate("/");
  };

  // Check if user has admin role
  const isAdmin =
    user.role === "club_admin" || user.role === "commission_admin" || user.role === "super_admin";

  // Get initials
  const getInitials = () => {
    return `${user.firstName.charAt(0)}${user.lastName.charAt(0)}`.toUpperCase();
  };

  return (
    <div ref={menuRef} className="user-menu-container relative">
      {/* User Avatar Button */}
      <button
        onClick={toggleMenu}
        className="user-menu-button flex items-center justify-center w-10 h-10 lg:w-11 lg:h-11 rounded-full border-2 border-gold-700 bg-black text-gold-700 hover:bg-gold-900/20 transition-all duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-gold-500 focus-visible:ring-offset-2"
        aria-label="Menu utilisateur"
        aria-expanded={isOpen}
        type="button"
      >
        {user.photo ? (
          <img src={user.photo} alt="" className="w-full h-full rounded-full object-cover" />
        ) : (
          <div className="w-full h-full flex items-center justify-center text-[var(--text-heading-xs)] font-[var(--font-weight-black)]">
            <span>{getInitials()}</span>
          </div>
        )}
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div
          className="user-menu-dropdown absolute right-0 mt-2 w-56 bg-primary-800 rounded-lg shadow-xl border border-primary-700/50 z-[var(--z-popover)] animation-fade-in"
          role="menu"
          aria-label="Menu utilisateur"
        >
          {/* User Info Header */}
          <div className="px-4 py-3 border-b border-primary-700/50">
            <p className="text-body-sm font-semibold text-white truncate">
              {user.firstName} {user.lastName}
            </p>
            <p className="text-body-xs text-primary-300 truncate">{user.email}</p>
          </div>

          {/* Menu Items */}
          <div className="py-2" role="none">
            <Link
              to={`/joueurs/${user.id}`}
              onClick={closeMenu}
              className="user-menu-item flex items-center gap-3 px-4 py-2 text-body-sm text-primary-200 hover:bg-primary-700/50 hover:text-white transition-colors"
              role="menuitem"
            >
              <Icon name="User" size="sm" className="shrink-0" />
              <span>Mon profil</span>
            </Link>

            {isAdmin && (
              <Link
                to="/admin"
                onClick={closeMenu}
                className="user-menu-item flex items-center gap-3 px-4 py-2 text-body-sm text-primary-200 hover:bg-primary-700/50 hover:text-white transition-colors"
                role="menuitem"
              >
                <Icon name="Settings" size="sm" className="shrink-0" />
                <span>Administration</span>
              </Link>
            )}

            <div className="border-t border-primary-700/50 my-2" role="separator" />

            <button
              onClick={handleLogout}
              className="user-menu-item w-full flex items-center gap-3 px-4 py-2 text-body-sm text-danger-500 hover:bg-danger-600/20 hover:text-danger-400 transition-colors"
              type="button"
              role="menuitem"
            >
              <Icon name="LogOut" size="sm" className="shrink-0" />
              <span>Déconnexion</span>
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default UserMenu;
