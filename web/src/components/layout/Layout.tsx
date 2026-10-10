import { ReactNode } from 'react';
import { TopBar } from './TopBar';
import { Header } from './Header';
import { Footer } from './Footer';

export interface LayoutProps {
  children: ReactNode;
  showHeader?: boolean;
  showFooter?: boolean;
  showTopBar?: boolean;
}

export function Layout({
  children,
  showHeader = true,
  showFooter = true,
  showTopBar = true,
}: LayoutProps) {
  // Disable live matches API call until backend endpoint is available
  const hasLiveMatches = false;
  const liveMatchesCount = 0;

  return (
    <div style={styles.layout}>
      {/* TopBar */}
      {showTopBar && (
        <TopBar hasLiveMatches={hasLiveMatches} liveMatchesCount={liveMatchesCount} />
      )}

      {/* Header */}
      {showHeader && <Header showCTA={!hasLiveMatches} />}

      {/* Main Content */}
      <main style={styles.main}>
        {children}
      </main>

      {/* Footer */}
      {showFooter && <Footer />}
    </div>
  );
}

const styles = {
  layout: {
    display: 'flex',
    flexDirection: 'column' as const,
    minHeight: '100vh',
    backgroundColor: 'var(--bg-dark)',
  } as React.CSSProperties,
  
  main: {
    flex: 1,
    display: 'flex',
    flexDirection: 'column' as const,
    backgroundColor: 'var(--bg-dark)',
  } as React.CSSProperties,
};

export default Layout;