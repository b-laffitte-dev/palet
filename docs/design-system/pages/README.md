# Pages - Palet Vendeen Design System

> Interface specifications for all pages in the Palet Vendeen application

---

## Page Overview

This directory contains the interface specifications for all pages in the Palet Vendeen application, based on:
- The **homepage.svg** maquette
- The **business requirements** from `/docs/contexte-metier/`
- **Best practices** from sports platforms (Top 14, Ligue 1, ESPN)

---

## Page List

| Page | File | Purpose | Complexity |
|------|------|---------|------------|
| **Homepage** | [homepage.md](homepage.md) | Main landing page with live scores, standings, tournaments | ⭐⭐⭐⭐⭐ |
| **Championship** | [championnat.md](championnat.md) | Championship standings, results, statistics | ⭐⭐⭐⭐ |
| **Tournament** | [tournament.md](tournament.md) | Tournament details, participants, results | ⭐⭐⭐⭐ |
| **Club** | [club.md](club.md) | Club profile, team, players, results | ⭐⭐⭐ |
| **Player** | [player.md](player.md) | Player profile, statistics, career | ⭐⭐⭐ |
| **Admin** | [admin.md](admin.md) | Administration dashboard | ⭐⭐⭐ |
| **Match** | [match.md](match.md) | Live match details, statistics | ⭐⭐⭐⭐ |
| **Results** | [results.md](results.md) | Results entry and management | ⭐⭐ |
| **News** | [news.md](news.md) | News articles, announcements | ⭐⭐ |
| **Registration** | [registration.md](registration.md) | Tournament registration flow | ⭐⭐⭐ |
| **Login** | [login.md](login.md) | Authentication | ⭐⭐ |

---

## Page Architecture

