# Match Page - Palet Vendeen

> Page displaying live or completed match details, statistics, and timeline

---

## Overview

The **Match Page** provides comprehensive information about a specific match, including:
- **Match header** with teams, scores, and status
- **Live score** updates for in-progress matches
- **Match details** (date, time, location, tournament context)
- **Score progression** and timeline of events
- **Statistics** for teams and players
- **Match information** (referee, duration, rules)
- **Related matches** (same tournament, same teams)

**Inspiration**: ESPN match pages, Ligue 1 live scores, Top 14 match centers

**Business Context**:
- Matches in Palet Vendeen can be part of a tournament, championship round, or friendly
- Each match has a specific format (13 or 15 points to win)
- Matches are played in sets/rounds (each "manche" is played to the target points)
- Live scoring is critical for following ongoing matches

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ MATCH HEADER (140px)                                          │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Breadcrumb]                                                 ││
│  │ Samedi 15 Novembre 2025 - 14:00                              ││
│  │                                                         ││
│  │  [Team A Logo]    [VS Badge]    [Team B Logo]              ││
│  │  TEAM A             12 - 15            TEAM B                ││
│  │  Club A             [BADGE: En cours]     Club B             ││
│  │                                                         ││
│  │  [Location] • [Tournament/Championship] • [Manche 1/3]     ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ LIVE SCORE BAR (50px) - if match is in progress               │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [LIVE] Match en cours - 55:32                              ││
│  │ [12-10] [12-11] [12-12] [13-12] [13-13] [13-14] [13-15]    ││
│  │       ▲                                                    ││
│  │       Dernier point: TEAM B                                ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ MATCH TABS (48px)                                             │
│  [Résumé]  [Score détaillés]  [Statistiques]  [Infosa]  [Suivis]│
├─────────────────────────────────────────────────────────────┤
│ TAB CONTENT                                                   │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ SUMMARY TAB                                                ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Match Overview                                        │    ││
│  │  │  Status: TERMINÉ                                      │    ││
│  │  │  Durée: 55:32                                          │    ││
│  │  │  Victorieux: TEAM B                                    │    ││
│  │  │  Points: 12 - 15                                      │    ││
│  │  │  Score par manche: 4-3, 3-4, 5-8                      │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Match Timeline                                        │    ││
│  │  │  Timeline of key events with timestamps             │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Key Moments                                           │    ││
│  │  │  Highlights from the match                           │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ DETAILED SCORES TAB                                      ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Manches (Rounds)                                    │    ││
│  │  │  ┌─────────────────────────────────────────────┐ │    ││
│  │  │  │ Manche 1  │  4 - 3  │  TEAM A        │  7 min │ │    ││
│  │  │  ├─────────────────────────────────────────────┤ │    ││
│  │  │  │ Manche 2  │  3 - 4  │  TEAM B        │  8 min │ │    ││
│  │  │  ├─────────────────────────────────────────────┤ │    ││
│  │  │  │ Manche 3  │  5 - 8  │  TEAM B        │ 12 min│ │    ││
│  │  │  └─────────────────────────────────────────────┘ │    ││
│  │  │                                                         │ │    ││
│  │  │ Total: 12 - 15                                           │ │    ││
│  │  │ Victorieux: TEAM B                                      │ │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Shot-by-shot breakdown (for advanced tracking)      │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ STATISTICS TAB                                             ││
│  │  ┌──────────────────┬──────────────────┐                 ││
│  │  │ TEAM A Stats     │ TEAM B Stats     │                 ││
│  │  │                  │                  │                 ││
│  │  │ Précision: 72%   │ Précision: 78%   │                 ││
│  │  │ Points marqués:  │ Points marqués:  │                 ││
│  │  │   12            │   15            │                 ││
│  │  │ Points contre:   │ Points contre:   │                 ││
│  │  │   15            │   12            │                 ││
│  │  │ Meilleur lancer:│ Meilleur lancer:│                 ││
│  │  │   4.2m          │   4.5m          │                 ││
│  │  └──────────────────┴──────────────────┘                 ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Player Statistics (if available)                      │    ││
│  │  │  [Player 1 Stats] [Player 2 Stats]                 │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ INFO TAB                                                  ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Match Information                                     │    ││
│  │  │  Arbitre: Jean Dupont                                 │    ││
│  │  │  Plaque: #12345                                      │    ││
│  │  │  Maître: Standard FNSMR                              │    ││
│  │  │  Catégorie: FONTE                                    │    ││
│  │  │  Distance: 3,80m                                     │    ││
│  │  │  Points pour victoire: 15                             │    ││
│  │  │  Format: 3 manches gagnantes                          │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Tournament/Championship Context                      │    ││
│  │  │  [Tournament Name]                                  │    ││
│  │  │  Phase: Phase finale                                │    ││
│  │  │  Round: 1/4 Finale                                   │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ FOLLOW TAB                                               ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Live Updates (for in-progress matches)               │    ││
│  │  │  Enable push notifications                          │    ││
│  │  │  Subscribe to match updates                          │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Related Matches                                      │    ││
│  │  │  Next match for TEAM A                               │    ││
│  │  │  Next match for TEAM B                               │    ││
│  │  │  Previous meeting between teams                      │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ MATCH ACTIONS (80px)                                          │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Partager] [Favoris] [Noter le match] [Signaler un problème]││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Match Header

**Purpose**: Clear match identification and current status

