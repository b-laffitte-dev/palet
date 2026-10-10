# Typography - Palet Vendeen Design System

> Typographie system based on homepage.svg maquette and palet vendeen identity

---

## Philosophy

Combines:
- Modernity: Clean sans-serif (Segoe UI / Inter)
- Tradition: Clear hierarchy with generous sizes
- Sport: Bold weights for titles and scores
- Precision: Important data highlighted

---

## Font Stack

### Primary Font
```
Font Family: "Segoe UI", -apple-system, BlinkMacSystemFont, "Roboto", "Helvetica Neue", Arial, sans-serif
```

### Web Alternative (Recommended)
```
Font Family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif
```

---

## Type Scale

| Name | Size (px) | Size (rem) | Usage | Line Height | Weight | Letter Spacing |
|------|-----------|------------|-------|-------------|--------|----------------|
| Display XL | 72 | 4.5rem | Hero titles | 1.1 | 700 | 0.05em |
| Display L | 48 | 3rem | Section titles | 1.2 | 700 | 0.02em |
| Display M | 36 | 2.25rem | Card titles | 1.2 | 700 | 0 |
| Heading XL | 30 | 1.875rem | Team names | 1.3 | 700 | 0 |
| Heading L | 28 | 1.75rem | Section headings | 1.3 | 700 | 0 |
| Heading M | 24 | 1.5rem | Logo | 1.2 | 700 | 0 |
| Heading S | 20 | 1.25rem | Player names | 1.4 | 600 | 0 |
| Heading XS | 18 | 1.125rem | Subtitles | 1.4 | 600 | 0 |
| Body L | 16 | 1rem | Body text | 1.6 | 400 | 0 |
| Body M | 15 | 0.9375rem | Secondary text | 1.5 | 400 | 0 |
| Body S | 14 | 0.875rem | Meta data | 1.5 | 400 | 0 |
| Body XS | 13 | 0.8125rem | Table headers | 1.4 | 400 | 0 |
| Caption | 12 | 0.75rem | Small text | 1.4 | 400 | 0 |

---

## CSS Variables

### Native CSS
```css
:root {
  /* Font Family */
  --font-primary: "Segoe UI", -apple-system, BlinkMacSystemFont, "Roboto", "Helvetica Neue", Arial, sans-serif;
  --font-mono: "SF Mono", "Monaco", "Courier New", monospace;
  
  /* Font Sizes */
  --font-size-display-xl: 4.5rem;
  --font-size-display-l: 3rem;
  --font-size-display-m: 2.25rem;
  --font-size-heading-xl: 1.875rem;
  --font-size-heading-l: 1.75rem;
  --font-size-heading-m: 1.5rem;
  --font-size-heading-s: 1.25rem;
  --font-size-heading-xs: 1.125rem;
  --font-size-body-l: 1rem;
  --font-size-body-m: 0.9375rem;
  --font-size-body-s: 0.875rem;
  --font-size-body-xs: 0.8125rem;
  --font-size-caption: 0.75rem;
  
  /* Font Weights */
  --font-weight-light: 300;
  --font-weight-regular: 400;
  --font-weight-medium: 500;
  --font-weight-semi-bold: 600;
  --font-weight-bold: 700;
  --font-weight-extra-bold: 800;
  
  /* Line Heights */
  --line-height-display: 1.1;
  --line-height-heading: 1.2;
  --line-height-body: 1.6;
  --line-height-caption: 1.4;
}
```

### SCSS Variables
```scss
$font-primary: "Segoe UI", -apple-system, BlinkMacSystemFont, "Roboto", "Helvetica Neue", Arial, sans-serif;
$font-mono: "SF Mono", "Monaco", "Courier New", monospace;

$font-size-display-xl: 4.5rem;
$font-size-display-l: 3rem;
$font-size-display-m: 2.25rem;
$font-size-heading-xl: 1.875rem;
$font-size-heading-l: 1.75rem;
$font-size-heading-m: 1.5rem;
$font-size-heading-s: 1.25rem;
$font-size-heading-xs: 1.125rem;
$font-size-body-l: 1rem;
$font-size-body-m: 0.9375rem;
$font-size-body-s: 0.875rem;
$font-size-body-xs: 0.8125rem;
$font-size-caption: 0.75rem;

$font-weight-light: 300;
$font-weight-regular: 400;
$font-weight-medium: 500;
$font-weight-semi-bold: 600;
$font-weight-bold: 700;
$font-weight-extra-bold: 800;

$line-height-display: 1.1;
$line-height-heading: 1.2;
$line-height-body: 1.6;
$line-height-caption: 1.4;
```

---

