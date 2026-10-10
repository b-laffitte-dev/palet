# Layouts - Palet Vendeen Design System

> Page layouts, grids, and spacing for the Palet Vendeen application

---

## Layout Architecture

```
Root
├── TopBar (40px fixed)
├── Header (80px fixed)
├── Main Content (flexible)
│   ├── Hero (optional)
│   ├── Featured Content
│   ├── Main Content Grid
│   └── Sidebar (optional)
└── Footer (70px fixed)
```

---

## Breakpoints

```css
:root {
  --breakpoint-xs: 0px;
  --breakpoint-sm: 640px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 1024px;
  --breakpoint-xl: 1280px;
  --breakpoint-2xl: 1440px;
  --breakpoint-3xl: 1920px;
}
```

| Breakpoint | Min Width | Usage |
|------------|-----------|-------|
| xs | 0px | Mobile portrait |
| sm | 640px | Mobile landscape |
| md | 768px | Tablet |
| lg | 1024px | Desktop small |
| xl | 1280px | Desktop standard |
| 2xl | 1440px | Desktop wide (maquette base) |
| 3xl | 1920px | Desktop ultra-wide |

---

## Grid System

### 12-Column Grid

```css
:root {
  --grid-columns: 12;
  --gutter-width: 24px; /* var(--space-lg) */
  --max-width: 1440px;
}

.grid-container {
  width: 100%;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--gutter-width);
}

/* Responsive grid */
.grid {
  display: grid;
  grid-template-columns: repeat(var(--grid-columns), 1fr);
  gap: var(--gutter-width);
}

/* Column sizing */
.col-1 { grid-column: span 1; }
.col-2 { grid-column: span 2; }
.col-3 { grid-column: span 3; }
.col-4 { grid-column: span 4; }
.col-5 { grid-column: span 5; }
.col-6 { grid-column: span 6; }
.col-7 { grid-column: span 7; }
.col-8 { grid-column: span 8; }
.col-9 { grid-column: span 9; }
.col-10 { grid-column: span 10; }
.col-11 { grid-column: span 11; }
.col-12 { grid-column: span 12; }

/* Offset columns */
.offset-1 { grid-column-start: 2; }
.offset-2 { grid-column-start: 3; }
.offset-3 { grid-column-start: 4; }
/* ... */
```

---

## Page Layouts

### 1. Homepage Layout

**Desktop (1440px)**
```
┌─────────────────────────────────────────────┐
│ TOP BAR (40px)                                       │
├─────────────────────────────────────────────┤
│ HEADER (80px)                                        │
├─────────────────────────────────────────────┤
│ HERO - Match Display (520px)                       │
├─────────────────────────────────────────────┤
│ LIVE SCORES TICKER (130px)                          │
├─────────────────────────────────────────────┤
│ CLASSIFICATION TABLE (400px)                       │
├─────────────────────────────────────────────┤
│ TOURNAMENTS CARDS (290px each, 3 per row)          │
├─────────────────────────────────────────────┤
│ BEST PLAYERS CARDS (260px each, 3 per row)        │
├─────────────────────────────────────────────┤
│ SPACES SECTION (220px)                              │
├─────────────────────────────────────────────┤
│ FOOTER (70px)                                       │
└─────────────────────────────────────────────┘
```

**Mobile (< 768px)**
```
┌─────────────────────┐
│ TOP BAR (40px)       │
├─────────────────────┤
│ HEADER (60px)        │
├─────────────────────┤
│ HERO (simplified)    │
├─────────────────────┤
│ LIVE SCORES          │
├─────────────────────┤
│ CLASSIFICATION       │
├─────────────────────┤
│ TOURNAMENTS (stacked)│
├─────────────────────┤
│ PLAYERS (stacked)    │
├─────────────────────┤
│ FOOTER (70px)        │
└─────────────────────┘
```

---

### 2. Championship Page Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│ PAGE HEADER                                        │
│ ┌─────────────────────────────────────────┐│
│  │ Championship Title + Navigation          ││
│  └─────────────────────────────────────────┘│
├─────────────────────────────────────────────┤
│ FILTERS + SEARCH                                    │
├─────────────────────────────────────────────┤
│ TITLE + STATS                                      │
│  ┌──────────────┬─────────────────────────┐│
│  │ Classification │ Season Stats              ││
│  └──────────────┴─────────────────────────┘│
├─────────────────────────────────────────────┤
│ CLASSIFICATION TABLE                               │
├─────────────────────────────────────────────┤
│ MATCH RESULTS (2-3 columns)                        │
├─────────────────────────────────────────────┤
│ NEXT MATCHES (2-3 columns)                        │
└─────────────────────────────────────────────┘
```

---

### 3. Tournament Detail Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│ TOURNAMENT HEADER                                  │
│  ┌─────────────────────────────────────────┐│
│  │ Title, Date, Location, Status             ││
│  │ [Registration Button]                     ││
│  └─────────────────────────────────────────┘│
├─────────────────────────────────────────────┤
│ TOURNAMENT NAVIGATION                              │
│ [Overview | Teams | Matches | Results | Stats]  │
├─────────────────────────────────────────────┤
│ MAIN CONTENT                                       │
│ (varies by tab)                                    │
├─────────────────────────────────────────────┤
│ RELATED TOURNAMENTS                                │
└─────────────────────────────────────────────┘
```

