# Player Profile Page - Palet Vendeen

> Player profile displaying statistics, history, achievements, and contact information

---

## Overview

The **Player Profile Page** provides comprehensive information about a specific player including:
- **Personal information** (name, photo, club affiliation)
- **Career statistics** (matches, wins, points, averages)
- **Tournament history** (participations, results)
- **Achievements and badges** (titles, records)
- **Performance analytics** (charts, trends)
- **Contact and social** (if public)

**Business Context**: Players can be Individuals (1v1), Doublette members, or Triplette members. Stats track individual performance across all competitions.

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ PROFILE HEADER (200px)                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  ┌──────────┐  ┌─────────────────────────────────────┐  ││
│  │  │          │  │ [Breadcrumb]                             │  ││
│  │  │  [Photo] │  │ Prenom NOM                              │  ││
│  │  │  120px  │  │ ┌─────────────────────────────────┐  │  ││
│  │  │  ✓     │  │ │ STATS SUMMARY                      │  │  ││
│  │  │        │  │ │ ┌─────┬─────┬─────┬─────┐          │  │  ││
│  │  └────────┘  │ │ │ MP  │ Vic │ Nul │ Déf │          │  │  ││
│  │              │ │ │ 452 │ 245 │ 32 │ 175 │          │  │  ││
│  │  [Badge]     │ │ └─────┴─────┴─────┴─────┘          │  │  ││
│  │  [Badge]     │ │ Points: 12,452 pts - Moy: 11.2     │  │  ││
│  │              │ └─────────────────────────────────┘  │  ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ ACTIONS BAR (50px)                                             │
│  [Edit Profile] [Share] [Download Stats] [Message]            │
├─────────────────────────────────────────────────────────────┤
│ NAVIGATION TABS (48px)                                        │
│  [Statistiques] [Historique] [Palmarès] [Équipe] [Infos]       │
├─────────────────────────────────────────────────────────────┤
│ TAB CONTENT                                                   │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ STATISTICS TAB                                             ││
│  │  ┌─────────────────┬─────────────────────────────────┐ ││
│  │  │ PERFORMANCE      │ DETAILED STATS                  │ ││
│  │  │ CHARTS          │  ┌─────────────────────────────┐│ ││
│  │  │                 │  │ Compétitions: 124             ││ ││
│  │  │ [Line Chart]     │  │ ████████░░░░ Désignation         ││ ││
│  │  │ [Bar Chart]      │  │ ██████░░░░░░ Coupe de France    ││ ││
│  │  │ [Radar Chart]    │  │ ████░░░░░░░░ Championnat         ││ ││
│  │  │                 │  │ ██░░░░░░░░░░ Tourn. Local        ││ ││
│  │  │                 │  └─────────────────────────────┘│ ││
│  │  └─────────────────┴─────────────────────────────────┘ ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Profile Header

```jsx
<ProfileHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem href="/joueurs">Joueurs</BreadcrumbItem>
    <BreadcrumbItem active>{player.firstName} {player.lastName}</BreadcrumbItem>
  </Breadcrumb>
  
  <ProfileMain>
    <ProfileAvatar src={player.photo} size="xl" status={player.status} />
    <ProfileIdentity>
      <ProfileName size="display-m" weight="bold">
        {player.firstName} {player.lastName}
      </ProfileName>
      <ProfileAlias size="body-m" color="gray-500">
        "{player.alias}"
      </ProfileAlias>
      <ProfileClub>
        <ClubLogo club={player.club} size="sm" />
        <ClubName size="body-m" weight="semibold">{player.club.name}</ClubName>
        <ClubDivision size="body-s" color="gray-500">
          {player.club.division} - {player.club.league}
        </ClubDivision>
      </ProfileClub>
    </ProfileIdentity>
    
    <ProfileStatsSummary>
      <StatsGrid>
        <StatItem label="Matchs joués">{player.stats.matchesPlayed}</StatItem>
        <StatItem label="Victoires">{player.stats.wins}</StatItem>
        <StatItem label="Nuls">{player.stats.draws}</StatItem>
        <StatItem label="Défaites">{player.stats.losses}</StatItem>
      </StatsGrid>
      <StatsHighlight>
        <Text size="body-l" weight="bold" color="primary-700">
          {player.stats.totalPoints} points
        </Text>
        <Text size="body-s" color="gray-500">
          Moyenne: {player.stats.averagePoints.toFixed(1)}
        </Text>
      </StatsHighlight>
    </ProfileStatsSummary>
  </ProfileMain>
  
  <ProfileBadges>
    {player.badgeCategories.map(category => (
      <BadgeGroup key={category.id} title={category.name}>
        {category.badges.map(badge => (
          <Badge 
            key={badge.id} 
            variant={badge.rarity}
            tooltip={badge.description}
          >
            {badge.icon && <Icon name={badge.icon} size="xs" />}
            {badge.name}
          </Badge>
        ))}
      </BadgeGroup>
    ))}
  </ProfileBadges>
</ProfileHeader>
```

