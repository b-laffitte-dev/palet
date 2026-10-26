# Guide du Responsive Design - CDSP 79

## Introduction

Ce guide explique comment l'application a été rendue responsive pour fonctionner correctement sur tous les appareils, des téléphones mobiles aux grands écrans.

## Modifications Apportées

### 1. Structure des Fichiers CSS

#### `src/styles/design-system.css`
- Fichier principal contenant les variables CSS et le système de design
- **Import les fichiers responsables via `@import`**

#### `src/styles/responsive.css`
- Contient toutes les **media queries** et classes utilitaires responsive
- Basé sur une approche **mobile-first**
- Breakpoints : 320px, 480px, 640px, 768px, 1024px, 1280px, 1440px

#### `src/styles/global.css`
- Contient les **classes utilitaires** globales (similaires à Tailwind)
- Utilise les variables CSS définies dans design-system.css

#### `src/styles/components.css`
- Contient les **styles spécifiques aux composants** pour le responsive
- Adaptations pour : boutons, cartes, tableaux, formulaires, modales, etc.

### 2. Composants Adaptés

#### Layout Components
- **`Header.tsx`** : Ajout du menu hamburger pour mobile
- **`TopBar.tsx`** : Adaptation pour affichage compact sur mobile
- **`Footer.tsx`** : Grilles responsives pour les sections
- **`Layout.tsx`** : Gestion des paddings responsives
- **`MobileNav.tsx`** (NOUVEAU) : Menu de navigation mobile

#### Common Components
- **`UserMenu.tsx`** : Conversion des styles inline en classes CSS
- **`Button.tsx`** : Variantes adaptées pour mobile
- **`Card.tsx`** : Grilles responsives
- **`Table.tsx`** : Tableaux scrollables horizontalement sur mobile

### 3. Hooks

#### `useResponsive.ts` (NOUVEAU)
Hook pour détecter le type d'appareil et les dimensions de l'écran :

```typescript
import { useResponsive } from "../hooks/useResponsive";

const MyComponent = () => {
  const { device, width, height, orientation, isMobile, isDesktop } = useResponsive();
  
  if (isMobile()) {
    return <MobileView />;
  }
  
  return <DesktopView />;
};
```

## Breakpoints

Les breakpoints utilisés suivent une approche mobile-first :

| Classe | Breakpoint | Description |
|--------|------------|-------------|
| Default | < 640px | Mobile (téléphones) |
| `sm:` | ≥ 640px | Petit mobile / Tablette portrait |
| `md:` | ≥ 768px | Tablette / Petit desktop |
| `lg:` | ≥ 1024px | Desktop |
| `xl:` | ≥ 1280px | Grand desktop |
| `2xl:` | ≥ 1440px | Très grand écran |

## Utilisation des Classes Responsive

### Affichage / Masquage

```html
<!-- Masqué sur mobile, visible à partir de 640px -->
<div className="hidden sm:block">Desktop only</div>

<!-- Visible sur mobile, masqué à partir de 640px -->
<div className="sm:hidden">Mobile only</div>
```

### Grilles

```html
<!-- 1 colonne sur mobile, 2 sur tablette, 3 sur desktop -->
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  <!-- Items -->
</div>
```

### Largeurs

```html
<!-- 100% sur mobile, 50% sur tablette -->
<div className="w-full md:w-1/2">Half width on tablet+</div>
```

### Textes

```html
<!-- Centre sur mobile, aligné à gauche sur desktop -->
<p className="text-center md:text-left">Responsive text</p>
```

## Composants Responsive Clés

### Menu de Navigation Mobile

Le composant `MobileNav` est utilisé dans `Header` pour gérer la navigation sur mobile :

```tsx
// Dans Header.tsx
const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

return (
  <>
    <Header />
    <MobileNav isOpen={isMobileMenuOpen} onClose={() => setIsMobileMenuOpen(false)} />
  </>
);
```

### Boutons Responsive

Utilisez les classes de taille ou le hook `useResponsive` :

```tsx
<Button 
  size="m"
  fullWidth={true} // Pleine largeur sur mobile
  className="sm:w-auto" // Largeur auto à partir de 640px
>
  Click me
</Button>
```

### Tableaux Responsive

Tous les tableaux sont automatiquement scrollables horizontalement sur mobile :

```tsx
<Table className="table-responsive-container">
  {/* Table content */}
</Table>
```

Ou utilisez simplement le conteneur par défaut :

```tsx
<div className="overflow-x-auto">
  <table>{/* ... */}</table>
</div>
```

### Cartes Responsive

Utilisez les grilles responsives :

