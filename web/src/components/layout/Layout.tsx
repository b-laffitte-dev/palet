import { ReactNode, useState, useEffect } from 'react';
import { TopBar } from './TopBar';
import { Header } from './Header';
import { Footer } from './Footer';
import { api } from '../../api';

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
  const [hasLiveMatches, setHasLiveMatches] = useState(false);
  const [liveMatchesCount, setLiveMatchesCount] = useState(0);

  // Fetch live matches data for TopBar
  useEffect(() => {
    const fetchLiveMatches = async () => {
      try {
        const response = await api().GET('/matches/live');
        const liveMatches = (response.data as Array<any>) || [];
        setHasLiveMatches(liveMatches.length > 0);
        setLiveMatchesCount(liveMatches.length);
      } catch (error) {
        console.error('Error fetching live matches:', error);
        setHasLiveMatches(false);
        setLiveMatchesCount(0);
      }
    };

    // Initial fetch
    fetchLiveMatches();

    // Poll every 30 seconds for live matches
    const interval = setInterval(fetchLiveMatches, 30000);

    return () => clearInterval(interval);
  }, []);

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
  } as React.CSSProperties,
  
  main: {
    flex: 1,
    display: 'flex',
    flexDirection: 'column' as const,
  } as React.CSSProperties,
};

export default Layout;