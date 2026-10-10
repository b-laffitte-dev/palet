# Components - Palet Vendeen Design System

> UI Component library based on homepage.svg maquette and business requirements

---

## Component Architecture

```
docs/design-system/
├── colors.md          # Color palette
├── typography.md      # Type system
├── components.md       # This file - UI components
├── layouts.md         # Page layouts and grids
├── icons.md           # Icon system
└── README.md          # Design system overview
```

---

## Design Tokens

### Spacing System (8px base grid)
```css
:root {
  --space-xs: 0.25rem;  /* 4px */
  --space-sm: 0.5rem;   /* 8px */
  --space-md: 1rem;    /* 16px */
  --space-lg: 1.5rem;  /* 24px */
  --space-xl: 2rem;    /* 32px */
  --space-2xl: 3rem;   /* 48px */
  --space-3xl: 4rem;   /* 64px */
  --space-4xl: 6rem;   /* 96px */
}
```

### Border Radius
```css
:root {
  --radius-none: 0;
  --radius-sm: 0.25rem;   /* 4px */
  --radius-md: 0.375rem;  /* 6px */
  --radius-lg: 0.5rem;    /* 8px */
  --radius-xl: 0.75rem;   /* 12px */
  --radius-2xl: 1rem;     /* 16px */
  --radius-full: 9999px; /* Circle */
}
```

### Shadows
```css
:root {
  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
  --shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}
```

### Transitions
```css
:root {
  --transition-fast: 150ms ease;
  --transition-normal: 250ms ease;
  --transition-slow: 350ms ease;
}
```

---

## Base Components

---

### 1. Buttons

#### Button Primary (Gold CTA)
```css
.button-primary {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-xl);
  background-color: var(--gold-700);
  color: var(--primary-700);
  font-size: var(--font-size-body-s);
  font-weight: var(--font-weight-bold);
  line-height: 1;
  border: none;
  border-radius: var(--radius-xl);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.button-primary:hover {
  background-color: var(--gold-600);
}

.button-primary:active {
  background-color: var(--gold-800);
}

.button-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
```

**Usage:**
```html
<button class="button-primary">S'inscrire</button>
<button class="button-primary">
  <svg>...</svg> Suivre en direct
</button>
```

---

#### Button Secondary (Green)
```css
.button-secondary {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-xl);
  background-color: var(--primary-700);
  color: var(--gray-100);
  font-size: var(--font-size-body-s);
  font-weight: var(--font-weight-bold);
  line-height: 1;
  border: none;
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.button-secondary:hover {
  background-color: var(--primary-600);
}
```

**Usage:**
```html
<button class="button-secondary">Voir la poule</button>
```

---

#### Button Outline
```css
.button-outline {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-lg);
  background-color: transparent;
  color: var(--primary-700);
  font-size: var(--font-size-body-s);
  font-weight: var(--font-weight-bold);
  line-height: 1;
  border: 2px solid var(--primary-700);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.button-outline:hover {
  background-color: var(--primary-50);
}
```

---

#### Button Ghost
```css
.button-ghost {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background-color: transparent;
  color: var(--gold-700);
  font-size: var(--font-size-body-xs);
  font-weight: var(--font-weight-semi-bold);
  line-height: 1;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.button-ghost:hover {
  background-color: rgba(245, 213, 118, 0.1);
}
```

**Usage:**
```html
<a class="button-ghost">Tout le classement &rarr;</a>
```

---

### 2. Badges

#### Badge Status
```css
.badge {
  display: inline-flex;
  align-items: center;
  padding: var(--space-xs) var(--space-sm);
  font-size: var(--font-size-caption);
  font-weight: var(--font-weight-bold);
  line-height: 1.4;
  border-radius: var(--radius-full);
  text-transform: uppercase;
}

.badge-success {
  background-color: var(--success);
  color: var(--gray-100);
}

.badge-warning {
  background-color: var(--warning);
  color: var(--gray-100);
}

.badge-danger {
  background-color: var(--danger);
  color: var(--gray-100);
}

.badge-info {
  background-color: var(--info);
  color: var(--gray-100);
}

.badge-primary {
  background-color: var(--primary-700);
  color: var(--gray-100);
}

.badge-gold {
  background-color: var(--gold-700);
  color: var(--primary-700);
}
```

