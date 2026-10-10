# Results Page - Palet Vendeen

> Page displaying match results, standings, and competition outcomes

---

## Overview

The **Results Page** provides comprehensive access to match results across all competitions:

- **Live Results**: Real-time results for ongoing competitions
- **Recent Results**: Latest match outcomes
- **Competition Results**: Organized by tournament or championship
- **Player/Team Results**: Filtered by specific participants
- **Historical Results**: Archive of past competitions
- **Statistics & Analysis**: Performance metrics and trends

**Business Context**: Results are the core data of the platform. They feed into classifications, rankings, and player statistics. Results must be accurate, auditable, and easily accessible.

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ PAGE HEADER (120px)                                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Breadcrumb]                                                 ││
│  │ RÉSULTATS                                                     ││
│  │ Suivez les résultats des compétitions en temps réel        ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ FILTERS BAR (60px)                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Type ▼] [Catégorie ▼] [Saison ▼] [Date ▼] [Search...]      ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ QUICK ACCESS (80px)                                            │
│  [Derniers résultats] [Résultats en direct] [Classements]     │
├─────────────────────────────────────────────────────────────┤
│ MAIN CONTENT                                                 │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                                                                 ││
│  │  ┌─────────────────────────┐  ┌─────────────────────────┐│
│  │  │ RÉSULTATS RÉCENTS          │  │ PROCHAINS MATCHS          ││
│  │  │                             │  │ (Optional sidebar)        ││
│  │  │ [Result Card 1]           │  │ [Next Match Card 1]       ││
│  │  │ [Result Card 2]           │  │ [Next Match Card 2]       ││
│  │  │ [Result Card 3]           │  │ [Next Match Card 3]       ││
│  │  │ ...                       │  │ ...                       ││
│  │  │                             │  │                             ││
│  │  │ [Load More / Pagination]   │  │                             ││
│  │  └─────────────────────────┘  └─────────────────────────┘││
│  │                                                                 ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ FEATURED SECTIONS                                             │
│  [Top Performers] [Biggest Upsets] [Record Breakers]           │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Page Header

```jsx
<PageHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem active>Résultats</BreadcrumbItem>
  </Breadcrumb>
  
  <PageTitle size="display-l" weight="bold">
    RÉSULTATS
  </PageTitle>
  
  <PageSubtitle size="body-l" color="gray-500">
    Suivez les résultats des compétitions de palet vendéen en temps réel
  </PageSubtitle>
</PageHeader>
```

---

### 2. Quick Access Bar

```jsx
<QuickAccessBar>
  <QuickAccessItem href="/results/recent" active={tab === 'recent'}>
    <Icon name="clock" size="sm" />
    <Text>Derniers résultats</Text>
    {recentCount > 0 && <Badge>{recentCount}</Badge>}
  </QuickAccessItem>
  
  <QuickAccessItem href="/results/live" active={tab === 'live'}>
    <Icon name="zap" size="sm" />
    <Text>Résultats en direct</Text>
    {liveCount > 0 && <Badge live>{liveCount}</Badge>}
  </QuickAccessItem>
  
  <QuickAccessItem href="/results/standings" active={tab === 'standings'}>
    <Icon name="trophy" size="sm" />
    <Text>Classements</Text>
  </QuickAccessItem>
  
  <QuickAccessItem href="/results/calendar" active={tab === 'calendar'}>
    <Icon name="calendar" size="sm" />
    <Text>Calendrier complet</Text>
  </QuickAccessItem>
</QuickAccessBar>
```

---

### 3. Filters Bar

