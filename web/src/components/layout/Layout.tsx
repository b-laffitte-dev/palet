import { ReactNode } from "react";
import { TopBar } from "./TopBar";
import { Header } from "./Header";
import { Footer } from "./Footer";

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
    <div className="layout flex flex-col min-h-[100vh] bg-[var(--bg-dark)]">
      {/* TopBar */}
      {showTopBar && <TopBar hasLiveMatches={hasLiveMatches} liveMatchesCount={liveMatchesCount} />}

      {/* Header */}
      {showHeader && <Header showCTA={!hasLiveMatches} />}

      {/* Main Content */}
      <main className="flex-1 flex flex-col bg-[var(--bg-dark)] px-[var(--spacing-4)] py-[var(--spacing-4)] sm:px-[var(--spacing-6)] sm:py-[var(--spacing-6)] lg:px-[var(--spacing-8)] lg:py-[var(--spacing-8)] transition-all duration-200">
        {children}
      </main>

      {/* Footer */}
      {showFooter && <Footer />}
    </div>
  );
}

export default Layout;