**Usage:**
```html
<span class="badge badge-success">Finale</span>
<span class="badge badge-warning">Barrage</span>
<span class="badge badge-danger">Descente D2</span>
<span class="badge badge-gold">MVP</span>
```

---

#### Badge Forme (Match History)
```css
.badge-form {
  display: inline-flex;
  gap: 2px;
}

.badge-form-item {
  width: 14px;
  height: 14px;
  border-radius: var(--radius-sm);
}

.badge-form-win { background-color: var(--success); }
.badge-form-loss { background-color: var(--danger); }
.badge-form-draw { background-color: var(--warning); }
```

**Usage:**
```html
<div class="badge-form">
  <span class="badge-form-item badge-form-win"></span>
  <span class="badge-form-item badge-form-win"></span>
  <span class="badge-form-item badge-form-loss"></span>
  <span class="badge-form-item badge-form-win"></span>
</div>
```

---

### 3. Cards

#### Card Base
```css
.card {
  background-color: var(--gray-100);
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-xl);
  padding: var(--space-lg);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-normal);
}

.card:hover {
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}
```

---

#### Card Tournament
```css
.card-tournament {
  width: 100%;
  max-width: 420px;
  background-color: var(--gray-100);
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.card-tournament-header {
  height: 90px;
  padding: var(--space-md) var(--space-lg);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.card-tournament-header.primary { background-color: var(--primary-700); color: var(--gray-100); }
.card-tournament-header.gold { background-color: var(--gold-900); color: var(--gray-100); }
.card-tournament-header.amber { background-color: var(--amber-900); color: var(--gray-100); }

.card-tournament-tag {
  padding: var(--space-xs) var(--space-sm);
  background-color: var(--gold-700);
  color: var(--primary-700);
  font-size: var(--font-size-body-xs);
  font-weight: var(--font-weight-bold);
  border-radius: var(--radius-md);
}

.card-tournament-body {
  padding: var(--space-lg);
}

.card-tournament-title {
  font-size: var(--font-size-heading-s);
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
  line-height: 1.2;
  margin-bottom: var(--space-md);
}

.card-tournament-detail {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: var(--font-size-body-s);
  color: var(--gray-500);
  line-height: 1.5;
  margin-bottom: var(--space-sm);
}

.card-tournament-detail:last-child {
  margin-bottom: 0;
}

.card-tournament-actions {
  margin-top: var(--space-lg);
}
```

**Usage:**
```html
<article class="card-tournament">
  <header class="card-tournament-header primary">
    <div>
      <span class="card-tournament-tag">OPEN NATIONAL</span>
      <h3 class="card-tournament-title">Open de Lucon</h3>
    </div>
  </header>
  <div class="card-tournament-body">
    <p class="card-tournament-detail">
      <svg>...</svg> Dimanche 6 juillet 2025
    </p>
    <p class="card-tournament-detail">
      <svg>...</svg> Terrain clos de Lucon (85)
    </p>
    <p class="card-tournament-detail">
      <svg>...</svg> 64 joueurs - 8 poules - elimination directe
    </p>
  </div>
  <div class="card-tournament-actions">
    <button class="button-secondary">S'inscrire en ligne</button>
  </div>
</article>
```

---

#### Card Player
```css
.card-player {
  width: 100%;
  max-width: 420px;
  background-color: var(--gray-100);
  border: 1px solid var(--gray-300);
  border-radius: var(--radius-xl);
  overflow: hidden;
  position: relative;
}

.card-player-accent {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 6px;
}

.card-player-accent.gold { background: linear-gradient(90deg, var(--gold-900), var(--gold-700)); }
.card-player-accent.primary { background-color: var(--primary-700); }
.card-player-accent.amber { background-color: var(--amber-900); }

.card-player-avatar {
  width: 84px;
  height: 84px;
  border-radius: var(--radius-full);
  background-color: var(--primary-700);
  color: var(--gray-100);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--font-size-heading-m);
  font-weight: var(--font-weight-bold);
  margin: var(--space-lg);
}

.card-player-content {
  padding: 0 var(--space-lg) var(--space-lg);
}

.card-player-name {
  font-size: var(--font-size-heading-s);
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
  line-height: 1.2;
  margin-bottom: var(--space-xs);
}

.card-player-club {
  font-size: var(--font-size-body-s);
  color: var(--gray-500);
  line-height: 1.4;
  margin-bottom: var(--space-sm);
}

.card-player-badge {
  display: inline-block;
  padding: var(--space-xs) var(--space-sm);
  background-color: var(--gold-500);
  color: var(--gold-900);
  font-size: var(--font-size-caption);
  font-weight: var(--font-weight-bold);
  border-radius: var(--radius-md);
  margin-bottom: var(--space-md);
}

.card-player-stats {
  list-style: none;
  padding: 0;
  margin: 0;
}

.card-player-stats li {
  display: flex;
  justify-content: space-between;
  padding: var(--space-xs) 0;
  font-size: var(--font-size-body-s);
  color: var(--gray-700);
  line-height: 1.5;
}

.card-player-stats li span:last-child {
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
}
```