```jsx
<ResultsFilters>
  <FilterGroup>
    <FilterLabel>Type de compétition</FilterLabel>
    <Select value={typeFilter} onChange={setTypeFilter}>
      <Option value="all">Toutes les compétitions</Option>
      <Option value="championship">Championnats</Option>
      <Option value="tournament">Tournois</Option>
      <Option value="cup">Coupe de France</Option>
      <Option value="friendly">Matchs amicaux</Option>
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Catégorie</FilterLabel>
    <Select value={categoryFilter} onChange={setCategoryFilter}>
      <Option value="all">Toutes les catégories</Option>
      <Option value="individual">Individuel</Option>
      <Option value="doublette">Doublette</Option>
      <Option value="triplette">Triplette</Option>
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Saison</FilterLabel>
    <Select value={seasonFilter} onChange={setSeasonFilter}>
      <Option value="all">Toutes les saisons</Option>
      <Option value="2025-2026">2025/2026</Option>
      <Option value="2024-2025">2024/2025</Option>
      <Option value="2023-2024">2023/2024</Option>
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Division</FilterLabel>
    <Select value={divisionFilter} onChange={setDivisionFilter}>
      <Option value="all">Toutes les divisions</Option>
      <Option value="D1">Division 1</Option>
      <Option value="D2">Division 2</Option>
      <Option value="D3">Division 3</Option>
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Date</FilterLabel>
    <DateRangePicker 
      value={[startDate, endDate]} 
      onChange={setDateRange}
      presets={[
        { label: 'Aujourd\'hui', value: [today, today] },
        { label: 'Hier', value: [yesterday, yesterday] },
        { label: '7 derniers jours', value: [subDays(today, 7), today] },
        { label: '30 derniers jours', value: [subDays(today, 30), today] },
        { label: 'Cette saison', value: [seasonStart, today] },
      ]}
    />
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Rechercher</FilterLabel>
    <Input 
      type="search" 
      placeholder="Joueur, équipe, club..." 
      value={searchQuery} 
      onChange={setSearchQuery}
    />
  </FilterGroup>
  
  <FilterActions>
    <ButtonSecondary onClick={resetFilters}>
      <Icon name="refresh" size="sm" /> Réinitialiser
    </ButtonSecondary>
  </FilterActions>
</ResultsFilters>
```

---

### 4. Result Card

```jsx
<ResultCard match={match} detailed={detailed}>
  <CardHeader>
    <CompetitionInfo>
      <CompetitionName size="body-s" color="gray-500">
        {match.competition.name}
      </CompetitionName>
      <CompetitionType size="body-xs" uppercase>
        {match.competition.type === 'championship' && `Championnat - Journée ${match.journee}`}
        {match.competition.type === 'tournament' && `Tournoi - ${match.phase}`}
        {match.competition.type === 'cup' && `Coupe de France - ${match.round}`}
      </CompetitionType>
    </CompetitionInfo>
    
    <MatchDateTime>
      <Icon name="calendar" size="xs" />
      <Text size="body-s" color="gray-500">{formatDate(match.date)}</Text>
      {match.time && (
        <>
          <Text size="body-s" color="gray-500"> à {match.time}</Text>
        </>
      )}
    </MatchDateTime>
  </CardHeader>
  
  <CardMain>
    <TeamsSection>
      <Team side="home">
        <TeamLogo src={match.team1.logo} size="sm" />
        <TeamName size="body-l" weight="bold">{match.team1.name}</TeamName>
        {detailed && (
          <TeamPlayers>
            {match.team1.players.map(player => (
              <PlayerName key={player.id} size="body-xs">
                {player.firstName} {player.lastName}
              </PlayerName>
            ))}
          </TeamPlayers>
        )}
      </Team>
      
      <ScoreSection>
        <Score value={match.team1.score} winner={match.winner === 'team1'} />
        <ScoreSeparator />
        <Score value={match.team2.score} winner={match.winner === 'team2'} />
        
        {detailed && match.manches && (
          <MancheScores>
            {match.manches.map((manche, index) => (
              <MancheScore key={index}>
                <Text size="caption">{manche.team1}</Text>
                <Text size="caption">-</Text>
                <Text size="caption">{manche.team2}</Text>
              </MancheScore>
            ))}
          </MancheScores>
        )}
      </ScoreSection>
      
      <Team side="away">
        <TeamLogo src={match.team2.logo} size="sm" />
        <TeamName size="body-l" weight="bold">{match.team2.name}</TeamName>
        {detailed && (
          <TeamPlayers>
            {match.team2.players.map(player => (
              <PlayerName key={player.id} size="body-xs">
                {player.firstName} {player.lastName}
              </PlayerName>
            ))}
          </TeamPlayers>
        )}
      </Team>
    </TeamsSection>
    
    {detailed && (
      <MatchDetails>
        <DetailItem>
          <Icon name="location" size="xs" />
          <Text size="body-xs">{match.location}</Text>
        </DetailItem>
        <DetailItem>
          <Icon name="target" size="xs" />
          <Text size="body-xs">
            {match.manches?.length || match.bestOf} manches
          </Text>
        </DetailItem>
        <DetailItem>
          <Icon name="clock" size="xs" />
          <Text size="body-xs">{match.duration}</Text>
        </DetailItem>
      </MatchDetails>
    )}
  </CardMain>
  
  <CardFooter>
    <StatusSection>
      <StatusBadge status={match.status}>
        {match.status === 'completed' && 'Terminé'}
        {match.status === 'in_progress' && 'En direct'}
        {match.status === 'postponed' && 'Reporté'}
        {match.status === 'cancelled' && 'Annulé'}
        {match.status === 'disputed' && 'Contesté'}
      </StatusBadge>
      
      {match.verified && (
        <VerifiedBadge>
          <Icon name="check-circle" size="xs" /> Vérifié
        </VerifiedBadge>
      )}
    </StatusSection>
    
    <ActionsSection>
      <ButtonGhost size="sm" onClick={() => viewMatchDetails(match.id)}>
        Voir le détail <Icon name="arrow-right" size="xs" />
      </ButtonGhost>
      
      {match.canEdit && (
        <ButtonGhost size="sm" onClick={() => editResult(match.id)}>
          <Icon name="edit" size="xs" />
        </ButtonGhost>
      )}
      
      <ButtonGhost size="sm" onClick={() => shareResult(match.id)}>
        <Icon name="share" size="xs" />
      </ButtonGhost>
    </ActionsSection>
  </CardFooter>
</ResultCard>
```

