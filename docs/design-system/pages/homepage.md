# Homepage - Palet Vendeen

> Main landing page for the Palet Vendeen application
> Reference: `/docs/maquettes/homepage.svg`

---

## Overview

The **Homepage** is the primary entry point for the Palet Vendeen application. It serves as a **dashboard** that provides:
- **Live match information** (like Top 14 style display)
- **Current scores and status**
- **Standings/classification**
- **Upcoming tournaments**
- **Featured players**
- **Quick access** to all sections

**Inspiration**: Top 14 Rugby homepage with a sport-specific focus on palet vendeen.

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
│  - Federation name                                              │
│  - Live indicator (red dot) + "EN DIRECT" text                 │
│  - Login / My licensed space link                              │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
│  ┌─────────────────────┬─────────────────────────────────┐│
│  │ LOGO (52px)          │ NAVIGATION                        ││
│  │  - Gold circles     │  - Championnat (active)          ││
│  │  - Green background  │  - Tournois                      ││
│  │  - "PALET VENDÉEN"   │  - Clubs                         ││
│  │  - Subtitle          │  - Joueurs                       ││
│  │                     │  - Fédération                    ││
│  │                     │  - Actualités                    ││
│  └─────────────────────┴─────────────────────────────────┘│
│                       + [S'INSCRIRE Button]                     │
├─────────────────────────────────────────────────────────────┤
│ HERO - MATCH DISPLAY (520px)                                    │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ FINAL DU CHAMPIONNAT DE VENDÉE — DIVISION 1            ││
│  │ Samedi 14 juin 2025 - 15h00 - Terrain clos de La Roche   ││
│  │                                                         ││
│  │  ┌─────────┐       ┌─────┐       ┌─────────┐          ││
│  │  │   PC    │       │ 72 │       │   FC    │          ││
│  │  │  PALET  │       ├─────┤       │ FONTE-  │          ││
│  │  │  CLUB   │       │ 68 │       │  NAY    │          ││
│  │  │         │       └─────┘       │         │          ││
│  │  │ LA ROCHE│       ● LIVE        │ CLOS    │          ││
│  │  │ -SUR-YON│       Manche 24      │ FONTEN- │          ││
│  │  │         │                     │ OIS     │          ││
│  │  │ Vainqueur│                     │ Vainqueur│          ││
│  │  │  poule A│                     │  poule B│          ││
│  │  └─────────┘                     └─────────┘          ││
│  │                                                         ││
│  │ Terrain clos municipal - entrée gratuite - buvette      ││
│  │ et fan zone                                               ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ LIVE SCORES TICKER (130px)                                     │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ ⏱ SCORES EN DIRECT — 12e journée                          ││
│  │                                                         ││
│  │ [Lacon PC 45-38 Les Sables   Manche 18 - en cours]       ││
│  │ [Challans 52-52 Montaigu     Manche 21 - en cours]        ││
│  │ [Saint-Gilles 61-44 Pouzaugues   Terminé]                ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ CLASSIFICATION - DIVISION 1 (400px)                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ CHAMPIONNAT — DIVISION 1                              ││
│  │ Classement après 12 journées                          ││
│  │                                          [Tout le classement →]││
│  │                                                         ││
│  │ POS  CLUB           MJ  G  P  DIFF  PTS  FORME          ││
│  │ 1    La Roche...   12 10  2  +84  30   █████            ││
│  │ 2    Clos Font...   12  9  3  +61  28   █████            ││
│  │ 3    Lacon PC       12  8  4  +38  26   █████            ││
│  │                                                         ││
│  │ Legend: [Finale] [Barrage] [Descente D2]                 ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ TOURNOIS À VENIR (420px)                                       │
│  ┌──────────────┬──────────────┬──────────────┐            │
│  │ OPEN         │ CHAMPIONNAT  │ ÉVÉNEMENT     │            │
│  │ NATIONAL     │ DÉPARTEMENTAL │ FÉDÉRAL       │            │
│  │              │              │              │            │
│  │ Open de      │ Championnat   │ Internationaux│            │
│  │ Luçon        │ D2 - J13      │ de Vendée    │            │
│  │              │              │              │            │
│  │ [Inscriptions]│ [Billetterie] │ [Complet]     │            │
│  │              │              │              │            │
│  │ Dim 6 juil   │ Dim 29 juin   │ 15-17 août     │            │
│  │ 2025        │ 2025         │ 2025          │            │
│  │              │              │              │            │
│  │ Terrain...   │ Saint-Gilles │ La Roche...   │            │
│  │              │              │              │            │
│  │ 64 joueurs   │ 48 joueurs    │ 128 joueurs   │            │
│  │              │              │              │            │
│  │ [S'inscrire] │ [Voir la...]  │ [Suivre...]   │            │
│  └──────────────┴──────────────┴──────────────┘            │
├─────────────────────────────────────────────────────────────┤
│ LES MEILLEURS JOUEURS                                            │
│  ┌──────────────┬──────────────┬──────────────┐            │
│  │              │              │              │            │
│  │  JM   MVP    │  PB          │  AL          │            │
│  │ Jean Morice  │ Pierre Baudry│ Anaïs Léger │            │
│  │ La Roche...  │ Clos Font...  │ Luçon       │            │
│  │              │              │              │            │
│  │ Points: 412  │ Points: 445  │ Points: 398 │            │
│  │ Moy: 11.4    │ Moy: 10.8    │ Moy: 11.1    │            │
│  │ Gagnants:68% │ Gagnants:64%  │ Gagnants:71% │            │
│  │              │              │              │            │
│  └──────────────┴──────────────┴──────────────┘            │
├─────────────────────────────────────────────────────────────┤
│ NOS ESPACES (220px)                                             │
│  ┌──────────────┬──────────────┬──────────────┬──────────┐│
│  │ 🏛 Espace    │ 🏟 Espace    │ 👤 Espace    │ 📣 Espace ││
│  │ Fédération   │ Club         │ Joueur       │ Supporter ││
│  │              │              │              │          ││
│  │ Licences...  │ Effectifs...  │ Profil...    │ Pronos...  ││
│  └──────────────┴──────────────┴──────────────┴──────────┘│
├─────────────────────────────────────────────────────────────┤
│ FOOTER (70px)                                                   │
│  - Fédération de Palet Vendéen                                  │
│  - Mentions légales - RGPD - Contact                            │
│  - Maquette - Page d'accueil                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Components Used

### 1. Top Bar (`/docs/design-system/components.md`)

**Purpose**: Display live status and authentication links

```jsx
<TopBar>
  <TopBarLeft>
    <Text size="body-xs" color="primary-200">
      Fédération de Palet Vendéen
    </Text>
  </TopBarLeft>
  <TopBarCenter>
    <Icon name="live" size="sm" color="red-800" />
    <Text size="body-xs" color="white" weight="bold">
      EN DIRECT
    </Text>
  </TopBarCenter>
  <TopBarRight>
    <Link href="/login" color="primary-200" size="body-xs">
      Connexion
    </Link>
    <Text size="body-xs" color="primary-200">|</Text>
    <Link href="/mon-espace" color="primary-200" size="body-xs">
      Mon espace licencié
    </Link>
  </TopBarRight>
</TopBar>
```

---

### 2. Header (`/docs/design-system/components.md`)

**Purpose**: Main navigation and branding

```jsx
<Header>
  <NavLogo href="/">
    <NavLogoIcon size="52">
      <Icon name="palet-logo" size="52" />
    </NavLogoIcon>
    <NavLogoText>
      <Text size="heading-m" color="gold-700" weight="bold">
        PALET VENDÉEN
      </Text>
      <Text size="body-xs" color="primary-200">
        LE SITE OFFICIEL — CHAMPIONNATS, CLUBS & TOURNOIS
      </Text>
    </NavLogoText>
  </NavLogo>
  
  <NavLinks>
    <NavLink href="/championnat" active>
      Championnat
    </NavLink>
    <NavLink href="/tournois">
      Tournois
    </NavLink>
    <NavLink href="/clubs">
      Clubs
    </NavLink>
    <NavLink href="/joueurs">
      Joueurs
    </NavLink>
    <NavLink href="/federation">
      Fédération
    </NavLink>
    <NavLink href="/actualites">
      Actualités
    </NavLink>
  </NavLinks>
  
  <NavActions>
    <ButtonPrimary>
      S'inscrire
    </ButtonPrimary>
  </NavActions>
</Header>
```

---

### 3. Match Hero (`/docs/design-system/components.md`)

**Purpose**: Featured live match display (style Top 14)

```jsx
<MatchHero>
  <MatchHeroTitle>
    FINAL DU CHAMPIONNAT DE VENDÉE — DIVISION 1
  </MatchHeroTitle>
  <MatchHeroSubtitle>
    Samedi 14 juin 2025 · 15h00 · Terrain clos de La Roche-sur-Yon
  </MatchHeroSubtitle>
  
  <MatchContent>
    <MatchTeam>
      <MatchTeamLogo>
        <Icon name="palet-circle" size="160" />
        <Text size="heading-xl" weight="bold" color="primary-700">
          PC
        </Text>
        <Text size="body-m" weight="bold" color="primary-700">
          PALET CLUB
        </Text>
      </MatchTeamLogo>
      <MatchTeamName size="heading-xl" weight="bold">
        LA ROCHE-SUR-YON
      </MatchTeamName>
      <MatchTeamStatus size="body-s" color="primary-200">
        Vainqueur poule A
      </MatchTeamStatus>
    </MatchTeam>
    
    <MatchScore>
      <MatchScorePrimary size="display-m">
        72
      </MatchScorePrimary>
      <MatchScoreDivider />
      <MatchScoreSecondary size="display-m">
        68
      </MatchScoreSecondary>
      <MatchScoreLabel>
        <Icon name="live" size="sm" />
        <Text size="body-s" weight="bold" uppercase>
          LIVE — Manche 24
        </Text>
      </MatchScoreLabel>
    </MatchScore>
    
    <MatchTeam>
      <MatchTeamLogo>
        <Icon name="palet-circle" size="160" />
        <Text size="heading-xl" weight="bold" color="primary-700">
          FC
        </Text>
        <Text size="body-m" weight="bold" color="primary-700">
          FONTENAY
        </Text>
      </MatchTeamLogo>
      <MatchTeamName size="heading-xl" weight="bold">
        CLOS FONTENOIS
      </MatchTeamName>
      <MatchTeamStatus size="body-s" color="primary-200">
        Vainqueur poule B
      </MatchTeamStatus>
    </MatchTeam>
  </MatchContent>
  
  <MatchInfo>
    <Icon name="location" size="sm" />
    <Text size="body-m" weight="semi-bold">
      Terrain clos municipal — entrée gratuite · buvette et fan zone
    </Text>
  </MatchInfo>
</MatchHero>
```

---

### 4. Live Scores Ticker (`/docs/design-system/components.md`)

**Purpose**: Scrolling display of current live scores

```jsx
<LiveScores>
  <LiveScoresHeader>
    <Icon name="clock" size="sm" color="gold-700" />
    <Text size="body-m" weight="bold" color="gold-700" uppercase>
      Scores en direct — 12e journée
    </Text>
  </LiveScoresHeader>
  
  <LiveScoresList>
    <LiveScoreItem>
      <LiveScoreTeams>
        <Text size="body-m" color="white">Luçon PC</Text>
        <Text size="body-m" weight="bold" color="gold-700">45 — 38</Text>
        <Text size="body-m" color="white">Les Sables</Text>
      </LiveScoreTeams>
      <LiveScoreStatus size="caption" color="primary-200">
        Manche 18 · en cours
      </LiveScoreStatus>
    </LiveScoreItem>
    
    <LiveScoreItem>
      <LiveScoreTeams>
        <Text size="body-m" color="white">Challans</Text>
        <Text size="body-m" weight="bold" color="gold-700">52 — 52</Text>
        <Text size="body-m" color="white">Montaigu</Text>
      </LiveScoreTeams>
      <LiveScoreStatus size="caption" color="primary-200">
        Manche 21 · en cours
      </LiveScoreStatus>
    </LiveScoreItem>
    
    <LiveScoreItem>
      <LiveScoreTeams>
        <Text size="body-m" color="white">Saint-Gilles</Text>
        <Text size="body-m" weight="bold" color="gold-700">61 — 44</Text>
        <Text size="body-m" color="white">Pouzaugues</Text>
      </LiveScoreTeams>
      <LiveScoreStatus size="caption" color="primary-200">
        Terminé
      </LiveScoreStatus>
    </LiveScoreItem>
  </LiveScoresList>
</LiveScores>
```

---

### 5. Classification Section

**Purpose**: Display current championship standings

```jsx
<Section>
  <SectionHeader>
    <SectionTitle size="heading-l" weight="bold">
      CHAMPIONNAT — DIVISION 1
    </SectionTitle>
    <SectionSubtitle size="body-m" color="gray-500">
      Classement après 12 journées
    </SectionSubtitle>
    <SectionActions>
      <ButtonGhost>
        Tout le classement <Icon name="arrow-right" size="sm" />
      </ButtonGhost>
    </SectionActions>
  </SectionHeader>
  
  <ClassificationTable>
    <ClassificationTableHeader>
      <TableHeaderCell>POS</TableHeaderCell>
      <TableHeaderCell>CLUB</TableHeaderCell>
      <TableHeaderCell textAlign="center">MJ</TableHeaderCell>
      <TableHeaderCell textAlign="center">G</TableHeaderCell>
      <TableHeaderCell textAlign="center">P</TableHeaderCell>
      <TableHeaderCell textAlign="center">DIFF</TableHeaderCell>
      <TableHeaderCell textAlign="center">PTS</TableHeaderCell>
      <TableHeaderCell>FORME</TableHeaderCell>
    </ClassificationTableHeader>
    
    <ClassificationTableBody>
      <ClassificationRow qualified>
        <TableCell position>
          <Text size="body-l" weight="bold" color="primary-700">1</Text>
        </TableCell>
        <TableCell club>
          <ClubLogo abbr="LR" />
          <ClubName size="body-l" weight="bold" color="primary-700">
            La Roche-sur-Yon Palet Club
          </ClubName>
        </TableCell>
        <TableCell stat textAlign="center">12</TableCell>
        <TableCell stat textAlign="center">10</TableCell>
        <TableCell stat textAlign="center">2</TableCell>
        <TableCell stat textAlign="center">+84</TableCell>
        <TableCell pts textAlign="center">30</TableCell>
        <TableCell form>
          <FormBadge>
            <FormBadgeItem status="win" />
            <FormBadgeItem status="win" />
            <FormBadgeItem status="win" />
            <FormBadgeItem status="loss" />
            <FormBadgeItem status="win" />
          </FormBadge>
        </TableCell>
      </ClassificationRow>
      
      <ClassificationRow qualified>
        <TableCell position>2</TableCell>
        <TableCell club>
          <ClubLogo abbr="CF" />
          <ClubName>Clos Fontenois</ClubName>
        </TableCell>
        <TableCell stat>12</TableCell>
        <TableCell stat>9</TableCell>
        <TableCell stat>3</TableCell>
        <TableCell stat>+61</TableCell>
        <TableCell pts>28</TableCell>
        <TableCell form>
          <FormBadge>...</FormBadge>
        </TableCell>
      </ClassificationRow>
      
      <ClassificationRow barrage>
        <TableCell position>3</TableCell>
        <TableCell club>
          <ClubLogo abbr="LU" />
          <ClubName>Luçon Palet Club</ClubName>
        </TableCell>
        <TableCell stat>12</TableCell>
        <TableCell stat>8</TableCell>
        <TableCell stat>4</TableCell>
        <TableCell stat>+38</TableCell>
        <TableCell pts>26</TableCell>
        <TableCell form>...</TableCell>
      </ClassificationRow>
      
      {/* More rows... */}
    </ClassificationTableBody>
    
    <ClassificationLegend>
      <LegendItem>
        <LegendColor color="primary-600" />
        <Text size="caption" color="gray-500">Finale</Text>
      </LegendItem>
      <LegendItem>
        <LegendColor color="gold-700" />
        <Text size="caption" color="gray-500">Barrage</Text>
      </LegendItem>
      <LegendItem>
        <LegendColor color="red-800" />
        <Text size="caption" color="gray-500">Descente D2</Text>
      </LegendItem>
    </ClassificationLegend>
  </ClassificationTable>
</Section>
```

---

### 6. Tournaments Section

**Purpose**: Display upcoming tournaments

```jsx
<Section>
  <SectionHeader>
    <SectionTitle size="heading-l" weight="bold">
      TOURNOIS À VENIR
    </SectionTitle>
    <SectionSubtitle size="body-m" color="gray-500">
      Open, qualificatifs et championnats
    </SectionSubtitle>
  </SectionHeader>
  
  <TournamentsGrid>
    <TournamentCard type="open">
      <TournamentCardHeader type="open">
        <TournamentTag>OPEN NATIONAL</TournamentTag>
        <TournamentTitle size="heading-s" weight="bold">
          Open de Luçon
        </TournamentTitle>
      </TournamentCardHeader>
      <TournamentCardBody>
        <TournamentButton type="primary">
          <Icon name="plus" size="sm" />
          Inscriptions
        </TournamentButton>
        
        <TournamentDetail>
          <Icon name="calendar" size="sm" />
          <Text size="body-m" weight="semi-bold">
            Dimanche 6 juillet 2025
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="location" size="sm" />
          <Text size="body-s" color="gray-500">
            Terrain clos de Luçon (85)
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="users" size="sm" />
          <Text size="body-s" color="gray-500">
            64 joueurs · 8 poules · élimination directe
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="award" size="sm" />
          <Text size="body-s" color="gray-500">
            500 € de lots · buvette sur place
          </Text>
        </TournamentDetail>
        
        <TournamentActions>
          <ButtonSecondary>
            S'inscrire en ligne
          </ButtonSecondary>
        </TournamentActions>
      </TournamentCardBody>
    </TournamentCard>
    
    <TournamentCard type="championship">
      <TournamentCardHeader type="championship">
        <TournamentTag>CHAMPIONNAT DÉPARTEMENTAL</TournamentTag>
        <TournamentTitle size="heading-s" weight="bold">
          Championnat D2 — J13
        </TournamentTitle>
      </TournamentCardHeader>
      <TournamentCardBody>
        <TournamentButton type="outline">
          Billetterie
        </TournamentButton>
        
        <TournamentDetail>
          <Icon name="calendar" size="sm" />
          <Text size="body-m" weight="semi-bold">
            Dimanche 29 juin 2025
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="location" size="sm" />
          <Text size="body-s" color="gray-500">
            Saint-Gilles-Croix-de-Vie (85)
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="users" size="sm" />
          <Text size="body-s" color="gray-500">
            48 joueurs · 12 clubs représentés
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="trophy" size="sm" />
          <Text size="body-s" color="gray-500">
            Titre de champion D2 en jeu
          </Text>
        </TournamentDetail>
        
        <TournamentActions>
          <ButtonSecondary>
            Voir la poule
          </ButtonSecondary>
        </TournamentActions>
      </TournamentCardBody>
    </TournamentCard>
    
    <TournamentCard type="federal">
      <TournamentCardHeader type="federal">
        <TournamentTag>ÉVÉNEMENT FÉDÉRAL</TournamentTag>
        <TournamentTitle size="heading-s" weight="bold">
          Internationaux de Vendée
        </TournamentTitle>
      </TournamentCardHeader>
      <TournamentCardBody>
        <TournamentButton type="disabled">
          Complet
        </TournamentButton>
        
        <TournamentDetail>
          <Icon name="calendar" size="sm" />
          <Text size="body-m" weight="semi-bold">
            15–17 août 2025
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="location" size="sm" />
          <Text size="body-s" color="gray-500">
            La Roche-sur-Yon — 4 terrains
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="users" size="sm" />
          <Text size="body-s" color="gray-500">
            128 joueurs · délégations FR + MX
          </Text>
        </TournamentDetail>
        
        <TournamentDetail>
          <Icon name="video" size="sm" />
          <Text size="body-s" color="gray-500">
            Streaming + live score
          </Text>
        </TournamentDetail>
        
        <TournamentActions>
          <ButtonSecondary>
            Suivre en direct
          </ButtonSecondary>
        </TournamentActions>
      </TournamentCardBody>
    </TournamentCard>
  </TournamentsGrid>
</Section>
```

---

### 7. Best Players Section

**Purpose**: Highlight top-performing players

```jsx
<Section>
  <SectionHeader>
    <SectionTitle size="heading-l" weight="bold">
      LES MEILLEURS JOUEURS
    </SectionTitle>
    <SectionSubtitle size="body-m" color="gray-500">
      Statistiques de la saison
    </SectionSubtitle>
  </SectionHeader>
  
  <PlayersGrid>
    <PlayerCard accent="gold">
      <PlayerCardAccent color="gold" />
      <PlayerAvatar initials="JM" />
      <PlayerCardContent>
        <PlayerName size="heading-s" weight="bold">
          Jean Morice
        </PlayerName>
        <PlayerClub size="body-s" color="gray-500">
          La Roche-sur-Yon · Div. 1
        </PlayerClub>
        <PlayerBadge type="mvp">
          ★ Joueur MVP
        </PlayerBadge>
        <PlayerStats>
          <StatItem>
            <StatLabel>Points marqués :</StatLabel>
            <StatValue>412</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Moyenne / manche :</StatLabel>
            <StatValue>11,4</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Lancers gagnants :</StatLabel>
            <StatValue>68%</StatValue>
          </StatItem>
        </PlayerStats>
      </PlayerCardContent>
    </PlayerCard>
    
    <PlayerCard accent="primary">
      <PlayerCardAccent color="primary" />
      <PlayerAvatar initials="PB" />
      <PlayerCardContent>
        <PlayerName>Pierre Baudry</PlayerName>
        <PlayerClub>Clos Fontenois · Div. 1</PlayerClub>
        <PlayerBadge type="top-scorer">
          Meilleur marqueur
        </PlayerBadge>
        <PlayerStats>
          <StatItem>
            <StatLabel>Points marqués :</StatLabel>
            <StatValue>445</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Moyenne / manche :</StatLabel>
            <StatValue>10,8</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Lancers gagnants :</StatLabel>
            <StatValue>64%</StatValue>
          </StatItem>
        </PlayerStats>
      </PlayerCardContent>
    </PlayerCard>
    
    <PlayerCard accent="amber">
      <PlayerCardAccent color="amber" />
      <PlayerAvatar initials="AL" />
      <PlayerCardContent>
        <PlayerName>Anaïs Léger</PlayerName>
        <PlayerClub>Luçon · Div. 1 Féminines</PlayerClub>
        <PlayerBadge type="revelation">
          Révélation 2025
        </PlayerBadge>
        <PlayerStats>
          <StatItem>
            <StatLabel>Points marqués :</StatLabel>
            <StatValue>398</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Moyenne / manche :</StatLabel>
            <StatValue>11,1</StatValue>
          </StatItem>
          <StatItem>
            <StatLabel>Lancers gagnants :</StatLabel>
            <StatValue>71%</StatValue>
          </StatItem>
        </PlayerStats>
      </PlayerCardContent>
    </PlayerCard>
  </PlayersGrid>
</Section>
```

---

### 8. Spaces Section

**Purpose**: Quick access to different user areas

```jsx
<Section bgColor="primary-800">
  <SectionHeader>
    <SectionTitle size="heading-l" weight="bold" color="gold-700">
      NOS ESPACES
    </SectionTitle>
  </SectionHeader>
  
  <SpacesGrid>
    <SpaceCard>
      <SpaceIcon name="federation" />
      <SpaceTitle size="heading-xs" weight="bold">
        Espace Fédération
      </SpaceTitle>
      <SpaceDescription size="caption" color="primary-200">
        Licences, règles, arbitres, sélections
      </SpaceDescription>
    </SpaceCard>
    
    <SpaceCard>
      <SpaceIcon name="club" />
      <SpaceTitle weight="bold">
        Espace Club
      </SpaceTitle>
      <SpaceDescription color="primary-200">
        Effectifs, résultats, gestion des équipes
      </SpaceDescription>
    </SpaceCard>
    
    <SpaceCard>
      <SpaceIcon name="player" />
      <SpaceTitle weight="bold">
        Espace Joueur
      </SpaceTitle>
      <SpaceDescription color="primary-200">
        Profil, stats, palmarès personnel
      </SpaceDescription>
    </SpaceCard>
    
    <SpaceCard>
      <SpaceIcon name="supporter" />
      <SpaceTitle weight="bold">
        Espace Supporter
      </SpaceTitle>
      <SpaceDescription color="primary-200">
        Pronostics, votes, notifications
      </SpaceDescription>
    </SpaceCard>
  </SpacesGrid>
</Section>
```

---

### 9. Footer (`/docs/design-system/components.md`)

**Purpose**: Footer with links and copyright

```jsx
<Footer>
  <FooterLeft>
    <Link href="/" color="primary-200">
      Fédération de Palet Vendéen
    </Link>
    <Link href="/mentions-legales" color="primary-200">
      Mentions légales
    </Link>
    <Link href="/rgpd" color="primary-200">
      RGPD
    </Link>
    <Link href="/contact" color="primary-200">
      Contact
    </Link>
  </FooterLeft>
  <FooterRight>
    <Text size="body-xs" color="primary-200">
      Maquette — Page d'accueil
    </Text>
  </FooterRight>
</Footer>
```

---

## Data Requirements

### Live Match Data
```json
{
  "match": {
    "id": "match-123",
    "competition": "Championnat de Vendée - Division 1",
    "type": "Finale",
    "date": "2025-06-14",
    "time": "15:00",
    "location": "Terrain clos de La Roche-sur-Yon",
    "status": "live",
    "currentManche": 24,
    "team1": {
      "id": "team-1",
      "name": "La Roche-sur-Yon Palet Club",
      "abbr": "PC",
      "logo": "logo-pc.svg",
      "pool": "A",
      "score": 72
    },
    "team2": {
      "id": "team-2",
      "name": "Clos Fontenois",
      "abbr": "FC",
      "logo": "logo-cf.svg",
      "pool": "B",
      "score": 68
    },
    "info": "Terrain clos municipal — entrée gratuite · buvette et fan zone"
  }
}
```

---

### Live Scores Data
```json
{
  "liveScores": {
    "journee": 12,
    "matches": [
      {
        "id": "match-456",
        "team1": {"name": "Luçon PC", "score": 45},
        "team2": {"name": "Les Sables", "score": 38},
        "status": "in_progress",
        "manche": 18
      },
      {
        "id": "match-789",
        "team1": {"name": "Challans", "score": 52},
        "team2": {"name": "Montaigu", "score": 52},
        "status": "in_progress",
        "manche": 21
      },
      {
        "id": "match-101",
        "team1": {"name": "Saint-Gilles", "score": 61},
        "team2": {"name": "Pouzaugues", "score": 44},
        "status": "finished",
        "manche": 24
      }
    ]
  }
}
```

---

### Classification Data
```json
{
  "classification": {
    "competition": "Championnat de Vendée - Division 1",
    "journee": 12,
    "season": "2024-2025",
    "teams": [
      {
        "position": 1,
        "team": {
          "id": "team-1",
          "name": "La Roche-sur-Yon Palet Club",
          "abbr": "LR",
          "logo": "logo-lr.svg"
        },
        "matches": 12,
        "wins": 10,
        "losses": 2,
        "diff": 84,
        "points": 30,
        "form": ["win", "win", "win", "loss", "win"],
        "status": "qualified"
      },
      {
        "position": 2,
        "team": {
          "id": "team-2",
          "name": "Clos Fontenois",
          "abbr": "CF",
          "logo": "logo-cf.svg"
        },
        "matches": 12,
        "wins": 9,
        "losses": 3,
        "diff": 61,
        "points": 28,
        "form": ["win", "loss", "win", "win", "win"],
        "status": "qualified"
      },
      {
        "position": 3,
        "team": {
          "id": "team-3",
          "name": "Luçon Palet Club",
          "abbr": "LU",
          "logo": "logo-lu.svg"
        },
        "matches": 12,
        "wins": 8,
        "losses": 4,
        "diff": 38,
        "points": 26,
        "form": ["draw", "win", "win", "win", "loss"],
        "status": "barrage"
      }
      // More teams...
    ]
  }
}
```

---

### Tournaments Data
```json
{
  "tournaments": [
    {
      "id": "tour-1",
      "type": "open",
      "name": "Open de Luçon",
      "category": "OPEN NATIONAL",
      "date": "2025-07-06",
      "location": "Terrain clos de Luçon (85)",
      "players": 64,
      "pools": 8,
      "format": "élimination directe",
      "prizes": "500 € de lots",
      "features": ["buvette sur place"],
      "status": "open",
      "registrationUrl": "/inscription/open-lucon"
    },
    {
      "id": "tour-2",
      "type": "championship",
      "name": "Championnat D2 — J13",
      "category": "CHAMPIONNAT DÉPARTEMENTAL",
      "date": "2025-06-29",
      "location": "Saint-Gilles-Croix-de-Vie (85)",
      "players": 48,
      "clubs": 12,
      "features": ["Titre de champion D2 en jeu"],
      "status": "open",
      "ticketUrl": "/billetterie/championnat-d2"
    },
    {
      "id": "tour-3",
      "type": "federal",
      "name": "Internationaux de Vendée",
      "category": "ÉVÉNEMENT FÉDÉRAL",
      "date": "2025-08-15 to 2025-08-17",
      "location": "La Roche-sur-Yon — 4 terrains",
      "players": 128,
      "features": ["délégations FR + MX", "Streaming + live score"],
      "status": "full",
      "streamUrl": "/live/internationaux"
    }
  ]
}
```

---

### Players Data
```json
{
  "players": [
    {
      "id": "player-1",
      "firstName": "Jean",
      "lastName": "Morice",
      "initials": "JM",
      "club": "La Roche-sur-Yon Palet Club",
      "division": "Div. 1",
      "photo": "jean-morice.jpg",
      "badges": ["MVP"],
      "stats": {
        "points": 412,
        "averagePerManche": 11.4,
        "winPercentage": 68
      }
    },
    {
      "id": "player-2",
      "firstName": "Pierre",
      "lastName": "Baudry",
      "initials": "PB",
      "club": "Clos Fontenois",
      "division": "Div. 1",
      "photo": "pierre-baudry.jpg",
      "badges": ["Meilleur marqueur"],
      "stats": {
        "points": 445,
        "averagePerManche": 10.8,
        "winPercentage": 64
      }
    },
    {
      "id": "player-3",
      "firstName": "Anaïs",
      "lastName": "Léger",
      "initials": "AL",
      "club": "Luçon",
      "division": "Div. 1 Féminines",
      "photo": "anais-leger.jpg",
      "badges": ["Révélation 2025"],
      "stats": {
        "points": 398,
        "averagePerManche": 11.1,
        "winPercentage": 71
      }
    }
  ]
}
```

---

## API Endpoints

| Data | Endpoint | Method | Parameters |
|------|----------|--------|------------|
| Live Match | `/api/matches/live` | GET | - |
| Live Scores | `/api/matches/live-scores` | GET | - |
| Classification | `/api/championships/{id}/classification` | GET | season, division |
| Tournaments | `/api/tournaments` | GET | status, type, limit |
| Tournament Detail | `/api/tournaments/{id}` | GET | - |
| Players | `/api/players/top` | GET | limit, division |
| Player Detail | `/api/players/{id}` | GET | - |

---

## Responsive Adaptations

### Mobile (< 768px)

1. **Top Bar**: Simplified, maybe hide "Fédération de Palet Vendéen"
2. **Header**: 
   - Logo smaller (40px)
   - Navigation collapses to hamburger menu
   - "S'inscrire" button becomes full-width or icon-only
3. **Match Hero**:
   - Stacked layout (teams above score)
   - Smaller team logos (120px)
   - Score font size reduced (32px)
4. **Live Scores**: Single column, vertical stack
5. **Classification Table**: Horizontal scroll for full table
6. **Tournament Cards**: Single column, stacked
7. **Player Cards**: Single column, stacked
8. **Spaces**: 2 columns, stacked on very small screens

---

### Tablet (768px - 1023px)

1. **Match Hero**: Teams side by side, but with less space
2. **Tournament Cards**: 2 columns
3. **Player Cards**: 2 columns
4. **Spaces**: 2 columns

---

### Desktop (1024px - 1279px)

1. **Match Hero**: Full layout as in maquette
2. **Tournament Cards**: 3 columns
3. **Player Cards**: 3 columns
4. **Spaces**: 2 or 4 columns

---

### Wide Desktop (1280px+)

1. All layouts as in the maquette (1440px base)
2. Consider adding more content or larger spacing

---

## Accessibility Features

1. **Live Region**: Use ARIA live region for scores
   ```html
   <div aria-live="polite" aria-atomic="true">
     <span class="match-score-primary">72</span>
   </div>
   ```

2. **Skip Links**: Add skip to main content
   ```html
   <a href="#main-content" class="skip-link">
     Aller au contenu principal
   </a>
   ```

3. **Semantic HTML**: Use proper heading hierarchy
4. **Alt Text**: All images have descriptive alt text
5. **Focus Management**: Visible focus states for all interactive elements
6. **Color Contrast**: All text meets WCAG AA standards

---

## Performance Optimizations

1. **Lazy Loading**: Load tournament and player cards as user scrolls
2. **Image Optimization**: Use WebP format, proper sizing
3. **Critical CSS**: Inline styles for above-the-fold content
4. **Font Loading**: Preload Inter font
5. **SVG Optimization**: All icons optimized with SVGO
6. **Code Splitting**: Load page-specific JavaScript when needed

---

## SEO Considerations

1. **Page Title**: "Palet Vendéen - Sports, Classements, Tournois en Vendée"
2. **Meta Description**: "Suivez les championnats, tournois et classements du palet vendéen. Inscriptions en ligne, résultats en direct, meilleurs joueurs."
3. **Open Graph**: Share previews for tournaments and matches
4. **Structured Data**: Schema.org for events, organizations, persons
5. **Canonical URL**: Prevent duplicate content issues

---

## Interaction Patterns

### 1. Live Score Updates
- **Auto-refresh**: Every 30 seconds for live matches
- **Visual indicator**: Fade or slide animation on score change
- **Sound option**: Optional audio notification for goal/point

### 2. Tournament Registration
- **Form validation**: Real-time validation
- **Step-by-step**: Multi-step form for complex registrations
- **Confirmation**: Email confirmation with calendar invite

### 3. Match Detail Navigation
- **Click on match**: Opens detailed match page
- **Hover effects**: Team cards slightly lift on hover
- **Quick stats**: Tooltip with team stats on hover

### 4. Filtering and Sorting
- **Classification table**: Sortable columns (click on header)
- **Tournament filters**: Dropdown or modal filter panel
- **Search**: Real-time search results

---

## Error States

### 1. No Live Matches
```jsx
<EmptyState type="no-live-matches">
  <Icon name="calendar" size="xl" />
  <Text size="heading-l" weight="bold">
    Aucun match en direct
  </Text>
  <Text size="body-m" color="gray-500">
    Le prochain match commence dans 2 jours
  </Text>
  <ButtonPrimary>
    Voir le calendrier
  </ButtonPrimary>
</EmptyState>
```

### 2. No Tournaments
```jsx
<EmptyState type="no-tournaments">
  <Icon name="tournament" size="xl" />
  <Text size="heading-l" weight="bold">
    Aucun tournoi à venir
  </Text>
  <Text size="body-m" color="gray-500">
    Revenez bientôt pour voir les prochains événements
  </Text>
</EmptyState>
```

### 3. Loading State
```jsx
<LoadingState>
  <Spinner size="xl" />
  <Text size="body-m" color="gray-500">
    Chargement des données...
  </Text>
</LoadingState>
```

### 4. Error State
```jsx
<ErrorState>
  <Icon name="alert" size="xl" color="danger" />
  <Text size="heading-l" weight="bold" color="danger">
    Erreur de chargement
  </Text>
  <Text size="body-m" color="gray-500">
    Impossible de charger les données. Veuillez réessayer.
  </Text>
  <ButtonPrimary onClick={retry}>
    Réessayer
  </ButtonPrimary>
</ErrorState>
```

---

## Analytics Events

Track these user interactions:

1. **Match View**: `match_viewed` (match_id, competition, date)
2. **Tournament Click**: `tournament_clicked` (tournament_id, type)
3. **Player Click**: `player_clicked` (player_id, club)
4. **Registration Start**: `registration_started` (tournament_id, type)
5. **Registration Complete**: `registration_completed` (tournament_id, user_id)
6. **Space Navigation**: `space_navigated` (space_type)
7. **Live Score View**: `live_score_viewed` (match_id)

---

## Summary

| Section | Component | Purpose | Data Source |
|---------|-----------|---------|-------------|
| Top Bar | TopBar | Live status, auth | User session |
| Header | Header | Navigation, branding | - |
| Hero | MatchHero | Featured live match | `/api/matches/live` |
| Live Scores | LiveScores | Current scores | `/api/matches/live-scores` |
| Classification | ClassificationTable | Standings | `/api/championships/{id}/classification` |
| Tournaments | TournamentsGrid | Upcoming events | `/api/tournaments` |
| Players | PlayersGrid | Top performers | `/api/players/top` |
| Spaces | SpacesGrid | User area access | - |
| Footer | Footer | Links, copyright | - |

---

## Implementation Checklist

- [ ] Top Bar component
- [ ] Header with navigation
- [ ] Match Hero component (most complex)
- [ ] Live Scores Ticker component
- [ ] Classification Table component
- [ ] Tournament Card component
- [ ] Player Card component
- [ ] Space Card component
- [ ] Footer component
- [ ] Responsive adaptations for all components
- [ ] Data fetching from API endpoints
- [ ] Error states for all sections
- [ ] Loading states for all sections
- [ ] Empty states for all sections
- [ ] Accessibility features
- [ ] Performance optimizations
- [ ] Analytics tracking

---

*Page specification - Homepage*
*Reference: `/docs/maquettes/homepage.svg`*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Steps:**
1. Implement the components in your framework
2. Connect to the API endpoints
3. Test responsive behavior
4. Verify accessibility
5. Optimize performance

---

**Need implementation help?**
Let me know which framework you are using (React, Vue, Svelte, etc.) and I can provide framework-specific code examples for any of these components.