---

### 2. Profile Actions Bar

```jsx
<ProfileActions>
  {isCurrentUser && (
    <>
      <ButtonPrimary onClick={handleEdit}>
        <Icon name="edit" size="sm" /> Modifier le profil
      </ButtonPrimary>
      <ButtonSecondary onClick={handleSettings}>
        <Icon name="settings" size="sm" /> Paramètres
      </ButtonSecondary>
    </>
  )}
  
  <Dropdown label="Plus d'actions">
    <DropdownItem onClick={handleShare}>
      <Icon name="share" size="sm" /> Partager le profil
    </DropdownItem>
    <DropdownItem onClick={handleDownload}>
      <Icon name="download" size="sm" /> Télécharger les stats
    </DropdownItem>
    <DropdownItem onClick={handlePrint}>
      <Icon name="printer" size="sm" /> Imprimer le profil
    </DropdownItem>
    
    {!isCurrentUser && (
      <DropdownItem onClick={handleMessage}>
        <Icon name="mail" size="sm" /> Envoyer un message
      </DropdownItem>
    )}
  </Dropdown>
</ProfileActions>
```

---

### 3. Navigation Tabs

```jsx
<ProfileTabs defaultActive="stats">
  <Tab id="stats">
    Statistiques
    {player.stats.newHighlights > 0 && <TabBadge>{player.stats.newHighlights}</TabBadge>}
  </Tab>
  <Tab id="history">Historique</Tab>
  <Tab id="achievements">Palmarès</Tab>
  <Tab id="team">Mes équipes</Tab>
  <Tab id="info">Informations</Tab>
</ProfileTabs>
```

---

### 4. Statistics Tab

**Purpose**: Display player performance statistics and analytics

```jsx
<StatisticsTab>
  <StatsLayout>
    <StatsMain>
      <StatsSection title="Évolution des performances">
        <PerformanceChart 
          type="line" 
          data={performanceData} 
          labels={timeLabels}
          series={["Points par manche", "Moyenne mobile"]}
        />
      </StatsSection>
      
      <StatsSection title="Répartition des résultats">
        <ResultsChart type="doughnut" data={resultsDistribution} />
        <ChartLegend data={resultsDistribution} />
      </StatsSection>
      
      <StatsSection title="Statistiques détaillées">
        <DetailedStatsTable>
          <StatsTableRow label="Meilleur score" value={player.stats.bestScore} />
          <StatsTableRow label="Meilleure série" value={player.stats.bestStreak} />
          <StatsTableRow label="Victoires consécutives" value={player.stats.maxWinStreak} />
          <StatsTableRow label="% de victoires" value={`${player.stats.winRate}%`} />
          <StatsTableRow label="Points par manche" value={player.stats.avgPointsPerManche.toFixed(2)} />
          <StatsTableRow label="Précision" value={`${player.stats.accuracy}%`} />
        </DetailedStatsTable>
      </StatsSection>
      
      <StatsSection title="Statistiques par compétition">
        <CompetitionStatsTable>
          {player.competitionStats.map(comp => (
            <CompetitionStatsRow 
              key={comp.competitionId}
              competition={comp.competitionName}
              matches={comp.matches}
              wins={comp.wins}
              points={comp.points}
              average={comp.average}
              bestResult={comp.bestResult}
            />
          ))}
        </CompetitionStatsTable>
      </StatsSection>
    </StatsMain>
    
    <StatsSidebar>
      <StatsCard title="Classement actuel">
        <CurrentRanking 
          rank={player.currentRank}
          division={player.currentDivision}
          points={player.currentPoints}
          previousRank={player.previousRank}
          trend={player.rankTrend}
        />
      </StatsCard>
      
      <StatsCard title="Record personnel">
        <PersonalBest 
          score={player.personalBest.score}
          tournament={player.personalBest.tournament}
          date={player.personalBest.date}
          opponent={player.personalBest.opponent}
        />
      </StatsCard>
      
      <StatsCard title="Statistiques par position">
        <PositionStats byPosition={player.stats.byPosition} />
      </StatsCard>
    </StatsSidebar>
  </StatsLayout>
</StatisticsTab>
```