**Usage:**
```html
<article class="card-player">
  <div class="card-player-accent gold"></div>
  <div class="card-player-avatar">JM</div>
  <div class="card-player-content">
    <h3 class="card-player-name">Jean Morice</h3>
    <p class="card-player-club">La Roche-sur-Yon - Div. 1</p>
    <span class="card-player-badge">MVP</span>
    <ul class="card-player-stats">
      <li><span>Points marques:</span> <span>412</span></li>
      <li><span>Moyenne / manche:</span> <span>11,4</span></li>
      <li><span>Lancers gagnants:</span> <span>68%</span></li>
    </ul>
  </div>
</article>
```

---

### 4. Match Display (Hero)

#### Match Hero
```css
.match-hero {
  position: relative;
  background: linear-gradient(135deg, var(--primary-700), var(--primary-600));
  color: var(--gray-100);
  padding: var(--space-4xl) var(--space-lg);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.match-hero::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.05);
}

.match-hero-title {
  font-size: var(--font-size-heading-l);
  font-weight: var(--font-weight-bold);
  color: var(--gold-700);
  text-align: center;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: var(--space-sm);
}

.match-hero-subtitle {
  font-size: var(--font-size-body-m);
  color: var(--primary-100);
  text-align: center;
  margin-bottom: var(--space-3xl);
}

.match-hero-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-4xl);
}

.match-team {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-md);
}

.match-team-logo {
  width: 160px;
  height: 160px;
  border: 4px solid var(--gold-700);
  border-radius: var(--radius-full);
  background-color: var(--gray-100);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-md);
}

.match-team-logo-abbr {
  font-size: var(--font-size-heading-xl);
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
  line-height: 1;
}

.match-team-logo-name {
  font-size: var(--font-size-body-m);
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
  text-align: center;
}

.match-team-name {
  font-size: var(--font-size-heading-xl);
  font-weight: var(--font-weight-bold);
  color: var(--gray-100);
  text-align: center;
}

.match-team-status {
  font-size: var(--font-size-body-s);
  color: var(--primary-200);
  text-align: center;
}

.match-score {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-sm);
}

.match-score-primary {
  font-size: var(--font-size-display-m);
  font-weight: var(--font-weight-bold);
  color: var(--gray-100);
  line-height: 1;
}

.match-score-divider {
  width: 2px;
  height: 40px;
  background-color: var(--gold-700);
}

.match-score-secondary {
  font-size: var(--font-size-display-m);
  font-weight: var(--font-weight-bold);
  color: var(--gold-700);
  line-height: 1;
}

.match-score-label {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  font-size: var(--font-size-body-s);
  font-weight: var(--font-weight-bold);
  color: var(--gray-100);
  text-transform: uppercase;
}

.match-score-label::before {
  content: '';
  width: 8px;
  height: 8px;
  background-color: var(--red-800);
  border-radius: var(--radius-full);
}

.match-info {
  font-size: var(--font-size-body-m);
  color: var(--gray-100);
  text-align: center;
  margin-top: var(--space-3xl);
  font-weight: var(--font-weight-semi-bold);
}
```

