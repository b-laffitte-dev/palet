# Championship Page - Palet Vendeen

> Page displaying championship standings, results, and statistics

---

## Overview

The **Championship Page** provides comprehensive information about a specific championship, including:
- **Classification/table** with current standings
- **Match results** from previous and current rounds
- **Next matches** schedule
- **Season statistics** and highlights
- **Filters** for different divisions, seasons, and categories

**Inspiration**: Ligue 1 Uber Eats championship pages, Top 14 standings

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
│  │ CHAMPIONNAT DE VENDÉE                                       ││
│  │ Divisions available [D1 ▼]  [Season 2025/2026 ▼]         ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ FILTERS BAR (60px)                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [All Divisions ▼]  [All Journées ▼]  [Search...]          ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ STATS OVERVIEW (120px)                                        │
│  ┌─────────────────┬─────────────────┬─────────────────┐│
│  │ Leader           │ Points           │ Top Scorer      ││
│  │ [Team Logo]      │ [Chart]          │ [Player Avatar]  ││
│  │ La Roche...      │ 30 pts          │ Jean Morice     ││
│  │ 10 wins          │                 │ 412 pts         ││
│  └─────────────────┴─────────────────┴─────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ CLASSIFICATION TABLE (flexible height)                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Full width table with all teams                           ││
│  │ Same as homepage but with more columns                    ││
│  │ [POS | CLUB | MJ | G | N | P | BP | BC | DIFF | PTS | FORME]│
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ TABS NAVIGATION (48px)                                        │
│  [Résultats]  [Prochains matchs]  [Statistiques]  [Calendrier] │
├─────────────────────────────────────────────────────────────┤
│ TAB CONTENT                                                   │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ RESULTS TABLE                                              ││
│  │  ┌─────────────────────┐                                ││
│  │  │ JOURNÉE 12            │                                ││
│  │  ├─────────────────────┤                                ││
│  │  │ Sat 14 Jun            │                                ││
│  │  ├─────────────────────┤                                ││
│  │  │ [Match Card 1]       │                                ││
│  │  │ [Match Card 2]       │                                ││
│  │  │ [Match Card 3]       │                                ││
│  │  │ ...                  │                                ││
│  │  └─────────────────────┘                                ││
│  │                                                         ││
│  │ Pagination [1 | 2 | 3 | 4 | 5 >]                          ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Page Header

**Purpose**: Clear page title and context

```jsx
<PageHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem active>Championnat</BreadcrumbItem>
  </Breadcrumb>
  
  <PageTitle size="display-m" weight="bold">
    CHAMPIONNAT DE VENDÉE
  </PageTitle>
  
  <PageSubtitle size="body-m" color="gray-500">
    Suivez les classements et résultats en temps réel
  </PageSubtitle>
  
  <PageActions>
    <Select value="D1" onChange={handleDivisionChange}>
      <Option value="D1">Division 1</Option>
      <Option value="D2">Division 2</Option>
      <Option value="D3">Division 3</Option>
    </Select>
    
    <Select value="2025-2026" onChange={handleSeasonChange}>
      <Option value="2025-2026">Saison 2025/2026</Option>
      <Option value="2024-2025">Saison 2024/2025</Option>
      <Option value="2023-2024">Saison 2023/2024</Option>
    </Select>
  </PageActions>
</PageHeader>
```

---

### 2. Filters Bar

**Purpose**: Filter championship data

```jsx
<FiltersBar>
  <FilterGroup>
    <FilterLabel>Division</FilterLabel>
    <Select value={division} onChange={setDivision}>
      <Option value="all">Toutes les divisions</Option>
      <Option value="D1">Division 1</Option>
      <Option value="D2">Division 2</Option>
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Journée</FilterLabel>
    <Select value={journee} onChange={setJournee}>
      <Option value="all">Toutes les journées</Option>
      <Option value="12">Journée 12</Option>
      <Option value="11">Journée 11</Option>
      {/* More options */}
    </Select>
  </FilterGroup>
  
  <FilterGroup>
    <FilterLabel>Rechercher</FilterLabel>
    <Input 
      type="search" 
      placeholder="Club, joueur..." 
      value={search} 
      onChange={setSearch}
    />
  </FilterGroup>
  
  <FilterActions>
    <ButtonSecondary onClick={resetFilters}>
      Réinitialiser
    </ButtonSecondary>
  </FilterActions>
</FiltersBar>
```