---

### 5. Recent Results Grid

```jsx
<RecentResults>
  <SectionHeader>
    <SectionTitle size="heading-m">Derniers résultats</SectionTitle>
    <SectionActions>
      <ButtonSecondary size="sm" onClick={viewAllResults}>
        Voir tous les résultats
      </ButtonSecondary>
    </SectionActions>
  </SectionHeader>
  
  <ResultsGroup byDate>
    {groupedResults.map((group, date) => (
      <DateGroup key={date} date={date}>
        <DateHeader>
          <DateTitle size="heading-s">{formatDate(date)}</DateTitle>
          <DateSubtitle size="body-s" color="gray-500">
            {getDayName(date)} - {group.matches.length} matchs
          </DateSubtitle>
        </DateHeader>
        
        <ResultsGrid>
          {group.matches.map(match => (
            <ResultCard 
              key={match.id} 
              match={match} 
              detailed={false}
              compact={true}
            />
          ))}
        </ResultsGrid>
      </DateGroup>
    ))}
  </ResultsGroup>
  
  <LoadMore onClick={loadMoreResults} loading={loadingMore} />
</RecentResults>
```

---

### 6. Live Results

```jsx
<LiveResults>
  <SectionHeader>
    <SectionTitle size="heading-m">Résultats en direct</SectionTitle>
    <LiveIndicator>
      <LiveBadge>EN DIRECT</LiveBadge>
      <AutoRefreshToggle 
        refreshing={autoRefresh} 
        onToggle={toggleAutoRefresh}
      />
    </LiveIndicator>
  </SectionHeader>
  
  {liveMatches.length > 0 ? (
    <LiveMatchesGrid>
      {liveMatches.map(match => (
        <LiveMatchCard 
          key={match.id} 
          match={match} 
          currentManche={match.currentManche}
          currentScores={match.currentScores}
        />
      ))}
    </LiveMatchesGrid>
  ) : (
    <EmptyState type="no-live-matches">
      <Icon name="zap" size="xl" />
      <Text size="heading-l">Aucun match en direct</Text>
      <Text size="body-m" color="gray-500">
        Aucun match n'est actuellement en cours
      </Text>
    </EmptyState>
  )}
</LiveResults>
```

---

### 7. Competition Results