**Usage:**
```html
<div class="match-hero">
  <h1 class="match-hero-title">Finale du Championnat de Vendee - Division 1</h1>
  <p class="match-hero-subtitle">Samedi 14 juin 2025 - 15h00 - Terrain clos de La Roche-sur-Yon</p>
  <div class="match-hero-content">
    <div class="match-team">
      <div class="match-team-logo">
        <span class="match-team-logo-abbr">PC</span>
        <span class="match-team-logo-name">PALET CLUB</span>
      </div>
      <h2 class="match-team-name">LA ROCHE-SUR-YON</h2>
      <p class="match-team-status">Vainqueur poule A</p>
    </div>
    <div class="match-score">
      <span class="match-score-primary">72</span>
      <div class="match-score-divider"></div>
      <span class="match-score-secondary">68</span>
      <span class="match-score-label">LIVE - Manche 24</span>
    </div>
    <div class="match-team">
      <div class="match-team-logo">
        <span class="match-team-logo-abbr">FC</span>
        <span class="match-team-logo-name">FONTENAY</span>
      </div>
      <h2 class="match-team-name">CLOS FONTENOIS</h2>
      <p class="match-team-status">Vainqueur poule B</p>
    </div>
  </div>
  <p class="match-info">Terrain clos municipal - entree gratuite - buvette et fan zone</p>
</div>
```

---

### 5. Live Scores Ticker

```css
.live-scores {
  background-color: var(--primary-800);
  padding: var(--space-md) var(--space-lg);
  overflow-x: auto;
  overflow-y: hidden;
  white-space: nowrap;
}

.live-scores-header {
  font-size: var(--font-size-body-m);
  font-weight: var(--font-weight-bold);
  color: var(--gold-700);
  margin-bottom: var(--space-sm);
  text-transform: uppercase;
}

.live-scores-list {
  display: flex;
  gap: var(--space-md);
  list-style: none;
  padding: 0;
  margin: 0;
}

.live-score-item {
  flex-shrink: 0;
  background-color: var(--primary-900);
  border-radius: var(--radius-lg);
  padding: var(--space-sm) var(--space-md);
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.live-score-teams {
  display: flex;
  gap: var(--space-md);
  font-size: var(--font-size-body-m);
  color: var(--gray-100);
}

.live-score-score {
  font-size: var(--font-size-body-l);
  font-weight: var(--font-weight-bold);
  color: var(--gold-700);
}

.live-score-status {
  font-size: var(--font-size-caption);
  color: var(--primary-200);
}
```

**Usage:**
```html
<div class="live-scores">
  <h3 class="live-scores-header">Scores en direct - 12e journee</h3>
  <ul class="live-scores-list">
    <li class="live-score-item">
      <div class="live-score-teams">
        <span>Lacon PC</span>
        <span>Les Sables</span>
      </div>
      <span class="live-score-score">45 - 38</span>
      <span class="live-score-status">Manche 18 - en cours</span>
    </li>
    <li class="live-score-item">
      <div class="live-score-teams">
        <span>Challans</span>
        <span>Montaigu</span>
      </div>
      <span class="live-score-score">52 - 52</span>
      <span class="live-score-status">Manche 21 - en cours</span>
    </li>
  </ul>
</div>
```

---

### 6. Tables

#### Table Base
```css
.table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--font-size-body-m);
}

.table-header {
  background-color: transparent;
  text-align: left;
}

.table-header th {
  padding: var(--space-md) var(--space-sm);
  font-size: var(--font-size-body-xs);
  font-weight: var(--font-weight-semi-bold);
  color: var(--gray-500);
  text-transform: uppercase;
  letter-spacing: 0.02em;
  border-bottom: 1px solid var(--gray-300);
}

.table-row {
  border-bottom: 1px solid var(--gray-300);
}

.table-row:last-child {
  border-bottom: none;
}

.table-row:hover {
  background-color: var(--primary-50);
}

.table-row-qualified {
  background-color: var(--primary-50);
}

.table-row-barrage {
  background-color: var(--gold-500);
}

.table-row-relegation {
  background-color: rgba(231, 76, 60, 0.05);
}

.table-cell {
  padding: var(--space-md) var(--space-sm);
  vertical-align: middle;
}

.table-cell-position {
  width: 60px;
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
}

.table-cell-club {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.table-cell-club-logo {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  background-color: var(--primary-700);
  color: var(--gray-100);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--font-size-caption);
  font-weight: var(--font-weight-bold);
  flex-shrink: 0;
}

.table-cell-club-name {
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
}

.table-cell-stat {
  text-align: center;
  font-weight: var(--font-weight-medium);
  color: var(--gray-700);
}

.table-cell-pts {
  text-align: center;
  font-weight: var(--font-weight-bold);
  color: var(--primary-700);
}

.table-cell-form {
  display: flex;
  gap: 2px;
  justify-content: center;
}
```