---

### 3. Stats Overview

**Purpose**: Quick summary of key statistics

```jsx
<StatsOverview>
  <StatCard>
    <StatCardIcon name="trophy" />
    <StatCardLabel>Leader</StatCardLabel>
    <StatCardContent>
      <ClubLogo team={leader} size="sm" />
      <ClubName>{leader.name}</ClubName>
      <ClubStats>
        <StatItem>
          <StatValue>{leader.wins}</StatValue>
          <StatLabel>Victoires</StatLabel>
        </StatItem>
        <StatItem>
          <StatValue>{leader.points}</StatValue>
          <StatLabel>Points</StatLabel>
        </StatItem>
      </ClubStats>
    </StatCardContent>
  </StatCard>
  
  <StatCard>
    <StatCardIcon name="chart" />
    <StatCardLabel>Classement</StatCardLabel>
    <StatCardContent>
      <ClassificationChart data={chartData} />
      <ChartLabel>Top 5 équipes</ChartLabel>
    </StatCardContent>
  </StatCard>
  
  <StatCard>
    <StatCardIcon name="target" />
    <StatCardLabel>Meilleur marqueur</StatCardLabel>
    <StatCardContent>
      <PlayerAvatar player={topScorer} />
      <PlayerName>{topScorer.firstName} {topScorer.lastName}</PlayerName>
      <PlayerStats>
        <StatValue>{topScorer.stats.points}</StatValue>
        <StatLabel>Points</StatLabel>
      </PlayerStats>
    </StatCardContent>
  </StatCard>
</StatsOverview>
```

---

### 4. Enhanced Classification Table

**Purpose**: Detailed championship standings

```jsx
<ClassificationTable enhanced>
  <ClassificationTableHeader>
    <TableHeaderCell>POS</TableHeaderCell>
    <TableHeaderCell>CLUB</TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('mj')}>
      MJ <SortIcon direction={sortDirection} />
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('g')}>
      G
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('n')}>
      N
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('p')}>
      P
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('bp')}>
      BP
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('bc')}>
      BC
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('diff')}>
      DIFF
    </TableHeaderCell>
    <TableHeaderCell textAlign="center" sortable onClick={() => sortBy('pts')}>
      PTS
    </TableHeaderCell>
    <TableHeaderCell>FORME</TableHeaderCell>
  </ClassificationTableHeader>
  
  <ClassificationTableBody>
    {teams.map((team, index) => (
      <ClassificationRow 
        key={team.id} 
        position={team.position}
        qualified={team.status === 'qualified'}
        barrage={team.status === 'barrage'}
        relegated={team.status === 'relegated'}
      >
        <TableCell position>
          <Text size="body-l" weight="bold" color="primary-700">
            {team.position}
          </Text>
        </TableCell>
        <TableCell club>
          <ClubLogo team={team} size="sm" />
          <ClubName size="body-l" weight="bold" color="primary-700">
            {team.name}
          </ClubName>
        </TableCell>
        <TableCell stat textAlign="center">{team.stats.matches}</TableCell>
        <TableCell stat textAlign="center">{team.stats.wins}</TableCell>
        <TableCell stat textAlign="center">{team.stats.draws}</TableCell>
        <TableCell stat textAlign="center">{team.stats.losses}</TableCell>
        <TableCell stat textAlign="center">{team.stats.pointsFor}</TableCell>
        <TableCell stat textAlign="center">{team.stats.pointsAgainst}</TableCell>
        <TableCell stat textAlign="center">
          {team.stats.diff >= 0 ? `+${team.stats.diff}` : team.stats.diff}
        </TableCell>
        <TableCell pts textAlign="center">{team.stats.points}</TableCell>
        <TableCell form>
          <FormBadge>
            {team.form.map((result, i) => (
              <FormBadgeItem key={i} status={result} />
            ))}
          </FormBadge>
        </TableCell>
      </ClassificationRow>
    ))}
  </ClassificationTableBody>
</ClassificationTable>
```

---

### 5. Tabs Navigation

**Purpose**: Navigate between different views

