import { Link } from "react-router-dom";
import { useAuth } from "../../hooks/useAuth";
import { LiveIndicator } from "../common/LiveIndicator";
import { UserMenu } from "../common/UserMenu";

export interface TopBarProps {
  hasLiveMatches?: boolean;
  liveMatchesCount?: number;
}

export function TopBar({ hasLiveMatches = false, liveMatchesCount = 0 }: TopBarProps) {
  const { user, isAuthenticated } = useAuth();

  return (
    <div className="topbar h-[40px] lg:h-[40px] bg-[#000000] color-[var(--text-primary)] flex align-items-center sticky top-0 z-[var(--z-fixed)] shadow-[0_2px_4px_rgba(0,0,0,0.3)] border-b border-border-dark">
      <div className="container w-full max-w-[var(--container-max)] mx-auto px-[var(--container-padding)] py-0 flex items-center justify-between">
        <div className="flex items-center gap-[var(--spacing-4)] min-w-0">
          {/* Federation Name */}
          <Link
            to="/"
            className="federation-name text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] text-[var(--color-gold)] no-underline letter-[var(--letter-spacing-wide)] truncate"
          >
            CDSP 79 - Palet Fonte
          </Link>

          {/* Live Indicator */}
          {hasLiveMatches && (
            <div className="hidden md:flex">
              <LiveIndicator count={liveMatchesCount} />
            </div>
          )}
        </div>

        <div className="flex items-center gap-[var(--spacing-4)] auth-links">
          {isAuthenticated ? (
            <>
              {/* User Menu - Desktop */}
              <div className="hidden lg:flex">
                <UserMenu user={user!} />
              </div>

              {/* User Menu - Mobile */}
              <div className="lg:hidden">
                <UserMenu user={user!} />
              </div>
            </>
          ) : (
            <>
              {/* Live Text - Desktop */}
              <span className="hidden md:flex text-[var(--text-body-sm)] font-[var(--font-weight-semibold)] text-[var(--color-danger-500)] no-underline">
                EN DIRECT
              </span>

              {/* Authentication Links - Desktop */}
              <div className="hidden lg:flex items-center gap-[var(--spacing-2)]">
                <Link
                  to="/connexion"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-[var(--text-secondary)] no-underline hover:text-[var(--color-gold)] transition-colors"
                >
                  Connexion
                </Link>
                <span className="text-[var(--text-tertiary)] text-[var(--text-body-sm)]">|</span>
                <Link
                  to="/inscription"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-[var(--text-secondary)] no-underline hover:text-[var(--color-gold)] transition-colors"
                >
                  Mon espace
                </Link>
              </div>

              {/* Mobile Authentication Link */}
              <div className="lg:hidden">
                <Link
                  to="/connexion"
                  className="text-[var(--text-body-sm)] font-[var(--font-weight-medium)] text-[var(--text-secondary)] no-underline hover:text-[var(--color-gold)] transition-colors"
                >
                  Connexion
                </Link>
              </div>

              {/* CTA Button - Desktop */}
              <Link
                to="/inscription"
                className="hidden lg:flex items-center gap-[var(--spacing-2)] px-[var(--spacing-3)] py-[var(--spacing-1)] bg-[var(--accent-yellow)] text-black text-[var(--text-body-sm)] font-[var(--font-weight-bold)] rounded-[var(--radius-sm)] no-underline transition-all hover:bg-[#ffe082] active:bg-[#ffb300]"
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
