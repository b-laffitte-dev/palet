# Icons - Palet Vendeen Design System

> Icon system for the Palet Vendeen application

---

## Icon Philosophy

The icon system follows these principles:
- **Simple**: Clean, recognizable shapes
- **Consistent**: Uniform stroke width (2px) and size (24px default)
- **Accessible**: Meets WCAG standards
- **Themed**: Matches the color palette
- **SVG-based**: Scalable and performant

---

## Icon Sizes

| Size | Usage | Pixel Value |
|------|-------|-------------|
| xs | Small UI elements | 16px |
| sm | Buttons, list items | 20px |
| md | Standard (default) | 24px |
| lg | Section headers | 28px |
| xl | Large features | 32px |
| 2xl | Hero sections | 40px |

---

## Icon Library

### 1. Navigation Icons

#### Home
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
  <polyline points="9,22 9,12 15,12 15,22"/>
</svg>
```
**Usage**: Championship, Tournaments, Clubs navigation

---

#### Championship (Trophy)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M19 11H5a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-8a2 2 0 0 0-2-2z"/>
  <path d="M12 11V5"/>
  <path d="M9 8l3-3 3 3"/>
  <path d="M12 5V3"/>
  <path d="M12 3h-2"/>
  <path d="M12 3h2"/>
</svg>
```
**Usage**: Championship page, navigation

---

#### Tournament
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14,2 14,8 20,8"/>
  <line x1="16" y1="13" x2="8" y2="13"/>
  <line x1="16" y1="17" x2="8" y2="17"/>
  <polyline points="10,9 9,9 8,9"/>
</svg>
```
**Usage**: Tournaments page, navigation

---

#### Club
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
  <circle cx="9" cy="7" r="4"/>
  <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
  <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
</svg>
```
**Usage**: Clubs page, navigation

---

#### Player
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <path d="M8 14s1.5 2 4 2 4-2 4-2"/>
  <line x1="9" y1="9" x2="9.01" y2="9"/>
  <line x1="15" y1="9" x2="15.01" y2="9"/>
</svg>
```
**Usage**: Players page, navigation

---

#### Federation
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
</svg>
```
**Usage**: Federation page, navigation

---

#### News
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14,2 14,8 20,8"/>
  <line x1="16" y1="13" x2="8" y2="13"/>
  <line x1="16" y1="17" x2="8" y2="17"/>
  <polyline points="10,9 9,9 8,9"/>
</svg>
```
**Usage**: Actualites page, navigation

---

### 2. Sport Icons

#### Palet (Game Piece)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
  <circle cx="12" cy="12" r="10"/>
  <circle cx="12" cy="12" r="6" fill="none" stroke="#0d3b2e" stroke-width="2"/>
  <circle cx="12" cy="12" r="3" fill="#0d3b2e"/>
</svg>
```
**Usage**: Logo, game-related elements

---

#### Palet Club (Team)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
</svg>
```
**Usage**: Team/club icons

---

#### Score
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="3" y="4" width="18" height="16" rx="2"/>
  <path d="M16 20V4"/>
  <path d="M8 4v16"/>
  <path d="M3 10h18"/>
</svg>
```
**Usage**: Score displays, results

---

#### Winner
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M22 11.08V12a10 10 0 1 1-5.93-5.93"/>
  <polyline points="22,4 12,14.01 9,11.01"/>
</svg>
```
**Usage**: Winners, champions

---

#### Medal
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <path d="M12 6v6l4 2"/>
</svg>
```
**Usage**: Awards, MVP, achievements

---

#### Stats
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M18 20V10"/>
  <path d="M12 20V4"/>
  <path d="M6 20v-6"/>
</svg>
```
**Usage**: Statistics, analytics

---

### 3. Date/Time Icons

#### Calendar
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
  <line x1="16" y1="2" x2="16" y2="6"/>
  <line x1="8" y1="2" x2="8" y2="6"/>
  <line x1="3" y1="10" x2="21" y2="10"/>
</svg>
```
**Usage**: Tournament dates, match schedules

---