---

### 5. History Tab

**Purpose**: Display match and tournament history

```jsx
<HistoryTab>
  <HistoryFilters>
    <FilterGroup>
      <FilterLabel>Type</FilterLabel>
      <Select 
        value={filterType} 
        onChange={setFilterType}
        options={[
          { value: 'all', label: 'Tous' },
          { value: 'championship', label: 'Championnat' },
          { value: 'tournament', label: 'Tournois' },
          { value: 'friendly', label: 'Amicaux' }
        ]}
      />
    </FilterGroup>
    
    <FilterGroup>
      <FilterLabel>Saison</FilterLabel>
      <Select 
        value={filterSeason} 
        onChange={setFilterSeason}
        options={player.seasons}
      />
    </FilterGroup>
    
    <FilterGroup>
      <FilterLabel>Résultat</FilterLabel>
      <Select 
        value={filterResult} 
        onChange={setFilterResult}
        options={[
          { value: 'all', label: 'Tous' },
          { value: 'win', label: 'Victoires' },
          { value: 'draw', label: 'Nuls' },
          { value: 'loss', label: 'Défaites' }
        ]}
      />
    </FilterGroup>
  </HistoryFilters>
  
  <HistoryTimeline>
    {filteredHistory.map(year => (
      <HistoryYear key={year.year} year={year.year}>
        <YearHeader>
          <YearTitle size="heading-s">{year.year}</YearTitle>
          <YearStats>
            {year.stats.matches} matchs - {year.stats.wins} victoires
          </YearStats>
        </YearHeader>
        
        <YearMatches>
          {year.matches.map(match => (
            <HistoryMatchCard 
              key={match.id}
              match={match}
              playerId={player.id}
              onViewDetails={viewMatchDetails}
            />
          ))}
        </YearMatches>
      </HistoryYear>
    ))}
  </HistoryTimeline>
  
  <HistoryPagination 
    current={currentPage} 
    total={totalPages} 
    onChange={setCurrentPage}
  />
</HistoryTab>
```

---

### 6. Achievements Tab

**Purpose**: Display player trophies, badges, and records

```jsx
<AchievementsTab>
  <AchievementsHeader>
    <AchievementsTitle size="heading-m">
      PALMARÈS DE {player.firstName}
    </AchievementsTitle>
    <AchievementsSummary>
      <SummaryItem>
        <Count size="display-m" weight="bold">{player.trophies.total}</Count>
        <Label size="body-s">Titres</Label>
      </SummaryItem>
      <SummaryItem>
        <Count size="display-m" weight="bold">{player.badges.total}</Count>
        <Label size="body-s">Badges</Label>
      </SummaryItem>
      <SummaryItem>
        <Count size="display-m" weight="bold">{player.records.total}</Count>
        <Label size="body-s">Records</Label>
      </SummaryItem>
    </AchievementsSummary>
  </AchievementsHeader>
  
  <AchievementsSections>
    <AchievementsSection title="Titres remportés">
      <TrophyGrid>
        {player.trophies.list.map(trophy => (
          <TrophyCard 
            key={trophy.id}
            trophy={trophy}
            year={trophy.year}
            competition={trophy.competition}
            category={trophy.category}
          />
        ))}
      </TrophyGrid>
    </AchievementsSection>
    
    <AchievementsSection title="Badges obtenus">
      <BadgeGrid>
        {player.badges.categories.map(category => (
          <BadgeCategory key={category.id}>
            <CategoryTitle size="heading-s">{category.name}</CategoryTitle>
            <BadgesRow>
              {category.badges.map(badge => (
                <BadgeCard 
                  key={badge.id}
                  badge={badge}
                  date={badge.date}
                  description={badge.description}
                />
              ))}
            </BadgesRow>
          </BadgeCategory>
        ))}
      </BadgeGrid>
    </AchievementsSection>
    
    <AchievementsSection title="Records personnels">
      <RecordsTable>
        {player.records.list.map(record => (
          <RecordRow 
            key={record.id}
            type={record.type}
            value={record.value}
            tournament={record.tournament}
            date={record.date}
            verified={record.verified}
          />
        ))}
      </RecordsTable>
    </AchievementsSection>
    
    <AchievementsSection title="Meilleures performances">
      <BestPerformancesList>
        {player.bestPerformances.map(perf => (
          <BestPerformance 
            key={perf.id}
            position={perf.position}
            tournament={perf.tournament}
            year={perf.year}
            result={perf.result}
          />
        ))}
      </BestPerformancesList>
    </AchievementsSection>
  </AchievementsSections>
</AchievementsTab>
```