```tsx
<div className="card-grid">
  {items.map(item => (
    <Card key={item.id}>{/* ... */}</Card>
  ))}
</div>
```

## Classes Utilitaires Disponibles

### Display
- `.block`, `.flex`, `.grid`, `.hidden`
- `.sm:block`, `.md:flex`, `.lg:hidden` (responsive)

### Flexbox
- `.flex-row`, `.flex-col`
- `.items-center`, `.justify-between`
- `.gap-1` à `.gap-8`
- `.flex-1`, `.grow`, `.shrink-0`

### Grid
- `.grid-cols-1` à `.grid-cols-12`
- `.grid-cols-auto`, `.grid-cols-auto-fit`
- `.grid-gap-1` à `.grid-gap-8`

### Espacement
- `.p-1` à `.p-8` (padding)
- `.px-1`, `.py-1`, `.pt-1`, `.pr-1`, `.pb-1`, `.pl-1`
- `.m-1` à `.m-8` (margin)
- `.mt-1`, `.mr-1`, `.mb-1`, `.ml-1`
- `.mx-auto`, `.my-auto`

### Tailles
- `.w-1` à `.w-64` (largeur)
- `.h-1` à `.h-64` (hauteur)
- `.w-full`, `.h-full`
- `.w-1/2`, `.w-1/3`, `.w-2/3`

### Texte
- `.text-xs`, `.text-sm`, `.text-base`, `.text-lg`, `.text-xl`
- `.font-bold`, `.font-semibold`, `.font-medium`
- `.text-center`, `.text-left`, `.text-right`
- `.uppercase`, `.lowercase`, `.capitalize`

### Couleurs
- `.text-primary`, `.text-secondary`, `.text-tertiary`
- `.text-gold`, `.text-gold-pure`
- `.bg-primary`, `.bg-secondary`, `.bg-tertiary`
- `.bg-gold`, `.bg-gold-pure`
- `.border-gold`, `.border-primary`

### Bordures & Rayons
- `.border`, `.border-2`, `.border-4`
- `.border-gold`, `.border-primary`, `.border-neutral`
- `.rounded-sm`, `.rounded`, `.rounded-md`, `.rounded-lg`, `.rounded-xl`, `.rounded-full`

### Ombres
- `.shadow-sm`, `.shadow`, `.shadow-md`, `.shadow-lg`, `.shadow-xl`
- `.shadow-gold-sm`, `.shadow-gold-m`, `.shadow-gold-l`

### Opacité
- `.opacity-0` à `.opacity-100` (par pas de 5)

### Transitions & Animations
- `.transition`, `.transition-all`
- `.transition-colors`, `.transition-opacity`, `.transition-transform`
- `.duration-100` à `.duration-1000`
- `.animate-spin`, `.animate-pulse`, `.animate-bounce`

## Bonnes Pratiques

### 1. Mobile First

Toujours concevoir en pensant mobile d'abord :
- Commencez par le style de base (pour mobile)
- Ajoutez des media queries pour améliorer l'expérience sur desktop

### 2. Utilisez les Variables CSS

```css
/* ❌ Évitez */
background-color: #1a3d2e;

/* ✅ Préférez */
background-color: var(--color-green-dark);
```

### 3. Classes Utilitaires vs Styles Inline

```tsx
/* ❌ Évitez les styles inline */
<div style={{ padding: '16px', margin: '8px' }}>...</div>

/* ✅ Préférez les classes CSS */
<div className="p-4 m-2">...</div>
```

### 4. Test sur Plusieurs Tailles d'Écran

Utilisez les outils de développement du navigateur pour tester :
- iPhone SE (320px)
- iPhone 12 (390px)
- iPad (768px)
- Desktop (1024px+)

### 5. Performance

- Limitez le nombre de media queries imbriquées
- Utilisez `will-change: transform` pour les animations
- Évitez les `position: absolute` inutiles

## Exemples Complets

### Page avec Header Responsive

```tsx
import { Layout, Header, MobileNav } from "../components";
import { useState } from "react";

const MyPage = () => {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  return (
    <Layout>
      <Header onMenuToggle={() => setIsMobileMenuOpen(!isMobileMenuOpen)} />
      <MobileNav 
        isOpen={isMobileMenuOpen} 
        onClose={() => setIsMobileMenuOpen(false)} 
      />
      
      <main className="px-4 py-6 sm:px-6 lg:px-8">
        {/* Contenu */}
      </main>
    </Layout>
  );
};
```

### Grille de Cartes Responsive