### Common Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
│  - Federation name                                              │
│  - Live indicator                                              │
│  - User authentication                                         │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
│  - Logo and brand                                              │
│  - Main navigation                                             │
│  - Primary CTA (S'inscrire)                                    │
├─────────────────────────────────────────────────────────────┤
│ MAIN CONTENT                                                   │
│  - Page-specific content                                       │
│  - Varies by page                                              │
└─────────────────────────────────────────────────────────────┘
│ FOOTER (70px)                                                  │
│  - Links                                                       │
│  - Copyright                                                   │
└─────────────────────────────────────────────────────────────┘
```

---

### Page Templates by Type

#### 1. Public Pages (No Auth Required)
- Homepage
- Championship
- Tournament
- Club
- Player
- Match
- News
- Results (view only)

#### 2. User Pages (Auth Required)
- My Profile
- My Club
- My Matches
- My Statistics
- My Notifications

#### 3. Admin Pages (Admin Role Required)
- Admin Dashboard
- Tournament Management
- Club Management
- Player Management
- Match Management
- Results Entry
- User Management

---

## Navigation Structure

### Main Navigation (Header)
```
┌─────────────────────────────────────────────────────────────┐
│ PALET VENDÉEN                                          [S'INSCRIRE]│
│ LE SITE OFFICIEL — CHAMPIONNATS, CLUBS & TOURNOIS            │
├─────────────────────────────────────────────────────────────┤
│ Championnat | Tournois | Clubs | Joueurs | Fédération | Actualités │
└─────────────────────────────────────────────────────────────┘
```

### User Menu (When Logged In)
- Mon Profil
- Mon Club
- Mes Matchs
- Mes Statistiques
- Mes Notifications
- Déconnexion

### Admin Menu (When Admin)
- Dashboard
- Tournois
- Clubs
- Joueurs
- Résultats
- Utilisateurs
- Paramètres

---

## Page Specifications Status

| Page | Status | Last Updated | Priority |
|------|--------|--------------|----------|
| Homepage | ✅ Complete | 09/10/2026 | High |
| Championship | ✅ Complete | 09/10/2026 | High |
| Tournament | ✅ Complete | 09/10/2026 | High |
| Club | ✅ Complete | 09/10/2026 | Medium |
| Player | ✅ Complete | 09/10/2026 | Medium |
| Admin | ✅ Complete | 09/10/2026 | Medium |
| Match | ✅ Complete | 09/10/2026 | High |
| Results | ✅ Complete | 09/10/2026 | Medium |
| News | ✅ Complete | 09/10/2026 | Low |
| Registration | ✅ Complete | 09/10/2026 | High |
| Login | ✅ Complete | 09/10/2026 | Medium |

---

## User Flows

### 1. Visitor Flow (No Auth)
```
Homepage
├── View Live Match → Match Detail
├── View Classification → Championship Page
├── View Tournament → Tournament Detail
├── View Club → Club Page
├── View Player → Player Profile
└── Read News → News Article
```

### 2. Player Flow (Authenticated)
```
Homepage
├── Login → My Profile
│   ├── View My Stats
│   ├── View My Matches
│   ├── View My Club
│   └── Update Profile
├── Register for Tournament → Registration Flow
└── View Notifications
```

### 3. Club Admin Flow
```
Homepage
├── Login → My Club (Admin View)
│   ├── Manage Team
│   ├── View Club Stats
│   ├── Register for Tournaments
│   └── Communicate with Members
└── Manage Club Profile
```

### 4. Federation Admin Flow
```
Homepage
├── Login → Admin Dashboard
│   ├── Manage Championships
│   ├── Manage Tournaments
│   ├── Manage Clubs
│   ├── Manage Players
│   ├── Enter Results
│   └── Manage Users
└── View Reports
```

---

## Common Components Used Across Pages

### Layout Components
- **TopBar**: Consistent across all pages
- **Header**: Consistent across all pages
- **Footer**: Consistent across all pages
- **Section**: Container for content sections
- **Container**: Responsive width container

### UI Components
- **Button**: Primary, Secondary, Outline, Ghost
- **Card**: Base, Tournament, Player, Club, Match
- **Table**: Classification, Results, Statistics
- **Badge**: Status indicators
- **Tabs**: Navigation between views
- **Modal**: Overlay dialogs
- **Form**: Input, Select, Checkbox, Radio
- **Pagination**: Navigation through lists

### Data Display Components
- **MatchDisplay**: Live match with scores
- **ClassificationTable**: Championship standings
- **TournamentCard**: Tournament information
- **PlayerCard**: Player profile summary
- **ClubCard**: Club information
- **StatChart**: Data visualization

---

## Page-Specific Requirements

### Homepage
- **Primary Goal**: Showcase live content and key information
- **Key Features**: Live match, scores ticker, classification, tournaments, top players
- **Reference**: [homepage.md](homepage.md) - Complete specification

### Championship
- **Primary Goal**: Display championship information and standings
- **Key Features**: Classification table, results, next matches, statistics, calendar
- **Reference**: [championnat.md](championnat.md) - Complete specification

### Tournament
- **Primary Goal**: Show tournament details and allow registration
- **Key Features**: Tournament info, participants, schedule, results, registration
- **Status**: To be documented

### Club
- **Primary Goal**: Present club information and team
- **Key Features**: Club profile, team roster, results, statistics, contact
- **Status**: To be documented

### Player
- **Primary Goal**: Display player profile and statistics
- **Key Features**: Profile, career stats, achievements, match history
- **Status**: To be documented

### Admin
- **Primary Goal**: Manage all aspects of the application
- **Key Features**: Dashboard, management interfaces, reports
- **Status**: To be documented

### Match
- **Primary Goal**: Show detailed match information
- **Key Features**: Match details, live scores, statistics, timeline
- **Status**: To be documented

---

## Data Flow

### API Integration

All pages consume data from the backend API:

```
Frontend (React/Vite)
    ↓ (HTTP requests)
Backend (Hono/Node.js)
    ↓ (Database queries)
PostgreSQL Database
    ↑
Backend API (REST/GraphQL)
    ↑
Frontend State Management (TanStack Query)
```

### Data Fetching Patterns

1. **Server-Side Rendering (SSR)**: Initial page load data
2. **Client-Side Fetching**: Dynamic data updates
3. **Caching**: Reduce redundant API calls
4. **Optimistic Updates**: For user actions (likes, registrations)
5. **Real-time Updates**: WebSockets for live scores

---

## Accessibility Requirements

All pages must meet **WCAG 2.1 AA** standards:

### Keyboard Accessibility
- [ ] All interactive elements focusable
- [ ] Logical tab order
- [ ] Visible focus indicators
- [ ] Skip links for main content

### Screen Reader Support
- [ ] Semantic HTML
- [ ] ARIA labels where needed
- [ ] Text alternatives for images
- [ ] Form labels and instructions

### Color Contrast
- [ ] Minimum 4.5:1 for normal text
- [ ] Minimum 3:1 for large text
- [ ] No color-only information

### Motion
- [ ] Respect `prefers-reduced-motion`
- [ ] No auto-playing videos
- [ ] Control for animations

---

## Performance Requirements

All pages must meet **Core Web Vitals** standards:

### Largest Contentful Paint (LCP)
- **Target**: < 2.5 seconds
- **Optimizations**: 
  - Server-side rendering
  - Critical CSS inlined
  - Optimized images
  - Lazy loading for non-critical resources

### First Input Delay (FID)
- **Target**: < 100 milliseconds
- **Optimizations**:
  - Code splitting
  - Minimal JavaScript bundles
  - Efficient event handlers

### Cumulative Layout Shift (CLS)
- **Target**: < 0.1
- **Optimizations**:
  - Proper sizing for images and media
  - Reserved space for dynamic content
  - Font loading strategies

---

## Responsive Design Requirements

All pages must work well on:

| Breakpoint | Width | Requirements |
|------------|-------|--------------|
| Mobile Portrait | < 640px | Touch-friendly, readable, usable |
| Mobile Landscape | 640px - 767px | Optimized for landscape orientation |
| Tablet | 768px - 1023px | 2-column layouts, optimized |
| Desktop Small | 1024px - 1279px | Full desktop experience |
| Desktop | 1280px - 1439px | Matches maquette (1440px base) |
| Desktop Wide | 1440px+ | Enhanced spacing, additional content |

---

## SEO Requirements

All pages must have:

1. **Unique Page Title**: Descriptive and keyword-rich
2. **Meta Description**: Informative and compelling
3. **Canonical URL**: Prevent duplicate content
4. **Structured Data**: Schema.org markup
5. **Open Graph Tags**: Social sharing previews
6. **Sitemap**: Included in sitemap.xml
7. **Robots Meta**: Control search engine indexing

---

## Security Requirements

1. **Authentication**: Protected routes for user/admin pages
2. **Authorization**: Role-based access control
3. **CSRF Protection**: For form submissions
4. **Input Validation**: Client and server-side
5. **Rate Limiting**: Prevent abuse
6. **HTTPS**: All pages served over HTTPS
7. **CSP**: Content Security Policy headers

---

## Testing Requirements

### Functional Testing
- [ ] All links work correctly
- [ ] All forms submit properly
- [ ] All interactive elements respond
- [ ] Error states display correctly
- [ ] Loading states work properly

### Visual Testing
- [ ] Consistent with design system
- [ ] Responsive across breakpoints
- [ ] Cross-browser compatibility
- [ ] Accessibility verified

### Performance Testing
- [ ] Core Web Vitals met
- [ ] Load time acceptable
- [ ] Memory usage reasonable

### User Testing
- [ ] Usability testing with target users
- [ ] Feedback collected and addressed
- [ ] Iterative improvements made

---

## Implementation Guidelines

### 1. Start with the Homepage
- Implement the **TopBar**, **Header**, and **Footer** first
- Then implement the **MatchHero** component
- Add the **LiveScores** ticker
- Build the **ClassificationTable**
- Create the **TournamentCard** and **PlayerCard** components

### 2. Move to Championship Page
- Reuse components from homepage
- Add enhanced **ClassificationTable** with sorting
- Implement **Tabs** component
- Create **Results** and **Next Matches** views
- Add **Statistics** with charts

### 3. Build Tournament Pages
- Create **TournamentDetail** page
- Implement **Registration** flow
- Add **Participants** list
- Create **Schedule** view
- Build **Results** view

### 4. Create Club Pages
- Build **ClubProfile** page
- Implement **Team** view
- Create **Players** list
- Add **Results** view
- Build **Statistics** view

### 5. Develop Player Pages
- Create **PlayerProfile** page
- Implement **Stats** view
- Add **Career** view
- Create **Gallery** view
- Build **Achievements** section

### 6. Admin Interface
- Build **AdminDashboard**
- Create **Management** interfaces
- Implement **Results Entry**
- Add **Reports** section

---

## File Naming Conventions

### Page Components
- **File**: `pages/{PageName}.jsx` or `pages/{PageName}/index.jsx`
- **Example**: `pages/championnat/index.jsx`

### Sub-components
- **File**: `pages/{PageName}/{ComponentName}.jsx`
- **Example**: `pages/championnat/ClassificationTable.jsx`

### Shared Components
- **File**: `components/{ComponentName}.jsx`
- **Example**: `components/Button.jsx`

---

## Code Organization

```
web/
├── src/
│   ├── pages/
│   │   ├── homepage/
│   │   │   ├── index.jsx          # Homepage
│   │   │   ├── MatchHero.jsx      # Match hero component
│   │   │   ├── LiveScores.jsx     # Live scores ticker
│   │   │   ├── Classification.jsx  # Classification section
│   │   │   └── Tournaments.jsx     # Tournaments section
│   │   │
│   │   ├── championnat/
│   │   │   ├── index.jsx          # Championship page
│   │   │   ├── ClassificationTable.jsx
│   │   │   ├── ResultsTab.jsx
│   │   │   ├── NextMatchesTab.jsx
│   │   │   └── StatsTab.jsx
│   │   │
│   │   ├── tournament/
│   │   │   └── ...
│   │   │
│   │   └── ...
│   │
│   ├── components/
│   │   ├── common/
│   │   │   ├── Button.jsx
│   │   │   ├── Card.jsx
│   │   │   ├── Table.jsx
│   │   │   └── ...
│   │   │
│   │   ├── layout/
│   │   │   ├── TopBar.jsx
│   │   │   ├── Header.jsx
│   │   │   └── Footer.jsx
│   │   │
│   │   └── domain/
│   │       ├── Match.jsx
│   │       ├── Tournament.jsx
│   │       ├── Club.jsx
│   │       └── Player.jsx
│   │
│   ├── hooks/
│   │   ├── useChampionship.js
│   │   ├── useTournaments.js
│   │   └── ...
│   │
│   ├── services/
│   │   ├── api.js
│   │   ├── championship.js
│   │   └── ...
│   │
│   └── styles/
│       ├── global.css
│       ├── design-system.css
│       └── components.css
│
└── public/
    └── assets/
```

---

## Development Workflow

### 1. Design First
- Review the page specification in this directory
- Check the maquette in `/docs/maquettes/`
- Understand the business requirements in `/docs/contexte-metier/`

### 2. Component Development
- Build reusable components first
- Use the design system tokens (colors, typography, spacing)
- Test components in isolation (Storybook)

### 3. Page Assembly
- Assemble the page using the components
- Implement data fetching
- Add state management
- Handle user interactions

### 4. Testing
- Test functionality
- Verify responsiveness
- Check accessibility
- Validate performance

### 5. Review
- Design review (matches maquette)
- Code review (best practices)
- QA review (testing complete)

---

## Tools & Libraries

### Recommended Libraries

| Purpose | Library | Usage |
|---------|---------|-------|
| UI Components | @headlessui/react | Accessible headless components |
| Icons | lucide-react | Beautiful, simple icons |
| Charts | chart.js + react-chartjs-2 | Data visualization |
| Forms | react-hook-form + zod | Form validation |
| Table | @tanstack/react-table | Flexible tables |
| Calendar | react-datepicker | Date/calendar picker |
| Animations | framer-motion | Smooth animations |
| Styling | Tailwind CSS | Utility-first CSS |
| State Management | @tanstack/react-query | Server state management |

---

## Getting Started

1. **Read the homepage specification**: [homepage.md](homepage.md)
2. **Review the design system**: `/docs/design-system/README.md`
3. **Check the business context**: `/docs/contexte-metier/README_contexte_palet_vendeen.md`
4. **Start implementing**: Begin with the shared components (TopBar, Header, Footer)

---

## Contributing

To contribute to the page specifications:

1. **Review existing specs**: Understand the patterns and conventions
2. **Follow the template**: Use the same structure as existing pages
3. **Add details**: Include components, data requirements, and examples
4. **Add diagrams**: ASCII diagrams help visualize layouts
5. **Keep updated**: Update specs as requirements change

---

## Priority Order

**High Priority** (MVP):
1. ✅ Homepage
2. ✅ Championship
3. ✅ Tournament
4. ✅ Match
5. ✅ Registration

**Medium Priority** (v1.1):
1. ✅ Club
2. ✅ Player
3. ✅ Admin Dashboard

**Low Priority** (v1.2+):
1. ✅ News
2. ✅ Login
2. ⏳ Login
3. ⏳ User Profile

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-10-09 | Initial structure created, Homepage and Championship specs completed |
| 1.1 | 2026-10-09 | All page specifications completed (Tournament, Match, Registration, Club, Player, Admin, Results, News, Login) |

---

## Contacts

For questions about page specifications:
- **Design Questions**: Review the maquettes and design system
- **Business Questions**: Review the contexte-metier documentation
- **Technical Questions**: Review the architecture and stack technique docs

---

*Pages Documentation - Palet Vendeen Design System*
*Last Updated: 09/10/2026*
*Version: 1.0*

---

**Next Steps:**
1. ✅ All page specifications completed
2. Start implementing the shared components (TopBar, Header, Footer)
3. Begin with the Homepage implementation
4. Continue with Championship and Tournament pages

---

**Need Help?**
If you need more detailed specifications for any page, or if you need implementation examples for a specific framework, please let me know!