**Usage:**
```html
<table class="table">
  <thead class="table-header">
    <tr>
      <th>POS</th>
      <th>CLUB</th>
      <th>MJ</th>
      <th>G</th>
      <th>P</th>
      <th>DIFF</th>
      <th>PTS</th>
      <th>FORME</th>
    </tr>
  </thead>
  <tbody>
    <tr class="table-row table-row-qualified">
      <td class="table-cell table-cell-position">1</td>
      <td class="table-cell table-cell-club">
        <span class="table-cell-club-logo">LR</span>
        <span class="table-cell-club-name">La Roche-sur-Yon Palet Club</span>
      </td>
      <td class="table-cell table-cell-stat">12</td>
      <td class="table-cell table-cell-stat">10</td>
      <td class="table-cell table-cell-stat">2</td>
      <td class="table-cell table-cell-stat">+84</td>
      <td class="table-cell table-cell-pts">30</td>
      <td class="table-cell table-cell-form">
        <span class="badge-form-item badge-form-win"></span>
        <span class="badge-form-item badge-form-win"></span>
        <span class="badge-form-item badge-form-win"></span>
        <span class="badge-form-item badge-form-loss"></span>
        <span class="badge-form-item badge-form-win"></span>
      </td>
    </tr>
  </tbody>
</table>
```

---

### 7. Form Elements

#### Input Text
```css
.input {
  width: 100%;
  padding: var(--space-sm) var(--space-md);
  font-size: var(--font-size-body-m);
  color: var(--gray-900);
  background-color: var(--gray-100);
  border: 2px solid var(--gray-300);
  border-radius: var(--radius-lg);
  transition: all var(--transition-normal);
}

.input:focus {
  outline: none;
  border-color: var(--primary-600);
  box-shadow: 0 0 0 3px rgba(26, 107, 76, 0.1);
}

.input::placeholder {
  color: var(--gray-500);
}
```

---

#### Select
```css
.select {
  width: 100%;
  padding: var(--space-sm) var(--space-md);
  font-size: var(--font-size-body-m);
  color: var(--gray-900);
  background-color: var(--gray-100);
  border: 2px solid var(--gray-300);
  border-radius: var(--radius-lg);
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%230d3b2e' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right var(--space-sm) center;
  transition: all var(--transition-normal);
}

.select:focus {
  outline: none;
  border-color: var(--primary-600);
  box-shadow: 0 0 0 3px rgba(26, 107, 76, 0.1);
}
```

---

#### Checkbox & Radio
```css
.checkbox,
.radio {
  width: 20px;
  height: 20px;
  border: 2px solid var(--gray-300);
  border-radius: var(--radius-sm);
  appearance: none;
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.checkbox:checked,
.radio:checked {
  border-color: var(--primary-700);
  background-color: var(--primary-700);
}

.checkbox:checked::after,
.radio:checked::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 6px;
  width: 5px;
  height: 10px;
  border: solid var(--gray-100);
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.checkbox:focus,
.radio:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(26, 107, 76, 0.1);
}
```

---

### 8. Navigation

#### Top Bar
```css
.top-bar {
  height: 40px;
  background-color: var(--primary-800);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-lg);
  font-size: var(--font-size-body-xs);
}

.top-bar-left {
  color: var(--primary-200);
}

.top-bar-center {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--gray-100);
}

.top-bar-center::before {
  content: '';
  width: 8px;
  height: 8px;
  background-color: var(--red-800);
  border-radius: var(--radius-full);
}

.top-bar-right {
  color: var(--primary-200);
}

.top-bar-right a {
  color: var(--primary-200);
  text-decoration: none;
}

.top-bar-right a:hover {
  color: var(--gold-700);
}
```

---

