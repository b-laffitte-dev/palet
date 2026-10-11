import { Link } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { LiveIndicator } from "../common/LiveIndicator";
import { Logo } from "../common/Logo";

export interface TopBarProps {
  hasLiveMatches?: boolean;
  liveMatchesCount?: number;
}

/**
 * TopBar - Top banner with federation name and live indicators
 * Similar to Top 14's top banner
 */
export function TopBar({ hasLiveMatches = false, liveMatchesCount = 0 }: TopBarProps) {
  const { user, isAuthenticated } = useAuth();

  return (
    <div
      className="topbar h-[40px] bg-[#000000] text-[var(--text-primary)] 
                     flex items-center sticky top-0 z-[var(--z-fixed)] 
                     shadow-[0_2px_4px_rgba(0,0,0,0.3)] border-b border-border-dark"
    >
      <div
        className="container w-full max-w-[var(--container-max)] mx-auto 
                   px-[var(--container-padding)] py-0 flex items-center justify-between"
      >
        {/* Left Section - Federation Name and Live */}
        <div className="flex items-center gap-[var(--spacing-4)] min-w-0">
          {/* Logo and Federation Name */}
          <Link
            to="/"
            className="federation-name flex items-center gap-[var(--spacing-2)] text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] 
                       text-brand-green no-underline letter-[var(--letter-spacing-wide)] 
                       truncate uppercase"
          >
            <Logo variant="icon" size="sm" />
            <span className="hidden sm:inline">COMMISSION DÉPARTEMENTALE PALET 79</span>
          </Link>

          {/* Live Indicator */}
          {hasLiveMatches && (
            <div className="hidden md:flex items-center">
              <LiveIndicator count={liveMatchesCount} />
            </div>
          )}
        </div>

        {/* Right Section - Auth and Social */}
        <div className="flex items-center gap-[var(--spacing-4)] auth-links">
          {isAuthenticated ? (
            <>
              {/* User Menu - Desktop */}
              <div className="hidden lg:flex items-center gap-[var(--spacing-2)]">
                <span className="text-[var(--text-body-xs)] text-text-tertiary">
                  Connecté
                </span>
                <Link
                  to="/mon-compte"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                             text-brand-green no-underline hover:text-brand-green-400 transition-colors"
                >
                  {user?.firstName}
                </Link>
              </div>
            </>
          ) : (
            <>
              {/* Live Text - Desktop */}
              {hasLiveMatches && (
                <span
                  className="hidden md:flex text-[var(--text-body-sm)] 
                             font-[var(--font-weight-semibold)] text-brand-orange 
                             no-underline uppercase"
                >
                  EN DIRECT
                </span>
              )}

              {/* Authentication Links - Desktop */}
              <div className="hidden lg:flex items-center gap-[var(--spacing-2)]">
                <Link
                  to="/connexion"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                             text-[var(--text-secondary)] no-underline 
                             hover:text-brand-green transition-colors"
                >
                  Connexion
                </Link>
                <span className="text-[var(--text-tertiary)] text-[var(--text-body-sm)]">|</span>
                <Link
                  to="/inscription"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                             text-[var(--text-secondary)] no-underline 
                             hover:text-brand-green transition-colors"
                >
                  Inscription
                </Link>
              </div>

              {/* Mobile Authentication Link */}
              <div className="lg:hidden">
                <Link
                  to="/connexion"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] 
                             text-[var(--text-secondary)] no-underline 
                             hover:text-brand-green transition-colors"
                >
                  Connexion
                </Link>
              </div>

              {/* CTA Button - Desktop */}
              <Link
                to="/inscription"
                className="hidden lg:flex items-center gap-[var(--spacing-2)] 
                           px-[var(--spacing-3)] py-[var(--spacing-1)] 
                           bg-brand-orange text-black 
                           text-[var(--text-body-sm)] font-[var(--font-weight-bold)] 
                           rounded-[var(--radius-sm)] no-underline 
                           transition-all hover:bg-brand-orange-400 active:bg-brand-orange-600"
              >
                S'inscrire
              </Link>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

export default TopBar;
