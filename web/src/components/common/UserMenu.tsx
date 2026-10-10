import { useState, useRef, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Icon } from './Icon';
import { User } from '../../types';

export interface UserMenuProps {
  user: User;
  onLogout?: () => void;
}

export function UserMenu({ user, onLogout }: UserMenuProps) {
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
      document.addEventListener('mousedown', handleClickOutside);
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);

  const toggleMenu = () => setIsOpen(!isOpen);
  const closeMenu = () => setIsOpen(false);

  // Check if user has admin role
  const isAdmin = user.role === 'club_admin' || user.role === 'commission_admin' || user.role === 'super_admin';

  return (
    <div ref={menuRef} style={styles.container}>
      {/* User Avatar Button */}
      <button onClick={toggleMenu} style={styles.userButton} aria-label="Menu utilisateur">
        {user.photo ? (
          <img 
            src={user.photo} 
            alt="" 
            style={styles.avatar}
          />
        ) : (
          <div style={styles.avatarPlaceholder}>
            <span style={styles.avatarText}>{user.firstName.charAt(0)}{user.lastName.charAt(0)}</span>
          </div>
        )}
        <span style={styles.userName}>{user.firstName}</span>
        <Icon name="ChevronDown" size="xs" />
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div style={styles.dropdown} role="menu">
          <div style={styles.dropdownHeader}>
            <div style={styles.headerAvatar}>
              {user.photo ? (
                <img src={user.photo} alt="" style={styles.headerAvatarImg} />
              ) : (
                <div style={styles.headerAvatarPlaceholder}>
                  <span style={styles.avatarText}>{user.firstName.charAt(0)}{user.lastName.charAt(0)}</span>
                </div>
              )}
            </div>
            <div style={styles.headerInfo}>
              <div style={styles.headerName}>{user.firstName} {user.lastName}</div>
              <div style={styles.headerEmail}>{user.email}</div>
            </div>
          </div>

          <div style={styles.dropdownDivider} />

          <nav style={styles.dropdownNav}>
            <Link 
              to="/mon-profil" 
              style={styles.dropdownItem}
              onClick={closeMenu}
            >
              <Icon name="User" size="sm" />
              <span>Mon Profil</span>
            </Link>

            {user.clubId && (
              <Link 
                to="/mon-club" 
                style={styles.dropdownItem}
                onClick={closeMenu}
              >
                <Icon name="Home" size="sm" />
                <span>Mon Club</span>
              </Link>
            )}

            <Link 
              to="/mes-matchs" 
              style={styles.dropdownItem}
              onClick={closeMenu}
            >
              <Icon name="Target" size="sm" />
              <span>Mes Matchs</span>
            </Link>

            <Link 
              to="/mes-statistiques" 
              style={styles.dropdownItem}
              onClick={closeMenu}
            >
              <Icon name="BarChart3" size="sm" />
              <span>Mes Statistiques</span>
            </Link>

            <Link 
              to="/mes-notifications" 
              style={styles.dropdownItem}
              onClick={closeMenu}
            >
              <Icon name="Bell" size="sm" />
              <span>Mes Notifications</span>
            </Link>

            {isAdmin && (
              <>
                <div style={styles.dropdownDivider} />
                <div style={styles.dropdownSectionLabel}>Administration</div>
                
                {user.role === 'super_admin' && (
                  <Link 
                    to="/admin" 
                    style={styles.dropdownItem}
                    onClick={closeMenu}
                  >
                    <Icon name="Shield" size="sm" />
                    <span>Tableau de bord</span>
                  </Link>
                )}
                
                {(user.role === 'club_admin' || user.role === 'commission_admin' || user.role === 'super_admin') && (
                  <Link 
                    to="/admin/competitions" 
                    style={styles.dropdownItem}
                    onClick={closeMenu}
                  >
                    <Icon name="Trophy" size="sm" />
                    <span>Compétitions</span>
                  </Link>
                )}

                {user.role === 'super_admin' && (
                  <Link 
                    to="/admin/utilisateurs" 
                    style={styles.dropdownItem}
                    onClick={closeMenu}
                  >
                    <Icon name="Users" size="sm" />
                    <span>Utilisateurs</span>
                  </Link>
                )}
              </>
            )}

            <div style={styles.dropdownDivider} />

            <button 
              onClick={() => {
                closeMenu();
                onLogout?.();
              }}
              style={styles.dropdownItem}
            >
              <Icon name="LogOut" size="sm" />
              <span>Déconnexion</span>
            </button>
          </nav>
        </div>
      )}
    </div>
  );
}