## Tailwind Configuration

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'Segoe UI', '-apple-system', 'BlinkMacSystemFont', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
        mono: ['SF Mono', 'Monaco', 'Courier New', 'monospace'],
      },
      fontSize: {
        'display-xl': ['4.5rem', { lineHeight: '1.1', letterSpacing: '0.05em' }],
        'display-l': ['3rem', { lineHeight: '1.1', letterSpacing: '0.05em' }],
        'display-m': ['2.25rem', { lineHeight: '1.2' }],
        'heading-xl': ['1.875rem', { lineHeight: '1.2' }],
        'heading-l': ['1.75rem', { lineHeight: '1.2' }],
        'heading-m': ['1.5rem', { lineHeight: '1.2' }],
        'heading-s': ['1.25rem', { lineHeight: '1.3' }],
        'heading-xs': ['1.125rem', { lineHeight: '1.3' }],
        'body-l': ['1rem', { lineHeight: '1.6' }],
        'body-m': ['0.9375rem', { lineHeight: '1.5' }],
        'body-s': ['0.875rem', { lineHeight: '1.5' }],
        'body-xs': ['0.8125rem', { lineHeight: '1.4' }],
        'caption': ['0.75rem', { lineHeight: '1.4' }],
      },
      fontWeight: {
        light: 300,
        regular: 400,
        medium: 500,
        semi-bold: 600,
        bold: 700,
        extra-bold: 800,
      },
    },
  },
}
```

---

## Usage Examples

### Hero Title
```html
<h1 class="text-display-m font-bold text-gold-700 uppercase tracking-wider">
  Finale du Championnat de Vendee
</h1>
```

### Score Display
```html
<div class="flex items-center gap-4">
  <span class="text-display-m font-bold text-white">72</span>
  <span class="text-display-m font-bold text-gold-700">68</span>
</div>
```

### Player Card
```html
<h3 class="text-heading-s font-bold text-primary-700">Jean Morice</h3>
<p class="text-body-m text-gray-500">La Roche-sur-Yon - Div. 1</p>
<span class="text-caption font-bold text-white bg-gold-700 px-3 py-1 rounded-full">
  MVP
</span>
```

### Table Header
```html
<th class="text-body-xs font-semi-bold text-gray-500 uppercase tracking-wider text-left">
  Club
</th>
```

---

## Character Set

| Symbol | Code | Usage | Example |
|--------|------|-------|---------|
| -> | `&rarr;` | Links | Tout le classement &rarr; |
| • | `&bullet;` | Indicators | &bullet; LIVE |
| — | `&mdash;` | Separator | Finale &mdash; Division 1 |
| · | `&middot;` | Light separator | 15h00 &middot; Terrain |
| ★ | `&star;` | Badge | &star; Joueur MVP |

---

## Best Practices

### Do
- Use rem units for scalability
- Maintain clear hierarchy (1-2-3 levels)
- Bold weights for important numbers and data
- Gold (#f5d576) for live/active elements
- Primary green (#0d3b2e) for main titles
- Gray (#7a8b84) for secondary text

### Dont
- More than 3 different fonts on same page
- Italic (rarely used in sport design)
- Underline (except for links)
- ALL CAPS for more than 2-3 words
- Font sizes too close together

---

## Google Fonts Integration

Add to your `<head>`:
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap" rel="stylesheet">
```

Then use:
```css
:root {
  --font-primary: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}
```

---

## Statistics from Maquette

| Element | Size | Weight | Color | Occurrences |
|---------|------|--------|-------|-------------|
| Hero Title | 18px | Bold | #f5d576 | 1 |
| Team Names | 30px | Bold | #fff | 2 |
| Score | 42px | Bold | #fff / #f5d576 | 2 |
| Section Title | 28px | Bold | #0d3b2e | 4 |
| Card Title | 20px | Bold | #0d3b2e | 6 |
| Table Header | 13px | Semi-Bold | #7a8b84 | 8 |
| Table Row | 16px | Bold | #0d3b2e | 6 |
| Badge | 12px | Bold | #fff | 5 |
| Button | 14px | Bold | various | 8 |

---

## Visual Hierarchy

```
1. Score (42px, Bold, Gold/White) - Who is winning?
2. Team Names (30px, Bold, White) - Who is playing?
3. Match Title (18px, Bold, Gold) - What competition?
4. Next Tournaments (20px, Bold, Primary) - What is next?
```

This hierarchy allows users to understand the information in less than 2 seconds.

---

*Document generated on 09/10/2026*
*Based on homepage.svg maquette and palet vendeen context*
*Inspired by: Top 14, Ligue 1, ESPN, BBC Sport*
