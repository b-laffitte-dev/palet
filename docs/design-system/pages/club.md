# Club Page - Palet Vendeen

> Club profile page displaying team information, players, results, and statistics

---

## Overview

The **Club Page** provides comprehensive information about a specific club, including:
- **Club profile** (name, location, history, contact)
- **Team information** (current teams, divisions)
- **Players roster** (club members with statistics)
- **Results and standings** (in current championships)
- **Tournament participation** (past and upcoming tournaments)
- **Club statistics** (performance, rankings)
- **News and announcements** (club-specific)

**Inspiration**: Top 14 club pages, Ligue 1 club profiles, local sports club websites

**Business Context**:
- Clubs are the base organization in Palet Vendeen
- Each club has multiple teams across different divisions
- Clubs are affiliated with commissions (CVDP for Vendée)
- Clubs organize their own tournaments and events

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ HEADER (80px)                                                  │
├─────────────────────────────────────────────────────────────┤
│ CLUB HEADER (160px)                                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ [Breadcrumb]                                                 ││
│  │ [Club Logo]                                                 ││
│  │ CLUB DE LA ROCHE                                           ││
│  │ FONDÉ EN 1985 • CVDP • Division 1                           ││
│  │                                                         ││
│  │ [Location] • [Website] • [Email] • [Phone]                  ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ CLUB NAVIGATION TABS (48px)                                   │
│  [Profil]  [Équipes]  [Joueurs]  [Résultats]  [Tournois]  [Stats]│
├─────────────────────────────────────────────────────────────┤
│ TAB CONTENT                                                   │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ PROFILE TAB                                                ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Club Information                                     │    ││
│  │  │  Description, history, president, etc.             │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Club Gallery                                          │    ││
│  │  │  [Photo 1] [Photo 2] [Photo 3]                       │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         │    ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Social Links                                          │    ││
│  │  │  [Facebook] [Twitter] [Instagram] [Website]          │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ TEAMS TAB                                                  ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Teams in Different Divisions                         │    ││
│  │  │  Division 1: La Roche A          [1er]              │    ││
│  │  │  Division 2: La Roche B          [3ème]             │    ││
│  │  │  Division 3: La Roche C          [2ème]             │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Team Details                                           │    ││
│  │  │  [Team Card with players, stats, next match]         │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ PLAYERS TAB                                               ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ [Search...] [Filter: All ▼] [Sort: Name ▼]           │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ [Player Card 1] [Player Card 2] [Player Card 3]    │    ││
│  │  │ [Player Card 4] [Player Card 5] ...                 │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  Pagination [1 | 2 | 3 | ...]                            ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ RESULTS TAB                                               ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Current Standings                                     │    ││
│  │  │  [Classification Table for each team]                │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Last Matches                                         │    ││
│  │  │  [Match List with results]                         │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Next Matches                                         │    ││
│  │  │  [Upcoming Match List]                              │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ TOURNAMENTS TAB                                            ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Upcoming Tournaments                                 │    ││
│  │  │  [Tournament Card 1] [Tournament Card 2] ...         │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Past Tournaments                                     │    ││
│  │  │  Results and performance                            │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Organized Tournaments                                 │    ││
│  │  │  Tournaments hosted by this club                    │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ STATISTICS TAB                                            ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Club Performance Stats                                 │    ││
│  │  │  Titles, rankings, best performances                   │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  │                                                         ││
│  │  ┌─────────────────────────────────────────────────┐    ││
│  │  │ Player Statistics Aggregate                          │    ││
│  │  │  Charts and data visualization                        │    ││
│  │  └─────────────────────────────────────────────────┘    ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ SIDEBAR (300px) - Optional for Desktop                        │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ QUICK INFO                                              ││
│  │  Präsident: Jean Dupont                                 ││
│  │  Vice-Président: Marie Martin                           ││
│  │  Trésorier: Pierre Leroy                                ││
│  │                                                         ││
│  │  Nombre de membres: 45                                  ││
│  │  Nombre d'équipes: 3                                    ││
│  │  Fondation: 1985                                        ││
│  │                                                         ││
│  │  MEILLEURS RÉSULTATS                                    ││
│  │  1. Championnat D1 2024 - 1er                           ││
│  │  2. Coupe de France 2023 - 1/4 Finale                   ││
│  │  3. Tournoi de Buzay 2025 - Vainqueur                  ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ PROCHAIN MATCH                                          ││
│  │  La Roche A vs Buzay United                            ││
│  │  Samedi 18 Octobre - 14:00                              ││
│  │  [Voir le match →]                                      ││
│  └─────────────────────────────────────────────────────────┘│
│                                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ ACTIONS                                                ││
│  │  [S'INSCRIRE AU CLUB]                                  ││
│  │  [CONTACTER LE CLUB]                                  ││
│  │  [VOIR LE CALENDRIER]                                  ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Club Header

**Purpose**: Clear club identification and branding

```jsx
<ClubHeader club={club}>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem href="/clubs">Clubs</BreadcrumbItem>
    <BreadcrumbItem active>{club.name}</BreadcrumbItem>
  </Breadcrumb>
  
  <ClubBranding>
    <ClubLogo src={club.logo} alt={club.name} size="lg" />
    <ClubTitle>{club.name}</ClubTitle>
    <ClubTagline>
      {club.foundedYear && `Fondé en ${club.foundedYear}`}
      {club.commission && ` • ${club.commission}`}
      {club.division && ` • ${club.division}`}
    </ClubTagline>
  </ClubBranding>
  
  <ClubMeta>
    <MetaItem icon="location">{club.location}</MetaItem>
    {club.website && <MetaItem icon="globe"><a href={club.website}>Site web</a></MetaItem>}
    {club.email && <MetaItem icon="mail"><a href={`mailto:${club.email}`}>Email</a></MetaItem>}
    {club.phone && <MetaItem icon="phone">{club.phone}</MetaItem>}
  </ClubMeta>
  
  <ClubActions>
    {userCanEdit && (
      <ButtonPrimary icon="edit" href={`/admin/clubs/${club.slug}/edit`}>
        Modifier
      </ButtonPrimary>
    )}
    <ButtonSecondary icon="share">Partager</ButtonSecondary>
    <ButtonSecondary icon="heart" active={isFavorite}>Favoris</ButtonSecondary>
  </ClubActions>
</ClubHeader>
```

---

### 2. Club Navigation Tabs

**Purpose**: Navigation between different club views

```jsx
<ClubTabs value={activeTab} onChange={setActiveTab}>
  <Tab value="profile" label="Profil" icon="info" />
  <Tab value="teams" label="Équipes" icon="users" badge={club.teamsCount} />
  <Tab value="players" label="Joueurs" icon="user" badge={club.playersCount} />
  <Tab value="results" label="Résultats" icon="trophy" />
  <Tab value="tournaments" label="Tournois" icon="flag" badge={club.upcomingTournamentsCount} />
  <Tab value="stats" label="Statistiques" icon="chart" />
</ClubTabs>
```

---

### 3. Profile Tab

**Purpose**: Club information and description

```jsx
<ProfileTab club={club}>
  <Section title="À propos du club">
    <ClubDescription>
      <MarkdownContent>{club.description}</MarkdownContent>
    </ClubDescription>
    
    <ClubDetailsGrid>
      <DetailItem label="Nom complet">{club.fullName}</DetailItem>
      <DetailItem label="Nom court">{club.shortName || club.name}</DetailItem>
      <DetailItem label="Fondé en">{club.foundedYear}</DetailItem>
      <DetailItem label="SIRET">{club.siret || 'Non spécifié'}</DetailItem>
      <DetailItem label="Affiliation">
        <AffiliationBadge affiliation={club.affiliation} />
      </DetailItem>
      <DetailItem label="Commission">
        <CommissionBadge commission={club.commission} />
      </DetailItem>
    </ClubDetailsGrid>
  </Section>
  
  <Section title="Dirigeants">
    <LeadersGrid>
      {club.leaders.map(leader => (
        <LeaderCard 
          key={leader.id}
          leader={leader}
          role={leader.role}
        />
      ))}
    </LeadersGrid>
  </Section>
  
  <Section title="Coordonnées">
    <ContactInfo>
      <ContactItem icon="location" label="Adresse">
        {club.address}, {club.postalCode} {club.city}
      </ContactItem>
      <ContactItem icon="globe" label="Site web">
        {club.website ? <a href={club.website} target="_blank">{club.website}</a> : 'Non spécifié'}
      </ContactItem>
      <ContactItem icon="mail" label="Email">
        {club.email ? <a href={`mailto:${club.email}`}>{club.email}</a> : 'Non spécifié'}
      </ContactItem>
      <ContactItem icon="phone" label="Téléphone">
        {club.phone || 'Non spécifié'}
      </ContactItem>
    </ContactInfo>
  </Section>
  
  {club.gallery && club.gallery.length > 0 && (
    <Section title="Galerie">
      <Gallery grid>
        {club.gallery.map((image, index) => (
          <GalleryItem 
            key={index}
            src={image.src}
            alt={image.alt}
            thumbnail
          />
        ))}
      </Gallery>
    </Section>
  )}
  
  <Section title="Réseaux sociaux">
    <SocialLinks>
      {club.social?.facebook && <SocialLink icon="facebook" href={club.social.facebook} />}
      {club.social?.twitter && <SocialLink icon="twitter" href={club.social.twitter} />}
      {club.social?.instagram && <SocialLink icon="instagram" href={club.social.instagram} />}
      {club.social?.youtube && <SocialLink icon="youtube" href={club.social.youtube} />}
    </SocialLinks>
  </Section>
</ProfileTab>
```

**Leader Card**:
```jsx
const LeaderCard = ({ leader, role }) => {
  const roleLabels = {
    president: 'Président',
    vice_president: 'Vice-Président',
    treasurer: 'Trésorier',
    secretary: 'Secrétaire',
    coach: 'Entraîneur',
    delegate: 'Délégué'
  };
  
  return (
    <Card className="leader-card">
      <CardHeader>
        <Avatar src={leader.avatar} size="md" />
        <LeaderName>{leader.name}</LeaderName>
        <RoleBadge>{roleLabels[role] || role}</RoleBadge>
      </CardHeader>
      <CardBody>
        {leader.email && <ContactLink icon="mail" href={`mailto:${leader.email}`}>Email</ContactLink>}
        {leader.phone && <ContactLink icon="phone" href={`tel:${leader.phone}`}>{leader.phone}</ContactLink>}
      </CardBody>
    </Card>
  );
};
```

---

### 4. Teams Tab

**Purpose**: Display club teams across divisions

```jsx
<TeamsTab club={club}>
  <TeamsHeader>
    <TeamsTitle>{club.teamsCount} équipes en compétition</TeamsTitle>
    
    <TeamsFilters>
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
        <FilterLabel>Catégorie</FilterLabel>
        <Select value={categoryFilter} onChange={setCategoryFilter}>
          <Option value="all">Toutes les catégories</Option>
          <Option value="FONTE">FONTE</Option>
          <Option value="LAITON">LAITON</Option>
        </Select>
      </FilterGroup>
    </TeamsFilters>
  </TeamsHeader>
  
  <TeamsGrid>
    {filteredTeams.map(team => (
      <TeamCard 
        key={team.id}
        team={team}
        showStanding
        showNextMatch
      />
    ))}
  </TeamsGrid>
  
  {filteredTeams.length === 0 && (
    <NoTeams>
      <InfoIcon />
      Aucun équipe trouvée pour les critères sélectionnés
      <ButtonSecondary onClick={() => setFilters({})}>Réinitialiser</ButtonSecondary>
    </NoTeams>
  )}
</TeamsTab>
```

**Team Card**:
```jsx
const TeamCard = ({ team, showStanding, showNextMatch }) => {
  return (
    <Card className="team-card">
      <CardHeader>
        <TeamLogo team={team} size="sm" />
        <TeamInfo>
          <TeamName>{team.name}</TeamName>
          <TeamDivision>{team.division} • {team.category}</TeamDivision>
        </TeamInfo>
        {showStanding && team.standing && (
          <StandingBadge position={team.standing.position} />
        )}
      </CardHeader>
      
      <CardBody>
        <TeamPlayers>
          {team.players.slice(0, 4).map(player => (
            <PlayerChip key={player.id} player={player} />
          ))}
          {team.players.length > 4 && (
            <MorePlayers>+{team.players.length - 4} autres</MorePlayers>
          )}
        </TeamPlayers>
        
        <TeamStats>
          <StatItem label="Victoires">{team.stats.wins}</StatItem>
          <StatItem label="Nuls">{team.stats.draws}</StatItem>
          <StatItem label="Défaites">{team.stats.losses}</StatItem>
          <StatItem label="Points">{team.stats.points}</StatItem>
        </TeamStats>
      </CardBody>
      
      {showNextMatch && team.nextMatch && (
        <CardFooter>
          <NextMatch match={team.nextMatch} />
        </CardFooter>
      )}
      
      <CardActions>
        <ButtonGhost onClick={() => navigate(`/equipes/${team.slug}`)}>
          Voir l'équipe
        </ButtonGhost>
      </CardActions>
    </Card>
  );
};
```

---

### 5. Players Tab

**Purpose**: Display club players roster

```jsx
<PlayersTab club={club}>
  <PlayersHeader>
    <PlayersTitle>{club.playersCount} joueurs membres</PlayersTitle>
    
    <PlayersActions>
      <SearchInput 
        placeholder="Rechercher un joueur..." 
        value={searchQuery} 
        onChange={setSearchQuery}
      />
      
      <FilterGroup>
        <FilterLabel>Catégorie</FilterLabel>
        <Select value={categoryFilter} onChange={setCategoryFilter}>
          <Option value="all">Tous</Option>
          <Option value="FONTE">FONTE</Option>
          <Option value="LAITON">LAITON</Option>
        </Select>
      </FilterGroup>
      
      <FilterGroup>
        <FilterLabel>Statut</FilterLabel>
        <Select value={statusFilter} onChange={setStatusFilter}>
          <Option value="all">Tous</Option>
          <Option value="active">Actifs</Option>
          <Option value="inactive">Inactifs</Option>
        </Select>
      </FilterGroup>
      
      <SortSelect value={sortBy} onChange={setSortBy}>
        <Option value="name">Trier par nom</Option>
        <Option value="ranking">Trier par classement</Option>
        <Option value="points">Trier par points</Option>
      </SortSelect>
    </PlayersActions>
  </PlayersHeader>
  
  <PlayersGrid>
    {filteredPlayers.map(player => (
      <PlayerCard 
        key={player.id}
        player={player}
        showClub={false}
        showStats
      />
    ))}
  </PlayersGrid>
  
  {filteredPlayers.length === 0 && (
    <NoPlayers>
      <InfoIcon />
      Aucun joueur trouvé
    </NoPlayers>
  )}
  
  <Pagination 
    currentPage={currentPage} 
    totalPages={totalPages} 
    onPageChange={setCurrentPage}
  />
</PlayersTab>
```

---

### 6. Results Tab

**Purpose**: Display club results and standings

```jsx
<ResultsTab club={club}>
  <CurrentStandings>
    <Section title="Classements actuels">
      <StandingsGrid>
        {club.teams.map(team => (
          <StandingCard 
            key={team.id}
            team={team}
            championship={team.championship}
          />
        ))}
      </StandingsGrid>
    </Section>
  </CurrentStandings>
  
  <LastMatches>
    <Section title="Derniers matchs">
      <MatchesFilter>
        <FilterGroup>
          <FilterLabel>Équipe</FilterLabel>
          <Select value={teamFilter} onChange={setTeamFilter}>
            <Option value="all">Toutes les équipes</Option>
            {club.teams.map(team => (
              <Option key={team.id} value={team.id}>{team.name}</Option>
            ))}
          </Select>
        </FilterGroup>
        
        <FilterGroup>
          <FilterLabel>Résultat</FilterLabel>
          <Select value={resultFilter} onChange={setResultFilter}>
            <Option value="all">Tous</Option>
            <Option value="win">Victoires</Option>
            <Option value="draw">Nuls</Option>
            <Option value="loss">Défaites</Option>
          </Select>
        </FilterGroup>
      </MatchesFilter>
      
      <MatchesList>
        {filteredMatches.map(match => (
          <MatchResultCard 
            key={match.id}
            match={match}
            showTeam
          />
        ))}
      </MatchesList>
    </Section>
  </LastMatches>
  
  <NextMatches>
    <Section title="Prochains matchs">
      <NextMatchesList>
        {club.upcomingMatches.map(match => (
          <NextMatchCard 
            key={match.id}
            match={match}
            showCountdown
          />
        ))}
      </NextMatchesList>
    </Section>
  </NextMatches>
</ResultsTab>
```

---

### 7. Tournaments Tab

**Purpose**: Display tournament participation

```jsx
<TournamentsTab club={club}>
  <UpcomingTournaments>
    <Section title="Tournois à venir" subtitle={`${club.upcomingTournamentsCount} tournois`}>
      <TournamentCards>
        {club.upcomingTournaments.map(tournament => (
          <TournamentCard 
            key={tournament.id}
            tournament={tournament}
            showRegistrationStatus={user?.clubId === club.id}
          />
        ))}
      </TournamentCards>
    </Section>
  </UpcomingTournaments>
  
  <PastTournaments>
    <Section title="Tournois passés" subtitle="Historique des participations"}>
      <PastTournamentsFilters>
        <FilterGroup>
          <FilterLabel>Année</FilterLabel>
          <Select value={yearFilter} onChange={setYearFilter}>
            <Option value="all">Toutes les années</Option>
            <Option value="2025">2025</Option>
            <Option value="2024">2024</Option>
            <Option value="2023">2023</Option>
          </Select>
        </FilterGroup>
      </PastTournamentsFilters>
      
      <PastTournamentsList>
        {filteredPastTournaments.map(tournament => (
          <PastTournamentCard 
            key={tournament.id}
            tournament={tournament}
            result={tournament.result}
          />
        ))}
      </PastTournamentsList>
    </Section>
  </PastTournaments>
  
  {club.organizedTournaments.length > 0 && (
    <OrganizedTournaments>
      <Section title="Tournois organisés" subtitle="Tournois accueillis par le club"}>
        <TournamentCards>
          {club.organizedTournaments.map(tournament => (
            <TournamentCard 
              key={tournament.id}
              tournament={tournament}
              showOrganizerBadge
            />
          ))}
        </TournamentCards>
      </Section>
    </OrganizedTournaments>
  )}
</TournamentsTab>
```

---

### 8. Statistics Tab

**Purpose**: Display club statistics and analytics

```jsx
<StatisticsTab club={club}>
  <ClubPerformance>
    <Section title="Performance du club">
      <PerformanceGrid>
        <StatCard>
          <StatValue>{club.stats.totalTitles}</StatValue>
          <StatLabel>Titres remportés</StatLabel>
        </StatCard>
        <StatCard>
          <StatValue>{club.stats.bestRanking}</StatValue>
          <StatLabel>Meilleur classement</StatLabel>
        </StatCard>
        <StatCard>
          <StatValue>{club.stats.totalMatches}</StatValue>
          <StatLabel>Matchs joués</StatLabel>
        </StatCard>
        <StatCard>
          <StatValue>{club.stats.winRate}%</StatValue>
          <StatLabel>Taux de victoire</StatLabel>
        </StatCard>
      </PerformanceGrid>
      
      <BestResults>
        <Subtitle>Meilleurs résultats</Subtitle>
        <ResultsList>
          {club.bestResults.map((result, index) => (
            <ResultItem 
              key={result.id}
              result={result}
              rank={index + 1}
            />
          ))}
        </ResultsList>
      </BestResults>
    </Section>
  </ClubPerformance>
  
  <SeasonStats>
    <Section title="Statistiques par saison">
      <SeasonSelector 
        seasons={club.seasons} 
        selectedSeason={selectedSeason} 
        onSelect={setSelectedSeason}
      />
      
      <SeasonStatsCharts>
        <ChartCard title="Performance par division">
          <BarChart 
            data={seasonStats.byDivision} 
            xAxis="Division" 
            yAxis="Points"
          />
        </ChartCard>
        
        <ChartCard title="Évolution des résultats">
          <LineChart 
            data={seasonStats.overTime} 
            xAxis="Saison" 
            yAxis="Position"
          />
        </ChartCard>
        
        <ChartCard title="Répartition par catégorie">
          <PieChart 
            data={seasonStats.byCategory} 
            labelKey="category" 
            valueKey="count"
          />
        </ChartCard>
      </SeasonStatsCharts>
    </Section>
  </SeasonStats>
  
  <PlayerStats>
    <Section title="Statistiques des joueurs">
      <PlayerStatsFilters>
        <FilterGroup>
          <FilterLabel>Saison</FilterLabel>
          <Select value={playerSeasonFilter} onChange={setPlayerSeasonFilter}>
            <Option value="all">Toutes les saisons</Option>
            {club.seasons.map(s => (
              <Option key={s} value={s}>{s}</Option>
            ))}
          </Select>
        </FilterGroup>
      </PlayerStatsFilters>
      
      <TopPerformers>
        <TopPerformersGrid>
          <TopPerformerCard 
            title="Meilleur marqueur" 
            player={club.topScorer} 
            stat={club.topScorerStats}
          />
          <TopPerformerCard 
            title="Meilleure précision" 
            player={club.topAccuracy} 
            stat={club.topAccuracyStats}
          />
          <TopPerformerCard 
            title="Plus de victoires" 
            player={club.mostWins} 
            stat={club.mostWinsStats}
          />
        </TopPerformersGrid>
      </TopPerformers>
      
      <PlayerStatsTable>
        <TableHeader>
          <TableCell>Joueur</TableCell>
          <TableCell>Matchs</TableCell>
          <TableCell>Victoires</TableCell>
          <TableCell>Points</TableCell>
          <TableCell>Précision</TableCell>
          <TableCell>Classement</TableCell>
        </TableHeader>
        
        {club.playerStats.map(playerStat => (
          <TableRow key={playerStat.player.id}>
            <TableCell>
              <PlayerCell player={playerStat.player} />
            </TableCell>
            <TableCell>{playerStat.matches}</TableCell>
            <TableCell>{playerStat.wins}</TableCell>
            <TableCell>{playerStat.points}</TableCell>
            <TableCell>
              <ProgressBar value={playerStat.accuracy} />
            </TableCell>
            <TableCell>{playerStat.ranking}</TableCell>
          </TableRow>
        ))}
      </PlayerStatsTable>
    </Section>
  </PlayerStats>
</StatisticsTab>
```

---

### 9. Sidebar

**Purpose**: Quick information and actions (desktop only)

```jsx
<ClubSidebar club={club}>
  <QuickInfo>
    <QuickInfoTitle>Informations rapides</QuickInfoTitle>
    <QuickInfoGrid>
      <QuickInfoItem label="Président">{club.president?.name}</QuickInfoItem>
      <QuickInfoItem label="Membres">{club.playersCount}</QuickInfoItem>
      <QuickInfoItem label="Équipes">{club.teamsCount}</QuickInfoItem>
      <QuickInfoItem label="Fondé en">{club.foundedYear}</QuickInfoItem>
      <QuickInfoItem label="Affiliation">{club.affiliation}</QuickInfoItem>
    </QuickInfoGrid>
  </QuickInfo>
  
  <BestResults>
    <BestResultsTitle>Meilleurs résultats récents</BestResultsTitle>
    <BestResultsList>
      {club.recentBestResults.map((result, index) => (
        <BestResultItem 
          key={result.id}
          result={result}
          rank={index + 1}
        />
      ))}
    </BestResultsList>
  </BestResults>
  
  <NextMatch>
    <NextMatchTitle>Prochain match</NextMatchTitle>
    {club.nextMatch ? (
      <NextMatchCard match={club.nextMatch} />
    ) : (
      <NoNextMatch>Aucun match à venir</NoNextMatch>
    )}
  </NextMatch>
  
  <ClubActions>
    <ClubActionsTitle>Actions</ClubActionsTitle>
    <ActionsList>
      {user && user.clubId !== club.id && (
        <ActionButton icon="user-plus" onClick={handleJoinRequest}>
          S'inscrire au club
        </ActionButton>
      )}
      <ActionButton icon="mail" onClick={handleContact}>
        Contacter le club
      </ActionButton>
      <ActionButton icon="calendar" href={`/calendrier?club=${club.slug}`}>
        Voir le calendrier
      </ActionButton>
      {userCanEdit && (
        <ActionButton icon="edit" href={`/admin/clubs/${club.slug}/edit`}>
          Modifier le club
        </ActionButton>
      )}
    </ActionsList>
  </ClubActions>
</ClubSidebar>
```

---

## Data Requirements

### API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/clubs/{slug}` | GET | Get club details | Club |
| `/api/clubs/{slug}/teams` | GET | Get club teams | Team[] |
| `/api/clubs/{slug}/players` | GET | Get club players | Player[] |
| `/api/clubs/{slug}/matches` | GET | Get club matches | Match[] |
| `/api/clubs/{slug}/tournaments` | GET | Get club tournaments | Tournament[] |
| `/api/clubs/{slug}/stats` | GET | Get club statistics | ClubStats |
| `/api/clubs/{slug}/leaders` | GET | Get club leaders | Leader[] |
| `/api/clubs/{slug}/favorite` | POST | Add to favorites | Favorite |
| `/api/clubs/{slug}/favorite` | DELETE | Remove from favorites | void |
| `/api/clubs/{slug}/join` | POST | Request to join club | JoinRequest |

### Club Object

```typescript
interface Club {
  id: string;
  slug: string;
  name: string;
  shortName: string;
  fullName: string;
  
  // Branding
  logo: string;
  colors: ClubColors;
  
  // Basic info
  foundedYear: number;
  description: string;
  location: string;
  address: string;
  postalCode: string;
  city: string;
  coordinates: { lat: number; lng: number };
  
  // Contact
  email: string;
  phone: string;
  website: string;
  
  // Affiliation
  affiliation: string; // e.g., "FNSMR"
  commission: string; // e.g., "CVDP"
  
  // Counts
  playersCount: number;
  teamsCount: number;
  upcomingTournamentsCount: number;
  
  // Leaders
  president: Leader;
  leaders: Leader[];
  
  // Social
  social: ClubSocial;
  
  // Gallery
  gallery: GalleryImage[];
  
  // Statistics
  stats: ClubStats;
  bestResults: ClubResult[];
  recentBestResults: ClubResult[];
  
  // Teams
  teams: Team[];
  
  // Players
  players: Player[];
  playerStats: PlayerStat[];
  topScorer: Player;
  topAccuracy: Player;
  mostWins: Player;
  
  // Matches
  upcomingMatches: Match[];
  recentMatches: Match[];
  nextMatch: Match | null;
  
  // Tournaments
  upcomingTournaments: Tournament[];
  pastTournaments: Tournament[];
  organizedTournaments: Tournament[];
  
  // Seasons
  seasons: string[];
  
  // Metadata
  createdAt: Date;
  updatedAt: Date;
  isFeatured: boolean;
  isVerified: boolean;
}

interface ClubStats {
  totalTitles: number;
  bestRanking: number | string;
  totalMatches: number;
  winRate: number;
  byDivision: Record<string, DivisionStats>;
  byCategory: Record<string, CategoryStats>;
  overTime: SeasonPerformance[];
}

interface Team {
  id: string;
  slug: string;
  name: string;
  division: string;
  category: string;
  championship: Championship;
  players: Player[];
  captain: Player;
  coach: User | null;
  stats: TeamStats;
  standing: ChampionshipStanding | null;
  nextMatch: Match | null;
  upcomingMatches: Match[];
}
```

---

## Page States

### 1. Club Not Found (404)
```jsx
<ClubNotFound>
  <NotFoundIcon />
  <NotFoundTitle>Club introuvable</NotFoundTitle>
  <NotFoundMessage>Le club que vous cherchez n'existe pas ou a été supprimé.</NotFoundMessage>
  <ButtonPrimary href="/clubs">Voir tous les clubs</ButtonPrimary>
</ClubNotFound>
```

### 2. Loading State
```jsx
<ClubSkeleton>
  <ClubHeaderSkeleton />
  <ClubTabsSkeleton />
  <ContentSkeleton />
  <SidebarSkeleton />
</ClubSkeleton>
```

### 3. Empty Data States
```jsx
// No teams
<NoTeams>
  <InfoIcon />
  Ce club n'a pas encore d'équipes en compétition
</NoTeams>

// No players
<NoPlayers>
  <InfoIcon />
  Ce club n'a pas encore de joueurs enregistrés
</NoPlayers>

// No matches
<NoMatches>
  <InfoIcon />
  Aucun match trouvé pour ce club
</NoMatches>

// No tournaments
<NoTournaments>
  <InfoIcon />
  Ce club ne participe à aucun tournoi actuellement
</NoTournaments>
```

---

## Error Handling

```jsx
// 404
if (error?.status === 404) return <ClubNotFound />;

// 403
if (error?.status === 403) {
  return <AccessDenied>Vous n'avez pas la permission de consulter ce club.</AccessDenied>;
}

// 500
if (error?.status >= 500) {
  return (
    <ServerError>
      <ServerErrorIcon />
      Une erreur est survenue. Veuillez réessayer plus tard.
      <ButtonPrimary onClick={() => refetch()}>Réessayer</ButtonPrimary>
    </ServerError>
  );
}
```

---

## Responsive Design

### Mobile (< 640px)
- Stacked layout
- Tabs scrollable horizontally
- Sidebar content moved into main content
- Cards full width

### Tablet (640px - 1023px)
- 2-column layout for some sections
- Sidebar visible as compact version
- Cards in grid (2 per row)

### Desktop (1024px+)
- Full layout as shown in diagrams
- Sidebar on the right
- Content area wide

---

## Accessibility

### Semantic HTML
```jsx
<main>
  <header>
    <h1>{club.name}</h1>
  </header>
  <nav aria-label="Navigation du club">...</nav>
  <aside aria-label="Informations complémentaires">...</aside>
</main>
```

### ARIA Labels
```jsx
<ClubTabs role="tablist" aria-label="Sections du club">
  <Tab role="tab" aria-selected={activeTab === 'profile'} ... />
</ClubTabs>

<TabPanel role="tabpanel" aria-labelledby="profile-tab" ... />
```

---

## Performance

### Data Fetching
```jsx
const { data: club } = useQuery({
  queryKey: ['club', slug],
  queryFn: () => fetchClub(slug),
  staleTime: 5 * 60 * 1000,
});

const { data: teams } = useQuery({
  queryKey: ['club-teams', slug],
  queryFn: () => fetchClubTeams(slug),
  enabled: !!club,
});

const { data: players } = useQuery({
  queryKey: ['club-players', slug, page],
  queryFn: () => fetchClubPlayers(slug, page),
  enabled: !!club,
});
```

### Lazy Loading
```jsx
const StatisticsTab = React.lazy(() => import('./StatisticsTab'));
const TournamentsTab = React.lazy(() => import('./TournamentsTab'));

<Suspense fallback={<TabSkeleton />}>
  {activeTab === 'stats' && <StatisticsTab club={club} />}
  {activeTab === 'tournaments' && <TournamentsTab club={club} />}
</Suspense>
```

---

## SEO

```jsx
<Helmet>
  <title>{club.name} - Palet Vendéen</title>
  <meta name="description" content={club.description || `Club de palet vendéen - ${club.location}`} />
  <meta name="keywords" content={`palet vendéen, club, ${club.name}, ${club.location}, ${club.commission}`} />
  
  <meta property="og:title" content={`${club.name} - Palet Vendéen`} />
  <meta property="og:description" content={club.description || `Club de palet vendéen à ${club.location}`} />
  <meta property="og:image" content={club.logo} />
  
  <link rel="canonical" href={`https://palet-vendeen.fr/clubs/${club.slug}`} />
  
  <script type="application/ld+json">
    {JSON.stringify(clubSchema(club))}
  </script>
</Helmet>
```

---

## File Structure

```
web/src/pages/club/
├── index.jsx              # Club page wrapper
├── ClubHeader.jsx         # Club header
├── ClubTabs.jsx           # Navigation tabs
├── ClubSidebar.jsx        # Sidebar component
├── tabs/
│   ├── ProfileTab.jsx     # Profile tab
│   ├── TeamsTab.jsx       # Teams tab
│   ├── PlayersTab.jsx     # Players tab
│   ├── ResultsTab.jsx     # Results tab
│   ├── TournamentsTab.jsx # Tournaments tab
│   └── StatisticsTab.jsx   # Statistics tab
├── components/
│   ├── TeamCard.jsx
│   ├── PlayerCard.jsx
│   ├── LeaderCard.jsx
│   ├── MatchResultCard.jsx
│   ├── NextMatchCard.jsx
│   ├── TournamentCard.jsx
│   ├── StatCard.jsx
│   ├── Gallery.jsx
│   └── ...
└── hooks/
    └── useClub.js          # Club data hook
```

---

## Next Steps

1. Create **Player page**
2. Create **Admin pages**
3. Create **News page**
4. Create **Login page**

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Business Context - Club Structure](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)
- [Tournament Page](tournament.md)
- [Match Page](match.md)