const styles = {
  container: {
    position: 'relative',
  } as React.CSSProperties,
  
  userButton: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-2)',
    padding: 'var(--spacing-2) var(--spacing-3)',
    background: 'transparent',
    border: 'none',
    cursor: 'pointer',
    color: 'var(--color-neutral-50)',
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-medium)',
    borderRadius: 'var(--radius-full)',
    transition: 'background-color var(--transition-colors)',
    ':hover': {
      backgroundColor: 'rgba(255, 255, 255, 0.1)',
    },
  } as React.CSSProperties,
  
  avatar: {
    width: '32px',
    height: '32px',
    borderRadius: 'var(--radius-full)',
    objectFit: 'cover',
  } as React.CSSProperties,
  
  avatarPlaceholder: {
    width: '32px',
    height: '32px',
    borderRadius: 'var(--radius-full)',
    backgroundColor: 'var(--color-primary-600)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  } as React.CSSProperties,
  
  avatarText: {
    fontSize: 'var(--text-body-xs)',
    fontWeight: 'var(--font-weight-bold)',
    color: 'white',
    textTransform: 'uppercase',
  } as React.CSSProperties,
  
  userName: {
    color: 'var(--color-neutral-50)',
  } as React.CSSProperties,
  
  // Dropdown styles
  dropdown: {
    position: 'absolute',
    top: '100%',
    right: 0,
    marginTop: 'var(--spacing-2)',
    width: '240px',
    backgroundColor: 'white',
    borderRadius: 'var(--radius-l)',
    boxShadow: 'var(--shadow-xl)',
    border: '1px solid var(--border-primary)',
    zIndex: 'var(--z-dropdown)',
    animation: 'fadeIn var(--transition-normal)',
  } as React.CSSProperties,
  
  dropdownHeader: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-3)',
    padding: 'var(--spacing-3)',
    borderBottom: '1px solid var(--border-primary)',
  } as React.CSSProperties,
  
  headerAvatar: {
    width: '48px',
    height: '48px',
    borderRadius: 'var(--radius-full)',
    overflow: 'hidden',
  } as React.CSSProperties,
  
  headerAvatarImg: {
    width: '100%',
    height: '100%',
    objectFit: 'cover',
  } as React.CSSProperties,
  
  headerAvatarPlaceholder: {
    width: '100%',
    height: '100%',
    backgroundColor: 'var(--color-primary-100)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  } as React.CSSProperties,
  
  headerInfo: {
    flex: 1,
  } as React.CSSProperties,
  
  headerName: {
    fontSize: 'var(--text-body-m)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--text-primary)',
  } as React.CSSProperties,
  
  headerEmail: {
    fontSize: 'var(--text-body-xs)',
    color: 'var(--text-tertiary)',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
  } as React.CSSProperties,
  
  dropdownDivider: {
    height: '1px',
    backgroundColor: 'var(--border-primary)',
    margin: 'var(--spacing-2) 0',
  } as React.CSSProperties,
  
  dropdownNav: {
    padding: 'var(--spacing-2) 0',
  } as React.CSSProperties,
  
  dropdownSectionLabel: {
    fontSize: 'var(--text-body-xs)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'var(--text-tertiary)',
    textTransform: 'uppercase',
    letterSpacing: 'var(--letter-spacing-wide)',
    padding: 'var(--spacing-2) var(--spacing-3)',
  } as React.CSSProperties,
  
  dropdownItem: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-2)',
    padding: 'var(--spacing-2) var(--spacing-3)',
    background: 'transparent',
    border: 'none',
    cursor: 'pointer',
    color: 'var(--text-secondary)',
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-medium)',
    textAlign: 'left' as const,
    width: '100%',
    transition: 'background-color var(--transition-colors), color var(--transition-colors)',
    ':hover': {
      backgroundColor: 'var(--color-neutral-50)',
      color: 'var(--text-primary)',
    },
  } as React.CSSProperties,
};

export default UserMenu;