```jsx
<CompetitionResults competition={competition}>
  <CompetitionHeader>
    <CompetitionTitle size="heading-l">
      {competition.name} - {competition.season}
    </CompetitionTitle>
    <CompetitionSubtitle>
      {competition.type === 'championship' && `Journée ${competition.currentJournee}`}
      {competition.type === 'tournament' && `${competition.phase} - ${competition.format}`}
    </CompetitionSubtitle>
    
    <CompetitionActions>
      <ButtonSecondary onClick={viewCompetition}>
        Voir la compétition
      </ButtonSecondary>
      <ButtonSecondary onClick={viewStandings}>
        Voir le classement
      </ButtonSecondary>
    </CompetitionActions>
  </CompetitionHeader>
  
  {competition.type === 'championship' && (
    <ChampionshipResults 
      journee={currentJournee} 
      matches={journeeMatches} 
      onChangeJournee={setJournee}
    />
  )}
  
  {competition.type === 'tournament' && (
    <TournamentResults 
      tournament={competition} 
      phases={phases} 
      selectedPhase={selectedPhase} 
      onSelectPhase={setSelectedPhase}
    />
  )}
  
  <ResultsTabs defaultActive="all">
    <Tab id="all">Tous les résultats</Tab>
    <Tab id="by-date">Par date</Tab>
    <Tab id="by-team">Par équipe</Tab>
    <Tab id="by-player">Par joueur</Tab>
  </ResultsTabs>
  
  <TabPanel id="all">
    <AllResults matches={allMatches} pagination={pagination} />
  </TabPanel>
  
  <TabPanel id="by-date">
    <ResultsByDate matchesByDate={matchesByDate} />
  </TabPanel>
  
  <TabPanel id="by-team">
    <ResultsByTeam 
      teams={competition.teams} 
      matches={allMatches} 
      onSelectTeam={setSelectedTeam}
    />
  </TabPanel>
  
  <TabPanel id="by-player">
    <ResultsByPlayer 
      players={competitionPlayers} 
      matches={allMatches} 
      onSelectPlayer={setSelectedPlayer}
    />
  </TabPanel>
</CompetitionResults>
```

---

### 8. Standings & Rankings

```jsx
<StandingsSection>
  <SectionHeader>
    <SectionTitle size="heading-m">Classements</SectionTitle>
    <SectionActions>
      <ButtonSecondary size="sm" onClick={viewAllStandings}>
        Voir tous les classements
      </ButtonSecondary>
    </SectionActions>
  </SectionHeader>
  
  <StandingsTabs defaultActive="championship">
    <Tab id="championship">Championnat</Tab>
    <Tab id="tournament">Tournois</Tab>
    <Tab id="player">Joueurs</Tab>
    <Tab id="team">Équipes</Tab>
  </StandingsTabs>
  
  <TabPanel id="championship">
    <ChampionshipStandings 
      championships={activeChampionships} 
      onSelectChampionship={setSelectedChampionship}
    />
  </TabPanel>
  
  <TabPanel id="tournament">
    <TournamentStandings 
      tournaments={recentTournaments} 
      onSelectTournament={setSelectedTournament}
    />
  </TabPanel>
  
  <TabPanel id="player">
    <PlayerRankings 
      rankings={playerRankings} 
      categories={categories} 
      onSelectCategory={setSelectedCategory}
    />
  </TabPanel>
  
  <TabPanel id="team">
    <TeamRankings 
      rankings={teamRankings} 
      divisions={divisions} 
      onSelectDivision={setSelectedDivision}
    />
  </TabPanel>
</StandingsSection>
```

---

### 9. Statistics & Insights

```jsx
<ResultsStatistics>
  <SectionHeader>
    <SectionTitle size="heading-m">Statistiques et insights</SectionTitle>
  </SectionHeader>
  
  <StatsGrid>
    <StatCard title="Matchs joués aujourd'hui">
      <StatValue size="display-l">{stats.matchesToday}</StatValue>
      <StatChange trend={stats.matchesTodayTrend} />
    </StatCard>
    
    <StatCard title="Points marqués">
      <StatValue size="display-l">{stats.totalPoints}</StatValue>
      <StatLabel>Moyenne: {stats.avgPoints}</StatLabel>
    </StatCard>
    
    <StatCard title="Plus grand écart">
      <StatValue size="display-l">{stats.biggestMargin}</StatValue>
      <StatDetail>
        {stats.biggestMarginTeam1} {stats.biggestMarginScore1} - {stats.biggestMarginScore2} {stats.biggestMarginTeam2}
      </StatDetail>
    </StatCard>
    
    <StatCard title="Victoires à domicile">
      <StatValue size="display-l">{stats.homeWins}%</StatValue>
      <StatLabel>{stats.homeWinsCount} victoires sur {stats.homeMatches} matchs</StatLabel>
    </StatCard>
  </StatsGrid>
  
  <ChartsSection>
    <ChartCard title="Répartition des résultats">
      <ResultsDistributionChart data={resultsDistribution} />
    </ChartCard>
    
    <ChartCard title="Performance par jour de la semaine">
      <DayOfWeekChart data={performanceByDay} />
    </ChartCard>
    
    <ChartCard title="Évolution des scores">
      <ScoreTrendChart data={scoreTrend} />
    </ChartCard>
  </ChartsSection>
</ResultsStatistics>
```