```jsx
<Tabs defaultActive="results">
  <Tab id="results">
    Résultats
    {unreadResults && <TabBadge>{unreadResults}</TabBadge>}
  </Tab>
  <Tab id="next-matches">
    Prochains matchs
    {upcomingMatches > 0 && <TabBadge>{upcomingMatches}</TabBadge>}
  </Tab>
  <Tab id="stats">Statistiques</Tab>
  <Tab id="calendar">Calendrier</Tab>
</Tabs>
```

---

### 6. Results Tab Content

**Purpose**: Display match results

```jsx
<TabPanel id="results">
  <ResultsHeader>
    <ResultsTitle size="heading-m">
      Résultats - {selectedJournee || 'Toutes les journées'}
    </ResultsTitle>
    <ResultsActions>
      <Select value={selectedJournee} onChange={setSelectedJournee}>
        <Option value="all">Toutes les journées</Option>
        <Option value="12">Journée 12</Option>
        <Option value="11">Journée 11</Option>
        {/* More */}
      </Select>
    </ResultsActions>
  </ResultsHeader>
  
  <ResultsGroup byDate>
    {groupedResults.map((group, date) => (
      <ResultsDateGroup key={date} date={date}>
        <DateGroupHeader>
          <DateTitle size="heading-s">{formatDate(date)}</DateTitle>
          <DateSubtitle size="body-s" color="gray-500">
            {getDayName(date)}
          </DateSubtitle>
        </DateGroupHeader>
        
        <ResultsList>
          {group.matches.map(match => (
            <MatchResultCard key={match.id}>
              <MatchResultTeams>
                <MatchResultTeam>
                  <ClubLogo team={match.team1} size="xs" />
                  <ClubName size="body-m">{match.team1.name}</ClubName>
                </MatchResultTeam>
                <MatchResultScore>
                  <Score value={match.team1.score} />
                  <ScoreSeparator />
                  <Score value={match.team2.score} />
                </MatchResultScore>
                <MatchResultTeam>
                  <ClubLogo team={match.team2} size="xs" />
                  <ClubName size="body-m">{match.team2.name}</ClubName>
                </MatchResultTeam>
              </MatchResultTeams>
              
              <MatchResultStatus>
                <StatusBadge status={match.status}>
                  {match.status === 'finished' && 'Terminé'}
                  {match.status === 'postponed' && 'Reporté'}
                  {match.status === 'cancelled' && 'Annulé'}
                </StatusBadge>
                <StatusDetails>
                  <Text size="caption">Manche {match.manche}</Text>
                  {match.duration && <Text size="caption">- {match.duration}</Text>}
                </StatusDetails>
              </MatchResultStatus>
              
              <MatchResultActions>
                <ButtonGhost onClick={() => viewMatch(match.id)}>
                  Voir le détail <Icon name="arrow-right" size="xs" />
                </ButtonGhost>
              </MatchResultActions>
            </MatchResultCard>
          ))}
        </ResultsList>
      </ResultsDateGroup>
    ))}
  </ResultsGroup>
  
  <Pagination 
    current={currentPage} 
    total={totalPages} 
    onChange={setCurrentPage}
  />
</TabPanel>
```

---

### 7. Next Matches Tab Content

**Purpose**: Display upcoming matches