```tsx
import { Card, ResponsiveContainer } from "../components";

const MyCardsGrid = ({ items }) => {
  return (
    <ResponsiveContainer>
      <div className="card-grid">
        {items.map(item => (
          <Card key={item.id} className="card-responsive">
            {/* Contenu de la carte */}
          </Card>
        ))}
      </div>
    </ResponsiveContainer>
  );
};
```

### Tableau avec Colonnes Responsives

```tsx
import { Table, TableHeader, TableBody, TableRow, TableCell } from "../components";

const MyTable = ({ data }) => {
  return (
    <div className="table-responsive-container">
      <Table>
        <TableHeader>
          <TableRow>
            <TableCell>Nom</TableCell>
            <TableCell className="hidden sm:table-cell">Description</TableCell>
            <TableCell className="hidden md:table-cell">Statut</TableCell>
            <TableCell>Actions</TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.map(row => (
            <TableRow key={row.id}>
              <TableCell>{row.name}</TableCell>
              <TableCell className="hidden sm:table-cell">{row.description}</TableCell>
              <TableCell className="hidden md:table-cell">{row.status}</TableCell>
              <TableCell>{/* Actions */}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
};
```

### Formulaire Responsive

```tsx
import { Form, TextField, Button } from "../components";

const MyForm = () => {
  return (
    <div className="form-responsive">
      <form className="space-y-6">
        <TextField 
          label="Nom" 
          name="name" 
          required 
          className="form-field-responsive"
        />
        
        <div className="form-row-responsive">
          <TextField 
            label="Email" 
            name="email" 
            type="email" 
            className="form-field-responsive"
          />
          <TextField 
            label="Téléphone" 
            name="phone" 
            className="form-field-responsive"
          />
        </div>
        
        <Button type="submit" fullWidth className="btn-responsive">
          Soumettre
        </Button>
      </form>
    </div>
  );
};
```

## Dépannage

### Problème: Les Media Queries ne Fonctionnent Pas

**Solution :** 
1. Vérifiez que le viewport est correctement configuré dans `index.html`
2. Assurez-vous que les fichiers CSS sont importés
3. Testez dans un navigateur moderne

### Problème: Le Menu Mobile ne s'Affiche Pas

**Solution :**
1. Vérifiez que le composant `MobileNav` est utilisé
2. Assurez-vous que l'état `isOpen` est correctement géré
3. Vérifiez qu'il n'y a pas de `z-index` qui bloque le menu

### Problème: Les Tableaux Dépassent de l'Écran

**Solution :**
1. Utilisez `.table-responsive-container` ou `overflow-x-auto`
2. Vérifiez que les colonnes ont un `min-width` raisonnable
3. Considérez masquer certaines colonnes sur mobile avec `.hidden sm:table-cell`

### Problème: Les Images sont Trop Grandes

**Solution :**
1. Utilisez `.responsive-img` ou les classes suivantes :
   ```html
   <img className="w-full h-auto max-w-full" />
   ```
2. Considérez utiliser `object-fit: cover` pour les images dans des conteneurs de taille fixe

## Ressources

- [MDN Responsive Design](https://developer.mozilla.org/fr/docs/Learn/CSS/CSS_layout/Responsive_Design)
- [CSS Media Queries](https://developer.mozilla.org/fr/docs/Web/CSS/Media_Queries)
- [Mobile-First CSS](https://www.smashingmagazine.com/2011/11/responsive-web-design-what-it-is-and-how-to-use-it/)

## Checklist de Test

- [ ] Test sur iPhone SE (320px × 568px)
- [ ] Test sur iPhone 12 (390px × 844px)
- [ ] Test sur iPad (768px × 1024px)
- [ ] Test sur Desktop (1024px × 768px)
- [ ] Test sur Grand Écran (1440px+)
- [ ] Vérifier l'orientation portrait/paysage sur mobile
- [ ] Tester avec le clavier (accessibilité)
- [ ] Tester avec différents navigateurs (Chrome, Safari, Firefox)

## Statut

✅ **Complété**
- Système de design responsive complet
- Breakpoints définis pour tous les appareils
- Composants de layout adaptés (Header, TopBar, Footer)
- Menu de navigation mobile avec hamburger
- Tableaux scrollables horizontalement
- Grilles responsives pour cartes et éléments
- Hook `useResponsive` pour la détection d'appareil
- Classes utilitaires complètes

🎯 **À Tester**
- Tester sur différents appareils réels
- Vérifier les performances sur mobile
- Corriger les éventuels problèmes d'affichage

📝 **À Améliorer**
- Optimisation des images pour mobile
- Lazy loading des composants lourds
- Service Worker pour le cache offline
