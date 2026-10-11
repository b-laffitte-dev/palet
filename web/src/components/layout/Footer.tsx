import { Link } from "react-router-dom";
import { Icon } from "../common/Icon";

export interface FooterProps {
  className?: string;
}

export function Footer({ className = "" }: FooterProps) {
  const currentYear = new Date().getFullYear();

  const footerLinks = {
    platform: [
      { to: "/a-propos", label: "À propos" },
      { to: "/membres", label: "Membres CDSP 79" },
      { to: "/documents", label: "Documents" },
      { to: "/reglement", label: "Règlement" },
    ],
    legal: [
      { to: "/mentions-legales", label: "Mentions légales" },
      { to: "/politique-confidentialite", label: "Politique de confidentialité" },
      { to: "/cgu", label: "CGU" },
    ],
    support: [
      { to: "/contact", label: "Contact" },
      { to: "/faq", label: "FAQ" },
      { to: "/logiciel", label: "Logiciel Gestion Match" },
    ],
    social: [
      { to: "https://facebook.com/CDSP79", label: "Facebook", icon: "facebook" },
      { to: "https://twitter.com/CDSP79", label: "Twitter", icon: "twitter" },
      { to: "https://instagram.com/CDSP79", label: "Instagram", icon: "instagram" },
    ],
  };

  return (
    <footer className={`bg-[#000000] color-[var(--text-secondary)] pt-[var(--spacing-12)] pb-[var(--spacing-6)] border-t border-border-dark ${className}`}>
      <div className="container w-full max-w-[var(--container-max)] mx-auto px-[var(--container-padding)]">
        {/* Main Footer Content */}
        <div className="main-content display-grid grid-template-columns-[1fr_2fr] gap-[var(--spacing-12)] mb-[var(--spacing-10)]">
          {/* Logo and Description */}
          <div className="brand-section display-flex flex-direction-column gap-[var(--spacing-4)]">
            <Link to="/" className="logo-link inline-flex align-items-center gap-[var(--spacing-2)] color-[var(--color-primary-400)] no-underline font-size-[var(--text-heading-m)] font-[var(--font-weight-bold)]">
              <div className="w-10 h-10 rounded-full border-2 border-[var(--color-gold)] flex items-center justify-center bg-black">
                <span className="text-[var(--text-body-sm)] font-[var(--font-weight-black)] text-[var(--color-gold)]">
                  PC
                </span>
              </div>
              <span className="logo-text color-[var(--text-primary)]">CDSP 79</span>
            </Link>
            <p className="description font-size-[var(--text-body-sm)] line-height-[var(--line-height-relaxed)] color-[var(--color-neutral-400)] max-w-[300px]">
              Commission Deux-Sévrienne du Palet. Suivez le Championnat 79, la Coupe 79, et les
              résultats du palet fonte en temps réel.
            </p>
            <div className="social-links flex gap-[var(--spacing-3)]">
              {footerLinks.social.map((link) => (
                <a
                  key={link.to}
                  href={link.to}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="social-link color-[var(--color-neutral-400)] transition-colors hover:color-[var(--color-primary-400)]"
                  aria-label={link.label}
                >
                  <Icon name={link.icon as any} size="m" />
                </a>
              ))}
            </div>
          </div>

          {/* Navigation Sections */}
          <div className="nav-sections grid grid-cols-[repeat(3,1fr)] gap-[var(--spacing-6)]">
            <FooterNavSection title="Plateforme" links={footerLinks.platform} />
            <FooterNavSection title="Juridique" links={footerLinks.legal} />
            <FooterNavSection title="Support" links={footerLinks.support} />
          </div>
        </div>

        {/* Footer Bottom - Copyright */}
        <div className="bottom-bar flex items-center justify-between pt-[var(--spacing-6)] border-t border-border-secondary">
          <div className="copyright font-size-[var(--text-body-xs)] color-[var(--text-light)]">
            © {currentYear} CDSP 79 — Tous droits réservés
          </div>
          <div className="footer-meta flex items-center gap-[var(--spacing-2)] font-size-[var(--text-body-xs)] color-[var(--text-light)]">
            <span>CDSP 79 | FNSMR | CRSMR Nouvelle-Aquitaine</span>
            <span className="meta-separator color-[var(--text-tertiary)]">|</span>
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
    <div className="nav-section flex flex-col gap-[var(--spacing-3)]">
      <h3 className="nav-section-title font-size-[var(--text-body-sm)] font-[var(--font-weight-semibold)] color-[var(--text-primary)] uppercase letter-[var(--letter-spacing-wide)]">
        {title}
      </h3>
      <ul className="nav-section-list list-style-none m-0 p-0 flex flex-col gap-[var(--spacing-2)]">
        {links.map((link) => (
          <li key={link.to} className="nav-section-item">
            <Link
              to={link.to}
              className="nav-section-link font-size-[var(--text-body-sm)] color-[var(--text-tertiary)] no-underline transition-colors hover:color-[var(--color-primary-400)]"
            >
              {link.label}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Footer;
