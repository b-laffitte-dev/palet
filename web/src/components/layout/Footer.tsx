import { Link } from 'react-router-dom';
import { Icon } from '../common/Icon';

export interface FooterProps {
  className?: string;
}

export function Footer({ className = '' }: FooterProps) {
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    platform: [
      { to: '/a-propos', label: 'À propos' },
      { to: '/equipe', label: 'L\'équipe' },
      { to: '/partenaires', label: 'Partenaires' },
      { to: '/nous-soutenir', label: 'Nous soutenir' },
    ],
    legal: [
      { to: '/mentions-legales', label: 'Mentions légales' },
      { to: '/politique-confidentialite', label: 'Politique de confidentialité' },
      { to: '/cgu', label: 'CGU' },
    ],
    support: [
      { to: '/contact', label: 'Contact' },
      { to: '/faq', label: 'FAQ' },
      { to: '/aide', label: 'Aide' },
    ],
    social: [
      { to: 'https://facebook.com/paletvendeen', label: 'Facebook', icon: 'facebook' },
      { to: 'https://twitter.com/paletvendeen', label: 'Twitter', icon: 'twitter' },
      { to: 'https://instagram.com/paletvendeen', label: 'Instagram', icon: 'instagram' },
    ],
  };

  return (
    <footer style={styles.footer} className={className}>
      <div className="container" style={styles.container}>
        {/* Main Footer Content */}
        <div style={styles.mainContent}>
          {/* Logo and Description */}
          <div style={styles.brandSection}>
            <Link to="/" style={styles.logoLink}>
              <Icon name="Target" size="l" />
              <span style={styles.logoText}>Palet Vendéen</span>
            </Link>
            <p style={styles.description}>
              Le site officiel de la Fédération du Palet Vendéen. Suivez les championnats, tournois,
              et résultats en temps réel.
            </p>
            <div style={styles.socialLinks}>
              {footerLinks.social.map((link) => (
                <a 
                  key={link.to} 
                  href={link.to} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  style={styles.socialLink}
                  aria-label={link.label}
                >
                  <Icon name={link.icon as any} size="m" />
                </a>
              ))}
            </div>
          </div>

          {/* Navigation Sections */}
          <div style={styles.navSections}>
            <FooterNavSection title="Plateforme" links={footerLinks.platform} />
            <FooterNavSection title="Juridique" links={footerLinks.legal} />
            <FooterNavSection title="Support" links={footerLinks.support} />
          </div>
        </div>

        {/* Footer Bottom - Copyright */}
        <div style={styles.bottomBar}>
          <div style={styles.copyright}>
            © {currentYear} Palet Vendéen — Tous droits réservés
          </div>
          <div style={styles.footerMeta}>
            <span>FNSMR</span>
            <span style={styles.metaSeparator}>|</span>
            <span>Version 1.0.0</span>
          </div>
        </div>
      </div>
    </footer>
  );
}

export interface FooterNavSectionProps {
  title: string;
  links: Array<{ to: string; label: string }>;
}

function FooterNavSection({ title, links }: FooterNavSectionProps) {
  return (
    <div style={styles.navSection}>
      <h3 style={styles.navSectionTitle}>{title}</h3>
      <ul style={styles.navSectionList}>
        {links.map((link) => (
          <li key={link.to} style={styles.navSectionItem}>
            <Link to={link.to} style={styles.navSectionLink}>
              {link.label}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}

const styles = {
  footer: {
    backgroundColor: 'var(--color-neutral-900)',
    color: 'var(--color-neutral-300)',
    padding: 'var(--spacing-12) 0 var(--spacing-6)',
  } as React.CSSProperties,
  
  container: {
    width: '100%',
    maxWidth: 'var(--container-max)',
    margin: '0 auto',
    padding: '0 var(--container-padding)',
  } as React.CSSProperties,
  
  mainContent: {
    display: 'grid',
    gridTemplateColumns: '1fr 2fr',
    gap: 'var(--spacing-12)',
    marginBottom: 'var(--spacing-10)',
  } as React.CSSProperties,
  
  brandSection: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: 'var(--spacing-4)',
  } as React.CSSProperties,
  
  logoLink: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: 'var(--spacing-2)',
    color: 'var(--color-primary-400)',
    textDecoration: 'none',
    fontSize: 'var(--text-heading-m)',
    fontWeight: 'var(--font-weight-bold)',
  } as React.CSSProperties,
  
  logoText: {
    color: 'white',
  } as React.CSSProperties,
  
  description: {
    fontSize: 'var(--text-body-sm)',
    lineHeight: 'var(--line-height-relaxed)',
    color: 'var(--color-neutral-400)',
    maxWidth: '300px',
  } as React.CSSProperties,
  
  socialLinks: {
    display: 'flex',
    gap: 'var(--spacing-3)',
  } as React.CSSProperties,
  
  socialLink: {
    color: 'var(--color-neutral-400)',
    transition: 'color var(--transition-colors)',
    ':hover': {
      color: 'var(--color-primary-400)',
    },
  } as React.CSSProperties,
  
  navSections: {
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: 'var(--spacing-6)',
  } as React.CSSProperties,
  
  navSection: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: 'var(--spacing-3)',
  } as React.CSSProperties,
  
  navSectionTitle: {
    fontSize: 'var(--text-body-sm)',
    fontWeight: 'var(--font-weight-semibold)',
    color: 'white',
    textTransform: 'uppercase' as const,
    letterSpacing: 'var(--letter-spacing-wide)',
  } as React.CSSProperties,
  
  navSectionList: {
    listStyle: 'none',
    margin: 0,
    padding: 0,
    display: 'flex',
    flexDirection: 'column' as const,
    gap: 'var(--spacing-2)',
  } as React.CSSProperties,
  
  navSectionItem: {
  } as React.CSSProperties,
  
  navSectionLink: {
    fontSize: 'var(--text-body-sm)',
    color: 'var(--color-neutral-400)',
    textDecoration: 'none',
    transition: 'color var(--transition-colors)',
    ':hover': {
      color: 'var(--color-primary-400)',
    },
  } as React.CSSProperties,
  
  bottomBar: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingTop: 'var(--spacing-6)',
    borderTop: '1px solid var(--border-dark)',
  } as React.CSSProperties,
  
  copyright: {
    fontSize: 'var(--text-body-xs)',
    color: 'var(--color-neutral-500)',
  } as React.CSSProperties,
  
  footerMeta: {
    display: 'flex',
    alignItems: 'center',
    gap: 'var(--spacing-2)',
    fontSize: 'var(--text-body-xs)',
    color: 'var(--color-neutral-500)',
  } as React.CSSProperties,
  
  metaSeparator: {
    color: 'var(--color-neutral-600)',
  } as React.CSSProperties,
};

// Responsive styles
const responsiveStyles = {
  '@media (max-width: 1024px)': {
    mainContent: {
      gridTemplateColumns: '1fr',
      gap: 'var(--spacing-8)',
    } as React.CSSProperties,
  },
  '@media (max-width: 768px)': {
    navSections: {
      gridTemplateColumns: 'repeat(2, 1fr)',
    } as React.CSSProperties,
  },
  '@media (max-width: 640px)': {
    mainContent: {
      gap: 'var(--spacing-6)',
    } as React.CSSProperties,
    navSections: {
      gridTemplateColumns: '1fr',
    } as React.CSSProperties,
    bottomBar: {
      flexDirection: 'column' as const,
      gap: 'var(--spacing-4)',
      textAlign: 'center' as const,
    } as React.CSSProperties,
  },
};

// Merge responsive styles
Object.assign(styles, responsiveStyles);

export default Footer;