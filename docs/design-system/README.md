# Design System - Palet Vendeen

> Complete design system for the Palet Vendeen web application

---

## Overview

The **Palet Vendeen Design System** is a comprehensive collection of design guidelines, components, and patterns for building a consistent and cohesive user experience across the Palet Vendeen web application.

This design system is inspired by:
- The **Top 14 Rugby** platform (reference from the project brief)
- Modern sports platforms (ESPN, BBC Sport, Ligue 1)
- The **homepage.svg** maquette provided in the project
- The **business requirements** documented in `/docs/contexte-metier/`

---

## Design Principles

### 1. Tradition Meets Modernity
- **Respect the heritage** of palet vendeen (centuries-old tradition)
- **Modern digital experience** for today's users
- **Balance** between classic and contemporary

### 2. Sport-Centric Design
- **Hierarchy** that highlights scores, rankings, and live action
- **Clarity** in presenting complex sports data
- **Excitement** through dynamic visual elements

### 3. User-Focused
- **Intuitive navigation** for all user types (players, clubs, fans, federations)
- **Accessible** to everyone, including users with disabilities
- **Responsive** across all devices and screen sizes

### 4. Performance First
- **Fast loading** for users on all connection speeds
- **Optimized assets** (SVGs, fonts, images)
- **Efficient rendering** with modern CSS techniques

---

## Directory Structure

```
docs/design-system/
├── README.md              # This file - Design system overview
├── colors.md              # Color palette and usage guidelines
├── typography.md          # Typography system and scale
├── components.md          # UI component library
├── layouts.md             # Page layouts and grids
├── icons.md               # Icon system and library
└── pages/                 # Page-specific UI (coming soon)
    ├── homepage.md
    ├── championship.md
    ├── tournament.md
    ├── club.md
    ├── player.md
    └── admin.md
```

---

## Quick Start

### For Developers

1. **Read the colors.md** to understand the palette
2. **Read the typography.md** to understand text styles
3. **Browse components.md** for reusable UI elements
4. **Check layouts.md** for page structure guidance
5. **Use icons.md** for the icon library

### For Designers

1. **Start with colors.md** - all color values and usage
2. **Check typography.md** - type scale and hierarchy
3. **Reference homepage.svg** in `/docs/maquettes/` for visual inspiration
4. **Review layouts.md** for grid and spacing systems

---

## Color System

### Primary Colors
| Color | Hex | Usage |
|-------|-----|-------|
| Primary 700 | #0d3b2e | Main brand color, headers, text |
| Primary 600 | #1a6b4c | Secondary brand color, accents |
| Primary 200 | #9fc4b5 | Light accents, hover states |

### Accent Colors
| Color | Hex | Usage |
|-------|-----|-------|
| Gold 700 | #f5d576 | Primary CTA, live indicators |
| Gold 800 | #d4a017 | Gradient stop, depth |

### Status Colors
| Color | Hex | Usage |
|-------|-----|-------|
| Success | #2ecc71 | Positive states, confirmations |
| Warning | #f39c12 | Attention, neutral states |
| Danger | #e74c3c | Errors, negative states |

**Full palette:** See [colors.md](./colors.md) for complete color system

---

## Typography System

### Font Stack
```
Primary: "Segoe UI", -apple-system, BlinkMacSystemFont, "Roboto", sans-serif
Fallback: Arial, Helvetica, sans-serif
Web Alternative: "Inter" (recommended for better cross-platform consistency)
```

### Type Scale
| Name | Size | Weight | Line Height | Usage |
|------|------|--------|-------------|-------|
| Display XL | 4.5rem | 700 | 1.1 | Hero titles |
| Display L | 3rem | 700 | 1.2 | Section titles |
| Heading L | 1.75rem | 700 | 1.2 | Card titles |
| Body L | 1rem | 400 | 1.6 | Body text |
| Caption | 0.75rem | 400 | 1.4 | Small text |

**Full typography:** See [typography.md](./typography.md) for complete type system

---

## Component Library

### Base Components
| Component | Variants | Description |
|-----------|----------|-------------|
| Button | Primary, Secondary, Outline, Ghost | Call-to-action elements |
| Badge | Success, Warning, Danger, Info, Primary, Gold | Status indicators |
| Card | Base, Tournament, Player, Club | Content containers |
| Table | Base, Classification, Results | Data tables |
| Input | Text, Select, Checkbox, Radio | Form elements |

### Specialized Components
| Component | Description |
|-----------|-------------|
| Match Hero | Live match display (like Top 14) |
| Live Scores | Horizontal scrolling score ticker |
| Form Badge | Match history indicator |
| Navigation | Top bar, main nav, footer |
| Logo | Brand identity with concentric circles |

**Full components:** See [components.md](./components.md) for complete library

---

## Layout System

### Breakpoints
| Name | Width | Usage |
|------|-------|-------|
| xs | 0px | Mobile portrait |
| sm | 640px | Mobile landscape |
| md | 768px | Tablet |
| lg | 1024px | Desktop small |
| xl | 1280px | Desktop standard |
| 2xl | 1440px | Desktop wide (maquette base) |