---

### 4. Club Page Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│ CLUB HEADER                                        │
│  ┌─────────────────────────────────────────┐│
│  │ Logo | Name | Stats | Social Links       ││
│  └─────────────────────────────────────────┘│
├─────────────────────────────────────────────┤
│ CLUB NAVIGATION                                     │
│ [Overview | Team | Players | Results | Calendar] │
├─────────────────────────────────────────────┤
│ MAIN CONTENT                                       │
│  ┌──────────────────────┬──────────────────┐│
│  │ Team Overview          │ Recent Matches    ││
│  ├──────────────────────┼──────────────────┤│
│  │ Club History           │ Next Matches      ││
│  └──────────────────────┴──────────────────┘│
├─────────────────────────────────────────────┤
│ SPONSORS                                             │
└─────────────────────────────────────────────┘
```

---

### 5. Player Profile Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│ PLAYER HEADER                                       │
│  ┌─────────────────────────────────────────┐│
│  │ Avatar | Name | Badges | Social Links     ││
│  └─────────────────────────────────────────┘│
├─────────────────────────────────────────────┤
│ PLAYER NAVIGATION                                   │
│ [Overview | Stats | Career | Gallery]             │
├─────────────────────────────────────────────┤
│ MAIN CONTENT                                       │
│  ┌──────────────────────┬──────────────────┐│
│  │ Current Season Stats   │ Career Stats      ││
│  ├──────────────────────┼──────────────────┤│
│  │ Charts                │ Achievements      ││
│  └──────────────────────┴──────────────────┘│
├─────────────────────────────────────────────┤
│ RELATED PLAYERS                                    │
└─────────────────────────────────────────────┘
```

---

### 6. Match Detail Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│ MATCH HEADER (Similar to Hero)                      │
├─────────────────────────────────────────────┤
│ MATCH NAVIGATION                                    │
│ [Overview | Lineups | Stats | Timeline]            │
├─────────────────────────────────────────────┤
│ MAIN CONTENT                                       │
│  ┌─────────────────────────────────────────┐│
│  │ Match Summary                              ││
│  ├─────────────────────────────────────────┤│
│  │ Score Progression (Chart)                 ││
│  ├─────────────────────────────────────────┤│
│  │ Player Stats                               ││
│  └─────────────────────────────────────────┘│
├─────────────────────────────────────────────┤
│ NEXT MATCHES FOR TEAMS                            │
└─────────────────────────────────────────────┘
```

---

### 7. Admin Layout

```
┌─────────────────────────────────────────────┐
│ TOP BAR                                              │
├─────────────────────────────────────────────┤
│ HEADER                                              │
├─────────────────────────────────────────────┤
│  ┌──────────────┬─────────────────────────┐│
│  │ SIDEBAR      │ MAIN CONTENT             ││
│  │              │                          ││
│  │ Dashboard    │ Dashboard Overview       ││
│  │ Tournaments  │ Tournament List          ││
│  │ Clubs       │ Club Management          ││
│  │ Players     │ Player Management         ││
│  │ Matches     │ Match Management          ││
│  │ Results     │ Results Entry             ││
│  │ Settings    │ Application Settings      ││
│  │              │                          ││
│  └──────────────┴─────────────────────────┘│
└─────────────────────────────────────────────┘
```

---

## Grid Templates

### 1. Homepage Grid

```css
.homepage-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: var(--space-lg);
}

/* Hero Section */
.hero-section {
  grid-column: 1 / -1;
}

/* Live Scores */
.live-scores-section {
  grid-column: 1 / -1;
}

/* Classification */
.classification-section {
  grid-column: 1 / -1;
}

/* Tournaments Grid */
.tournaments-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-lg);
}

/* Players Grid */
.players-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-lg);
}
```

---

### 2. Championship Page Grid

```css
.championship-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: var(--space-lg);
}

/* Filters */
.filters-section {
  grid-column: 1 / -1;
}

/* Title and Stats */
.title-stats-section {
  grid-column: 1 / -1;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-xl);
}

/* Classification Table */
.classification-table-section {
  grid-column: 1 / -1;
}