```jsx
<MatchHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    {match.tournament && (
      <BreadcrumbItem href={`/tournois/${match.tournament.slug}`}>
        {match.tournament.name}
      </BreadcrumbItem>
    )}
    {match.championship && (
      <BreadcrumbItem href={`/championnat/${match.championship.slug}`}>
        Championnat
      </BreadcrumbItem>
    )}
    <BreadcrumbItem active>Match</BreadcrumbItem>
  </Breadcrumb>
  
  <MatchDateTime>
    <DateTimeRow>
      <DateDisplay date={match.date} format="long" />
      <TimeDisplay time={match.time} />
    </DateTimeRow>
    <MatchStatus Badge status={match.status} />
  </MatchDateTime>
  
  <MatchTeams>
    <TeamSide alignment="left">
      <TeamLogo team={match.team1} size="lg" />
      <TeamInfo>
        <TeamName>{match.team1.name}</TeamName>
        <TeamClub>{match.team1.club.name}</TeamClub>
      </TeamInfo>
    </TeamSide>
    
    <MatchVsBadge />
    
    <TeamSide alignment="right">
      <TeamLogo team={match.team2} size="lg" />
      <TeamInfo>
        <TeamName>{match.team2.name}</TeamName>
        <TeamClub>{match.team2.club.name}</TeamClub>
      </TeamInfo>
    </TeamSide>
  </MatchTeams>
  
  <MatchScore>
    <ScoreDisplay>
      <ScoreValue status={getScoreStatus('team1', match)}>
        {match.score1}
      </ScoreValue>
      <ScoreSeparator>:</ScoreSeparator>
      <ScoreValue status={getScoreStatus('team2', match)}>
        {match.score2}
      </ScoreValue>
    </ScoreDisplay>
    
    {match.status === 'in_progress' && (
      <LiveIndicator />
    )}
    
    {match.winner && (
      <WinnerBadge winner={match.winner} />
    )}
  </MatchScore>
  
  <MatchMeta>
    <MetaItem icon="location">
      <MetaText>{match.location}</MetaText>
    </MetaItem>
    <MetaItem icon="trophy">
      <MetaText>
        {match.tournament ? match.tournament.name : match.championship ? match.championship.name : 'Match amical'}
      </MetaText>
    </MetaItem>
    <MetaItem icon="list">
      <MetaText>Manche {match.currentRound || match.totalRounds}</MetaText>
    </MetaItem>
    {match.referee && (
      <MetaItem icon="user">
        <MetaText>Arbitre: {match.referee}</MetaText>
      </MetaItem>
    )}
  </MatchMeta>
</MatchHeader>
```

**Score Status**:
- `winning`: Text color `success-600`, font weight `bold`
- `losing`: Text color `error-600`
- `tied`: Text color `gray-600`

**Match Status Badge**:
- `scheduled`: Background `gray-100`, Text `gray-800`, Label `À venir`
- `in_progress`: Background `primary-50`, Text `primary-800`, Label `En cours`
- `completed`: Background `success-50`, Text `success-800`, Label `Terminé`
- `cancelled`: Background `error-50`, Text `error-800`, Label `Annulé`
- `postponed`: Background `warning-50`, Text `warning-800`, Label `Reporté`

---

### 2. Live Score Bar

**Purpose**: Real-time score updates for in-progress matches

```jsx
{match.status === 'in_progress' && (
  <LiveScoreBar>
    <LiveIndicator>
      <PulseDot />
      <LiveText>Match en cours</LiveText>
    </LiveIndicator>
    
    <DurationDisplay>
      <ClockIcon />
      <DurationText>{formatDuration(match.duration)}</DurationText>
    </DurationDisplay>
    
    <ScoreProgress>
      {match.scoreHistory.map((score, index) => (
        <ScorePoint 
          key={index}
          score={score}
          isCurrent={index === match.scoreHistory.length - 1}
        >
          {score.team1}-{score.team2}
        </ScorePoint>
      ))}
    </ScoreProgress>
    
    <LastPoint>
      <ArrowIcon direction={match.lastPoint.scoringTeam} />
      <LastPointText>
        Dernier point: {match.lastPoint.scoringTeam === 'team1' ? match.team1.name : match.team2.name}
      </LastPointText>
    </LastPoint>
  </LiveScoreBar>
)}
```

**Score Progress Visualization**:
- Each score point is a pill-shaped badge
- Winning team's score in each point is highlighted in their team color
- Current score has a different background
- Arrow shows progression direction

---

### 3. Match Tabs

**Purpose**: Navigation between different match views

```jsx
<MatchTabs value={activeTab} onChange={setActiveTab}>
  <Tab value="summary" label="Résumé" icon="file-text" />
  <Tab value="scores" label="Scores détaillés" icon="list" />
  <Tab value="stats" label="Statistiques" icon="chart" />
  <Tab value="info" label="Infos" icon="info" />
  <Tab value="follow" label="Suivi" icon="bell" />
</MatchTabs>
```

---

### 4. Summary Tab

**Purpose**: Match overview and key information

#### Match Overview

```jsx
<Section title="Résumé du match">
  <OverviewGrid>
    <OverviewItem label="Statut">
      <StatusBadge status={match.status} />
    </OverviewItem>
    <OverviewItem label="Durée">
      {match.status === 'completed' ? match.duration : match.status === 'in_progress' ? match.currentDuration : 'À venir'}
    </OverviewItem>
    <OverviewItem label="Victoire">
      {match.winner ? (
        <WinnerDisplay winner={match.winner} />
      ) : (
        'Match non terminé'
      )}
    </OverviewItem>
    <OverviewItem label="Score final">
      <FinalScoreDisplay score1={match.score1} score2={match.score2} />
    </OverviewItem>
  </OverviewGrid>
  
  <OverviewGrid>
    <OverviewItem label="Manches">
      <RoundsDisplay 
        rounds={match.rounds} 
        currentRound={match.currentRound}
      />
    </OverviewItem>
    <OverviewItem label="Points totaux">
      {match.score1 + match.score2}
    </OverviewItem>
    <OverviewItem label="Écart">
      <DiffDisplay diff={Math.abs(match.score1 - match.score2)} />
    </OverviewItem>
    <OverviewItem label="Format">
      {match.format === 'best_of_3' ? 'Meilleur des 3' : match.format === 'best_of_5' ? 'Meilleur des 5' : 'Simple'}
    </OverviewItem>
  </OverviewGrid>
</Section>
```

#### Match Timeline

```jsx
<Section title="Chronologie du match">
  <Timeline>
    {match.events.map(event => (
      <TimelineItem 
        key={event.id}
        timestamp={event.timestamp}
        type={event.type}
        team={event.team}
      >
        <TimelineTime>{formatTime(event.timestamp)}</TimelineTime>
        <TimelineIcon type={event.type} />
        <TimelineContent>
          <EventDescription event={event} match={match} />
        </TimelineContent>
      </TimelineItem>
    ))}
    
    {match.status === 'scheduled' && (
      <TimelineItem type="scheduled">
        <TimelineTime>{formatTime(match.time)}</TimelineTime>
        <TimelineIcon type="scheduled" />
        <TimelineContent>
          Match prévu - {formatDate(match.date)}
        </TimelineContent>
      </TimelineItem>
    )}
  </Timeline>
</Section>
```

**Event Types**:
- `point`: Point scored
- `round_win`: Round/manche won
- `match_start`: Match started
- `match_end`: Match ended
- `timeout`: Timeout
- `foul`: Foul/penalty

#### Key Moments