---

### 10. Featured Results

```jsx
<FeaturedResults>
  <SectionHeader>
    <SectionTitle size="heading-m">À ne pas manquer</SectionTitle>
  </SectionHeader>
  
  <FeaturedGrid>
    <FeaturedCard type="top-performer" title="Meilleur marqueur">
      <PlayerHighlight player={topScorer} />
      <HighlightDetail>
        {topScorer.points} points en {topScorer.matches} matchs
      </HighlightDetail>
    </FeaturedCard>
    
    <FeaturedCard type="biggest-upset" title="Plus grosse surprise">
      <MatchHighlight match={biggestUpset} />
      <HighlightDetail>
        Victoire de {biggestUpset.winner.name} ({biggestUpset.winner.rank})
        contre {biggestUpset.loser.name} ({biggestUpset.loser.rank})
      </HighlightDetail>
    </FeaturedCard>
    
    <FeaturedCard type="record-breaker" title="Record battu">
      <RecordHighlight record={latestRecord} />
      <HighlightDetail>
        {latestRecord.player.name} a battu le record de {latestRecord.recordType}
      </HighlightDetail>
    </FeaturedCard>
    
    <FeaturedCard type="perfect-game" title="Match parfait">
      <PerfectGameHighlight match={perfectGame} />
      <HighlightDetail>
        Victoire {perfectGame.score1} - {perfectGame.score2}
      </HighlightDetail>
    </FeaturedCard>
  </FeaturedGrid>
</FeaturedResults>
```

---

## Data Requirements

### Match Object

```typescript
interface Match {
  id: string;
  competitionId: string;
  competition: Competition;
  
  // Match info
  type: 'championship' | 'tournament' | 'cup' | 'friendly';
  journee: number | null;
  phase: string | null;
  round: string | null;
  
  // Teams
  team1: MatchTeam;
  team2: MatchTeam;
  
  // Scores
  team1Score: number | null;
  team2Score: number | null;
  winner: 'team1' | 'team2' | null;
  
  // Manche scores
  manches: MancheScore[] | null;
  bestOf: number | null;
  currentManche: number | null;
  currentScores: CurrentScores | null;
  
  // Status
  status: 'scheduled' | 'in_progress' | 'completed' | 'postponed' | 'cancelled' | 'disputed';
  verified: boolean;
  verifiedBy: string | null;
  verifiedAt: Date | null;
  
  // Details
  date: Date;
  time: string | null;
  location: string | null;
  duration: string | null;
  
  // Metadata
  canEdit: boolean;
  isLive: boolean;
  isRecent: boolean;
  
  // Stats
  stats: MatchStats;
  
  // Timeline
  createdAt: Date;
  updatedAt: Date;
  startedAt: Date | null;
  endedAt: Date | null;
}

interface MatchTeam {
  id: string;
  name: string;
  logo: string | null;
  players: Player[];
  rank: number | null;
  seed: number | null;
}

interface MancheScore {
  number: number;
  team1: number;
  team2: number;
  winner: 'team1' | 'team2' | null;
}
```

---

### Results Data Structure

```typescript
interface ResultsData {
  matches: Match[];
  total: number;
  pages: number;
  currentPage: number;
  groupedByDate: Record<string, DateGroup>;
  byCompetition: Record<string, CompetitionResults>;
  byTeam: Record<string, TeamResults>;
  byPlayer: Record<string, PlayerResults>;
  
  stats: ResultsStats;
  featured: FeaturedResults;
  
  filters: ActiveFilters;
}

interface DateGroup {
  date: string;
  dayName: string;
  matches: Match[];
  count: number;
}

interface CompetitionResults {
  competition: Competition;
  matches: Match[];
  standings: Standing[];
  stats: CompetitionStats;
}

interface ResultsStats {
  matchesToday: number;
  matchesTodayTrend: number;
  totalPoints: number;
  avgPoints: number;
  biggestMargin: number;
  biggestMarginMatch: Match | null;
  homeWins: number;
  homeWinsCount: number;
  homeMatches: number;
  resultsDistribution: DistributionData;
  performanceByDay: DayPerformance[];
  scoreTrend: TrendData;
}
```