#### Clock
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <polyline points="12,6 12,12 16,14"/>
</svg>
```
**Usage**: Match times, live updates

---

#### History
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <polyline points="12,6 12,12 16,14"/>
</svg>
```
**Usage**: Past matches, historical data

---

### 4. Location Icons

#### Location (Pin)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/>
  <circle cx="12" cy="10" r="3"/>
</svg>
```
**Usage**: Tournament locations, club addresses

---

#### Terrain
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="2" y="2" width="20" height="20" rx="2" ry="2"/>
  <line x1="2" y1="12" x2="22" y2="12"/>
  <line x1="12" y1="2" x2="12" y2="22"/>
</svg>
```
**Usage**: Playing fields, venues

---

### 5. Action Icons

#### Arrow Right
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="5" y1="12" x2="19" y2="12"/>
  <polyline points="12,5 19,12 12,19"/>
</svg>
```
**Usage**: Links, navigation

---

#### Arrow Left
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="19" y1="12" x2="5" y2="12"/>
  <polyline points="12,19 5,12 12,5"/>
</svg>
```
**Usage**: Back navigation, previous

---

#### Arrow Up
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="12" y1="19" x2="12" y2="5"/>
  <polyline points="5,12 12,5 19,12"/>
</svg>
```
**Usage**: Scroll to top, sort ascending

---

#### Arrow Down
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="12" y1="5" x2="12" y2="19"/>
  <polyline points="19,12 12,19 5,12"/>
</svg>
```
**Usage**: Scroll to bottom, sort descending

---

#### Plus
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="12" y1="5" x2="12" y2="19"/>
  <line x1="5" y1="12" x2="19" y2="12"/>
</svg>
```
**Usage**: Add new, create

---

#### Minus
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="5" y1="12" x2="19" y2="12"/>
</svg>
```
**Usage**: Remove, delete

---

#### Edit
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
  <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
</svg>
```
**Usage**: Edit player, club, tournament

---

#### Delete
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polyline points="3,6 5,6 21,6"/>
  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
</svg>
```
**Usage**: Delete, remove

---

#### Search
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="11" cy="11" r="8"/>
  <line x1="21" y1="21" x2="16.65" y2="16.65"/>
</svg>
```
**Usage**: Search players, clubs, tournaments

---

#### Filter
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polygon points="22,3 2,3 10,12.46 10,19 14,21 14,12.46 22,3"/>
</svg>
```
**Usage**: Filter tournaments, championships

---

#### Sort
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polyline points="8,22 16,22 20,16"/>
  <line x1="16" y1="12" x2="8" y2="12"/>
  <polyline points="8,6 16,6 20,10"/>
</svg>
```
**Usage**: Sort tables, lists

---

### 6. Social Icons

#### Share
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="18" cy="5" r="3"/>
  <circle cx="6" cy="12" r="3"/>
  <circle cx="18" cy="19" r="3"/>
  <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
  <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
</svg>
```
**Usage**: Share tournament, match

---

#### Facebook
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"/>
</svg>
```
**Usage**: Club social links

---

#### Twitter
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"/>
</svg>
```
**Usage**: Club social links

---

#### Instagram
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="2" y="2" width="20" height="20" rx="5" ry="5"/>
  <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"/>
  <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"/>
</svg>
```
**Usage**: Club social links

---

### 7. Notification Icons

#### Bell
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
  <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
</svg>
```
**Usage**: Notifications

---

#### Bell with Badge
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
  <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" stroke="currentColor" stroke-width="2"/>
  <path d="M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" stroke-width="2"/>
  <circle cx="16" cy="6" r="3" fill="var(--red-800)"/>
  <text x="16" y="8" text-anchor="middle" fill="white" font-size="8" font-weight="bold">3</text>
</svg>
```
**Usage**: Notifications with count

---

### 8. Status Icons

#### Live
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <circle cx="12" cy="12" r="3" fill="currentColor"/>
</svg>
```
**Usage**: Live matches, active status

---

#### Check (Success)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M20 6L9 17l-5-5"/>
</svg>
```
**Usage**: Success states, confirmed

---

#### X (Error)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <line x1="18" y1="6" x2="6" y2="18"/>
  <line x1="6" y1="6" x2="18" y2="18"/>
</svg>
```
**Usage**: Error states, closed