```jsx
<Section title="Moments clés">
  {match.highlights && match.highlights.length > 0 ? (
    <HighlightsList>
      {match.highlights.map((highlight, index) => (
        <HighlightCard 
          key={highlight.id}
          index={index + 1}
          type={highlight.type}
          time={highlight.time}
          team={highlight.team}
          description={highlight.description}
        />
      ))}
    </HighlightsList>
  ) : (
    <NoHighlights>
      <InfoIcon />
      Aucun moment clé enregistré pour ce match
    </NoHighlights>
  )}
</Section>
```

---

### 5. Detailed Scores Tab

**Purpose**: Breakdown of scoring by rounds/manches

```jsx
<Section title="Scores détaillés">
  <RoundsTable>
    <TableHeader>
      <TableCell>Manche</TableCell>
      <TableCell>{match.team1.name}</TableCell>
      <TableCell>{match.team2.name}</TableCell>
      <TableCell>Gagnant</TableCell>
      <TableCell>Durée</TableCell>
      <TableCell>Actions</TableCell>
    </TableHeader>
    
    {match.rounds.map((round, index) => (
      <TableRow key={round.id}>
        <TableCell>Manche {index + 1}</TableCell>
        <TableCell>
          <RoundScore score={round.score1} status={getRoundStatus('team1', round)} />
        </TableCell>
        <TableCell>
          <RoundScore score={round.score2} status={getRoundStatus('team2', round)} />
        </TableCell>
        <TableCell>
          {round.winner ? (
            <WinnerCell winner={round.winner} />
          ) : (
            'En cours'
          )}
        </TableCell>
        <TableCell>{round.duration}</TableCell>
        <TableCell>
          <RoundActions round={round} />
        </TableCell>
      </TableRow>
    ))}
    
    <TableFooter>
      <TableCell><strong>Total</strong></TableCell>
      <TableCell><strong>{match.score1}</strong></TableCell>
      <TableCell><strong>{match.score2}</strong></TableCell>
      <TableCell>
        <WinnerBadge winner={match.winner} />
      </TableCell>
      <TableCell><strong>{match.duration}</strong></TableCell>
      <TableCell></TableCell>
    </TableFooter>
  </RoundsTable>
  
  {match.shotTrackingEnabled && (
    <Section title="Détail des lancers">
      <ShotsTable>
        <TableHeader>
          <TableCell>#</TableCell>
          <TableCell>Équipe</TableCell>
          <TableCell>Joueur</TableCell>
          <TableCell>Distance</TableCell>
          <TableCell>Résultat</TableCell>
          <TableCell>Points</TableCell>
        </TableHeader>
        
        {match.shots.map((shot, index) => (
          <ShotRow 
            key={shot.id}
            index={index + 1}
            shot={shot}
            team={shot.team === 'team1' ? match.team1 : match.team2}
          />
        ))}
      </ShotsTable>
    </Section>
  )}
</Section>
```

**Shot Row Component**:
```jsx
const ShotRow = ({ index, shot, team }) => {
  const getResultColor = (result) => {
    switch(result) {
      case 'perfect': return 'success';
      case 'close': return 'primary';
      case 'miss': return 'error';
      default: return 'gray';
    }
  };
  
  return (
    <TableRow>
      <TableCell>{index}</TableCell>
      <TableCell>
        <TeamChip team={team} />
      </TableCell>
      <TableCell>
        {shot.player?.name || 'Équipe'}
      </TableCell>
      <TableCell>
        {shot.distance}m
      </TableCell>
      <TableCell>
        <ResultBadge result={shot.result} color={getResultColor(shot.result)} />
      </TableCell>
      <TableCell>
        {shot.points > 0 && `+${shot.points}`}
      </TableCell>
    </TableRow>
  );
};
```

---

### 6. Statistics Tab

**Purpose**: Team and player performance statistics

```jsx
<Section title="Statistiques du match">
  <TeamComparison>
    <TeamStats team={match.team1} stats={match.stats.team1} />
    <TeamStats team={match.team2} stats={match.stats.team2} />
  </TeamComparison>
  
  <ComparisonCharts>
    <ChartCard title="Précision">
      <BarChart 
        data={[
          { name: match.team1.name, value: match.stats.team1.accuracy },
          { name: match.team2.name, value: match.stats.team2.accuracy }
        ]}
        yAxis="%"
      />
    </ChartCard>
    
    <ChartCard title="Points par manche">
      <LineChart 
        data={match.stats.rounds}
        xAxis="Manche"
        yAxis="Points"
        series={[
          { name: match.team1.name, dataKey: 'team1' },
          { name: match.team2.name, dataKey: 'team2' }
        ]}
      />
    </ChartCard>
  </ComparisonCharts>
</Section>

{match.playersStats && match.playersStats.length > 0 && (
  <Section title="Statistiques par joueur">
    <PlayersStatsTable>
      <TableHeader>
        <TableCell>Joueur</TableCell>
        <TableCell>Équipe</TableCell>
        <TableCell>Lancers</TableCell>
        <TableCell>Points</TableCell>
        <TableCell>Précision</TableCell>
        <TableCell>Meilleur lancer</TableCell>
      </TableHeader>
      
      {match.playersStats.map(playerStat => (
        <TableRow key={playerStat.player.id}>
          <TableCell>
            <PlayerCell player={playerStat.player} />
          </TableCell>
          <TableCell>
            <TeamChip team={playerStat.team} />
          </TableCell>
          <TableCell>{playerStat.shots}</TableCell>
          <TableCell>{playerStat.points}</TableCell>
          <TableCell>
            <ProgressBar value={playerStat.accuracy} />
          </TableCell>
          <TableCell>{playerStat.bestShot}m</TableCell>
        </TableRow>
      ))}
    </PlayersStatsTable>
  </Section>
)}

{match.advancedStats && (
  <Section title="Statistiques avancées">
    <AdvancedStatsGrid>
      <StatCard>
        <StatLabel>Lancer le plus long</StatLabel>
        <StatValue>{match.advancedStats.longestShot}m</StatValue>
        <StatDetail>par {match.advancedStats.longestShotPlayer}</StatDetail>
      </StatCard>
      <StatCard>
        <StatLabel>Lancer le plus précis</StatLabel>
        <StatValue>{match.advancedStats.mostAccurateShot}%</StatValue>
        <StatDetail>par {match.advancedStats.mostAccuratePlayer}</StatDetail>
      </StatCard>
      <StatCard>
        <StatLabel>Série de points</StatLabel>
        <StatValue>{match.advancedStats.longestStreak}</StatValue>
        <StatDetail>par {match.advancedStats.longestStreakTeam}</StatDetail>
      </StatCard>
      <StatCard>
        <StatLabel>Temps moyen par manche</StatLabel>
        <StatValue>{match.advancedStats.avgRoundTime}</StatValue>
      </StatCard>
    </AdvancedStatsGrid>
  </Section>
)}
```