---

### 7. Team Tab

**Purpose**: Display player's teams and teammates

```jsx
<TeamTab>
  <TeamHeader>
    <TeamTitle size="heading-m">MES ÉQUIPES</TeamTitle>
    <TeamActions>
      {isCurrentUser && (
        <ButtonPrimary onClick={handleCreateTeam}>
          Créer une équipe
        </ButtonPrimary>
      )}
    </TeamActions>
  </TeamHeader>
  
  <CurrentTeams>
    <SectionTitle size="heading-s">Équipes actuelles</SectionTitle>
    <TeamsGrid>
      {player.currentTeams.map(team => (
        <TeamCard 
          key={team.id}
          team={team}
          isCaptain={team.captainId === player.id}
          stats={team.stats}
          onViewTeam={viewTeam}
        />
      ))}
    </TeamsGrid>
  </CurrentTeams>
  
  <PastTeams>
    <SectionTitle size="heading-s">Anciennes équipes</SectionTitle>
    <PastTeamsGrid>
      {player.pastTeams.map(team => (
        <PastTeamCard 
          key={team.id}
          team={team}
          season={team.season}
          stats={team.stats}
          onViewHistory={viewTeamHistory}
        />
      ))}
    </PastTeamsGrid>
  </PastTeams>
  
  <TeammatesSection>
    <SectionTitle size="heading-s">Coéquipiers fréquents</SectionTitle>
    <TeammatesGrid>
      {player.frequentTeammates.map(teammate => (
        <TeammateCard 
          key={teammate.id}
          player={teammate.player}
          timesTogether={teammate.count}
          winRate={teammate.winRate}
          onViewProfile={viewTeammateProfile}
        />
      ))}
    </TeammatesGrid>
  </TeammatesSection>
</TeamTab>
```

---

### 8. Info Tab

**Purpose**: Display personal information and settings

```jsx
<InfoTab>
  <InfoLayout>
    <InfoMain>
      <InfoSection title="Informations personnelles">
        <InfoGrid>
          <InfoItem label="Nom complet">{player.fullName}</InfoItem>
          <InfoItem label="Pseudo">{player.alias}</InfoItem>
          <InfoItem label="Date de naissance">{formatDate(player.birthDate)}</InfoItem>
          <InfoItem label="Âge">{player.age} ans</InfoItem>
          <InfoItem label="Lieu de naissance">{player.birthPlace}</InfoItem>
          <InfoItem label="Nationalité">{player.nationality}</InfoItem>
          <InfoItem label="Main dominante">{player.dominantHand}</InfoItem>
          <InfoItem label="Style de jeu">{player.playStyle}</InfoItem>
        </InfoGrid>
      </InfoSection>
      
      <InfoSection title="Informations de contact">
        <InfoGrid>
          <InfoItem label="Email">{player.email}</InfoItem>
          <InfoItem label="Téléphone">{player.phone || 'Non renseigné'}</InfoItem>
          <InfoItem label="Adresse">{player.address || 'Non renseignée'}</InfoItem>
          <InfoItem label="Ville">{player.city}</InfoItem>
          <InfoItem label="Code postal">{player.postalCode}</InfoItem>
        </InfoGrid>
      </InfoSection>
      
      <InfoSection title="Informations sportives">
        <InfoGrid>
          <InfoItem label="Licence FNSMR">{player.licenseNumber}</InfoItem>
          <InfoItem label="Date de la licence">{formatDate(player.licenseDate)}</InfoItem>
          <InfoItem label="Club actuel">{player.club.name}</InfoItem>
          <InfoItem label="N° de club">{player.clubNumber}</InfoItem>
          <InfoItem label="Catégorie">{player.category}</InfoItem>
          <InfoItem label="Date d'inscription">{formatDate(player.registrationDate)}</InfoItem>
        </InfoGrid>
      </InfoSection>
      
      <InfoSection title="Réseaux sociaux">
        <SocialLinks>
          {player.social.facebook && (
            <SocialLink href={player.social.facebook} platform="facebook" />
          )}
          {player.social.twitter && (
            <SocialLink href={player.social.twitter} platform="twitter" />
          )}
          {player.social.instagram && (
            <SocialLink href={player.social.instagram} platform="instagram" />
          )}
        </SocialLinks>
      </InfoSection>
    </InfoMain>
    
    <InfoSidebar>
      <InfoCard title="Activité récente">
        <RecentActivityList activities={player.recentActivities} />
      </InfoCard>
      
      <InfoCard title="Préférences">
        <PreferencesList preferences={player.preferences} />
      </InfoCard>
      
      {isCurrentUser && (
        <InfoCard title="Paramètres du compte">
          <AccountSettingsLink to="/settings" />
        </InfoCard>
      )}
    </InfoSidebar>
  </InfoLayout>
</InfoTab>
```