```jsx
<TabPanel id="next-matches">
  <NextMatchesHeader>
    <NextMatchesTitle size="heading-m">
      Prochains matchs
    </NextMatchesTitle>
    <NextMatchesActions>
      <ButtonSecondary onClick={() => setView('calendar')}>
        Voir le calendrier
      </ButtonSecondary>
    </NextMatchesActions>
  </NextMatchesHeader>
  
  <NextMatchesList>
    {nextMatches.map(match => (
      <NextMatchCard key={match.id}>
        <NextMatchHeader>
          <NextMatchDate size="body-s" color="gray-500">
            {formatDate(match.date)} - {formatTime(match.time)}
          </NextMatchDate>
          <NextMatchCompetition size="body-xs" color="gold-700" uppercase>
            {match.competition} - Journée {match.journee}
          </NextMatchCompetition>
        </NextMatchHeader>
        
        <NextMatchTeams>
          <NextMatchTeam>
            <ClubLogo team={match.team1} size="sm" />
            <ClubName size="body-m" weight="bold">{match.team1.name}</ClubName>
            <TeamStatus>
              {match.team1.position && `(${match.team1.position})`}
            </TeamStatus>
          </NextMatchTeam>
          <NextMatchVs>VS</NextMatchVs>
          <NextMatchTeam>
            <ClubLogo team={match.team2} size="sm" />
            <ClubName size="body-m" weight="bold">{match.team2.name}</ClubName>
            <TeamStatus>
              {match.team2.position && `(${match.team2.position})`}
            </TeamStatus>
          </NextMatchTeam>
        </NextMatchTeams>
        
        <NextMatchLocation>
          <Icon name="location" size="sm" />
          <Text size="body-s" color="gray-500">{match.location}</Text>
        </NextMatchLocation>
        
        <NextMatchActions>
          {match.registrationOpen && (
            <ButtonPrimary size="sm">
              S'inscrire
            </ButtonPrimary>
          )}
          <ButtonGhost size="sm" onClick={() => addToCalendar(match)}>
            <Icon name="calendar" size="sm" />
          </ButtonGhost>
        </NextMatchActions>
      </NextMatchCard>
    ))}
  </NextMatchesList>
  
  <Pagination 
    current={currentPage} 
    total={totalPages} 
    onChange={setCurrentPage}
  />
</TabPanel>
```

---

### 8. Statistics Tab Content

**Purpose**: Display championship statistics

```jsx
<TabPanel id="stats">
  <StatsLayout>
    <StatsMain>
      <StatsSection>
        <StatsSectionTitle size="heading-s">
          Classement par catégorie
        </StatsSectionTitle>
        <StatsCharts>
          <StatChart type="bar" title="Points par équipe">
            {chartData.points}
          </StatChart>
          <StatChart type="bar" title="Victoires par équipe">
            {chartData.wins}
          </StatChart>
        </StatsCharts>
      </StatsSection>
      
      <StatsSection>
        <StatsSectionTitle size="heading-s">
          Évolution du classement
        </StatsSectionTitle>
        <LineChart data={evolutionData} />
      </StatsSection>
      
      <StatsSection>
        <StatsSectionTitle size="heading-s">
          Statistiques par équipe
        </StatsSectionTitle>
        <StatsTable>
          {teamStats.map(team => (
            <StatsTableRow key={team.id}>
              <StatsTableCell team>
                <ClubLogo team={team} size="xs" />
                <ClubName size="body-s">{team.name}</ClubName>
              </StatsTableCell>
              <StatsTableCell numeric>{team.stats.wins}</StatsTableCell>
              <StatsTableCell numeric>{team.stats.draws}</StatsTableCell>
              <StatsTableCell numeric>{team.stats.losses}</StatsTableCell>
              <StatsTableCell numeric>{team.stats.pointsFor}</StatsTableCell>
              <StatsTableCell numeric>{team.stats.pointsAgainst}</StatsTableCell>
              <StatsTableCell numeric>
                {team.stats.diff >= 0 ? `+${team.stats.diff}` : team.stats.diff}
              </StatsTableCell>
            </StatsTableRow>
          ))}
        </StatsTable>
      </StatsSection>
    </StatsMain>
    
    <StatsSidebar>
      <StatsCard>
        <StatsCardTitle>Meilleur attaque</StatsCardTitle>
        <StatsCardContent>
          <ClubRanking list={topAttack} />
        </StatsCardContent>
      </StatsCard>
      
      <StatsCard>
        <StatsCardTitle>Meilleur défense</StatsCardTitle>
        <StatsCardContent>
          <ClubRanking list={topDefense} />
        </StatsCardContent>
      </StatsCard>
      
      <StatsCard>
        <StatsCardTitle>Plus grand écart</StatsCardTitle>
        <StatsCardContent>
          <BiggestMargin match={biggestMarginMatch} />
        </StatsCardContent>
      </StatsCard>
    </StatsSidebar>
  </StatsLayout>
</TabPanel>
```

---

### 9. Calendar Tab Content

**Purpose**: Display championship calendar