---

### 7. Info Tab

**Purpose**: Match metadata and context

```jsx
<Section title="Informations du match">
  <InfoGrid>
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="user" />
        <InfoCardTitle>Arbitre</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        {match.referee ? (
          <RefereeDisplay referee={match.referee} />
        ) : (
          'Non spécifié'
        )}
      </InfoCardBody>
    </InfoCard>
    
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="target" />
        <InfoCardTitle>Plaque</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        <PlaqueInfo plaque={match.plaque} />
      </InfoCardBody>
    </InfoCard>
    
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="bullseye" />
        <InfoCardTitle>Maître</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        <MaitreInfo maitre={match.maitre} />
      </InfoCardBody>
    </InfoCard>
    
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="ruler" />
        <InfoCardTitle>Distance</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        {match.distance}m ({match.category})
      </InfoCardBody>
    </InfoCard>
  </InfoGrid>
  
  <InfoGrid>
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="trophy" />
        <InfoCardTitle>Points pour victoire</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        {match.pointsToWin} points
      </InfoCardBody>
    </InfoCard>
    
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="list" />
        <InfoCardTitle>Format</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        {match.format === 'best_of_3' ? 'Meilleur des 3 manches' : 
         match.format === 'best_of_5' ? 'Meilleur des 5 manches' : 
         `Première à ${match.pointsToWin} points`}
      </InfoCardBody>
    </InfoCard>
    
    <InfoCard>
      <InfoCardHeader>
        <InfoCardIcon name="clock" />
        <InfoCardTitle>Durée estimée</InfoCardTitle>
      </InfoCardHeader>
      <InfoCardBody>
        ~1 heure par match
      </InfoCardBody>
    </InfoCard>
  </InfoGrid>
</Section>

<Section title="Contexte">
  {match.tournament && (
    <ContextCard>
      <ContextCardHeader>
        <ContextCardIcon name="trophy" />
        <ContextCardTitle>Tournoi</ContextCardTitle>
      </ContextCardHeader>
      <ContextCardBody>
        <TournamentContext tournament={match.tournament} match={match} />
      </ContextCardBody>
    </ContextCard>
  )}
  
  {match.championship && (
    <ContextCard>
      <ContextCardHeader>
        <ContextCardIcon name="flag" />
        <ContextCardTitle>Championnat</ContextCardTitle>
      </ContextCardHeader>
      <ContextCardBody>
        <ChampionshipContext championship={match.championship} match={match} />
      </ContextCardBody>
    </ContextCard>
  )}
  
  {!match.tournament && !match.championship && (
    <ContextCard>
      <ContextCardHeader>
        <ContextCardIcon name="heart" />
        <ContextCardTitle>Match amical</ContextCardTitle>
      </ContextCardHeader>
      <ContextCardBody>
        <FriendlyMatchInfo match={match} />
      </ContextCardBody>
    </ContextCard>
  )}
</Section>

<Section title="Règles appliquées">
  <RulesList>
    <RuleItem>
      <RuleIcon name="check-circle" />
      <RuleText>Distance de lancer: {match.distance}m ({match.category})</RuleText>
    </RuleItem>
    <RuleItem>
      <RuleIcon name="check-circle" />
      <RuleText>Points pour gagner: {match.pointsToWin}</RuleText>
    </RuleItem>
    <RuleItem>
      <RuleIcon name="check-circle" />
      <RuleText>Format: {match.formatDescription}</RuleText>
    </RuleItem>
    {match.specialRules && match.specialRules.length > 0 && (
      match.specialRules.map(rule => (
        <RuleItem key={rule}>
          <RuleIcon name="info-circle" />
          <RuleText>{rule}</RuleText>
        </RuleItem>
      ))
    )}
  </RulesList>
</Section>
```

---

### 8. Follow Tab

**Purpose**: Live updates and related content

```jsx
<Section title="Suivi du match">
  {match.status === 'in_progress' && (
    <LiveUpdates>
      <LiveUpdatesHeader>
        <LiveUpdatesTitle>Mises à jour en direct</LiveUpdatesTitle>
        <LiveUpdatesToggle 
          checked={liveUpdatesEnabled}
          onChange={setLiveUpdatesEnabled}
          label="Activer les notifications"
        />
      </LiveUpdatesHeader>
      
      <LiveUpdatesList>
        {liveUpdates.map(update => (
          <LiveUpdateItem 
            key={update.id}
            update={update}
            isNew={isNewUpdate(update)}
          />
        ))}
      </LiveUpdatesList>
    </LiveUpdates>
  )}
  
  {match.status !== 'scheduled' && (
    <RelatedMatches>
      <RelatedMatchesTitle>Matchs liés</RelatedMatchesTitle>
      
      <RelatedMatchesGrid>
        <RelatedMatchGroup title="Prochain match de TEAM A">
          {nextMatches.team1.map(nextMatch => (
            <RelatedMatchCard 
              key={nextMatch.id}
              match={nextMatch}
              currentTeam={match.team1}
            />
          ))}
        </RelatedMatchGroup>
        
        <RelatedMatchGroup title="Prochain match de TEAM B">
          {nextMatches.team2.map(nextMatch => (
            <RelatedMatchCard 
              key={nextMatch.id}
              match={nextMatch}
              currentTeam={match.team2}
            />
          ))}
        </RelatedMatchGroup>
        
        <RelatedMatchGroup title="Derniers matchs entre ces équipes">
          {previousMeetings.map(prevMatch => (
            <RelatedMatchCard 
              key={prevMatch.id}
              match={prevMatch}
              showResult
            />
          ))}
        </RelatedMatchGroup>
        
        <RelatedMatchGroup title="Matchs dans le même tournoi">
          {sameTournamentMatches.map(tourMatch => (
            <RelatedMatchCard 
              key={tourMatch.id}
              match={tourMatch}
              showTournamentContext
            />
          ))}
        </RelatedMatchGroup>
      </RelatedMatchesGrid>
    </RelatedMatches>
  )}
  
  {match.status === 'scheduled' && (
    <Countdown>
      <CountdownTitle>Début du match</CountdownTitle>
      <CountdownDisplay 
        targetDate={new Date(`${match.date}T${match.time}`)}
        showDays
      />
      <CountdownActions>
        <ButtonPrimary icon="bell" onClick={handleSetReminder}>
          Me rappeler
        </ButtonPrimary>
      </CountdownActions>
    </Countdown>
  )}
</Section>
```