---

## Data Requirements

### Player Object

```typescript
interface Player {
  id: string;
  firstName: string;
  lastName: string;
  fullName: string;
  alias: string | null;
  email: string;
  photo: string | null;
  status: 'active' | 'inactive' | 'suspended' | 'banned';
  
  // Personal info
  birthDate: Date;
  age: number;
  birthPlace: string | null;
  nationality: string;
  dominantHand: 'right' | 'left' | 'ambidextrous';
  playStyle: string | null;
  
  // Contact info
  phone: string | null;
  address: string | null;
  city: string | null;
  postalCode: string | null;
  
  // Club info
  club: Club;
  clubId: string;
  clubNumber: string | null;
  category: string;
  
  // License info
  licenseNumber: string;
  licenseDate: Date;
  registrationDate: Date;
  
  // Stats
  stats: PlayerStats;
  
  // Achievements
  trophies: PlayerTrophies;
  badges: PlayerBadges;
  records: PlayerRecords;
  
  // Teams
  currentTeams: PlayerTeam[];
  pastTeams: PastPlayerTeam[];
  frequentTeammates: TeammateRelation[];
  
  // History
  matchHistory: PlayerMatchHistory;
  competitionStats: CompetitionStats[];
  
  // Recent activity
  recentActivities: Activity[];
  
  // Preferences
  preferences: PlayerPreferences;
  
  // Social
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
  
  // Metadata
  createdAt: Date;
  updatedAt: Date;
  lastActiveAt: Date;
}
```

### Player Stats

```typescript
interface PlayerStats {
  matchesPlayed: number;
  wins: number;
  draws: number;
  losses: number;
  totalPoints: number;
  averagePoints: number;
  pointsPerManche: number;
  bestScore: number;
  bestStreak: number;
  maxWinStreak: number;
  winRate: number;
  accuracy: number;
  
  currentRank: number;
  currentDivision: string;
  currentPoints: number;
  previousRank: number;
  rankTrend: 'up' | 'down' | 'stable';
  newHighlights: number;
  
  byPosition: PositionStats;
  byCompetition: Record<string, CompetitionStats>;
  bySeason: Record<string, SeasonStats>;
}
```

### Player History

```typescript
interface PlayerMatchHistory {
  byYear: Record<string, YearHistory>;
  total: number;
  byType: Record<string, number>;
}

interface YearHistory {
  year: string;
  matches: MatchHistoryItem[];
  stats: {
    matches: number;
    wins: number;
    draws: number;
    losses: number;
    points: number;
  };
}
```

---

## API Endpoints

| Data | Endpoint | Method | Parameters |
|------|----------|--------|------------|
| Player profile | `/api/players/{id}` | GET | - |
| Player stats | `/api/players/{id}/stats` | GET | season, competition |
| Player history | `/api/players/{id}/history` | GET | year, type, result, page |
| Player achievements | `/api/players/{id}/achievements` | GET | category |
| Player teams | `/api/players/{id}/teams` | GET | current, past |
| Update profile | `/api/players/{id}` | PATCH | player data |
| Player photo | `/api/players/{id}/photo` | POST/PUT | photo file |
| Player search | `/api/players` | GET | query, club, category |