---

## API Endpoints

| Endpoint | Method | Description | Parameters |
|----------|--------|-------------|------------|
| `/api/results` | GET | List all results | type, category, season, division, dateFrom, dateTo, search, page, limit |
| `/api/results/recent` | GET | Get recent results | limit, offset |
| `/api/results/live` | GET | Get live results | - |
| `/api/results/competitions/{id}` | GET | Get competition results | competitionId, phase, journee |
| `/api/results/matches/{id}` | GET | Get match details | matchId |
| `/api/results/by-date/{date}` | GET | Get results by date | date |
| `/api/results/by-team/{id}` | GET | Get team results | teamId, competitionId, season |
| `/api/results/by-player/{id}` | GET | Get player results | playerId, competitionId, season |
| `/api/results/standings` | GET | Get current standings | competitionId, division, season |
| `/api/results/statistics` | GET | Get results statistics | timeRange, competitionId |
| `/api/results/featured` | GET | Get featured results | limit |
| `/api/results/search` | GET | Search results | query, filters |

---

## Page States

### Loading States

#### Initial Load
```jsx
<ResultsSkeleton>
  <SkeletonHeader />
  <SkeletonFilters />
  <SkeletonResults count={6} />
</ResultsSkeleton>
```

#### Loading More
```jsx
<LoadMore loading={loading}>
  <LoadingSpinner />
  <Text>Chargement...</Text>
</LoadMore>
```

### Error States

#### No Results Found
```jsx
<EmptyState type="no-results">
  <Icon name="search" size="xl" />
  <Title>Aucun résultat trouvé</Title>
  <Text>Essayez de modifier vos filtres ou votre recherche.</Text>
  <ButtonPrimary onClick={resetFilters}>
    Réinitialiser les filtres
  </ButtonPrimary>
</EmptyState>
```

#### No Live Matches
```jsx
<EmptyState type="no-live">
  <Icon name="zap" size="xl" />
  <Title>Aucun match en direct</Title>
  <Text>Aucun match n'est actuellement en cours. Revenez plus tard !</Text>
</EmptyState>
```

#### Competition Not Found
```jsx
<EmptyState type="not-found">
  <Icon name="trophy" size="xl" />
  <Title>Compétition introuvable</Title>
  <Text>La compétition que vous cherchez n'existe pas ou a été supprimée.</Text>
  <ButtonPrimary onClick={goToResults}>
    Retour aux résultats
  </ButtonPrimary>
</EmptyState>
```

---

## Responsive Design

### Mobile (< 640px)
- Single column layout
- Compact result cards
- Filters in collapsible drawer
- Simplified navigation

### Tablet (640px - 1023px)
- Two column layout for some sections
- Full width result cards
- Filters in single row with scrolling
- Side-by-side charts

### Desktop (1024px+)
- Full multi-column layout
- Sidebar for featured content
- Full width charts and tables
- All filters visible

---

## Accessibility

### Semantic HTML
```jsx
<main>
  <header>
    <h1>Résultats</h1>
    <nav aria-label="Navigation rapide">...</nav>
  </header>
  
  <section aria-labelledby="recent-results">
    <h2 id="recent-results">Derniers résultats</h2>
    ...
  </section>
  
  <section aria-labelledby="live-results">
    <h2 id="live-results">Résultats en direct</h2>
    <div role="status" aria-live="polite">...</div>
    ...
  </section>
</main>
```

### Keyboard Navigation
- All filters accessible via keyboard
- Result cards focusable
- Action buttons keyboard accessible
- Live updates announced

### Screen Reader Support
```jsx
// Live results announcements
useEffect(() => {
  if (liveMatches.length > 0) {
    const matchesText = liveMatches.map(m => 
      `${m.team1.name} ${m.currentScores?.team1 || '0'} - ${m.currentScores?.team2 || '0'} ${m.team2.name}`
    ).join(', ');
    announce(`Matchs en direct: ${matchesText}`);
  }
}, [liveMatches]);

// Filter changes
const handleFilterChange = (filter, value) => {
  setFilter(filter, value);
  announce(`Filtre modifié: ${filter} à ${value}`);
};
```