---

### 9. Match Actions

**Purpose**: User actions related to the match

```jsx
<MatchActionsBar>
  <ActionsGroup>
    <ButtonGhost icon="share" onClick={handleShare}>
      Partager
    </ButtonGhost>
    <ShareModal 
      isOpen={shareModalOpen}
      onClose={() => setShareModalOpen(false)}
      match={match}
    />
    
    <ButtonGhost icon="heart" onClick={handleToggleFavorite} active={isFavorite}>
      {isFavorite ? 'Enregistré' : 'Favoris'}
    </ButtonGhost>
    
    <ButtonGhost icon="star" onClick={handleRateMatch}>
      Noter le match
    </ButtonGhost>
    <RateMatchModal 
      isOpen={rateModalOpen}
      onClose={() => setRateModalOpen(false)}
      match={match}
    />
  </ActionsGroup>
  
  <ActionsGroup>
    <ButtonGhost icon="flag" onClick={handleReport}>
      Signaler un problème
    </ButtonGhost>
    <ReportModal 
      isOpen={reportModalOpen}
      onClose={() => setReportModalOpen(false)}
      match={match}
    />
    
    {userIsAdmin && (
      <>
        <ButtonGhost icon="edit" onClick={handleEditMatch}>
          Modifier
        </ButtonGhost>
        <ButtonGhost icon="trash" onClick={handleDeleteMatch} variant="danger">
          Supprimer
        </ButtonGhost>
      </>
    )}
    
    {userIsReferee && match.status === 'scheduled' && (
      <ButtonPrimary icon="play" onClick={handleStartMatch}>
        Commencer le match
      </ButtonPrimary>
    )}
    
    {userIsReferee && match.status === 'in_progress' && (
      <ButtonPrimary icon="stop" onClick={handleEndMatch}>
        Terminer le match
      </ButtonPrimary>
    )}
  </ActionsGroup>
</MatchActionsBar>
```

---

## Data Requirements

### API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/matches/{id}` | GET | Get match details | Match object |
| `/api/matches/{id}/events` | GET | Get match events/timeline | Event[] |
| `/api/matches/{id}/stats` | GET | Get match statistics | Stats |
| `/api/matches/{id}/highlights` | GET | Get match highlights | Highlight[] |
| `/api/matches/{id}/shots` | GET | Get shot tracking data | Shot[] |
| `/api/matches/{id}/live` | GET | Get live updates | LiveUpdate[] |
| `/api/matches/{id}/related` | GET | Get related matches | Match[] |
| `/api/matches/{id}/favorite` | POST | Add to favorites | Favorite |
| `/api/matches/{id}/favorite` | DELETE | Remove from favorites | void |
| `/api/matches/{id}/report` | POST | Report a problem | Report |
| `/api/matches/{id}/rating` | POST | Rate the match | Rating |

### Match Object

```typescript
interface Match {
  id: string;
  
  // Teams
  team1: MatchTeam;
  team2: MatchTeam;
  
  // Scores
  score1: number;
  score2: number;
  
  // Status
  status: 'scheduled' | 'in_progress' | 'completed' | 'cancelled' | 'postponed';
  
  // Dates
  date: Date;
  time: string;
  startTime: Date | null;
  endTime: Date | null;
  duration: string | null; // HH:MM
  currentDuration: string | null; // For in-progress matches
  
  // Location
  location: string;
  address: string | null;
  coordinates: { lat: number; lng: number } | null;
  
  // Context
  tournament: Tournament | null;
  championship: Championship | null;
  round: string | null; // e.g., "Journée 12", "1/4 Finale"
  phase: string | null;
  
  // Format
  category: 'FONTE' | 'LAITON' | 'BOIS';
  type: 'INDIVIDUEL' | 'DOUBLETTE' | 'TRIPLETTE';
  format: 'best_of_3' | 'best_of_5' | 'single_game';
  pointsToWin: 13 | 15;
  distance: 3.8 | 2.8;
  
  // Current state
  currentRound: number | null;
  totalRounds: number | null;
  
  // Results
  winner: 'team1' | 'team2' | 'draw' | null;
  
  // Officials
  referee: string | null;
  
  // Equipment
  plaque: PlaqueInfo;
  maitre: MaitreInfo;
  
  // Tracking
  shotTrackingEnabled: boolean;
  liveUpdatesEnabled: boolean;
  
  // Metadata
  createdAt: Date;
  updatedAt: Date;
  isFeatured: boolean;
  
  // Rounds
  rounds: MatchRound[];
  
  // Score history
  scoreHistory: ScoreHistory[];
  lastPoint: LastPointInfo;
  
  // Special rules
  specialRules: string[];
}

interface MatchTeam {
  id: string;
  name: string;
  slug: string;
  club: Club;
  players: Player[];
  logo: string | null;
  stats: TeamMatchStats;
}

interface MatchRound {
  id: string;
  number: number;
  score1: number;
  score2: number;
  winner: 'team1' | 'team2' | null;
  duration: string | null;
}

interface ScoreHistory {
  team1: number;
  team2: number;
  timestamp: Date;
  scoringTeam: 'team1' | 'team2';
}

interface LastPointInfo {
  scoringTeam: 'team1' | 'team2';
  points: number;
  timestamp: Date;
  player: Player | null;
}

interface TeamMatchStats {
  accuracy: number; // percentage
  pointsFor: number;
  pointsAgainst: number;
  shots: number;
  perfectShots: number;
  bestShot: number; // meters
}

interface PlaqueInfo {
  id: string;
  number: string;
  material: string;
  size: string;
  weight: number; // kg
}

interface MaitreInfo {
  id: string;
  type: 'standard' | 'custom';
  description: string;
}

// Event types
interface MatchEvent {
  id: string;
  type: 'point' | 'round_win' | 'match_start' | 'match_end' | 'timeout' | 'foul';
  timestamp: Date;
  team: 'team1' | 'team2' | null;
  player: Player | null;
  description: string;
  data: any;
}

// Highlight types
interface MatchHighlight {
  id: string;
  type: 'point' | 'round' | 'match' | 'performance';
  time: string;
  team: 'team1' | 'team2';
  player: Player | null;
  description: string;
  importance: 1 | 2 | 3; // 1 = low, 3 = high
}

// Shot tracking
interface MatchShot {
  id: string;
  team: 'team1' | 'team2';
  player: Player | null;
  distance: number; // meters from master
  result: 'perfect' | 'close' | 'miss' | 'foul';
  points: number;
  timestamp: Date;
}
```

