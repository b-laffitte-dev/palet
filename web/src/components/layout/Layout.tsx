import { ReactNode } from "react";
import { TopBar } from "./TopBar";
import { Header } from "./Header";
import { Footer } from "./Footer";
import { StickyHeader } from "./StickyHeader";

export interface LayoutProps {
  children: ReactNode;
  showHeader?: boolean;
  showFooter?: boolean;
  showTopBar?: boolean;
  hasLiveMatches?: boolean;
  liveMatchesCount?: number;
}

/**
 * Layout - Main application layout with Top 14 inspired structure
 * Contains TopBar, Header (with navigation), StickyHeader (on scroll), and Footer
 */
export function Layout({
  children,
  showHeader = true,
  showFooter = true,
  showTopBar = true,
  hasLiveMatches = false,
  liveMatchesCount = 0,
}: LayoutProps) {
  return (
    <div className="layout flex flex-col min-h-[100vh] bg-[var(--bg-dark)]">
      {/* TopBar - Federation banner at very top */}
      {showTopBar && <TopBar hasLiveMatches={hasLiveMatches} liveMatchesCount={liveMatchesCount} />}

      {/* Header - Main navigation and branding */}
      {showHeader && (
        <Header
          showCTA={!hasLiveMatches}
          hasLiveMatches={hasLiveMatches}
          liveMatchesCount={liveMatchesCount}
        />
      )}

      {/* Sticky Header - Appears on scroll */}
      {showHeader && <StickyHeader />}

      {/* Main Content */}
      <main className="flex-1 flex flex-col bg-[var(--bg-dark)]">{children}</main>

      {/* Footer */}
      {showFooter && <Footer />}
    </div>
  );
}

export default Layout;