### Grid System
- **12-column grid** with 24px gutters
- **Max width**: 1440px (matches maquette)
- **Responsive**: Mobile-first approach

### Spacing System
- **Base unit**: 8px
- **Scale**: 4px, 8px, 16px, 24px, 32px, 48px, 64px, 96px
- **Named**: xs, sm, md, lg, xl, 2xl, 3xl, 4xl

**Full layouts:** See [layouts.md](./layouts.md) for complete layout system

---

## Icon System

### Library
- **52 icons** covering all use cases
- **24px default** size
- **2px stroke** width for consistency
- **SVG-based** for scalability

### Categories
- Navigation (7 icons)
- Sport-specific (6 icons)
- Date/Time (3 icons)
- Location (2 icons)
- Action (10 icons)
- Social (4 icons)
- Notification (2 icons)
- Status (4 icons)
- Form (3 icons)
- Data (3 icons)
- Custom Palet Vendeen (5 icons)

**Full icon system:** See [icons.md](./icons.md) for complete icon library

---

## Implementation Guide

### CSS Variables

All design tokens are available as CSS custom properties:

```css
:root {
  /* Colors */
  --primary-700: #0d3b2e;
  --gold-700: #f5d576;
  --gray-100: #ffffff;
  
  /* Typography */
  --font-primary: "Segoe UI", -apple-system, ...;
  --font-size-display-xl: 4.5rem;
  --font-weight-bold: 700;
  
  /* Spacing */
  --space-sm: 0.5rem;
  --space-md: 1rem;
  
  /* Border Radius */
  --radius-lg: 0.5rem;
  --radius-xl: 0.75rem;
  
  /* Shadows */
  --shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
  --shadow-md: 0 4px 6px rgba(0,0,0,0.1);
}
```

### Tailwind Configuration

For projects using Tailwind CSS, use this configuration:

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eaf4ee',
          100: '#cfe8dc',
          200: '#9fc4b5',
          300: '#6bc4a0',
          400: '#48b088',
          500: '#2e9d72',
          600: '#1a6b4c',
          700: '#0d3b2e',
          800: '#0a2e23',
          900: '#071f17',
        },
        gold: {
          400: '#fdf3d8',
          500: '#f9e6a5',
          600: '#f7dd8e',
          700: '#f5d576',
          800: '#d4a017',
          900: '#b8860b',
        },
        red: {
          600: '#f1948a',
          700: '#ec7063',
          800: '#e74c3c',
          900: '#c0392b',
        },
      },
      fontFamily: {
        sans: ['Inter', 'Segoe UI', '-apple-system', ...],
      },
      fontSize: {
        'display-xl': ['4.5rem', { lineHeight: '1.1' }],
        'display-l': ['3rem', { lineHeight: '1.2' }],
        // ... all type scale sizes
      },
    },
  },
}
```

### React Component Example

```jsx
// Button.jsx
import React from 'react';
import styled from 'styled-components';

const StyledButton = styled.button`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: ${({ theme }) => theme.space.sm};
  padding: ${({ theme }) => theme.space.sm} ${({ theme }) => theme.space.xl};
  background-color: ${({ theme }) => theme.colors.gold[700]};
  color: ${({ theme }) => theme.colors.primary[700]};
  font-size: ${({ theme }) => theme.fontSize.bodyS};
  font-weight: ${({ theme }) => theme.fontWeight.bold};
  border: none;
  border-radius: ${({ theme }) => theme.radius.xl};
  cursor: pointer;
  transition: all 250ms ease;

  &:hover {
    background-color: ${({ theme }) => theme.colors.gold[600]};
  }
`;

const Button = ({ children, onClick, ...props }) => (
  <StyledButton onClick={onClick} {...props}>
    {children}
  </StyledButton>
);

