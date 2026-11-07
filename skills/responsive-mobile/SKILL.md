---
name: responsive-mobile
description: Design responsive mobile-first pour le frontend Dioxus web (rust-frontend/public/styles, index.html) et le mobile (rust-mobile) — breakpoints, touch, safe areas, viewport, perfs et accessibilité mobile. Le projet a déjà docs/RESPONSIVE_GUIDE.md et docs/RESPONSIVE_TEST_CHECKLIST.md.
triggers:
  - responsive, mobile, téléphone, smartphone, tablette, iPhone, Android, "petit écran", desktop
  - css, style, styliser, mise en page, affichage, apparence, design
  - breakpoint, "media query", tactile, touch, doigt, encoche, notch, "safe area", viewport, overflow
  - "trop petit", déborde, "défilement horizontal", zoom, lisibilité, contraste, "ça dépasse", "c'est coupé"
---

# Responsive mobile-first — Palet Vendéen

Le projet cible **web mobile d'abord**, puis desktop, plus iOS/Android via
`rust-mobile`. Les styles vivent dans `rust-frontend/public/styles/`
(`design-system.css`, `responsive.css`, `global.css`, …) et le viewport est
défini dans `rust-frontend/index.html`.

## Règles de base (mobile-first)

1. **Le CSS de base cible le mobile** (petit écran). On *augmente* pour les
   grands écrans avec `@media (min-width: ...)`, jamais l'inverse.
2. **2–3 breakpoints maximum**, choisis par le contenu, pas par les modèles
   d'appareils. Ex. : `640px` (grand mobile / petite tablette) et `1024px` (desktop).
3. **Unités relatives** : `rem` pour le texte, `%`/`fr` pour les layouts,
   `clamp(min, pref, max)` pour une typo fluide sans media query.
4. **Hauteur mobile** : préférer `100dvh`/`svh` à `100vh` (barre d'URL Safari).

## Fichiers concernés

- `rust-frontend/index.html` — meta viewport (voir ci-dessous).
- `rust-frontend/public/styles/responsive.css` — media queries.
- `rust-frontend/public/styles/design-system.css` — composants du design system.
- `docs/RESPONSIVE_GUIDE.md`, `docs/RESPONSIVE_TEST_CHECKLIST.md` — guides existants à suivre.

## Viewport et safe areas (index.html)

```html
<meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover">
```

- `viewport-fit=cover` + `env(safe-area-inset-*)` pour ne pas passer sous
  l'encoche / la barre home (iPhone) :

```css
.header {
  padding-top: env(safe-area-inset-top, 0px);
}
.footer {
  padding-bottom: env(safe-area-inset-bottom, 0px);
}
```

## Touch : règles mobile

- **Cibles tactiles ≥ 44×44 px** (48 px recommandé — Apple HIG / Material).
- Espacer les boutons d'au moins 8 px.
- Ne pas dépendre du `:hover` sur mobile : protéger avec `@media (hover: hover)`.
- `touch-action: manipulation` sur les boutons pour supprimer le délai de tap.

## Lisibilité

- Taille de police **≥ 16px sur les inputs** (sinon iOS zoome automatiquement au focus).
- Contraste WCAG AA (4.5:1 pour le texte) — vérifier avec les couleurs du design system.
- `line-height` confortable (≈1.5), textes jamais tronqués sans `text-overflow`.

## Layout

- Flexbox/Grid partout ; `min-width: 0` sur les enfants flex pour éviter les débordements.
- Images : `max-width: 100%; height: auto;`.
- Tableaux larges : les wrapper dans un conteneur `overflow-x: auto` plutôt que de les casser.
- Navigation mobile : menu hamburger plutôt qu'une barre horizontale trop large.

## Performance mobile (WASM)

- Build release déjà optimisé (`lto`, `opt-level 3` dans le `Cargo.toml` racine).
- Lazy-load les routes lourdes, compresser les images servies depuis `public/`.
- Tester sur un vrai appareil / throttle réseau 3G dans les DevTools.

## Check-list de vérification (résumé de docs/RESPONSIVE_TEST_CHECKLIST.md)

- [ ] 320px (petit mobile) → 1440px (desktop) sans débordement horizontal.
- [ ] Meta viewport présent avec `viewport-fit=cover`.
- [ ] Tous les boutons/liens ≥ 44px et cliquables au doigt.
- [ ] Pas de contenu caché sous l'encoche (safe areas).
- [ ] Inputs ≥ 16px, pas de zoom au focus sur iOS.
- [ ] Contraste AA, focus clavier visible.
- [ ] Testé en portrait et paysage.

## Références

- MDN — Responsive design : https://developer.mozilla.org/fr/docs/Learn/CSS/CSS_layout/Responsive_Design
- MDN — Meta viewport : https://developer.mozilla.org/docs/Web/HTML/Viewport_meta_tag
- Safe areas `env()` : https://webkit.org/blog/7929/designing-websites-for-iphone-x/
- Apple HIG — touch targets 44pt : https://developer.apple.com/design/human-interface-guidelines