### Color Contrast
- All text meets WCAG AA standards
- Score displays have sufficient contrast
- Status badges are distinguishable

---

## Performance

### Data Fetching
```jsx
// Main results with pagination
const { data: results } = useQuery({
  queryKey: ['results', filters, page],
  queryFn: () => fetchResults(filters, page),
  staleTime: 5 * 60 * 1000, // 5 minutes
  keepPreviousData: true,
});

// Live results with frequent updates
const { data: liveResults } = useQuery({
  queryKey: ['live-results'],
  queryFn: fetchLiveResults,
  refetchInterval: 30 * 1000, // 30 seconds
  refetchOnWindowFocus: true,
});

// Recent results for cache
const { data: recentResults } = useQuery({
  queryKey: ['recent-results'],
  queryFn: () => fetchResults({ limit: 20 }),
  staleTime: 15 * 60 * 1000, // 15 minutes
});
```

### Lazy Loading
```jsx
// Load more on scroll
const loadMoreRef = useRef(null);
useIntersectionObserver(loadMoreRef, (entries) => {
  if (entries[0].isIntersecting && hasNextPage) {
    fetchNextPage();
  }
});

// Charts loaded on visibility
const { ref: chartRef } = useInViewport();
<ChartCard ref={chartRef}>
  {isInViewport && <PerformanceChart data={chartData} />}
</ChartCard>
```

---

## SEO

### Main Results Page
```jsx
<Helmet>
  <title>Résultats - Palet Vendéen</title>
  <meta name="description" content="Consultez tous les résultats des compétitions de palet vendéen : championnats, tournois, coupe de France" />
  <meta property="og:title" content="Résultats - Palet Vendéen" />
  <meta property="og:description" content="Tous les résultats des compétitions de palet vendéen en temps réel" />
  <meta property="og:type" content="website" />
  <link rel="canonical" href="https://palet-vendeen.fr/resultats" />
</Helmet>
```

### Competition Results Page
```jsx
<Helmet>
  <title>{competition.name} - Résultats - Palet Vendéen</title>
  <meta name="description" content={`Résultats du ${competition.name} ${competition.season} - ${competition.type}`} />
  <meta property="og:title" content={`${competition.name} - Résultats`} />
  <link rel="canonical" href={`https://palet-vendeen.fr/resultats/${competition.id}`} />
</Helmet>
```

---

## File Structure

```
web/src/pages/results/
├── index.jsx                      # Main results page
├── ResultsProvider.jsx           # Results context
├── RecentResults.jsx
├── LiveResults.jsx
├── CompetitionResults.jsx
├── Standings.jsx
├── Statistics.jsx
├── FeaturedResults.jsx
├── components/
│   ├── ResultsHeader.jsx
│   ├── ResultsFilters.jsx
│   ├── QuickAccessBar.jsx
│   ├── ResultCard.jsx
│   ├── LiveMatchCard.jsx
│   ├── DateGroup.jsx
│   ├── ResultsGrid.jsx
│   ├── StandingsTable.jsx
│   ├── StatCard.jsx
│   ├── FeaturedCard.jsx
│   ├── MatchDetails.jsx
│   └── EmptyState.jsx
├── hooks/
│   ├── useResults.js
│   ├── useLiveResults.js
│   ├── useCompetitionResults.js
│   ├── useStandings.js
│   └── useResultsStats.js
└── utils/
    ├── resultsFormatting.js
    └── resultsFiltering.js
```

---

## Implementation Checklist

- [ ] Page Header with breadcrumb and title
- [ ] Quick Access Bar with navigation tabs
- [ ] Filters Bar with all filter options
- [ ] Recent Results section with date grouping
- [ ] Live Results section with auto-refresh
- [ ] Competition Results section
- [ ] Standings & Rankings section
- [ ] Statistics & Insights section
- [ ] Featured Results section
- [ ] Result Card component
- [ ] Live Match Card component
- [ ] All filter functionality
- [ ] Pagination and infinite scroll
- [ ] Responsive design
- [ ] Error and loading states
- [ ] Data fetching with caching
- [ ] Live updates
- [ ] Accessibility features
- [ ] SEO optimization

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Match Page](match.md)
- [Championship Page](championnat.md)
- [Tournament Page](tournament.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)

---

*Page specification - Results Page*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Pages**: [News](news.md) | [Login](login.md)