#### Main Navigation
```css
.nav-main {
  height: 80px;
  background-color: var(--primary-700);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-lg);
}

.nav-logo {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  text-decoration: none;
}

.nav-logo-icon {
  width: 52px;
  height: 52px;
  position: relative;
}

.nav-logo-icon-circle-1 {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: var(--radius-full);
  background: linear-gradient(180deg, var(--gold-700), var(--gold-800));
}

.nav-logo-icon-circle-2 {
  position: absolute;
  top: 10px;
  left: 10px;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  background-color: var(--primary-700);
}

.nav-logo-icon-circle-3 {
  position: absolute;
  top: 19px;
  left: 19px;
  width: 14px;
  height: 14px;
  border-radius: var(--radius-full);
  background: linear-gradient(180deg, var(--gold-700), var(--gold-800));
}

.nav-logo-text {
  color: var(--gold-700);
}

.nav-logo-title {
  font-size: var(--font-size-heading-m);
  font-weight: var(--font-weight-bold);
  line-height: 1;
}

.nav-logo-subtitle {
  font-size: var(--font-size-body-xs);
  color: var(--primary-200);
}

.nav-links {
  display: flex;
  align-items: center;
  gap: var(--space-2xl);
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-links a {
  color: var(--primary-100);
  font-size: var(--font-size-body-m);
  font-weight: var(--font-weight-semi-bold);
  text-decoration: none;
  padding: var(--space-sm) 0;
  transition: color var(--transition-normal);
}

.nav-links a:hover {
  color: var(--gold-700);
}

.nav-links a.active {
  color: var(--gold-700);
  text-decoration: underline;
}

.nav-actions {
  display: flex;
  gap: var(--space-md);
}
```

---

### 9. Footer

```css
.footer {
  background-color: var(--primary-900);
  padding: var(--space-lg) var(--space-lg);
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: var(--font-size-body-xs);
  color: var(--primary-200);
}

.footer-left,
.footer-right {
  display: flex;
  gap: var(--space-lg);
}

.footer a {
  color: var(--primary-200);
  text-decoration: none;
}

.footer a:hover {
  color: var(--gold-700);
}
```

---

### 10. Space Utilities (Tailwind-like)

```css
/* Padding */
.p-0 { padding: 0; }
.p-xs { padding: var(--space-xs); }
.p-sm { padding: var(--space-sm); }
.p-md { padding: var(--space-md); }
.p-lg { padding: var(--space-lg); }
.p-xl { padding: var(--space-xl); }
.p-2xl { padding: var(--space-2xl); }
.p-3xl { padding: var(--space-3xl); }

/* Padding Top */
.pt-0 { padding-top: 0; }
.pt-xs { padding-top: var(--space-xs); }
.pt-sm { padding-top: var(--space-sm); }
.pt-md { padding-top: var(--space-md); }
.pt-lg { padding-top: var(--space-lg); }

/* Padding Right */
.pr-0 { padding-right: 0; }
.pr-sm { padding-right: var(--space-sm); }
.pr-md { padding-right: var(--space-md); }
.pr-lg { padding-right: var(--space-lg); }

/* Padding Bottom */
.pb-0 { padding-bottom: 0; }
.pb-sm { padding-bottom: var(--space-sm); }
.pb-md { padding-bottom: var(--space-md); }
.pb-lg { padding-bottom: var(--space-lg); }

/* Padding Left */
.pl-0 { padding-left: 0; }
.pl-sm { padding-left: var(--space-sm); }
.pl-md { padding-left: var(--space-md); }
.pl-lg { padding-left: var(--space-lg); }

/* Padding X (horizontal) */
.px-0 { padding-left: 0; padding-right: 0; }
.px-sm { padding-left: var(--space-sm); padding-right: var(--space-sm); }
.px-md { padding-left: var(--space-md); padding-right: var(--space-md); }
.px-lg { padding-left: var(--space-lg); padding-right: var(--space-lg); }

/* Padding Y (vertical) */
.py-0 { padding-top: 0; padding-bottom: 0; }
.py-sm { padding-top: var(--space-sm); padding-bottom: var(--space-sm); }
.py-md { padding-top: var(--space-md); padding-bottom: var(--space-md); }

/* Margin */
.m-0 { margin: 0; }
.m-xs { margin: var(--space-xs); }
.m-sm { margin: var(--space-sm); }
.m-md { margin: var(--space-md); }
.m-lg { margin: var(--space-lg); }
.m-xl { margin: var(--space-xl); }

/* Margin Top */
.mt-0 { margin-top: 0; }
.mt-sm { margin-top: var(--space-sm); }
.mt-md { margin-top: var(--space-md); }
.mt-lg { margin-top: var(--space-lg); }
.mt-xl { margin-top: var(--space-xl); }

/* Margin Right */
.mr-0 { margin-right: 0; }
.mr-sm { margin-right: var(--space-sm); }
.mr-md { margin-right: var(--space-md); }

/* Margin Bottom */
.mb-0 { margin-bottom: 0; }
.mb-sm { margin-bottom: var(--space-sm); }
.mb-md { margin-bottom: var(--space-md); }
.mb-lg { margin-bottom: var(--space-lg); }

/* Margin Left */
.ml-0 { margin-left: 0; }
.ml-sm { margin-left: var(--space-sm); }
.ml-md { margin-left: var(--space-md); }

/* Gap */
.gap-0 { gap: 0; }
.gap-sm { gap: var(--space-sm); }
.gap-md { gap: var(--space-md); }
.gap-lg { gap: var(--space-lg); }
```