```jsx
<TabPanel id="calendar">
  <CalendarHeader>
    <CalendarTitle size="heading-m">
      Calendrier du Championnat - Division 1
    </CalendarTitle>
    <CalendarActions>
      <CalendarViewToggle 
        view={calendarView} 
        onChange={setCalendarView}
      />
      <Select value={selectedMonth} onChange={setSelectedMonth}>
        <Option value="all">Tous les mois</Option>
        <Option value="2025-06">Juin 2025</Option>
        <Option value="2025-07">Juillet 2025</Option>
        {/* More */}
      </Select>
    </CalendarActions>
  </CalendarHeader>
  
  {calendarView === 'month' && (
    <MonthCalendar 
      year={selectedYear} 
      month={selectedMonth} 
      events={calendarEvents}
    />
  )}
  
  {calendarView === 'list' && (
    <CalendarList>
      {calendarEvents.map(event => (
        <CalendarEvent key={event.id}>
          <EventDate>
            <EventDay size="display-m">{formatDay(event.date)}</EventDay>
            <EventMonth size="body-xs" color="gray-500">
              {formatMonth(event.date)}
            </EventMonth>
          </EventDate>
          <EventContent>
            <EventTitle size="body-m" weight="bold">
              {event.title}
            </EventTitle>
            <EventMeta size="body-s" color="gray-500">
              {event.time} - {event.location}
            </EventMeta>
          </EventContent>
        </CalendarEvent>
      ))}
    </CalendarList>
  )}
  
  {calendarView === 'agenda' && (
    <AgendaView 
      events={calendarEvents} 
      onEventClick={viewEvent}
    />
  )}
</TabPanel>
```

---

## Data Requirements

### Championship Data
```json
{
  "championship": {
    "id": "champ-vendee-2025",
    "name": "Championnat de Vendée",
    "season": "2025/2026",
    "currentJournee": 12,
    "totalJournees": 26,
    "divisions": ["D1", "D2", "D3"],
    "status": "in_progress"
  }
}
```

---

### Classification Data
```json
{
  "classification": {
    "championshipId": "champ-vendee-2025",
    "division": "D1",
    "journee": 12,
    "updatedAt": "2025-06-15T10:00:00Z",
    "teams": [
      {
        "position": 1,
        "team": {
          "id": "team-1",
          "name": "La Roche-sur-Yon Palet Club",
          "abbr": "LR",
          "logo": "/logos/lr.svg"
        },
        "stats": {
          "matches": 12,
          "wins": 10,
          "draws": 0,
          "losses": 2,
          "pointsFor": 412,
          "pointsAgainst": 328,
          "diff": 84,
          "points": 30
        },
        "form": ["win", "win", "win", "loss", "win", "win"],
        "status": "qualified"
      }
      // More teams...
    ]
  }
}
```

---

### Results Data
```json
{
  "results": {
    "championshipId": "champ-vendee-2025",
    "division": "D1",
    "journee": 12,
    "totalJournees": 26,
    "byDate": {
      "2025-06-14": {
        "date": "2025-06-14",
        "dayName": "Samedi",
        "matches": [
          {
            "id": "match-123",
            "team1": {
              "id": "team-1",
              "name": "La Roche-sur-Yon PC",
              "abbr": "LR",
              "logo": "/logos/lr.svg",
              "score": 72
            },
            "team2": {
              "id": "team-2",
              "name": "Clos Fontenois",
              "abbr": "CF",
              "logo": "/logos/cf.svg",
              "score": 68
            },
            "status": "finished",
            "manche": 24,
            "duration": "~2h30",
            "location": "Terrain clos de La Roche-sur-Yon"
          }
          // More matches...
        ]
      }
      // More dates...
    },
    "total": 45,
    "pages": 5
  }
}
```

---

### Next Matches Data
```json
{
  "nextMatches": {
    "championshipId": "champ-vendee-2025",
    "division": "D1",
    "matches": [
      {
        "id": "match-456",
        "date": "2025-06-21",
        "time": "15:00",
        "competition": "Championnat de Vendée - Division 1",
        "journee": 13,
        "team1": {
          "id": "team-1",
          "name": "La Roche-sur-Yon PC",
          "abbr": "LR",
          "logo": "/logos/lr.svg",
          "position": 1
        },
        "team2": {
          "id": "team-3",
          "name": "Luçon Palet Club",
          "abbr": "LU",
          "logo": "/logos/lu.svg",
          "position": 3
        },
        "location": "Terrain clos de Luçon",
        "status": "scheduled",
        "registrationOpen": true,
        "registrationDeadline": "2025-06-20T12:00:00Z"
      }
      // More matches...
    ],
    "total": 15,
    "pages": 2
  }
}
```