/* Match Results Grid */
.match-results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--space-lg);
}
```

---

## Container Patterns

### Full Width Container
```css
.container-full {
  width: 100%;
  padding: 0 var(--space-lg);
}
```

### Max Width Container
```css
.container-max {
  width: 100%;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--space-lg);
}
```

### Section Container
```css
.section {
  padding: var(--space-3xl) 0;
}

.section-header {
  margin-bottom: var(--space-2xl);
}

.section-title {
  font-size: var(--font-size-heading-l);
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
  margin-bottom: var(--space-sm);
}

.section-subtitle {
  font-size: var(--font-size-body-m);
  color: var(--gray-500);
}

.section-actions {
  display: flex;
  gap: var(--space-md);
  margin-top: var(--space-lg);
}
```

---

## Spacing Patterns

### Section Spacing
```css
/* Between sections */
.section + .section {
  margin-top: var(--space-4xl);
}

/* Section with same background */
.section-same-bg + .section-same-bg {
  margin-top: var(--space-3xl);
}

/* Section with different background */
.section-diff-bg + .section-diff-bg {
  margin-top: 0;
}
```

### Card Spacing
```css
/* In a grid */
.card-grid {
  display: grid;
  gap: var(--space-lg);
}

/* In a list */
.card-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}
```

### Element Spacing
```css
/* Stack */
.stack {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.stack-sm { gap: var(--space-sm); }
.stack-lg { gap: var(--space-lg); }

/* Inline */
.inline {
  display: flex;
  gap: var(--space-md);
}

.inline-sm { gap: var(--space-sm); }
.inline-lg { gap: var(--space-lg); }
```

---

## Responsive Layout Examples

### Tournament Cards Grid

```css
.tournament-cards {
  display: grid;
  grid-template-columns: repeat(1, 1fr);
  gap: var(--space-md);
}

@media (min-width: 768px) {
  .tournament-cards {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-lg);
  }
}

@media (min-width: 1024px) {
  .tournament-cards {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (min-width: 1440px) {
  .tournament-cards {
    grid-template-columns: repeat(4, 1fr);
  }
}
```

---

### Classification Table

```css
.classification-table-wrapper {
  overflow-x: auto;
  -webkit-overflow-scrolling: touch;
}

.classification-table {
  min-width: 800px;
}

@media (min-width: 768px) {
  .classification-table {
    min-width: 100%;
  }
}
```

---

### Hero Section

```css
.hero-section {
  padding: var(--space-3xl) var(--space-lg);
  text-align: center;
}

.hero-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-xl);
}

@media (min-width: 1024px) {
  .hero-content {
    flex-direction: row;
    justify-content: center;
    gap: var(--space-4xl);
  }
}
```

---

## Z-Index Scale

```css
:root {
  --z-dropdown: 100;
  --z-sticky: 200;
  --z-fixed: 300;
  --z-modal-backdrop: 400;
  --z-modal: 500;
  --z-popover: 600;
  --z-tooltip: 700;
}
```

---

## Positioning Patterns

### Sticky Header
```css
.header-sticky {
  position: sticky;
  top: 0;
  z-index: var(--z-sticky);
  background-color: var(--primary-700);
}
```

### Fixed Top Bar
```css
.top-bar-fixed {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: var(--z-fixed);
}

/* Push content down */
.body-with-topbar {
  padding-top: 40px; /* height of top bar */
}
```

---

## Overlay Patterns

### Modal
```css
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: var(--z-modal-backdrop);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-lg);
}

.modal {
  background-color: var(--gray-100);
  border-radius: var(--radius-xl);
  max-width: 600px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  z-index: var(--z-modal);
  padding: var(--space-xl);
}
```

---

### Sidebar (Admin)
```css
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  width: 240px;
  background-color: var(--primary-800);
  color: var(--gray-100);
  z-index: var(--z-fixed);
  padding: var(--space-xl) 0;
  overflow-y: auto;
}

.main-with-sidebar {
  margin-left: 240px;
}