---

#### Warning
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
  <line x1="12" y1="9" x2="12" y2="13"/>
  <line x1="12" y1="17" x2="12.01" y2="17"/>
</svg>
```
**Usage**: Warning states, attention needed

---

### 9. Form Icons

#### User
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
  <circle cx="12" cy="7" r="4"/>
</svg>
```
**Usage**: User profile, login

---

#### Email
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
  <polyline points="22,6 12,13 2,6"/>
</svg>
```
**Usage**: Email input, contact

---

#### Phone
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"/>
</svg>
```
**Usage**: Phone input, contact

---

### 10. Data Icons

#### Chart
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M18 20V10"/>
  <path d="M12 20V4"/>
  <path d="M6 20v-6"/>
</svg>
```
**Usage**: Statistics, analytics

---

#### Trend Up
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polyline points="22,11 16,11 12,16 8,11 2,11"/>
  <line x1="12" y1="2" x2="12" y2="16"/>
</svg>
```
**Usage**: Positive trends

---

#### Trend Down
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polyline points="22,13 16,13 12,8 8,13 2,13"/>
  <line x1="12" y1="20" x2="12" y2="14"/>
</svg>
```
**Usage**: Negative trends

---

## Icon Usage Guidelines

### 1. Size Selection
- **24px (md)**: Default for most UI elements
- **20px (sm)**: Buttons, list items with text
- **16px (xs)**: Small UI elements, table cells
- **28px (lg)**: Section headers, empty states
- **32px (xl)**: Important features, hero sections
- **40px (2xl)**: Main branding, special features

### 2. Color Selection
- **Primary**: `#0d3b2e` (main actions, important icons)
- **Secondary**: `#7a8b84` (less important, subtle icons)
- **Accent**: `#f5d576` (live states, highlights)
- **Success**: `#2ecc71` (positive states)
- **Warning**: `#f39c12` (attention needed)
- **Danger**: `#e74c3c` (errors, negative states)
- **White**: `#ffffff` (on dark backgrounds)

### 3. Placement
- **Left of text**: Most common (buttons, menu items)
- **Right of text**: Less common (indicators, status)
- **Standalone**: When icon meaning is clear without text
- **In buttons**: Left of text, or standalone for icon buttons

### 4. Accessibility
- **Always add ARIA labels** for icon-only buttons:
  ```html
  <button aria-label="Search">
    <svg>...</svg>
  </button>
  ```
- **Ensure sufficient color contrast** (minimum 3:1 for icons)
- **Provide text alternatives** for screen readers

---

## Icon Component (React Example)

```jsx
// Icon.jsx
import React from 'react';

const Icon = ({ 
  name, 
  size = 'md', 
  color = 'currentColor',
  strokeWidth = 2,
  className = '',
  ...props 
}) => {
  const sizes = {
    xs: 16,
    sm: 20,
    md: 24,
    lg: 28,
    xl: 32,
    '2xl': 40,
  };

  const icons = {
    home: (
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={strokeWidth}>
        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
        <polyline points="9,22 9,12 15,12 15,22"/>
      </svg>
    ),
    championship: (
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={strokeWidth}>
        <path d="M19 11H5a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-8a2 2 0 0 0-2-2z"/>
        <path d="M12 11V5"/>
        <path d="M9 8l3-3 3 3"/>
      </svg>
    ),
    // ... other icons
  };

  return (
    <svg 
      width={sizes[size]} 
      height={sizes[size]} 
      fill="none" 
      color={color}
      className={`icon icon-${size} ${className}`}
      aria-hidden="true"
      {...props}
    >
      {icons[name]}
    </svg>
  );
};

export default Icon;
```

---

## Icon Library as SVG Sprite

For better performance, use an SVG sprite:

```html
<!-- In your HTML -->
<svg class="icon icon-md" fill="none" stroke="currentColor" stroke-width="2">
  <use href="/icons.svg#home" />
</svg>
```

---

## Custom Icons for Palet Vendeen

