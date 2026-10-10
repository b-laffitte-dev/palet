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
            Fédération de Palet Vendéen
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
              <span style={styles.liveText}>EN DIRECT</span>
              <Link to="/connexion" style={styles.authLink}>
                Connexion
              </Link>
              <span style={styles.separator}>|</span>
              <Link to="/inscription" style={styles.authLink}>
                Mon espace
              </Link>
              <Link to="/inscription" style={styles.ctaButton}>
                5s'inscrire
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
    backgroundColor: '#000000',
    color: 'var(--text-primary)',
    display: 'flex',
    alignItems: 'center',
    position: 'sticky',
    top: 0,
    zIndex: 'var(--z-fixed)',
    boxShadow: '0 2px 4px rgba(0, 0, 0, 0.3)',
    borderBottom: '1px solid var(--border-dark)',
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
    color: 'var(--color-gold)',
    textDecoration: 'none',
    letterSpacing: 'var(--letter-spacing-wide)',
  } as React.CSSProperties,
  
  liveText: {
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--color-danger-500)',
    textDecoration: 'none',
  } as React.CSSProperties,
  
  authLink: {
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-medium)',
    color: 'var(--text-secondary)',
    textDecoration: 'none',
    transition: 'color var(--transition-colors)',
    ':hover': {
      color: 'var(--color-gold)',
    },
  } as React.CSSProperties,
  
  separator: {
    color: 'var(--text-tertiary)',
    fontSize: 'var(--text-body-sm)',
  } as React.CSSProperties,
  
  ctaButton: {
    backgroundColor: 'var(--accent-yellow)',
    color: '#000000',
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-bold)',
    padding: 'var(--spacing-1) var(--spacing-3)',
    borderRadius: 'var(--radius-sm)',
    textDecoration: 'none',
    transition: 'all var(--transition-colors)',
    ':hover': {
      backgroundColor: '#ffe082',
    },
  } as React.CSSProperties,
};

export default TopBar;