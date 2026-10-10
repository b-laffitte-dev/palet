# Tournament Page - Palet Vendeen

> Page displaying tournament details, participants, schedule, and results

---

## Overview

The **Tournament Page** provides comprehensive information about a specific tournament, including:
- **Tournament header** with key information (name, dates, location, status)
- **Tournament details** (category, type, format, registration info)
- **Participants** list with teams/players registered
- **Schedule** with match times and locations
- **Results** (brackets, standings, or pool results)
- **Registration** section for eligible players/teams
- **Statistics** and highlights

**Inspiration**: French Cup pages on FFF website, Top 14 knockout stages, ESPN tournament pages

**Business Context**: 
- Tournaments in Palet Vendeen include: Coupe de France (Fonte/Laiton/Bois - Doublette/Individuel), Coupe des Clubs, Tournois locaux
- Format: Poules (round-robin) → Elimination directe (knockout)
- Each tournament has specific rules (13 or 15 points to win)

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ PAGE HEADER (140px)                                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Breadcrumb]                                                 ││
│  │ COUPE DE FRANCE - FONTE - DOUBLETTE                         ││
│  │ Buzay, Vendée • 15-16 Novembre 2025 • [BADGE: En cours]    ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ TOURNAMENT INFO CARDS (100px)                                  │
│  ┌──────────────┬──────────────┬──────────────┬──────────────┐│
│  │ Catégorie    │ Type         │ Participants  │ Points         ││
│  │ FONTE        │ Doublette    │ 48 équipes    │ 15            ││
│  │ 3,80m        │             │               │ pour victoire ││
│  └──────────────┴──────────────┴──────────────┴──────────────┘│
├─────────────────────────────────────────────────────────────┤
│ TOURNAMENT TABS (48px)                                         │
│  [Vue d'ensemble]  [Participants]  [Calendrier]  [Résultats]  [Stats]│
├─────────────────────────────────────────────────────────────┤
│ TAB CONTENT                                                   │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ OVERVIEW TAB                                                ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ About the tournament                                  │    ││
│  │  │ Description, format, rules                           │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Tournament bracket or phase diagram                  │    ││
│  │  │ Visual representation of tournament structure       │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Key dates and timeline                                │    ││
│  │  │ Registration deadline, match days, etc.              │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ PARTICIPANTS TAB                                          ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ [Search...]    [Filter: All ▼]     [48 teams]        │    ││
│  │  ├─────────────────────────────────────────────────┤    ││
│  │  │ Group A         Group B         Group C           │    ││
│  │  │ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ │    ││
│  │  │ │Team 1 (A1)  │ │Team 1 (B1)  │ │Team 1 (C1)  │ │    ││
│  │  │ │Team 2 (A2)  │ │Team 2 (B2)  │ │Team 2 (C2)  │ │    ││
│  │  │ │Team 3 (A3)  │ │Team 3 (B3)  │ │Team 3 (C3)  │ │    ││
│  │  │ │Team 4 (A4)  │ │Team 4 (B4)  │ │Team 4 (C4)  │ │    ││
│  │  │ └─────────────┘ └─────────────┘ └─────────────┘ │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  Pagination [1 | 2 | 3 | 4 >]                          │    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ SCHEDULE TAB                                               ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ [Calendar view]    [List view]                        │    ││
│  │  ├─────────────────────────────────────────────────┤    ││
│  │  │ PHASE DE POULES                                    │    ││
│  │  │ ┌─────────────────────────────────────────────┐│    ││
│  │  │ │ Sam 15 Nov  │  Jouée 1  │  10:00  │  Team A vs Team││    ││
│  │  │ │              │           │         │  B              ││    ││
│  │  │ ├─────────────────────────────────────────────┤│    ││
│  │  │ │ Sam 15 Nov  │  Jouée 1  │  11:00  │  Team C vs Team││    ││
│  │  │ │              │           │         │  D              ││    ││
│  │  │ └─────────────────────────────────────────────┘│    ││
│  │  │                                                         ││    ││
│  │  │ PHASE FINALE                                             ││    ││
│  │  │ ┌─────────────────────────────────────────────┐│    ││
│  │  │ │ Dim 16 Nov │  1/8 Finale │  14:00 │ Winner A vs... ││    ││
│  │  │ └─────────────────────────────────────────────┘│    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ RESULTS TAB                                                ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ [Bracket view]    [Table view]                        │    ││
│  │  ├─────────────────────────────────────────────────┤    ││
│  │  │ Poule A Standing:                                   │    ││
│  │  │ POS | Team    | MJ | G | N | P | BP | BC | DIFF │    ││
│  │  │ 1   | Team A  | 3 | 3 | 0 | 0 | 45 | 20 | +25  │    ││
│  │  │ 2   | Team B  | 3 | 2 | 0 | 1 | 40 | 25 | +15  │    ││
│  │  │ ...                                              │    ││
│  │  │                                                         ││
│  │  │ Finals Results:                                         ││
│  │  │ ┌─────────────────────────────────────────────┐ │    ││
│  │  │ │ Quart de Finale 1: Team A 15-12 Team B           │ │    ││
│  │  │ │ Quart de Finale 2: Team C 15-14 Team D           │ │    ││
│  │  │ │ Demi-finale 1: Team A 15-13 Team C              │ │    ││
│  │  │ │ FINAL: Team A 15-10 Team E                       │ │    ││
│  │  │ └─────────────────────────────────────────────┘ │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ STATISTICS TAB                                             ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Top Performers:                                     │    ││
│  │  │ [Best Scorer] [Best Accuracy] [Most Wins]        │    ││
│  │  │                                                         │    ││
│  │  │ Tournament Stats:                                    │    ││
│  │  │ Total Matches: 63  │  Total Points: 2,145         │    ││
│  │  │ Highest Score: 15-0  │  Average Points/Match: 34   │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ REGISTRATION SECTION (if open and user eligible)             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ ⚡ INSCRIPTION OUVERTE - Clôture dans 5 jours             ││
│  │ Catégorie: Doublette FONTE • Places disponibles: 4/48     ││
│  │ [Button: S'INSCRIRE MAINTENANT] [Button: Voir détails]     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Page Header

**Purpose**: Clear tournament identification and status

```jsx
<PageHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem href="/tournois">Tournois</BreadcrumbItem>
    <BreadcrumbItem active>{tournament.name}</BreadcrumbItem>
  </Breadcrumb>
  
  <PageTitle size="display-m" weight="bold">
    {tournament.name}
  </PageTitle>
  
  <PageMeta>
    <MetaItem icon="location">
      <MetaLabel>{tournament.location}</MetaLabel>
    </MetaItem>
    <MetaItem icon="calendar">
      <MetaLabel>{formatDateRange(tournament.startDate, tournament.endDate)}</MetaLabel>
    </MetaItem>
    <MetaItem>
      <TournamentStatusBadge status={tournament.status} />
    </MetaItem>
  </PageMeta>
  
  <PageActions>
    <ButtonSecondary icon="share">Partager</ButtonSecondary>
    <ButtonSecondary icon="favorite">Favoris</ButtonSecondary>
    {userCanEdit && (
      <ButtonPrimary icon="edit" href={`/admin/tournois/${tournament.id}/edit`}>
        Modifier
      </ButtonPrimary>
    )}
  </PageActions>
</PageHeader>
```

**Status Badge Styles**:
- `upcoming`: Background `gray-100`, Text `gray-800`, Icon `calendar`
- `registration_open`: Background `gold-50`, Text `gold-800`, Icon `user-plus`
- `in_progress`: Background `primary-50`, Text `primary-800`, Icon `play`
- `completed`: Background `success-50`, Text `success-800`, Icon `check`
- `cancelled`: Background `error-50`, Text `error-800`, Icon `x`

---

### 2. Tournament Info Cards

**Purpose**: Quick summary of tournament characteristics

```jsx
<InfoCards>
  <InfoCard>
    <InfoCardIcon name="target" color="primary" />
    <InfoCardLabel>Catégorie</InfoCardLabel>
    <InfoCardValue>{tournament.category}</InfoCardValue>
    <InfoCardDetail>{tournament.category === 'FONTE' ? '3,80m' : '2,80m'}</InfoCardDetail>
  </InfoCard>
  
  <InfoCard>
    <InfoCardIcon name="users" color="secondary" />
    <InfoCardLabel>Type</InfoCardLabel>
    <InfoCardValue>{tournament.type}</InfoCardValue>
    <InfoCardDetail>{tournament.type === 'DOUBLETTE' ? '2v2' : tournament.type === 'INDIVIDUEL' ? '1v1' : tournament.type === 'TRIPLETTE' ? '3v3' : ''}</InfoCardDetail>
  </InfoCard>
  
  <InfoCard>
    <InfoCardIcon name="team" color="gold" />
    <InfoCardLabel>Participants</InfoCardLabel>
    <InfoCardValue>{tournament.participantsCount} équipes</InfoCardValue>
    <InfoCardDetail>{tournament.maxParticipants} max</InfoCardDetail>
  </InfoCard>
  
  <InfoCard>
    <InfoCardIcon name="trophy" color="success" />
    <InfoCardLabel>Victoire à</InfoCardLabel>
    <InfoCardValue>{tournament.pointsToWin} points</InfoCardValue>
    <InfoCardDetail>Format {tournament.format}</InfoCardDetail>
  </InfoCard>
</InfoCards>
```

---

### 3. Tournament Tabs

**Purpose**: Navigation between different tournament views

```jsx
<TournamentTabs value={activeTab} onChange={setActiveTab}>
  <Tab value="overview" label="Vue d'ensemble" icon="eye" />
  <Tab value="participants" label="Participants" icon="users" badge={tournament.participantsCount} />
  <Tab value="schedule" label="Calendrier" icon="calendar" badge={tournament.matchesCount} />
  <Tab value="results" label="Résultats" icon="trophy" />
  <Tab value="stats" label="Statistiques" icon="chart" />
</TournamentTabs>
```

---

### 4. Overview Tab

**Purpose**: Complete tournament information

#### About Section

```jsx
<Section title="À propos du tournoi">
  <MarkdownContent>{tournament.description}</MarkdownContent>
  
  <DetailsGrid>
    <DetailItem label="Organisateur">
      <OrganizerBadge organizer={tournament.organizer} />
    </DetailItem>
    <DetailItem label="Catégorie">{tournament.category}</DetailItem>
    <DetailItem label="Type">{tournament.type}</DetailItem>
    <DetailItem label="Format">{tournament.format}</DetailItem>
    <DetailItem label="Points pour victoire">{tournament.pointsToWin}</DetailItem>
    <DetailItem label="Distance">{tournament.distance}m</DetailItem>
  </DetailsGrid>
</Section>
```

#### Tournament Structure Diagram

```jsx
<Section title="Structure du tournoi">
  <TournamentStructureDiagram>
    <Phase label="Phase de poules">
      <PoolGroup label="Poule A">
        <PoolTeam>Équipe 1</PoolTeam>
        <PoolTeam>Équipe 2</PoolTeam>
        <PoolTeam>Équipe 3</PoolTeam>
        <PoolTeam>Équipe 4</PoolTeam>
      </PoolGroup>
      <PoolGroup label="Poule B">
        <PoolTeam>Équipe 5</PoolTeam>
        <PoolTeam>Équipe 6</PoolTeam>
        <PoolTeam>Équipe 7</PoolTeam>
        <PoolTeam>Équipe 8</PoolTeam>
      </PoolGroup>
      {/* More pools */}
    </Phase>
    
    <Phase label="Phase finale" direction="right">
      <BracketRound label="1/8 Finale" teams={16} />
      <BracketRound label="1/4 Finale" teams={8} />
      <BracketRound label="1/2 Finale" teams={4} />
      <BracketRound label="Finale" teams={2} />
    </Phase>
    
    <Winner>
      <TrophyIcon />
      <WinnerText>Vainqueur</WinnerText>
    </Winner>
  </TournamentStructureDiagram>
  
  <DiagramLegend>
    <LegendItem color="primary">Phase de poules - 4 équipes par poule</LegendItem>
    <LegendItem color="success">Phase finale - Élimination directe</LegendItem>
  </DiagramLegend>
</Section>
```

#### Key Dates Timeline

```jsx
<Section title="Dates importantes">
  <Timeline>
    <TimelineItem date="2025-10-01">
      <TimelineDate>1 Octobre 2025</TimelineDate>
      <TimelineTitle>Ouverture des inscriptions</TimelineTitle>
      <TimelineDescription>Les inscriptions sont ouvertes à tous les clubs affiliés</TimelineDescription>
    </TimelineItem>
    
    <TimelineItem date="2025-11-10">
      <TimelineDate>10 Novembre 2025</TimelineDate>
      <TimelineTitle>Clôture des inscriptions</TimelineTitle>
      <TimelineDescription>Dernier jour pour s'inscrire au tournoi</TimelineDescription>
    </TimelineItem>
    
    <TimelineItem date="2025-11-14">
      <TimelineDate>14 Novembre 2025</TimelineDate>
      <TimelineTitle>Publication des poules</TimelineTitle>
      <TimelineDescription>Annonce des compositions des poules</TimelineDescription>
    </TimelineItem>
    
    <TimelineItem date="2025-11-15">
      <TimelineDate>15 Novembre 2025</TimelineDate>
      <TimelineTitle>Début du tournoi - Phase de poules</TimelineTitle>
      <TimelineDescription>Journée 1 des matchs de poule</TimelineDescription>
    </TimelineItem>
    
    <TimelineItem date="2025-11-16">
      <TimelineDate>16 Novembre 2025</TimelineDate>
      <TimelineTitle>Phase finale et finale</TimelineTitle>
      <TimelineDescription>1/8 finales, quarts, demis et finale</TimelineDescription>
    </TimelineItem>
  </Timeline>
</Section>
```

---

### 5. Participants Tab

**Purpose**: List all registered participants

```jsx
<ParticipantsSection>
  <ParticipantsHeader>
    <ParticipantsCount>{tournament.participantsCount} équipes inscrites</ParticipantsCount>
    
    <ParticipantsActions>
      <SearchInput 
        placeholder="Rechercher une équipe ou un joueur..." 
        value={searchQuery} 
        onChange={setSearchQuery}
      />
      
      <FilterGroup>
        <FilterLabel>Poule</FilterLabel>
        <Select value={poolFilter} onChange={setPoolFilter}>
          <Option value="all">Toutes les poules</Option>
          <Option value="A">Poule A</Option>
          <Option value="B">Poule B</Option>
          {/* Dynamically generated from tournament */}
        </Select>
      </FilterGroup>
      
      <FilterGroup>
        <FilterLabel>Statut</FilterLabel>
        <Select value={statusFilter} onChange={setStatusFilter}>
          <Option value="all">Tous</Option>
          <Option value="confirmed">Confirmés</Option>
          <Option value="waiting">En attente</Option>
        </Select>
      </FilterGroup>
      
      <SortSelect value={sortBy} onChange={setSortBy}>
        <Option value="name">Trier par nom</Option>
        <Option value="club">Trier par club</Option>
        <Option value="ranking">Trier par classement</Option>
      </SortSelect>
    </ParticipantsActions>
  </ParticipantsHeader>
  
  <ParticipantsGrid>
    {groupedParticipants.map(pool => (
      <PoolGroup key={pool.id}>
        <PoolHeader>
          <PoolLabel>Poule {pool.name}</PoolLabel>
          <PoolStatus>{pool.status}</PoolStatus>
        </PoolHeader>
        
        <PoolTable>
          <TableHeader>
            <TableCell>Pos</TableCell>
            <TableCell>Équipe</TableCell>
            <TableCell>Club</TableCell>
            <TableCell>Joueurs</TableCell>
            <TableCell>MJ</TableCell>
            <TableCell>G</TableCell>
            <TableCell>Pts</TableCell>
            <TableCell>Bp</TableCell>
            <TableCell>Diff</TableCell>
          </TableHeader>
          
          {pool.teams.map((team, index) => (
            <TableRow key={team.id}>
              <TableCell>{index + 1}</TableCell>
              <TableCell>
                <TeamCell team={team} />
              </TableCell>
              <TableCell>
                <ClubBadge club={team.club} />
              </TableCell>
              <TableCell>
                {team.players.map(player => (
                  <PlayerChip key={player.id} player={player} />
                ))}
              </TableCell>
              <TableCell>{team.stats.matchesPlayed}</TableCell>
              <TableCell>{team.stats.wins}</TableCell>
              <TableCell><strong>{team.stats.points}</strong></TableCell>
              <TableCell>{team.stats.pointsFor}</TableCell>
              <TableCell>{team.stats.diff}</TableCell>
            </TableRow>
          ))}
        </PoolTable>
      </PoolGroup>
    ))}
  </ParticipantsGrid>
  
  <Pagination 
    currentPage={currentPage} 
    totalPages={totalPages} 
    onPageChange={setCurrentPage}
  />
</ParticipantsSection>
```

**Pool Group Component**:
```jsx
const PoolGroup = ({ pool }) => {
  const isQualified = pool.qualifiedTeams && pool.qualifiedTeams.length > 0;
  
  return (
    <div className="pool-group">
      <PoolHeader>
        <PoolLabel>Poule {pool.name}</PoolLabel>
        <PoolStatus>
          {pool.status === 'complete' && 'Complète'}
          {pool.status === 'in_progress' && 'En cours'}
          {pool.status === 'upcoming' && 'À venir'}
        </PoolStatus>
        {isQualified && (
          <QualifiedBadge count={pool.qualifiedTeams.length} />
        )}
      </PoolHeader>
      <PoolTable>{/* Team rows */}</PoolTable>
    </div>
  );
};
```

---

### 6. Schedule Tab

**Purpose**: Display tournament match schedule

```jsx
<ScheduleSection>
  <ScheduleHeader>
    <ScheduleTitle>Calendrier du tournoi</ScheduleTitle>
    
    <ScheduleActions>
      <ViewToggle 
        value={viewMode} 
        onChange={setViewMode}
        options={[
          { value: 'calendar', label: 'Calendrier', icon: 'calendar' },
          { value: 'list', label: 'Liste', icon: 'list' }
        ]} 
      />
      
      <PhaseFilter 
        value={phaseFilter} 
        onChange={setPhaseFilter}
        phases={tournament.phases}
      />
      
      <DateRangePicker 
        value={dateRange} 
        onChange={setDateRange}
        minDate={tournament.startDate}
        maxDate={tournament.endDate}
      />
    </ScheduleActions>
  </ScheduleHeader>
  
  {viewMode === 'calendar' ? (
    <CalendarView>
      <Calendar 
        events={matchesAsCalendarEvents} 
        dateRange={{ start: tournament.startDate, end: tournament.endDate }}
      />
    </CalendarView>
  ) : (
    <ScheduleList>
      {tournament.phases.map(phase => (
        <PhaseSection key={phase.id}>
          <PhaseHeader>
            <PhaseTitle>{phase.name}</PhaseTitle>
            <PhaseDates>{phase.startDate} - {phase.endDate}</PhaseDates>
          </PhaseHeader>
          
          {phase.rounds.map(round => (
            <RoundSection key={round.id}>
              <RoundHeader>
                <RoundTitle>{round.name}</RoundTitle>
              </RoundHeader>
              
              <MatchList>
                {round.matches.map(match => (
                  <MatchCard 
                    key={match.id}
                    match={match}
                    tournament={tournament}
                    showDetails
                    showLocation
                    showResult
                  />
                ))}
              </MatchList>
            </RoundSection>
          ))}
        </PhaseSection>
      ))}
    </ScheduleList>
  )}
</ScheduleSection>
```

**Match Card (Schedule Version)**:
```jsx
const ScheduleMatchCard = ({ match, tournament }) => {
  const hasResult = match.status === 'completed';
  
  return (
    <Card className="schedule-match-card">
      <CardHeader>
        <MatchDateTime>
          <MatchDate>{formatDate(match.date)}</MatchDate>
          <MatchTime>{formatTime(match.time)}</MatchTime>
        </MatchDateTime>
        
        <MatchStatusBadge status={match.status} />
        
        <MatchLocation>
          <LocationIcon />
          <LocationText>{match.location}</LocationText>
        </MatchLocation>
      </CardHeader>
      
      <CardBody>
        <MatchTeams>
          <TeamSide>
            <TeamLogo team={match.team1} size="sm" />
            <TeamName>{match.team1.name}</TeamName>
            {hasResult && <TeamScore>{match.score1}</TeamScore>}
          </TeamSide>
          
          <MatchSeparator>vs</MatchSeparator>
          
          <TeamSide>
            <TeamLogo team={match.team2} size="sm" />
            <TeamName>{match.team2.name}</TeamName>
            {hasResult && <TeamScore>{match.score2}</TeamScore>}
          </TeamSide>
        </MatchTeams>
        
        {!hasResult && (
          <MatchMeta>
            <MetaItem>
              <MetaIcon name="clock" />
              <MetaText>Durée estimée: 1h</MetaText>
            </MetaItem>
            {match.referee && (
              <MetaItem>
                <MetaIcon name="user" />
                <MetaText>Arbitre: {match.referee}</MetaText>
              </MetaItem>
            )}
          </MatchMeta>
        )}
      </CardBody>
      
      <CardFooter>
        <MatchActions>
          <ButtonGhost icon="eye" onClick={() => navigate(`/match/${match.id}`)}>
            Détails
          </ButtonGhost>
          {userIsAdmin && (
            <ButtonGhost icon="edit" onClick={() => navigate(`/admin/match/${match.id}/edit`)}>
              Modifier
            </ButtonGhost>
          )}
        </MatchActions>
      </CardFooter>
    </Card>
  );
};
```

---

### 7. Results Tab

**Purpose**: Display tournament results and final standings

```jsx
<ResultsSection>
  <ResultsHeader>
    <ResultsTitle>Résultats du tournoi</ResultsTitle>
    
    <ResultsActions>
      <ViewToggle 
        value={resultsView} 
        onChange={setResultsView}
        options={[
          { value: 'bracket', label: 'Tableau', icon: 'tree' },
          { value: 'table', label: 'Tableau', icon: 'table' },
          { value: 'list', label: 'Liste', icon: 'list' }
        ]} 
      />
      
      <PhaseFilter 
        value={phaseFilter} 
        onChange={setPhaseFilter}
        phases={tournament.phases}
      />
    </ResultsActions>
  </ResultsHeader>
  
  {resultsView === 'bracket' ? (
    <BracketView>
      <SingleEliminationBracket 
        matches={bracketMatches} 
        winner={tournament.winner}
      />
    </BracketView>
  ) : resultsView === 'table' ? (
    <ResultsTables>
      {tournament.phases.map(phase => (
        <PhaseResultsTable 
          key={phase.id}
          phase={phase}
          showQualified={phase.hasQualification}
        />
      ))}
    </ResultsTables>
  ) : (
    <ResultsList>
      <MatchResultsList 
        matches={allMatches}
        showRound
        showPhase
        showDate
      />
    </ResultsList>
  )}
</ResultsSection>
```

**Single Elimination Bracket**:
```jsx
const SingleEliminationBracket = ({ matches, winner }) => {
  // Group matches by round
  const rounds = groupMatchesByRound(matches);
  
  return (
    <BracketContainer>
      {rounds.map((round, roundIndex) => (
        <BracketRound 
          key={round.name} 
          round={round} 
          roundIndex={roundIndex}
          totalRounds={rounds.length}
        >
          <RoundLabel>{round.name}</RoundLabel>
          
          {round.matches.map((match, matchIndex) => (
            <BracketMatch 
              key={match.id}
              match={match}
              position={matchIndex}
              totalMatches={round.matches.length}
              isFinal={round.name === 'Finale'}
            />
          ))}
        </BracketRound>
      ))}
      
      {winner && (
        <WinnerDisplay>
          <TrophyIcon />
          <WinnerName>{winner.name}</WinnerName>
          <WinnerText>Vainqueur du tournoi</WinnerText>
        </WinnerDisplay>
      )}
    </BracketContainer>
  );
};
```

**Bracket Match Component**:
```jsx
const BracketMatch = ({ match, position, totalMatches, isFinal }) => {
  const winner = getMatchWinner(match);
  const isCompleted = match.status === 'completed';
  
  return (
    <BracketMatchContainer 
      className={classNames({
        'is-completed': isCompleted,
        'is-final': isFinal
      })}
    >
      <MatchTeamsVertical>
        <TeamBracketSlot 
          team={match.team1} 
          score={isCompleted ? match.score1 : null}
          isWinner={winner === 'team1'}
        />
        
        <MatchBracketSeparator />
        
        <TeamBracketSlot 
          team={match.team2} 
          score={isCompleted ? match.score2 : null}
          isWinner={winner === 'team2'}
        />
      </MatchTeamsVertical>
      
      {isCompleted && winner && (
        <WinnerArrow direction={winner === 'team1' ? 'left' : 'right'} />
      )}
      
      {isFinal && isCompleted && (
        <FinalLabel>FINALE</FinalLabel>
      )}
    </BracketMatchContainer>
  );
};
```

---

### 8. Statistics Tab

**Purpose**: Display tournament statistics and insights

```jsx
<StatisticsSection>
  <Section title="Meilleurs performeurs">
    <TopPerformersGrid>
      <TopPerformerCard type="scorer">
        <CardHeader>
          <CardTitle>Meilleur marqueur</CardTitle>
        </CardHeader>
        <CardBody>
          <PerformerProfile performer={topScorer} />
          <PerformerStat value={topScorer.points} label="Points" />
          <PerformerStat value={topScorer.matches} label="Matchs" />
          <PerformerStat value={topScorer.accuracy} label="Précision" />
        </CardBody>
      </TopPerformerCard>
      
      <TopPerformerCard type="accuracy">
        <CardHeader>
          <CardTitle>Meilleure précision</CardTitle>
        </CardHeader>
        <CardBody>
          <PerformerProfile performer={topAccuracy} />
          <PerformerStat value={topAccuracy.accuracy} label="Précision" />
          <PerformerStat value={topAccuracy.perfectShots} label="Lancers parfaits" />
        </CardBody>
      </TopPerformerCard>
      
      <TopPerformerCard type="wins">
        <CardHeader>
          <CardTitle>Plus de victoires</CardTitle>
        </CardHeader>
        <CardBody>
          <PerformerProfile performer={mostWins} />
          <PerformerStat value={mostWins.wins} label="Victoires" />
          <PerformerStat value={mostWins.winRate} label="Taux de victoire" />
        </CardBody>
      </TopPerformerCard>
    </TopPerformersGrid>
  </Section>
  
  <Section title="Statistiques du tournoi">
    <StatsGrid>
      <StatCard>
        <StatValue>{tournamentStats.totalMatches}</StatValue>
        <StatLabel>Matchs joués</StatLabel>
      </StatCard>
      <StatCard>
        <StatValue>{tournamentStats.totalPoints}</StatValue>
        <StatLabel>Points marqués</StatLabel>
      </StatCard>
      <StatCard>
        <StatValue>{tournamentStats.averagePoints}</StatValue>
        <StatLabel>Points/match (moyenne)</StatLabel>
      </StatCard>
      <StatCard>
        <StatValue>{tournamentStats.highestScore}</StatValue>
        <StatLabel>Score le plus élevé</StatLabel>
      </StatCard>
      <StatCard>
        <StatValue>{tournamentStats.closestMatch}</StatValue>
        <StatLabel>Match le plus serré</StatLabel>
      </StatCard>
      <StatCard>
        <StatValue>{tournamentStats.perfectGames}</StatValue>
        <StatLabel>Parties parfaites</StatLabel>
      </StatCard>
    </StatsGrid>
  </Section>
  
  <Section title="Évolution du tournoi">
    <ChartsGrid>
      <ChartCard title="Points par journée">
        <LineChart 
          data={pointsByRoundData} 
          xAxis="Journée" 
          yAxis="Points"
        />
      </ChartCard>
      <ChartCard title="Répartition des scores">
        <BarChart 
          data={scoreDistributionData} 
          xAxis="Score" 
          yAxis="Fréquence"
        />
      </ChartCard>
      <ChartCard title="Performance par poule">
        <BarChart 
          data={poolPerformanceData} 
          xAxis="Poule" 
          yAxis="Points moyens"
        />
      </ChartCard>
    </ChartsGrid>
  </Section>
  
  {tournament.records && tournament.records.length > 0 && (
    <Section title="Records du tournoi">
      <RecordsList>
        {tournament.records.map(record => (
          <RecordItem 
            key={record.id}
            record={record}
            player={record.player}
            team={record.team}
          />
        ))}
      </RecordsList>
    </Section>
  )}
</StatisticsSection>
```

---

### 9. Registration Section

**Purpose**: Allow users to register for the tournament (displayed conditionally)

```jsx
{tournament.registrationOpen && userCanRegister && (
  <RegistrationSection>
    <RegistrationHeader>
      <RegistrationTitle>
        <LightningIcon />
        Inscription ouverte
      </RegistrationTitle>
      <RegistrationSubtitle>
        Inscriptions ouvertes jusqu'au {formatDate(tournament.registrationDeadline)}
      </RegistrationSubtitle>
    </RegistrationHeader>
    
    <RegistrationInfo>
      <InfoGrid>
        <InfoItem label="Catégorie">{tournament.category}</InfoItem>
        <InfoItem label="Type">{tournament.type}</InfoItem>
        <InfoItem label="Places disponibles">{tournament.availableSlots} / {tournament.maxParticipants}</InfoItem>
        <InfoItem label="Frais d'inscription">{tournament.fee || 'Gratuit'}</InfoItem>
      </InfoGrid>
      
      {tournament.requirements && (
        <Requirements>
          <RequirementsTitle>Conditions de participation:</RequirementsTitle>
          <RequirementsList>
            {tournament.requirements.map((req, index) => (
              <RequirementItem key={index}>
                <CheckIcon />
                {req}
              </RequirementItem>
            ))}
          </RequirementsList>
        </Requirements>
      )}
    </RegistrationInfo>
    
    <RegistrationActions>
      <ButtonPrimary 
        size="lg" 
        icon="user-plus" 
        onClick={handleRegister}
        disabled={isRegistering || tournament.availableSlots <= 0}
      >
        {tournament.availableSlots <= 0 ? 'Complet' : 'S\'inscrire maintenant'}
      </ButtonPrimary>
      
      <ButtonSecondary 
        icon="info" 
        onClick={handleShowDetails}
      >
        Voir les détails
      </ButtonSecondary>
    </RegistrationActions>
    
    {tournament.availableSlots <= 5 && tournament.availableSlots > 0 && (
      <RegistrationWarning>
        <WarningIcon />
        Plus que {tournament.availableSlots} places disponibles !
      </RegistrationWarning>
    )}
  </RegistrationSection>
)}

{tournament.registrationOpen && !userCanRegister && (
  <RegistrationClosed>
    <ClosedIcon />
    <ClosedTitle>Inscriptions fermées ou non éligible</ClosedTitle>
    <ClosedMessage>
      {user ? 'Vous ne répondez pas aux critères de participation.' : 'Connectez-vous pour vous inscrire.'}
    </ClosedMessage>
    {!user && (
      <ButtonPrimary icon="login" href="/login">Se connecter</ButtonPrimary>
    )}
  </RegistrationClosed>
)}

{tournament.status !== 'upcoming' && (
  <RegistrationClosed>
    <ClosedIcon />
    <ClosedTitle>Inscriptions terminées</ClosedTitle>
    <ClosedMessage>
      Le tournoi a {tournament.status === 'completed' ? 'déjà eu lieu.' : 'commencé.'}
    </ClosedMessage>
  </RegistrationClosed>
)}
```

**Registration Flow**:

When user clicks "S'inscrire maintenant":
1. Check if user is logged in
2. If not, redirect to login with redirect back to tournament
3. If logged in, check eligibility:
   - Has valid license
   - Belongs to affiliated club
   - Meets category requirements
   - Team is complete (for team tournaments)
4. If eligible, show registration form:
   - For individual: confirm participation
   - For team: select/select teammate(s)
5. Payment (if applicable)
6. Confirmation

---

## Data Requirements

### API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/tournaments/{id}` | GET | Get tournament details | Tournament object |
| `/api/tournaments/{id}/participants` | GET | Get tournament participants | Participant[] |
| `/api/tournaments/{id}/matches` | GET | Get tournament matches | Match[] |
| `/api/tournaments/{id}/results` | GET | Get tournament results | Results |
| `/api/tournaments/{id}/stats` | GET | Get tournament statistics | Stats |
| `/api/tournaments/{id}/register` | POST | Register for tournament | Registration |
| `/api/tournaments/{id}/bracket` | GET | Get tournament bracket | Bracket |
| `/api/tournaments/{id}/phases` | GET | Get tournament phases | Phase[] |

### Tournament Object

```typescript
interface Tournament {
  id: string;
  name: string;
  slug: string;
  description: string;
  
  // Dates
  startDate: Date;
  endDate: Date;
  registrationOpenDate: Date;
  registrationDeadline: Date;
  
  // Location
  location: string;
  address: string;
  coordinates: { lat: number; lng: number };
  
  // Organization
  organizer: Club | Federation;
  category: 'FONTE' | 'LAITON' | 'BOIS';
  type: 'INDIVIDUEL' | 'DOUBLETTE' | 'TRIPLETTE';
  format: 'POULES' | 'ELIMINATION_DIRECTE' | 'POULES_ELIMINATION';
  
  // Rules
  pointsToWin: 13 | 15;
  distance: 3.8 | 2.8; // meters
  maxParticipants: number;
  
  // Status
  status: 'upcoming' | 'registration_open' | 'in_progress' | 'completed' | 'cancelled';
  
  // Counts
  participantsCount: number;
  matchesCount: number;
  availableSlots: number;
  
  // Fees
  fee: number | null;
  
  // Requirements
  requirements: string[];
  
  // Results
  winner: Team | Player | null;
  
  // Metadata
  createdAt: Date;
  updatedAt: Date;
  isFeatured: boolean;
  
  // Phases
  phases: TournamentPhase[];
  
  // Custom fields
  customFields: Record<string, any>;
}

interface TournamentPhase {
  id: string;
  name: string;
  type: 'pool' | 'knockout';
  startDate: Date;
  endDate: Date;
  rounds: TournamentRound[];
  pools: TournamentPool[];
  hasQualification: boolean;
  qualifiedTeamsCount: number;
}

interface TournamentRound {
  id: string;
  name: string;
  matches: TournamentMatch[];
}

interface TournamentPool {
  id: string;
  name: string;
  teams: TournamentTeam[];
  status: 'upcoming' | 'in_progress' | 'complete';
  qualifiedTeams: TournamentTeam[];
}

interface TournamentTeam {
  id: string;
  name: string;
  club: Club;
  players: Player[];
  stats: TeamStats;
  
  // For registration
  registrationDate: Date;
  registrationStatus: 'confirmed' | 'pending' | 'rejected';
  paymentStatus: 'paid' | 'pending' | 'waived';
}

interface TournamentMatch {
  id: string;
  round: string;
  date: Date;
  time: string;
  location: string;
  team1: TournamentTeam;
  team2: TournamentTeam;
  score1: number | null;
  score2: number | null;
  status: 'scheduled' | 'in_progress' | 'completed' | 'cancelled';
  referee: string | null;
  
  // For bracket
  winner: 'team1' | 'team2' | null;
  nextMatchId: string | null;
}
```

---

### Required Data for Page Render

**Initial Page Load**:
- Tournament object (basic info)
- Tournament status
- User authentication state
- User eligibility for registration

**Overview Tab**:
- Tournament description
- Tournament phases
- Key dates
- Tournament structure

**Participants Tab**:
- All participants
- Pool groupings
- Team statistics

**Schedule Tab**:
- All matches
- Grouped by phase and round
- Match statuses

**Results Tab**:
- All completed matches
- Bracket structure
- Final standings

**Statistics Tab**:
- Top performers
- Tournament stats
- Charts data

---

## Page States

### 1. Tournament Not Found (404)

```jsx
<TournamentNotFound>
  <NotFoundIcon />
  <NotFoundTitle>Tournoi introuvable</NotFoundTitle>
  <NotFoundMessage>
    Le tournoi que vous cherchez n'existe pas ou a été supprimé.
  </NotFoundMessage>
  <ButtonPrimary href="/tournois">Voir tous les tournois</ButtonPrimary>
</TournamentNotFound>
```

### 2. Loading State

```jsx
<TournamentSkeleton>
  <PageHeaderSkeleton />
  <InfoCardsSkeleton count={4} />
  <TabsSkeleton />
  <ContentSkeleton />
</TournamentSkeleton>
```

### 3. Upcoming Tournament

- Show registration section if open
- Show countdown to start
- Show "À venir" badge
- Schedule tab shows upcoming matches
- Results tab shows "No results yet"

### 4. In Progress Tournament

- Show "En cours" badge
- Show live matches in schedule
- Show current phase/round
- Results tab shows completed matches
- Statistics tab shows current stats

### 5. Completed Tournament

- Show "Terminé" badge
- Show winner prominently
- All tabs fully populated
- Registration closed

### 6. Cancelled Tournament

- Show "Annulé" badge
- Show cancellation reason
- Hide registration
- Disabled state for all interactive elements

---

## Error Handling

### API Errors

```jsx
// 404 - Tournament not found
if (error?.status === 404) {
  return <TournamentNotFound />;
}

// 403 - Access denied
if (error?.status === 403) {
  return (
    <AccessDenied>
      <AccessDeniedIcon />
      <AccessDeniedTitle>Accès refusé</AccessDeniedTitle>
      <AccessDeniedMessage>
        Vous n'avez pas la permission de consulter ce tournoi.
      </AccessDeniedMessage>
      <ButtonPrimary href="/">Retour à l'accueil</ButtonPrimary>
    </AccessDenied>
  );
}

// 500 - Server error
if (error?.status >= 500) {
  return (
    <ServerError>
      <ServerErrorIcon />
      <ServerErrorTitle>Erreur serveur</ServerErrorTitle>
      <ServerErrorMessage>
        Une erreur est survenue. Veuillez réessayer plus tard.
      </ServerErrorMessage>
      <ButtonPrimary onClick={() => refetch()}>Réessayer</ButtonPrimary>
    </ServerError>
  );
}

// Generic error
if (error) {
  return (
    <ErrorState>
      <ErrorIcon />
      <ErrorTitle>Erreur de chargement</ErrorTitle>
      <ErrorMessage>{error.message}</ErrorMessage>
      <ButtonPrimary onClick={() => refetch()}>Réessayer</ButtonPrimary>
    </ErrorState>
  );
}
```

---

## Responsive Design

### Mobile (< 640px)

```
┌──────────────────────┐
│ TOP BAR               │
├──────────────────────┤
│ HEADER                │
├──────────────────────┤
│ PAGE HEADER          │
│  [Name]              │
│  Location • Dates    │
│  [Status Badge]      │
├──────────────────────┤
│ INFO CARDS (2x2 grid)│
├──────────────────────┤
│ TABS (scrollable)     │
├──────────────────────┤
│ TAB CONTENT           │
│  (full width)        │
├──────────────────────┤
│ REGISTRATION         │
│  (full width)        │
└──────────────────────┘
```

**Adaptations**:
- Info cards: 2x2 grid instead of 4 inline
- Tabs: Scrollable horizontal layout
- Bracket: Vertical layout, simplified
- Participants: Single column, stacked
- Registration: Full width buttons

### Tablet (640px - 1023px)

```
┌──────────────────────────────────┐
│ TOP BAR                         │
├──────────────────────────────────┤
│ HEADER                          │
├──────────────────────────────────┤
│ PAGE HEADER                    │
├──────────────────────────────────┤
│ INFO CARDS (2x2 grid)           │
├──────────────────────────────────┤
│ TABS                           │
├──────────────────────────────────┤
│ TAB CONTENT                     │
│  (2-column layout possible)     │
├──────────────────────────────────┤
│ REGISTRATION                   │
└──────────────────────────────────┘
```

**Adaptations**:
- Info cards: 2x2 grid
- Bracket: 2-column layout
- Participants: 2-column grid for pools

### Desktop (1024px+)

Use full maquette layout as shown in the main diagram.

---

## Accessibility

### Semantic HTML

```jsx
// Good - Use semantic elements
<main>
  <header>
    <h1>{tournament.name}</h1>
  </header>
  
  <nav aria-label="Navigation du tournoi">
    <ul>
      <li><a href="#overview">Vue d'ensemble</a></li>
      <li><a href="#participants">Participants</a></li>
      {/* ... */}
    </ul>
  </nav>
  
  <section id="overview" aria-labelledby="overview-heading">
    <h2 id="overview-heading">Vue d'ensemble</h2>
    {/* Content */}
  </section>
</main>
```

### ARIA Attributes

```jsx
<TournamentTabs 
  role="tablist"
  aria-label="Navigation du tournoi"
>
  <Tab 
    role="tab" 
    aria-selected={activeTab === 'overview'}
    aria-controls="overview-panel"
    id="overview-tab"
  />
  {/* ... */}
</TournamentTabs>

<TabPanel 
  id="overview-panel"
  role="tabpanel"
  aria-labelledby="overview-tab"
  hidden={activeTab !== 'overview'}
>
  {/* Content */}
</TabPanel>
```

### Keyboard Navigation

- **Tabs**: Arrow keys to navigate between tabs
- **Search**: Clear button, escape to clear
- **Filters**: Keyboard accessible selects
- **Bracket**: Focusable elements, logical tab order

### Color Contrast

All color combinations meet WCAG 2.1 AA:
- Text on primary background: 4.5:1+
- Text on secondary background: 4.5:1+
- Interactive elements: 4.5:1+
- Status badges: 4.5:1+

---

## Performance Optimizations

### Data Fetching

```jsx
// Use TanStack Query for efficient data fetching
const { data: tournament, isLoading, error } = useQuery({
  queryKey: ['tournament', id],
  queryFn: () => fetchTournament(id),
  staleTime: 5 * 60 * 1000, // 5 minutes
});

// Prefetch related data
const { data: participants } = useQuery({
  queryKey: ['tournament-participants', id],
  queryFn: () => fetchParticipants(id),
  enabled: !!tournament,
  staleTime: 5 * 60 * 1000,
});

// Paginated data
const { data: matches } = useQuery({
  queryKey: ['tournament-matches', id, page],
  queryFn: () => fetchMatches(id, page),
  enabled: !!tournament,
  keepPreviousData: true,
});
```

### Image Optimization

```jsx
<TournamentImage 
  src={tournament.image} 
  alt={tournament.name}
  loading="lazy"
  sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
  width={400}
  height={200}
/>
```

### Lazy Loading

```jsx
// Lazy load heavy components
const BracketView = React.lazy(() => import('./BracketView'));

// Suspense boundary
<Suspense fallback={<BracketSkeleton />}>
  <BracketView bracket={bracket} />
</Suspense>
```

---

## SEO

### Page Metadata

```jsx
<Helmet>
  <title>{tournament.name} - Palet Vendéen</title>
  <meta name="description" content={tournament.description || `Découvrez le tournoi ${tournament.name} - calendrier, participants, résultats et inscriptions.`} />
  <meta name="keywords" content={`palet vendéen, tournoi, ${tournament.name}, ${tournament.location}, ${tournament.category}, ${tournament.type}`} />
  
  <meta property="og:title" content={`${tournament.name} - Palet Vendéen`} />
  <meta property="og:description" content={tournament.description || `Tournoi de palet vendéen à ${tournament.location}`} />
  <meta property="og:type" content="website" />
  <meta property="og:url" content={`https://palet-vendeen.fr/tournois/${tournament.slug}`} />
  
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content={`${tournament.name} - Palet Vendéen`} />
  
  <link rel="canonical" href={`https://palet-vendeen.fr/tournois/${tournament.slug}`} />
  
  <script type="application/ld+json">
    {JSON.stringify(tournamentSchema(tournament))}
  </script>
</Helmet>
```

---

## Integration Points

### With Other Pages

1. **Homepage**: Featured tournaments section links to tournament pages
2. **Championship**: May link to related tournaments
3. **Club**: Club page shows tournaments the club participates in
4. **Player**: Player page shows tournaments the player participates in
5. **Match**: Match page links to tournament context

### With Admin Interface

1. **Admin Tournament List**: Links to tournament pages
2. **Admin Tournament Edit**: Links to public tournament page
3. **Admin Results Entry**: Links to tournament results

---

## Testing Checklist

### Functional Tests

- [ ] Page loads with valid tournament ID
- [ ] 404 displayed for invalid tournament ID
- [ ] All tabs are functional
- [ ] Filters work correctly
- [ ] Search works in participants tab
- [ ] Sorting works in all tables
- [ ] Pagination works correctly
- [ ] Registration button works when eligible
- [ ] Registration button disabled when not eligible
- [ ] Error states display correctly
- [ ] Loading states work properly

### Visual Tests

- [ ] Layout matches maquette
- [ ] Responsive across all breakpoints
- [ ] Colors match design system
- [ ] Typography matches design system
- [ ] Spacing matches design system
- [ ] Icons are consistent

### Accessibility Tests

- [ ] Keyboard navigation works
- [ ] Screen reader compatible
- [ ] Color contrast meets WCAG AA
- [ ] Focus indicators visible
- [ ] Semantic HTML used
- [ ] ARIA attributes present

### Performance Tests

- [ ] Page loads in < 2.5 seconds
- [ ] No layout shifts (CLS < 0.1)
- [ ] Fast interaction response (FID < 100ms)
- [ ] Memory usage reasonable

---

## Implementation Notes

### Recommended Approach

1. **Create the basic layout first**
   - Header with breadcrumb and title
   - Info cards section
   - Tabs navigation

2. **Implement each tab separately**
   - Start with Overview tab
   - Then Participants tab
   - Then Schedule tab
   - Then Results tab
   - Finally Statistics tab

3. **Add registration section**
   - Conditional rendering based on status and eligibility
   - Integration with registration flow

4. **Add responsive styles**
   - Mobile-first approach
   - Test on all breakpoints

5. **Add accessibility features**
   - Semantic HTML
   - ARIA attributes
   - Keyboard navigation

6. **Optimize performance**
   - Lazy loading for heavy components
   - Efficient data fetching
   - Image optimization

---

## File Structure

```
web/src/pages/tournament/
├── index.jsx              # Main tournament page
├── TournamentHeader.jsx  # Tournament header component
├── TournamentInfoCards.jsx # Info cards section
├── TournamentTabs.jsx    # Tabs navigation
├── OverviewTab.jsx       # Overview tab content
├── ParticipantsTab.jsx   # Participants tab content
├── ScheduleTab.jsx       # Schedule tab content
├── ResultsTab.jsx        # Results tab content
├── StatisticsTab.jsx     # Statistics tab content
├── RegistrationSection.jsx # Registration section
├── components/
│   ├── TournamentStructureDiagram.jsx
│   ├── SingleEliminationBracket.jsx
│   ├── BracketMatch.jsx
│   ├── ScheduleMatchCard.jsx
│   ├── ParticipantTable.jsx
│   └── TopPerformerCard.jsx
└── hooks/
    ├── useTournament.js   # Custom hook for tournament data
    └── useTournamentTabs.js
```

---

## Dependencies

### External Libraries

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-router-dom": "^6.20.0",
    "@tanstack/react-query": "^5.0.0",
    "date-fns": "^2.30.0",
    "recharts": "^2.10.0",
    "framer-motion": "^10.16.0"
  }
}
```

### Internal Dependencies

- `components/Layout` (TopBar, Header, Footer)
- `components/Button`
- `components/Card`
- `components/Table`
- `components/Badge`
- `components/Tabs`
- `components/Modal`
- `components/Pagination`
- `components/Select`
- `components/Input`
- `hooks/useBreadcrumbs`
- `services/api`
- `styles/design-system.css`

---

## Example Data

### Sample Tournament Object

```json
{
  "id": "coupe-france-2025",
  "name": "Coupe de France - FONTE - Doublette",
  "slug": "coupe-france-fonte-doublette-2025",
  "description": "La Coupe de France de palet vendéen en fonte, catégorie doublette. Ouvert à tous les clubs affiliés à la FNSMR.",
  "startDate": "2025-11-15",
  "endDate": "2025-11-16",
  "registrationOpenDate": "2025-10-01",
  "registrationDeadline": "2025-11-10",
  "location": "Buzay, Vendée",
  "address": "Salle des sports de Buzay, 85320 Buzay",
  "coordinates": { "lat": 46.8, "lng": -1.5 },
  "organizer": { "id": "cvdp", "name": "CVDP", "type": "federation" },
  "category": "FONTE",
  "type": "DOUBLETTE",
  "format": "POULES_ELIMINATION",
  "pointsToWin": 15,
  "distance": 3.8,
  "maxParticipants": 48,
  "status": "registration_open",
  "participantsCount": 36,
  "matchesCount": 63,
  "availableSlots": 12,
  "fee": 20,
  "requirements": [
    "Licence FNSMR valide",
    "Club affilié",
    "Joueur classé ou autorisation"
  ],
  "winner": null,
  "isFeatured": true,
  "phases": [
    {
      "id": "pool-phase",
      "name": "Phase de poules",
      "type": "pool",
      "startDate": "2025-11-15",
      "endDate": "2025-11-15",
      "pools": [
        {
          "id": "A",
          "name": "Poule A",
          "teams": [],
          "status": "upcoming"
        }
        // More pools...
      ]
    },
    {
      "id": "knockout-phase",
      "name": "Phase finale",
      "type": "knockout",
      "startDate": "2025-11-16",
      "endDate": "2025-11-16",
      "rounds": []
    }
  ]
}
```

---

## Next Steps

After implementing the Tournament page:

1. **Create Match page** - For live match details
2. **Create Registration flow** - For tournament registration
3. **Create Club page** - For club profiles
4. **Create Player page** - For player profiles
5. **Create Admin pages** - For tournament management

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Design System - Layouts](/docs/design-system/layouts.md)
- [Design System - Icons](/docs/design-system/icons.md)
- [Business Context - Tournament Structure](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md#hi%C3%A9rarchie)
- [Maquette - homepage.svg](/docs/maquettes/homepage.svg)

---

*Tournament Page Specification - Palet Vendeen Design System*
*Last Updated: 09/10/2026*
*Version: 1.0*