---

### Required Data for Page Render

**Initial Page Load**:
- Match object (basic info)
- Team information for both teams
- Match status
- User authentication state
- User favorites status

**Summary Tab**:
- Match events/timeline
- Match highlights
- Match overview data

**Detailed Scores Tab**:
- Round-by-round scores
- Shot tracking data (if enabled)

**Statistics Tab**:
- Team statistics
- Player statistics (if available)
- Advanced statistics

**Info Tab**:
- Referee information
- Equipment information
- Tournament/championship context
- Rules information

**Follow Tab**:
- Live updates (if in progress)
- Related matches
- User preferences (favorites, reminders)

---

## Page States

### 1. Match Not Found (404)

```jsx
<MatchNotFound>
  <NotFoundIcon />
  <NotFoundTitle>Match introuvable</NotFoundTitle>
  <NotFoundMessage>
    Le match que vous cherchez n'existe pas ou a été supprimé.
  </NotFoundMessage>
  <ButtonPrimary href="/">Retour à l'accueil</ButtonPrimary>
  <ButtonSecondary href="/calendrier">Voir le calendrier</ButtonSecondary>
</MatchNotFound>
```

### 2. Loading State

```jsx
<MatchSkeleton>
  <MatchHeaderSkeleton />
  <LiveScoreBarSkeleton />
  <TabsSkeleton />
  <ContentSkeleton />
</MatchSkeleton>
```

### 3. Scheduled Match (À venir)

- Show countdown to match start
- Show "À venir" badge
- Schedule tab shows match information
- Results tab shows "Match non commencé"
- Live updates not available
- Show registration reminder option

### 4. In Progress Match (En cours)

- Show "En cours" badge
- Show live score bar with real-time updates
- Show current duration
- Show score progression
- Show last point information
- Enable live updates
- Show current round/manche

### 5. Completed Match (Terminé)

- Show "Terminé" badge
- Show winner prominently
- All tabs fully populated
- Statistics tab shows complete data
- Show final score
- Hide live updates

### 6. Cancelled Match (Annulé)

- Show "Annulé" badge
- Show cancellation reason if available
- Disabled state for interactive elements
- Show reschedule information if available

### 7. Postponed Match (Reporté)

- Show "Reporté" badge
- Show new date/time if available
- Show reason for postponement

---

## Error Handling

### API Errors

```jsx
// 404 - Match not found
if (error?.status === 404) {
  return <MatchNotFound />;
}

// 403 - Access denied
if (error?.status === 403) {
  return (
    <AccessDenied>
      <AccessDeniedIcon />
      <AccessDeniedTitle>Accès refusé</AccessDeniedTitle>
      <AccessDeniedMessage>
        Vous n'avez pas la permission de consulter ce match.
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

## Real-time Updates

### WebSocket Integration

```jsx
// Connect to WebSocket for live match updates
useEffect(() => {
  if (!match || match.status !== 'in_progress') return;
  
  const ws = new WebSocket(`wss://api.palet-vendeen.fr/matches/${match.id}/live`);
  
  ws.onopen = () => {
    console.log('Connected to match WebSocket');
    // Send authentication if needed
    ws.send(JSON.stringify({ type: 'auth', token: userToken }));
  };
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    switch (data.type) {
      case 'point':
        updateScore(data);
        addToTimeline(data);
        break;
      case 'round_end':
        updateRound(data);
        break;
      case 'match_end':
        updateMatchStatus('completed');
        break;
      case 'duration':
        updateDuration(data.duration);
        break;
      default:
        break;
    }
  };
  
  ws.onerror = (error) => {
    console.error('WebSocket error:', error);
  };
  
  ws.onclose = () => {
    console.log('WebSocket closed');
  };
  
  return () => {
    ws.close();
  };
}, [match?.id, match?.status]);

// Polling fallback for browsers without WebSocket support
useEffect(() => {
  if (!match || match.status !== 'in_progress' || !supportsWebSocket) {
    const interval = setInterval(() => {
      refetchMatch();
    }, 10000); // Poll every 10 seconds
    
    return () => clearInterval(interval);
  }
}, [match?.id, match?.status]);
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
│ MATCH HEADER          │
│  [Date]               │
│  [Team A]             │
│  VS                   │
│  [Team B]             │
│  [Score]              │
│  [Status]             │
│  [Meta]               │
├──────────────────────┤
│ LIVE SCORE BAR        │
│  (full width)        │
├──────────────────────┤
│ TABS (scrollable)     │
├──────────────────────┤
│ TAB CONTENT           │
│  (full width)        │
├──────────────────────┤
│ MATCH ACTIONS         │
│  (2x2 grid)          │
└──────────────────────┘
```

**Adaptations**:
- Teams displayed vertically: Team A above, VS badge, Team B below
- Score displayed large and centered
- Live score bar simplified
- Tabs scrollable horizontally
- Match actions in 2x2 grid

### Tablet (640px - 1023px)

```
┌──────────────────────────────────┐
│ TOP BAR                         │
├──────────────────────────────────┤
│ HEADER                          │
├──────────────────────────────────┤
│ MATCH HEADER                    │
│  [Date] [Status]                │
│  [Team A]  VS  [Team B]         │
│  [Score]                       │
│  [Meta]                         │
├──────────────────────────────────┤
│ LIVE SCORE BAR                  │
├──────────────────────────────────┤
│ TABS                           │
├──────────────────────────────────┤
│ TAB CONTENT                     │
│  (1 or 2 column layout)        │
├──────────────────────────────────┤
│ MATCH ACTIONS                   │
│  (horizontal)                   │
└──────────────────────────────────┘
```

**Adaptations**:
- Teams displayed horizontally with VS in center
- Live score bar with score progression
- Tabs always visible
- 2-column layout for some tab content

### Desktop (1024px+)

Use full maquette layout as shown in the main diagram.

---

## Accessibility

### Semantic HTML

```jsx
<main>
  <header>
    <h1>Match: {match.team1.name} vs {match.team2.name}</h1>
  </header>
  
  <nav aria-label="Navigation du match">
    <ul>
      <li><a href="#resume">Résumé</a></li>
      <li><a href="#scores">Scores détaillés</a></li>
      {/* ... */}
    </ul>
  </nav>
  
  <section id="resume" aria-labelledby="resume-heading">
    <h2 id="resume-heading">Résumé du match</h2>
    {/* Content */}
  </section>
  
  <aside aria-label="Actions du match">
    {/* Action buttons */}
  </aside>