---

### 11. Responsive Utilities

```css
/* Breakpoints */
:root {
  --breakpoint-sm: 640px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 1024px;
  --breakpoint-xl: 1280px;
  --breakpoint-2xl: 1440px;
}

/* Responsive prefixes */
@media (min-width: 640px) {
  .sm-p-0 { padding: 0; }
  .sm-p-md { padding: var(--space-md); }
  /* ... other sm variants */
}

@media (min-width: 768px) {
  .md-p-0 { padding: 0; }
  .md-p-md { padding: var(--space-md); }
  /* ... other md variants */
}
```

---

## Component Summary

| Component | Variants | States | Notes |
|-----------|----------|--------|-------|
| Button | Primary, Secondary, Outline, Ghost | Default, Hover, Active, Disabled | All have icon support |
| Badge | Success, Warning, Danger, Info, Primary, Gold | Default | Form badges are separate |
| Card | Base, Tournament, Player | Default, Hover | Responsive |
| Match Display | Hero | Default | Special layout for live matches |
| Live Scores | Ticker | Default | Horizontal scroll |
| Table | Base | Default, Hover, Qualified, Barrage, Relegation | Sortable |
| Form | Input, Select, Checkbox, Radio | Default, Focus, Disabled | Accessible |
| Navigation | Top Bar, Main Nav, Footer | Default | Sticky options |
| Spacing | All directions | - | Tailwind-like |

---

## Implementation Notes

### For React (Styled Components)
```javascript
// Example: Button component
import styled from 'styled-components';

export const ButtonPrimary = styled.button`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: ${({ theme }) => theme.space.sm};
  padding: ${({ theme }) => theme.space.sm} ${({ theme }) => theme.space.xl};
  background-color: ${({ theme }) => theme.colors.gold[700]};
  color: ${({ theme }) => theme.colors.primary[700]};
  font-size: ${({ theme }) => theme.fontSize.bodyS};
  font-weight: ${({ theme }) => theme.fontWeight.bold};
  line-height: 1;
  border: none;
  border-radius: ${({ theme }) => theme.radius.xl};
  cursor: pointer;
  transition: all ${({ theme }) => theme.transition.normal};
  
  &:hover {
    background-color: ${({ theme }) => theme.colors.gold[600]};
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
`;
```

---

### For React (Tailwind CSS)
```javascript
// Example: Button component
const ButtonPrimary = ({ children, onClick, disabled }) => (
  <button
    onClick={onClick}
    disabled={disabled}
    className=`inline-flex items-center justify-center gap-2 px-4 py-2 
      bg-gold-700 text-primary-700 font-bold text-body-s 
      rounded-xl hover:bg-gold-600 active:bg-gold-800 
      transition-all duration-250 ease
      ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`
  >
    {children}
  </button>
);
```

---

## Accessibility Notes

1. **Color Contrast**: All color combinations meet AA or AAA standards
2. **Focus States**: All interactive elements have visible focus states
3. **Keyboard Navigation**: All components are keyboard accessible
4. **Screen Readers**: Use semantic HTML (button, nav, table, etc.)
5. **Reduced Motion**: Respect `prefers-reduced-motion` media query

---

## Performance Notes

1. **CSS Variables**: Use CSS custom properties for dynamic theming
2. **Minimal Overrides**: Components should require minimal CSS overrides
3. **Reusable**: Each component should be reusable in multiple contexts
4. **Composition**: Use composition over inheritance for complex components

---

*Document generated on 09/10/2026*
*Based on homepage.svg maquette and business requirements*
*Inspired by: Top 14, Ligue 1, ESPN, BBC Sport design systems*

---

**Next Steps:**
1. Create the **layouts.md** file with page layouts
2. Create the **icons.md** file with icon system
3. Create the **README.md** file to document the design system
4. Implement these components in your framework of choice

---

**Need specific implementations?**
Let me know which framework you are using (React, Vue, Svelte, etc.) and I can generate framework-specific component implementations.
