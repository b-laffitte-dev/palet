import { Link } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';
import { LiveIndicator } from '../common/LiveIndicator';
import { UserMenu } from '../common/UserMenu';

export interface TopBarProps {
  hasLiveMatches?: boolean;
  liveMatchesCount?: number;
}

export function TopBar({ hasLiveMatches = false, liveMatchesCount = 0 }: TopBarProps) {
  const { user, isAuthenticated } = useAuth();

  return (
    <div className="topbar" style={styles.topbar}>
      <div className="container" style={styles.container}>
        <div style={styles.leftSection}>
          {/* Federation Name */}
          <Link to="/" style={styles.federationName}>
            Palet Vendéen
          </Link>
          
          {/* Live Indicator */}
          {hasLiveMatches && (
            <LiveIndicator count={liveMatchesCount} />
          )}
        </div>
        
        <div style={styles.rightSection}>
          {isAuthenticated ? (
            <>
              {/* User Menu */}
              <UserMenu user={user!} />
            </>
          ) : (
            <>
              {/* Authentication Links */}
              <Link to="/connexion" style={styles.authLink}>
                Se connecter
              </Link>
              <Link to="/inscription" style={styles.authLink}>
                S'inscrire
              </Link>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

const styles = {
  topbar: {
    height: '40px',
    backgroundColor: 'var(--color-neutral-900)',
    color: 'var(--color-neutral-50)',
    display: 'flex',
    alignItems: 'center',
    position: 'sticky',
    top: 0,
    zIndex: 'var(--z-fixed)',
    boxShadow: 'var(--shadow-sm)',
  } as React.CSSProperties,
  
  container: {
    width: '100%',
    maxWidth: 'var(--container-max)',
    margin: '0 auto',
    padding: '0 var(--container-padding)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
  } as React.CSSProperties,
  
  leftSection: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-4)',
  } as React.CSSProperties,
  
  rightSection: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-4)',
  } as React.CSSProperties,
  
  federationName: {
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--color-neutral-50)',
    textDecoration: 'none',
    letterSpacing: 'var(--letter-spacing-wide)',
  } as React.CSSProperties,
  
  authLink: {
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-medium)',
    color: 'var(--color-neutral-300)',
    textDecoration: 'none',
    transition: 'color var(--transition-colors)',
    ':hover': {
      color: 'var(--color-neutral-50)',
    },
  } as React.CSSProperties,
};

export default TopBar;