---

### Statistics Data
```json
{
  "statistics": {
    "championshipId": "champ-vendee-2025",
    "division": "D1",
    "leader": {
      "teamId": "team-1",
      "name": "La Roche-sur-Yon Palet Club",
      "wins": 10,
      "points": 30
    },
    "topScorer": {
      "playerId": "player-1",
      "firstName": "Jean",
      "lastName": "Morice",
      "club": "La Roche-sur-Yon PC",
      "stats": {
        "points": 412,
        "averagePerManche": 11.4
      }
    },
    "topAttack": [
      {
        "teamId": "team-1",
        "name": "La Roche-sur-Yon PC",
        "pointsFor": 412
      },
      {
        "teamId": "team-2",
        "name": "Clos Fontenois",
        "pointsFor": 385
      }
    ],
    "topDefense": [
      {
        "teamId": "team-2",
        "name": "Clos Fontenois",
        "pointsAgainst": 325
      },
      {
        "teamId": "team-1",
        "name": "La Roche-sur-Yon PC",
        "pointsAgainst": 328
      }
    ],
    "biggestMargin": {
      "matchId": "match-789",
      "team1": "La Roche-sur-Yon PC",
      "team2": "Pouzaugues",
      "score1": 85,
      "score2": 44,
      "margin": 41
    },
    "evolution": {
      "labels": ["J1", "J2", "J3", "J4", "J5"],
      "datasets": [
        {
          "label": "La Roche-sur-Yon",
          "data": [1, 1, 1, 1, 1]
        },
        {
          "label": "Clos Fontenois",
          "data": [2, 2, 2, 2, 2]
        }
      ]
    }
  }
}
```

---

### Calendar Data
```json
{
  "calendar": {
    "championshipId": "champ-vendee-2025",
    "division": "D1",
    "events": [
      {
        "id": "journee-13",
        "title": "Journée 13 - Championnat",
        "date": "2025-06-21",
        "time": "15:00",
        "location": "Multiples terrains",
        "type": "journee",
        "matches": 6
      },
      {
        "id": "journee-14",
        "title": "Journée 14 - Championnat",
        "date": "2025-06-28",
        "time": "15:00",
        "location": "Multiples terrains",
        "type": "journee",
        "matches": 6
      },
      {
        "id": "final",
        "title": "Finale du Championnat",
        "date": "2025-07-05",
        "time": "15:00",
        "location": "Terrain clos de La Roche-sur-Yon",
        "type": "final",
        "matches": 1
      }
      // More events...
    ]
  }
}
```

---

## API Endpoints

| Data | Endpoint | Method | Parameters |
|------|----------|--------|------------|
| Championship | `/api/championships/{id}` | GET | - |
| Classification | `/api/championships/{id}/classification` | GET | division, journee |
| Results | `/api/championships/{id}/results` | GET | division, journee, page, limit |
| Next Matches | `/api/championships/{id}/next-matches` | GET | division, limit |
| Statistics | `/api/championships/{id}/statistics` | GET | division |
| Calendar | `/api/championships/{id}/calendar` | GET | division, month |

---

## Responsive Adaptations

### Mobile (< 768px)

1. **Filters Bar**: Collapse to accordion or modal
2. **Stats Overview**: Stack cards vertically
3. **Classification Table**: Horizontal scroll, simplified columns
4. **Tabs**: Scrollable horizontal tabs
5. **Results**: Single column, stacked match cards
6. **Calendar**: Month view by default, list view option

---

### Tablet (768px - 1023px)

1. **Filters Bar**: Single row, compact
2. **Stats Overview**: 2 columns
3. **Classification Table**: Full width, all columns
4. **Results**: 2 columns grid
5. **Calendar**: Month and agenda view

---

### Desktop (1024px+)

1. **Full layout** as described above
2. **Enhanced Classification Table**: All columns visible
3. **Stats Sidebar**: Always visible
4. **Calendar**: All view options