@media (max-width: 1023px) {
  .sidebar {
    transform: translateX(-100%);
    transition: transform var(--transition-normal);
  }
  
  .sidebar.open {
    transform: translateX(0);
  }
  
  .main-with-sidebar {
    margin-left: 0;
  }
}
```

---

## Animation Patterns

### Fade In
```css
.fade-in {
  animation: fadeIn var(--transition-normal) ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
```

### Slide Up
```css
.slide-up {
  animation: slideUp var(--transition-normal) ease-out;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

### Slide In (from left)
```css
.slide-in-left {
  animation: slideInLeft var(--transition-normal) ease-out;
}

@keyframes slideInLeft {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
```

---

## Print Styles

```css
@media print {
  .no-print {
    display: none !important;
  }
  
  .print-only {
    display: block !important;
  }
  
  body {
    background-color: white;
    color: black;
  }
  
  .top-bar,
  .header,
  .footer {
    display: none;
  }
  
  .section {
    page-break-inside: avoid;
  }
}
```

---

## Dark Mode Support

```css
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: var(--primary-800);
    --bg-secondary: var(--primary-900);
    --text-primary: var(--gray-100);
    --text-secondary: var(--gray-200);
    --border-primary: var(--primary-700);
  }
}
```

---

## Reduced Motion Support

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

---

## Utility Layout Classes

```css
/* Display */
.block { display: block; }
.inline-block { display: inline-block; }
.inline { display: inline; }
.flex { display: flex; }
.inline-flex { display: inline-flex; }
.grid { display: grid; }
.hidden { display: none; }

/* Flex Direction */
.flex-row { flex-direction: row; }
.flex-col { flex-direction: column; }
.flex-row-reverse { flex-direction: row-reverse; }
.flex-col-reverse { flex-direction: column-reverse; }

/* Flex Wrap */
.flex-wrap { flex-wrap: wrap; }
.flex-nowrap { flex-wrap: nowrap; }

/* Flex Justify */
.justify-start { justify-content: flex-start; }
.justify-center { justify-content: center; }
.justify-end { justify-content: flex-end; }
.justify-between { justify-content: space-between; }
.justify-around { justify-content: space-around; }
.justify-evenly { justify-content: space-evenly; }

/* Flex Align */
.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.items-end { align-items: flex-end; }
.items-stretch { align-items: stretch; }
.items-baseline { align-items: baseline; }

/* Flex Self */
.self-auto { align-self: auto; }
.self-start { align-self: flex-start; }
.self-center { align-self: center; }
.self-end { align-self: flex-end; }
.self-stretch { align-self: stretch; }
.self-baseline { align-self: baseline; }

/* Grid */
.grid-cols-1 { grid-template-columns: repeat(1, 1fr); }
.grid-cols-2 { grid-template-columns: repeat(2, 1fr); }
.grid-cols-3 { grid-template-columns: repeat(3, 1fr); }
.grid-cols-4 { grid-template-columns: repeat(4, 1fr); }
.grid-cols-5 { grid-template-columns: repeat(5, 1fr); }
.grid-cols-6 { grid-template-columns: repeat(6, 1fr); }
.grid-cols-12 { grid-template-columns: repeat(12, 1fr); }

/* Position */
.relative { position: relative; }
.absolute { position: absolute; }
.fixed { position: fixed; }
.sticky { position: sticky; }

/* Position Top/Right/Bottom/Left */
.top-0 { top: 0; }
.right-0 { right: 0; }
.bottom-0 { bottom: 0; }
.left-0 { left: 0; }
.top-full { top: 100%; }
.right-full { right: 100%; }
.bottom-full { bottom: 100%; }
.left-full { left: 100%; }

/* Overflow */
.overflow-visible { overflow: visible; }
.overflow-hidden { overflow: hidden; }
.overflow-auto { overflow: auto; }
.overflow-scroll { overflow: scroll; }

/* Text */
.text-left { text-align: left; }
.text-center { text-align: center; }
.text-right { text-align: right; }
.text-justify { text-align: justify; }

.text-lowercase { text-transform: lowercase; }
.text-uppercase { text-transform: uppercase; }
.text-capitalize { text-transform: capitalize; }

.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
```

---

## Summary

| Layout | Breakpoints | Grid | Notes |
|--------|-------------|------|-------|
| Homepage | All | 12-col | Hero, Live Scores, Cards |
| Championship | All | 12-col | Table, Filters, Stats |
| Tournament | md+ | Flexible | Tabs, Grid |
| Club | md+ | Flexible | Tabs, Sidebar |
| Player | md+ | Flexible | Tabs, Grid |
| Match | md+ | Flexible | Tabs, Timeline |
| Admin | lg+ | Sidebar | Fixed sidebar |

---

**Recommendations:**
1. Use **CSS Grid** for complex layouts (tables, card grids)
2. Use **Flexbox** for simpler layouts (navigation, cards)
3. **Mobile-first** approach for responsive design
4. **Consistent spacing** using the 8px grid
5. **Test on real devices** for touch targets and readability

---

*Document generated on 09/10/2026*
*Based on homepage.svg maquette and business requirements*

---

**Next Steps:**
1. Create **icons.md** for icon system
2. Create **README.md** for design system overview
3. Implement layouts in your framework
4. Test responsive behavior

---

**Need framework-specific implementations?**
Let me know if you need Tailwind, Styled Components, or other framework-specific layout implementations.