</main>
```

### ARIA Live Regions

For real-time updates:

```jsx
<div aria-live="polite" aria-atomic="true">
  <LiveScoreDisplay score1={match.score1} score2={match.score2} />
</div>

<div aria-live="assertive" className="sr-only">
  {lastUpdateAnnouncement}
</div>
```

### Keyboard Navigation

- **Tabs**: Arrow keys to navigate
- **Timeline**: Focusable events
- **Actions**: Keyboard accessible buttons
- **Score display**: Focusable for screen readers

### Color Contrast

All color combinations meet WCAG 2.1 AA:
- Team names on backgrounds
- Score display
- Status badges
- Interactive elements

---

## Performance Optimizations

### Data Fetching

```jsx
// Use TanStack Query with appropriate caching
const { data: match, isLoading, error } = useQuery({
  queryKey: ['match', id],
  queryFn: () => fetchMatch(id),
  staleTime: 30 * 1000, // 30 seconds for live matches
  refetchInterval: match?.status === 'in_progress' ? 10000 : undefined,
});

// Prefetch related data
const { data: events } = useQuery({
  queryKey: ['match-events', id],
  queryFn: () => fetchMatchEvents(id),
  enabled: !!match,
  staleTime: 60 * 1000,
});

const { data: stats } = useQuery({
  queryKey: ['match-stats', id],
  queryFn: () => fetchMatchStats(id),
  enabled: !!match,
  staleTime: 5 * 60 * 1000,
});

// Separate query for live updates (WebSocket or polling)
const { data: liveData } = useQuery({
  queryKey: ['match-live', id],
  queryFn: () => fetchMatchLive(id),
  enabled: !!match && match.status === 'in_progress',
  refetchInterval: 5000,
  staleTime: 0, // Always fresh for live data
});
```

### Lazy Loading

```jsx
// Lazy load heavy components
const StatisticsTab = React.lazy(() => import('./StatisticsTab'));
const ShotTrackingTable = React.lazy(() => import('./ShotTrackingTable'));

// Suspense boundaries
<Suspense fallback={<StatisticsSkeleton />}>
  <StatisticsTab match={match} />
</Suspense>

<Suspense fallback={<ShotsSkeleton />}>
  <ShotTrackingTable shots={match.shots} />
</Suspense>
```

---

## SEO

### Page Metadata

```jsx
<Helmet>
  <title>{match.team1.name} vs {match.team2.name} - Palet Vendéen</title>
  <meta name="description" content={`Match de palet vendéen: ${match.team1.name} vs ${match.team2.name}. Score: ${match.score1}-${match.score2}. ${match.status === 'in_progress' ? 'En cours' : match.status === 'completed' ? 'Terminé' : 'À venir'}.`} />
  <meta name="keywords" content={`palet vendéen, match, ${match.team1.name}, ${match.team2.name}, ${match.tournament?.name || match.championship?.name || 'amical'}, ${match.location}`} />
  
  <meta property="og:title" content={`${match.team1.name} vs ${match.team2.name} - Palet Vendéen`} />
  <meta property="og:description" content={`Match de palet vendéen entre ${match.team1.name} et ${match.team2.name}. Score final: ${match.score1}-${match.score2}.`} />
  <meta property="og:type" content="website" />
  <meta property="og:url" content={`https://palet-vendeen.fr/match/${match.id}`} />
  <meta property="og:image" content={match.ogImage || match.team1.logo} />
  
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content={`${match.team1.name} vs ${match.team2.name} - Palet Vendéen`} />
  <meta name="twitter:description" content={`Match de palet vendéen: ${match.team1.name} vs ${match.team2.name}. Score: ${match.score1}-${match.score2}.`} />
  
  <link rel="canonical" href={`https://palet-vendeen.fr/match/${match.id}`} />
  
  <script type="application/ld+json">
    {JSON.stringify(matchSchema(match))}
  </script>
</Helmet>
```

---

## Integration Points

### With Other Pages

1. **Homepage**: Live match section links to match pages
2. **Tournament**: Tournament matches link to match pages
3. **Championship**: Championship matches link to match pages
4. **Club**: Club matches link to match pages
5. **Player**: Player matches link to match pages
6. **Calendar**: Calendar entries link to match pages

### With Admin Interface

1. **Admin Match List**: Links to match pages
2. **Admin Match Edit**: Links to public match page
3. **Admin Results Entry**: Links to match page
4. **Admin Live Scoring**: Real-time updates sent to match page

---

## Testing Checklist

### Functional Tests

- [ ] Page loads with valid match ID
- [ ] 404 displayed for invalid match ID
- [ ] All tabs are functional
- [ ] Real-time updates work (WebSocket or polling)
- [ ] Score progression displays correctly
- [ ] Timeline shows all events
- [ ] Statistics display correctly
- [ ] Favorites work
- [ ] Sharing works
- [ ] Reporting works
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
- [ ] ARIA live regions work
- [ ] ARIA attributes present

### Performance Tests

- [ ] Page loads in < 2.5 seconds
- [ ] No layout shifts (CLS < 0.1)
- [ ] Fast interaction response (FID < 100ms)
- [ ] Memory usage reasonable
- [ ] Real-time updates don't block UI

---

## Implementation Notes

### Recommended Approach

1. **Create the basic layout first**
   - Header with teams and score
   - Status badge and meta information
   - Tabs navigation

2. **Implement real-time functionality**
   - WebSocket connection for live matches
   - Fallback to polling
   - Live score bar

3. **Implement each tab separately**
   - Start with Summary tab
   - Then Detailed Scores tab
   - Then Statistics tab
   - Then Info tab
   - Finally Follow tab

4. **Add match actions**
   - Favorite, share, rate, report
   - Admin actions if applicable

5. **Add responsive styles**
   - Mobile-first approach
   - Test on all breakpoints

6. **Add accessibility features**
   - Semantic HTML
   - ARIA attributes
   - Keyboard navigation
   - ARIA live regions

7. **Optimize performance**
   - Efficient data fetching
   - Lazy loading for heavy components
   - Image optimization

---

## File Structure

```
web/src/pages/match/
├── index.jsx              # Main match page
├── MatchHeader.jsx        # Match header component
├── LiveScoreBar.jsx       # Live score bar component
├── MatchTabs.jsx          # Tabs navigation
├── SummaryTab.jsx         # Summary tab content
├── DetailedScoresTab.jsx  # Detailed scores tab
├── StatisticsTab.jsx      # Statistics tab content
├── InfoTab.jsx            # Info tab content
├── FollowTab.jsx          # Follow tab content
├── MatchActionsBar.jsx    # Match actions bar
├── components/
│   ├── TeamDisplay.jsx
│   ├── ScoreDisplay.jsx
│   ├── Timeline.jsx
│   ├── TimelineItem.jsx
│   ├── RoundTable.jsx
│   ├── ShotTable.jsx
│   ├── TeamStats.jsx
│   ├── LiveUpdateItem.jsx
│   ├── RelatedMatchCard.jsx
│   ├── Countdown.jsx
│   └── ...
├── hooks/
│   ├── useMatch.js        # Custom hook for match data
│   ├── useMatchLive.js    # Custom hook for live updates
│   └── useMatchTabs.js
└── utils/
    ├── formatters.js      # Date, time, score formatters
    └── matchHelpers.js    # Match-related utilities
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
- `components/ProgressBar`
- `hooks/useWebSocket`
- `hooks/useCountdown`
- `services/api`
- `services/websocket`
- `styles/design-system.css`