export default Button;
```

---

## Page Templates

### 1. Homepage
**Purpose**: Showcase live matches, upcoming tournaments, and key information

**Layout**:
- Top Bar (live indicator, login)
- Header (logo, navigation, CTA)
- Hero (live match display)
- Live Scores Ticker
- Classification Table
- Tournament Cards Grid
- Best Players Cards Grid
- Spaces/Navigation Links
- Footer

**Reference**: See `/docs/maquettes/homepage.svg`

---

### 2. Championship Page
**Purpose**: Display championship standings, results, and statistics

**Layout**:
- Page Header (title, navigation)
- Filters & Search
- Classification Table
- Match Results
- Next Matches
- Season Statistics

---

### 3. Tournament Detail
**Purpose**: Show tournament information, participants, and results

**Layout**:
- Tournament Header (title, date, location, status)
- Tournament Navigation (tabs)
- Main Content (based on tab)
- Related Tournaments

---

### 4. Club Page
**Purpose**: Present club information, team, players, and results

**Layout**:
- Club Header (logo, name, stats)
- Club Navigation (tabs)
- Main Content (2-3 columns)
- Sponsors Section

---

### 5. Player Profile
**Purpose**: Display player statistics, career, and achievements

**Layout**:
- Player Header (avatar, name, badges)
- Player Navigation (tabs)
- Main Content (2 columns)
- Related Players

---

### 6. Admin Dashboard
**Purpose**: Manage competitions, clubs, players, and results

**Layout**:
- Sidebar Navigation
- Main Content Area
- Dashboard Widgets

---

## Design Tokens Summary

### Colors
- **Primary**: 10 shades (50-900)
- **Gold**: 6 shades (400-900)
- **Red**: 4 shades (600-900)
- **Gray**: 9 shades (100-900)
- **Semantic**: success, warning, danger, info

### Typography
- **Font Families**: 2 (Primary, Mono)
- **Font Sizes**: 11 (display-xl to caption)
- **Font Weights**: 6 (light to extra-bold)
- **Line Heights**: 4 (display to caption)

### Spacing
- **Scale**: 8 values (xs to 4xl)
- **Breakpoints**: 7 (xs to 3xl)

### Border Radius
- **Scale**: 6 values (none to full)

### Shadows
- **Scale**: 4 values (sm to xl)

### Transitions
- **Scale**: 3 values (fast to slow)

---

## Accessibility

### Color Contrast
- All color combinations meet **WCAG 2.1 AA** standards
- Primary text on backgrounds: Minimum **4.5:1** contrast
- Large text: Minimum **3:1** contrast

### Keyboard Navigation
- All interactive elements are keyboard accessible
- Focus states are clearly visible
- Skip links available for screen readers

### Screen Reader Support
- Semantic HTML (button, nav, table, etc.)
- ARIA labels for icon-only elements
- Text alternatives for images

### Reduced Motion
- Respect `prefers-reduced-motion` media query
- No animations for users who prefer reduced motion

---

## Performance

### Optimizations
1. **SVG Icons**: Inlined or sprite-based for minimal HTTP requests
2. **CSS Variables**: Efficient theming with native CSS
3. **Font Loading**: Preconnect to Google Fonts (if using Inter)
4. **Lazy Loading**: Images and non-critical resources
5. **Critical CSS**: Inline critical styles for faster rendering

### Loading Strategy
```html
<!-- In <head> -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" rel="stylesheet">

<!-- Inline critical CSS -->
<style>
  :root { /* design tokens */ }
  /* base styles */
</style>

<!-- Load remaining CSS asynchronously -->
<link rel="stylesheet" href="styles.css" media="print" onload="this.media='all'">
```

---

## Tools & Resources

### Design Tools
- **Figma**: For mockups and prototyping (recommended)
- **SVGO**: For SVG optimization
- **Adobe Color**: For color palette testing
- **Coolors**: For color scheme generation

### Development Tools
- **Stylelint**: For CSS linting
- **Prettier**: For code formatting
- **PurgeCSS**: For removing unused CSS
- **Storybook**: For component documentation

### Testing Tools
- **WebAIM Contrast Checker**: For color accessibility
- **axe DevTools**: For accessibility testing
- **Lighthouse**: For performance audits
- **BrowserStack**: For cross-browser testing

---

## Contributing to the Design System

### Adding a New Component
1. Create a new component in the appropriate category
2. Add usage examples
3. Document props and variants
4. Add to the Storybook (if available)
5. Test across breakpoints

### Adding a New Color
1. Add to the color palette in `colors.md`
2. Add CSS variables
3. Add to Tailwind config (if applicable)
4. Test contrast ratios

### Adding a New Icon
1. Create the SVG (24x24, 2px stroke)
2. Add to the icon library in `icons.md`
3. Add usage examples
4. Test accessibility

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-10-09 | Initial release - Complete design system based on homepage.svg and business requirements |

---

## License

This design system is **free to use** for the Palet Vendeen project.

---

## Support

For questions or issues:
1. Check the documentation in this directory
2. Review the business requirements in `/docs/contexte-metier/`
3. Consult the homepage.svg maquette in `/docs/maquettes/`
4. Ask the project team

---

## Next Steps

1. **Implement the design system** in your chosen framework
2. **Create a Storybook** for component documentation and testing
3. **Set up a visual regression testing** system
4. **Create design system guidelines** for contributors
5. **Establish a review process** for design system changes

---

## Additional Resources

- [Homepage Maquette](/docs/maquettes/homepage.svg) - Visual reference
- [Business Context](/docs/contexte-metier/README_contexte_palet_vendeen.md) - Business requirements
- [Architecture Document](/docs/architecture.md) - Technical architecture
- [Stack Technique](/docs/stack-technique.md) - Technology choices

---

## Quick Links

- [Colors](./colors.md)
- [Typography](./typography.md)
- [Components](./components.md)
- [Layouts](./layouts.md)
- [Icons](./icons.md)

---

*Design System v1.0 - Created 09/10/2026*
*Inspired by Top 14, modern sports platforms, and Palet Vendeen heritage*

---

**Note**: This design system is a **living document** that will evolve as the project grows. Regular updates are expected based on user feedback and new requirements.

---

**Have feedback or suggestions?**
Please contribute to improving this design system by:
- Reporting inconsistencies
- Suggesting improvements
- Adding new components as needed
- Updating documentation

---

*Happy designing!* 🎨