---

## Page States

### Loading State
```jsx
<ProfileSkeleton />
```

### Error State
```jsx
<ErrorState type="player-not-found" />
```

### Private Profile
- Show limited information for non-authenticated users
- Show full information for the player themselves or authorized users

---

## Responsive Design

### Mobile (< 640px)
- Stack profile header vertically
- Single column tabs content
- Simplified stats display
- Horizontal scroll for badges and trophies

### Tablet (640px - 1023px)
- Two column profile header
- Two column stats layout
- Grid view for trophies and badges

### Desktop (1024px+)
- Full layout as shown above
- Sidebar always visible
- All charts visible

---

## Accessibility

### Semantic Structure
```jsx
<main>
  <header>
    <h1>Profil de {player.firstName} {player.lastName}</h1>
  </header>
  <nav aria-label="Navigation du profil">...</nav>
  <section aria-labelledby="stats-section">...</section>
  <section aria-labelledby="history-section">...</section>
</main>
```

### Chart Accessibility
- Text descriptions for all visualizations
- Keyboard navigation for interactive charts
- ARIA labels for chart elements

### Color Contrast
- All text meets WCAG AA standards
- High contrast mode support

---

## Performance

### Data Fetching
```jsx
const { data: player } = useQuery({
  queryKey: ['player', playerId],
  queryFn: () => fetchPlayer(playerId),
  staleTime: 5 * 60 * 1000,
});

const { data: stats } = useQuery({
  queryKey: ['player-stats', playerId, season],
  queryFn: () => fetchPlayerStats(playerId, season),
  enabled: !!player,
});
```

### Lazy Loading
- Charts loaded on tab activation
- History loaded on tab activation
- Images lazy loaded

### Memoization
- Cache player data
- Memoize chart data processing

---

## SEO

```jsx
<Helmet>
  <title>{player.firstName} {player.lastName} - Profil Joueur - Palet Vendéen</title>
  <meta name="description" content={`Profil de ${player.firstName} ${player.lastName} - ${player.club.name}. Statistiques, historique et palmares.`} />
  <meta property="og:title" content={`${player.firstName} ${player.lastName} - Palet Vendéen`} />
  <meta property="og:description" content={`Statistiques et palmares de ${player.firstName} ${player.lastName}`} />
  <meta property="og:image" content={player.photo || defaultOgImage} />
  <link rel="canonical" href={`https://palet-vendeen.fr/joueurs/${player.id}`} />
</Helmet>
```

---

## File Structure

```
web/src/pages/player/
├── index.jsx                    # Player profile page
├── PlayerProvider.jsx           # Context for player data
├── tabs/
│   ├── StatsTab.jsx
│   ├── HistoryTab.jsx
│   ├── AchievementsTab.jsx
│   ├── TeamTab.jsx
│   └── InfoTab.jsx
├── components/
│   ├── ProfileHeader.jsx
│   ├── ProfileActions.jsx
│   ├── ProfileBadges.jsx
│   ├── StatsOverview.jsx
│   ├── PerformanceChart.jsx
│   ├── HistoryMatchCard.jsx
│   ├── TrophyCard.jsx
│   ├── BadgeCard.jsx
│   ├── TeamCard.jsx
│   ├── TeammateCard.jsx
│   └── InfoGrid.jsx
├── hooks/
│   ├── usePlayer.js
│   ├── usePlayerStats.js
│   ├── usePlayerHistory.js
│   └── usePlayerAchievements.js
└── utils/
    ├── playerStats.js
    └── playerFormatting.js
```

---

## Implementation Checklist

- [ ] Profile Header with avatar, identity, and stats summary
- [ ] Profile Actions bar
- [ ] Navigation tabs
- [ ] Statistics Tab with charts and detailed stats
- [ ] History Tab with filters and timeline
- [ ] Achievements Tab with trophies, badges, records
- [ ] Team Tab with current/past teams and teammates
- [ ] Info Tab with personal and contact information
- [ ] Responsive design for all screen sizes
- [ ] Data fetching with proper caching
- [ ] Error and loading states
- [ ] Accessibility features
- [ ] SEO optimization

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Club Page](club.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)

---

*Page specification - Player Profile Page*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Pages**: [Admin](admin.md) | [Results](results.md) | [News](news.md) | [Login](login.md)