---

## Example Data

### Sample Match Object

```json
{
  "id": "match-abc123",
  "team1": {
    "id": "team-a",
    "name": "La Roche A",
    "slug": "la-roche-a",
    "club": {
      "id": "club-1",
      "name": "Club de La Roche",
      "slug": "club-la-roche"
    },
    "players": [
      {
        "id": "player-1",
        "name": "Jean Morice",
        "license": "LIC-001"
      },
      {
        "id": "player-2",
        "name": "Pierre Dupont",
        "license": "LIC-002"
      }
    ],
    "logo": "/logos/la-roche.png",
    "stats": {
      "accuracy": 72,
      "pointsFor": 12,
      "pointsAgainst": 15,
      "shots": 24,
      "perfectShots": 3,
      "bestShot": 4.2
    }
  },
  "team2": {
    "id": "team-b",
    "name": "Buzay United",
    "slug": "buzay-united",
    "club": {
      "id": "club-2",
      "name": "Club de Buzay",
      "slug": "club-buzay"
    },
    "players": [
      {
        "id": "player-3",
        "name": "Marie Martin",
        "license": "LIC-003"
      },
      {
        "id": "player-4",
        "name": "Thomas Leroy",
        "license": "LIC-004"
      }
    ],
    "logo": "/logos/buzay.png",
    "stats": {
      "accuracy": 78,
      "pointsFor": 15,
      "pointsAgainst": 12,
      "shots": 22,
      "perfectShots": 5,
      "bestShot": 4.5
    }
  },
  "score1": 12,
  "score2": 15,
  "status": "completed",
  "date": "2025-11-15",
  "time": "14:00",
  "startTime": "2025-11-15T14:00:00Z",
  "endTime": "2025-11-15T15:12:00Z",
  "duration": "01:12",
  "location": "Salle des sports de Buzay",
  "address": "85320 Buzay",
  "coordinates": { "lat": 46.8, "lng": -1.5 },
  "tournament": {
    "id": "coupe-france-2025",
    "name": "Coupe de France - FONTE - Doublette",
    "slug": "coupe-france-fonte-doublette-2025"
  },
  "championship": null,
  "round": "1/4 Finale",
  "phase": "Phase finale",
  "category": "FONTE",
  "type": "DOUBLETTE",
  "format": "best_of_3",
  "pointsToWin": 15,
  "distance": 3.8,
  "currentRound": 3,
  "totalRounds": 3,
  "winner": "team2",
  "referee": "Jean Dupont",
  "plaque": {
    "id": "plaque-123",
    "number": "12345",
    "material": "Plomb",
    "size": "45x45 cm",
    "weight": 20
  },
  "maitre": {
    "id": "maitre-1",
    "type": "standard",
    "description": "Maître FNSMR standard"
  },
  "shotTrackingEnabled": true,
  "liveUpdatesEnabled": true,
  "isFeatured": true,
  "rounds": [
    {
      "id": "round-1",
      "number": 1,
      "score1": 4,
      "score2": 3,
      "winner": "team1",
      "duration": "18:00"
    },
    {
      "id": "round-2",
      "number": 2,
      "score1": 3,
      "score2": 4,
      "winner": "team2",
      "duration": "22:00"
    },
    {
      "id": "round-3",
      "number": 3,
      "score1": 5,
      "score2": 8,
      "winner": "team2",
      "duration": "32:00"
    }
  ],
  "scoreHistory": [
    { "team1": 0, "team2": 0, "timestamp": "2025-11-15T14:00:00Z", "scoringTeam": null },
    { "team1": 1, "team2": 0, "timestamp": "2025-11-15T14:05:00Z", "scoringTeam": "team1" },
    { "team1": 1, "team2": 1, "timestamp": "2025-11-15T14:08:00Z", "scoringTeam": "team2" },
    { "team1": 2, "team2": 1, "timestamp": "2025-11-15T14:12:00Z", "scoringTeam": "team1" },
    { "team1": 4, "team2": 1, "timestamp": "2025-11-15T14:18:00Z", "scoringTeam": "team1" },
    { "team1": 4, "team2": 2, "timestamp": "2025-11-15T14:22:00Z", "scoringTeam": "team2" },
    { "team1": 4, "team2": 3, "timestamp": "2025-11-15T14:25:00Z", "scoringTeam": "team2" }
  ],
  "lastPoint": {
    "scoringTeam": "team2",
    "points": 1,
    "timestamp": "2025-11-15T15:12:00Z",
    "player": {
      "id": "player-4",
      "name": "Thomas Leroy"
    }
  },
  "specialRules": []
}
```

---

## Next Steps

After implementing the Match page:

1. **Create Registration page** - For tournament registration flow
2. **Create Club page** - For club profiles
3. **Create Player page** - For player profiles
4. **Create Admin pages** - For application management

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Design System - Layouts](/docs/design-system/layouts.md)
- [Design System - Icons](/docs/design-system/icons.md)
- [Business Context - Match Rules](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)
- [Tournament Page Specification](tournament.md)
- [Championship Page Specification](championnat.md)
- [Homepage Specification](homepage.md)

---

*Match Page Specification - Palet Vendeen Design System*
*Last Updated: 09/10/2026*
*Version: 1.0*