---

## Accessibility Features

1. **Sortable table headers**: Screen reader announcements
2. **Chart descriptions**: Text alternatives for visualizations
3. **Keyboard navigation**: Full keyboard support for all interactive elements
4. **Focus management**: Clear focus states for filters and tabs
5. **Color contrast**: All text meets WCAG AA standards

---

## Performance Optimizations

1. **Lazy loading**: Charts and statistics load on demand
2. **Pagination**: Results and matches loaded in pages
3. **Debounced search**: Filter updates debounced to reduce API calls
4. **Chart rendering**: Use canvas-based charts (Chart.js) for better performance
5. **Memoization**: Cache classification data to avoid unnecessary re-renders

---

## SEO Considerations

1. **Page Title**: "Championnat de Vendée - Division 1 - Classements, Résultats"
2. **Meta Description**: "Suivez le classement, les résultats et le calendrier du championnat de Vendée de palet. Division 1, 2, 3."
3. **Structured Data**: Schema.org for SportsEvent, SportsTeam, Person
4. **Open Graph**: Share previews for championship standings
5. **Canonical URL**: Prevent duplicate content for different divisions

---

## Interaction Patterns

### 1. Sorting
- Click on table header to sort
- First click: ascending
- Second click: descending
- Third click: reset to default
- Visual indicator (arrow icon)
- Screen reader announcement

### 2. Filtering
- Real-time filter updates (debounced)
- Reset button to clear all filters
- Persistent filters in URL (shareable)
- Filter count badge

### 3. Tab Navigation
- Click to switch tabs
- URL hash updates (#results, #next-matches, etc.)
- Tab state preserved on page reload
- Unread count badges

### 4. Calendar Interaction
- Click on date to view matches
- Switch between month/list/agenda views
- Navigation between months
- Add to personal calendar

---

## Error States

### 1. No Data
```jsx
<EmptyState type="no-data">
  <Icon name="chart" size="xl" />
  <Text size="heading-l" weight="bold">
    Aucun résultat trouvé
  </Text>
  <Text size="body-m" color="gray-500">
    Essayez de modifier vos filtres
  </Text>
  <ButtonPrimary onClick={resetFilters}>
    Réinitialiser les filtres
  </ButtonPrimary>
</EmptyState>
```

### 2. No Matches
```jsx
<EmptyState type="no-matches">
  <Icon name="calendar" size="xl" />
  <Text size="heading-l" weight="bold">
    Aucun match prévu
  </Text>
  <Text size="body-m" color="gray-500">
    Le championnat est en pause
  </Text>
</EmptyState>
```

---

## Summary

| Section | Component | Purpose | Data Source |
|---------|-----------|---------|-------------|
| Page Header | PageHeader | Title, context | - |
| Filters | FiltersBar | Data filtering | User input |
| Stats Overview | StatsOverview | Key statistics | `/api/championships/{id}/statistics` |
| Classification | ClassificationTable | Standings | `/api/championships/{id}/classification` |
| Tabs | Tabs | View switching | User input |
| Results | ResultsTab | Past matches | `/api/championships/{id}/results` |
| Next Matches | NextMatchesTab | Upcoming matches | `/api/championships/{id}/next-matches` |
| Statistics | StatisticsTab | Detailed stats | `/api/championships/{id}/statistics` |
| Calendar | CalendarTab | Schedule | `/api/championships/{id}/calendar` |

---

## Implementation Checklist

- [ ] Page Header with breadcrumb
- [ ] Filters Bar with responsive behavior
- [ ] Stats Overview cards
- [ ] Enhanced Classification Table with sorting
- [ ] Tabs Navigation
- [ ] Results Tab content
- [ ] Next Matches Tab content
- [ ] Statistics Tab content with charts
- [ ] Calendar Tab content
- [ ] Responsive adaptations
- [ ] Data fetching and caching
- [ ] Error states
- [ ] Loading states
- [ ] Accessibility features
- [ ] SEO optimization

---

*Page specification - Championship Page*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Page**: [Tournament Page](tournament.md)

---

**Need implementation help?**
Let me know if you need code examples for any of these components in your specific framework (React, Vue, etc.).