### Palet (Detailed)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
  <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
  <circle cx="12" cy="12" r="7" stroke="currentColor" stroke-width="2"/>
  <circle cx="12" cy="12" r="4" stroke="currentColor" stroke-width="2"/>
  <circle cx="12" cy="12" r="2" fill="currentColor"/>
</svg>
```

### Laiton Palet
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#f5d576" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <circle cx="12" cy="12" r="7"/>
  <circle cx="12" cy="12" r="4"/>
  <circle cx="12" cy="12" r="2" fill="#f5d576"/>
</svg>
```

### Fonte Palet
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#0d3b2e" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <circle cx="12" cy="12" r="7"/>
  <circle cx="12" cy="12" r="4"/>
  <circle cx="12" cy="12" r="2" fill="#0d3b2e"/>
</svg>
```

### Master (Small Palet)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none">
  <circle cx="12" cy="12" r="8" stroke="currentColor" stroke-width="2"/>
  <circle cx="12" cy="12" r="3" fill="currentColor"/>
</svg>
```

### Plaque (Board)
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="4" y="4" width="16" height="16" rx="2"/>
  <line x1="4" y1="12" x2="20" y2="12"/>
  <line x1="12" y1="4" x2="12" y2="20"/>
</svg>
```

---

## Icon Examples in Context

### Button with Icon
```jsx
<button className="button-secondary">
  <Icon name="calendar" size="sm" />
  <span>Calendrier</span>
</button>
```

### Navigation Item
```jsx
<NavItem to="/championnat">
  <Icon name="championship" size="md" />
  <span>Championnat</span>
</NavItem>
```

### Card Header
```jsx
<div className="card-tournament-header primary">
  <Icon name="tournament" size="lg" color="#f5d576" />
  <h3>Open de Lucon</h3>
</div>
```

### Live Status
```jsx
<div className="match-status">
  <Icon name="live" size="sm" color="#e74c3c" />
  <span>LIVE</span>
</div>
```

---

## Performance Tips

1. **Use SVG sprites** for better performance (single HTTP request)
2. **Inline critical icons** in HTML to reduce render-blocking
3. **Use CSS for simple icons** (like circles, squares) when possible
4. **Lazy load non-critical icons** for better initial load performance
5. **Optimize SVG paths** with tools like SVGO

---

## Accessibility Checklist

- [ ] All icons have appropriate ARIA labels or text alternatives
- [ ] Icon-only interactive elements have accessible names
- [ ] Sufficient color contrast (minimum 3:1)
- [ ] Icons are not the sole indicator of state (also use text/color)
- [ ] Decorative icons have `aria-hidden="true"`
- [ ] Interactive icons have proper focus states

---

## Icon Color Variants

### Based on Status
```css
.icon-success { color: var(--success); }
.icon-warning { color: var(--warning); }
.icon-danger { color: var(--danger); }
.icon-info { color: var(--info); }
```

### Based on Theme
```css
.icon-primary { color: var(--primary-700); }
.icon-secondary { color: var(--primary-200); }
.icon-accent { color: var(--gold-700); }
.icon-muted { color: var(--gray-500); }
```

---

## Summary

| Category | Icons | Count |
|----------|-------|-------|
| Navigation | home, championship, tournament, club, player, federation, news | 7 |
| Sport | palet, palet-club, score, winner, medal, stats | 6 |
| Date/Time | calendar, clock, history | 3 |
| Location | location, terrain | 2 |
| Action | arrow-*, plus, minus, edit, delete, search, filter, sort | 10 |
| Social | share, facebook, twitter, instagram | 4 |
| Notification | bell, bell-badge | 2 |
| Status | live, check, x, warning | 4 |
| Form | user, email, phone | 3 |
| Data | chart, trend-up, trend-down | 3 |
| Custom | palet, laiton, fonte, master, plaque | 5 |
| **Total** | | **52** |

---

*Document generated on 09/10/2026*
*Complete icon system for Palet Vendeen application*

---

**Next Steps:**
1. Create SVG sprite file with all icons
2. Implement Icon component in your framework
3. Add icons to design system Storybook
4. Test accessibility with screen readers

---

**Need more icons?**
Let me know which specific icons you need for your application, and I can add them to this library.
