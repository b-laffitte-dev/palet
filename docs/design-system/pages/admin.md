# Admin Dashboard - Palet Vendeen

> Administrative interface for managing competitions, players, clubs, and system settings

---

## Overview

The **Admin Dashboard** provides comprehensive administrative capabilities for managing the Palet Vendéen platform:

- **Competition Management**: Create, edit, and manage championships and tournaments
- **Player & Team Management**: Add, modify, and organize players and teams
- **Club Management**: Manage club information and affiliations
- **Results & Scoring**: Enter and validate match results
- **User Management**: Manage user accounts and permissions
- **System Settings**: Configure platform-wide settings
- **Reporting & Analytics**: Access insights and generate reports

**Business Context**: Multiple admin levels exist:
- **Super Admin**: Full system access (FNSMR level)
- **Commission Admin**: Regional access (CVDP, CDSMR level)
- **Club Admin**: Club-level access only

---

## Page Structure

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px)                                                  │
│  Palet Vendéen | [Notifications] [Profile] [Logout]              │
├─────────────────────────────────────────────────────────────┤
│ SIDEBAR NAVIGATION (240px, collapsible)                       │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ 📊 TABLEAU DE BORD                                    ││
│  │                                                                 ││
│  │ 🏆 COMPÉTITIONS                                       ││
│  │ ├── Championnat                                     ││
│  │ ├── Tournois                                         ││
│  │ └── Résultats                                        ││
│  │                                                                 ││
│  │ 👥 JOUEURS & ÉQUIPES                                 ││
│  │ ├── Joueurs                                         ││
│  │ ├── Équipes                                          ││
│  │ └── Clubs                                            ││
│  │                                                                 ││
│  │ ⚙️ GESTION SYSTÈME                                   ││
│  │ ├── Utilisateurs                                    ││
│  │ ├── Rôles & Permissions                             ││
│  │ └── Paramètres                                       ││
│  │                                                                 ││
│  │ 📈 RAPPORTS & STATS                                  ││
│  │ ├── Rapports                                        ││
│  │ └── Export                                          ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ MAIN CONTENT AREA                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ ADMIN HEADER (60px)                                       ││
│  │  ┌─────────────────────────────────────────────────┐   ││
│  │  │ Dashboard / Championnat                        [+]   │   ││
│  │  └─────────────────────────────────────────────────┘   ││
│  │                                                                 ││
│  │ PAGE CONTENT                                          ││
│  │  ┌─────────────────────────────────────────────────┐   ││
│  │  │ [Current view content]                            │   ││
│  │  │                                                     │   ││
│  │  │ 📊 DASHBOARD OVERVIEW (example)                   │   ││
│  │  │ ┌─────────┬─────────┬─────────┬─────────┐       │   ││
│  │  │ │ Totaux  │ En cours │ À venir  │ Terminés │       │   ││
│  │  │ │ 24      │ 8        │ 12       │ 4        │       │   ││
│  │  │ │ Compét. │ Compét. │ Compét. │ Compét. │       │   ││
│  │  │ ├─────────┼─────────┼─────────┼─────────┤       │   ││
│  │  │ │ 1,245   │ 342      │ 189      │ 714      │       │   ││
│  │  │ │ Joueurs │ Joueurs  │ Joueurs  │ Joueurs  │       │   ││
│  │  │ └─────────┴─────────┴─────────┴─────────┘       │   ││
│  │  └─────────────────────────────────────────────────┘   ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ FOOTER (40px)                                                  │
│  © 2026 Palet Vendéen - Version 1.0.0                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Navigation Structure

### Main Navigation Groups

#### 1. Dashboard
- Overview with key metrics and statistics

#### 2. Competitions Management
- **Championships** (`/admin/championships`)
  - List, create, edit, delete championships
  - Manage divisions and seasons
  - Configure classification rules
- **Tournaments** (`/admin/tournaments`)
  - List, create, edit, delete tournaments
  - Manage registrations and participants
  - Configure tournament structure (poules, brackets)
- **Results** (`/admin/results`)
  - Enter and validate match results
  - Manage score disputes
  - Bulk result import

#### 3. Players & Teams Management
- **Players** (`/admin/players`)
  - List, add, edit, suspend players
  - Manage player licenses
  - Import/export player data
- **Teams** (`/admin/teams`)
  - Create, modify, dissolve teams
  - Manage team memberships
  - Team history and stats
- **Clubs** (`/admin/clubs`)
  - List, create, edit club information
  - Manage club affiliations and hierarchies
  - Club statistics and reporting

#### 4. System Management
- **Users** (`/admin/users`)
  - List and manage user accounts
  - Account creation and deactivation
  - User activity monitoring
- **Roles & Permissions** (`/admin/roles`)
  - Define and manage user roles
  - Permission assignment
  - Audit logging
- **Settings** (`/admin/settings`)
  - Platform configuration
  - Season settings
  - General parameters

#### 5. Reports & Analytics
- **Reports** (`/admin/reports`)
  - Predefined report templates
  - Custom report builder
  - Scheduled report generation
- **Export** (`/admin/export`)
  - Data export capabilities
  - Integration with external systems
  - Export history

---

## Key Pages & Components

### 1. Dashboard Overview

```jsx
<AdminDashboard>
  <DashboardHeader>
    <WelcomeMessage>
      Bonjour, {user.firstName} !
    </WelcomeMessage>
    <DashboardActions>
      <ButtonPrimary onClick={createQuickAction}>
        <Icon name="plus" size="sm" /> Nouvelle compétition
      </ButtonPrimary>
      <ButtonSecondary onClick={viewPendingItems}>
        <Icon name="bell" size="sm" /> {pendingCount} en attente
      </ButtonSecondary>
    </DashboardActions>
  </DashboardHeader>
  
  <MetricsGrid>
    <MetricCard 
      title="Total Compétitions" 
      value={stats.totalCompetitions} 
      icon="trophy" 
      trend={stats.competitionsTrend}
      color="gold"
    />
    <MetricCard 
      title="Joueurs actifs" 
      value={stats.activePlayers} 
      icon="users" 
      trend={stats.playersTrend}
      color="primary"
    />
    <MetricCard 
      title="Clubs inscrits" 
      value={stats.registeredClubs} 
      icon="home" 
      trend={stats.clubsTrend}
      color="success"
    />
    <MetricCard 
      title="Matchs joués" 
      value={stats.totalMatches} 
      icon="target" 
      trend={stats.matchesTrend}
      color="info"
    />
  </MetricsGrid>
  
  <DashboardSections>
    <Section title="Prochaines compétitions">
      <UpcomingCompetitionsList competitions={upcoming} />
    </Section>
    
    <Section title="Alertes & Notifications">
      <AlertsList alerts={alerts} onDismiss={dismissAlert} />
    </Section>
    
    <Section title="Activité récente">
      <RecentActivityList activities={recentActivities} />
    </Section>
  </DashboardSections>
</AdminDashboard>
```

---

### 2. Competition Management

#### Championship List

```jsx
<ChampionshipsList>
  <ListHeader>
    <ListTitle>Championnats</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Nouveau championnat
      </ButtonPrimary>
      <Dropdown label="Exporter">
        <DropdownItem onClick={exportCSV}>CSV</DropdownItem>
        <DropdownItem onClick={exportJSON}>JSON</DropdownItem>
        <DropdownItem onClick={exportPDF}>PDF</DropdownItem>
      </Dropdown>
    </ListActions>
  </ListHeader>
  
  <ListFilters>
    <FilterGroup>
      <FilterLabel>Saison</FilterLabel>
      <Select value={seasonFilter} onChange={setSeasonFilter}>
        <Option value="all">Toutes les saisons</Option>
        <Option value="2025-2026">2025/2026</Option>
        <Option value="2024-2025">2024/2025</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Statut</FilterLabel>
      <Select value={statusFilter} onChange={setStatusFilter}>
        <Option value="all">Tous les statuts</Option>
        <Option value="active">Actif</Option>
        <Option value="pending">En attente</Option>
        <Option value="completed">Terminé</Option>
        <Option value="cancelled">Annulé</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Rechercher</FilterLabel>
      <Input type="search" placeholder="Nom, saison..." />
    </FilterGroup>
  </ListFilters>
  
  <ChampionshipsTable>
    <TableHeader>
      <TableHeaderCell sortable>Nom</TableHeaderCell>
      <TableHeaderCell sortable>Saison</TableHeaderCell>
      <TableHeaderCell sortable>Division</TableHeaderCell>
      <TableHeaderCell textAlign="center">Équipes</TableHeaderCell>
      <TableHeaderCell textAlign="center">Journées</TableHeaderCell>
      <TableHeaderCell textAlign="center">Statut</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {championships.map(champ => (
        <ChampionshipRow key={champ.id} championship={champ}>
          <TableCell>
            <ChampionshipName>{champ.name}</ChampionshipName>
            <ChampionshipCode>{champ.code}</ChampionshipCode>
          </TableCell>
          <TableCell>{champ.season}</TableCell>
          <TableCell>{champ.divisions.join(', ')}</TableCell>
          <TableCell textAlign="center">{champ.teamsCount}</TableCell>
          <TableCell textAlign="center">
            {champ.currentJournee} / {champ.totalJournees}
          </TableCell>
          <TableCell textAlign="center">
            <StatusBadge status={champ.status} />
          </TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => viewDetails(champ.id)}>
                <Icon name="eye" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => editChampionship(champ.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => duplicateChampionship(champ.id)}>
                <Icon name="copy" size="sm" />
              </ButtonGhost>
              {champ.status === 'pending' && (
                <ButtonGhost size="sm" onClick={() => publishChampionship(champ.id)}>
                  <Icon name="play" size="sm" />
                </ButtonGhost>
              )}
              {champ.status === 'active' && (
                <ButtonGhost size="sm" onClick={() => completeChampionship(champ.id)}>
                  <Icon name="check" size="sm" />
                </ButtonGhost>
              )}
            </ActionButtons>
          </TableCell>
        </ChampionshipRow>
      ))}
    </TableBody>
  </ChampionshipsTable>
  
  <Pagination 
    current={currentPage} 
    total={totalPages} 
    onChange={setCurrentPage}
  />
</ChampionshipsList>
```

---

#### Championship Create/Edit Form

```jsx
<ChampionshipForm onSubmit={handleSubmit}>
  <FormHeader>
    {editing ? 'Modifier le championnat' : 'Nouveau championnat'}
  </FormHeader>
  
  <FormTabs>
    <Tab id="info">Informations générales</Tab>
    <Tab id="structure">Structure & Divisions</Tab>
    <Tab id="schedule">Calendrier</Tab>
    <Tab id="teams">Équipes</Tab>
    <Tab id="settings">Paramètres</Tab>
  </FormTabs>
  
  <TabPanel id="info">
    <FormSection title="Informations de base">
      <FormGrid>
        <FormField label="Nom du championnat" required>
          <Input value={name} onChange={setName} />
        </FormField>
        <FormField label="Code" required>
          <Input value={code} onChange={setCode} maxLength={10} />
        </FormField>
        <FormField label="Saison" required>
          <Select value={season} onChange={setSeason}>
            <Option value="2025-2026">2025/2026</Option>
            <Option value="2024-2025">2024/2025</Option>
          </Select>
        </FormField>
        <FormField label="Catégorie" required>
          <Select value={category} onChange={setCategory}>
            <Option value="fonte">Fonte</Option>
            <Option value="laiton">Laiton</Option>
            <Option value="bois">Bois</Option>
            <Option value="mixte">Mixte</Option>
          </Select>
        </FormField>
        <FormField label="Description">
          <Input type="textarea" value={description} onChange={setDescription} />
        </FormField>
      </FormGrid>
    </FormSection>
    
    <FormSection title="Paramètres du championnat">
      <FormGrid>
        <FormField label="Type de points">
          <Select value={pointsType} onChange={setPointsType}>
            <Option value="13">13 points (standard)</Option>
            <Option value="15">15 points (finale)</Option>
          </Select>
        </FormField>
        <FormField label="Points pour victoire">
          <Input type="number" value={winPoints} onChange={setWinPoints} />
        </FormField>
        <FormField label="Points pour nul">
          <Input type="number" value={drawPoints} onChange={setDrawPoints} />
        </FormField>
        <FormField label="Points pour défaite">
          <Input type="number" value={lossPoints} onChange={setLossPoints} />
        </FormField>
        <FormField label="Format">
          <Select value={format} onChange={setFormat}>
            <Option value="round-robin">Round Robin (tous contre tous)</Option>
            <Option value="league">Ligue (aller-retour)</Option>
            <Option value="groups">Groupes + élimination directe</Option>
          </Select>
        </FormField>
      </FormGrid>
    </FormSection>
  </TabPanel>
  
  <TabPanel id="structure">
    <FormSection title="Divisions">
      <DivisionsEditor 
        divisions={divisions} 
        onAddDivision={addDivision} 
        onRemoveDivision={removeDivision} 
        onUpdateDivision={updateDivision}
      />
    </FormSection>
  </TabPanel>
  
  <TabPanel id="schedule">
    <FormSection title="Calendrier">
      <ScheduleEditor 
        schedule={schedule} 
        onChange={setSchedule}
        championships={otherChampionships}
      />
    </FormSection>
  </TabPanel>
  
  <TabPanel id="teams">
    <FormSection title="Équipes participantes">
      <TeamsSelector 
        selectedTeams={selectedTeams} 
        availableTeams={availableTeams} 
        onChange={setSelectedTeams}
      />
    </FormSection>
  </TabPanel>
  
  <TabPanel id="settings">
    <FormSection title="Paramètres avancés">
      <FormGrid>
        <FormField label="Inscription automatique">
          <Checkbox 
            checked={autoRegistration} 
            onChange={setAutoRegistration} 
            help="Permettre aux équipes de s'inscrire directement"
          />
        </FormField>
        <FormField label="Validation manuelle requise">
          <Checkbox 
            checked={manualValidation} 
            onChange={setManualValidation} 
            help="Requérir validation admin pour les inscriptions"
          />
        </FormField>
        <FormField label="Date limite d'inscription">
          <DatePicker 
            value={registrationDeadline} 
            onChange={setRegistrationDeadline}
          />
        </FormField>
        <FormField label="Date de début">
          <DatePicker value={startDate} onChange={setStartDate} />
        </FormField>
        <FormField label="Date de fin">
          <DatePicker value={endDate} onChange={setEndDate} />
        </FormField>
        <FormField label="Visible publiquement">
          <Checkbox 
            checked={isPublic} 
            onChange={setIsPublic} 
            help="Afficher dans les listes publiques"
          />
        </FormField>
        <FormField label="Archivé">
          <Checkbox 
            checked={isArchived} 
            onChange={setIsArchived} 
            help="Marquer comme archivé (lecture seule)"
          />
        </FormField>
      </FormGrid>
    </FormSection>
  </TabPanel>
  
  <FormActions>
    <ButtonSecondary onClick={handleCancel}>Annuler</ButtonSecondary>
    <ButtonPrimary type="submit" loading={saving}>
      {editing ? 'Enregistrer les modifications' : 'Créer le championnat'}
    </ButtonPrimary>
  </FormActions>
</ChampionshipForm>
```

---

### 3. Tournament Management

#### Tournament List

```jsx
<TournamentsList>
  <ListHeader>
    <ListTitle>Tournois</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Nouveau tournoi
      </ButtonPrimary>
      <ButtonSecondary onClick={importTournaments}>
        <Icon name="upload" size="sm" /> Importer
      </ButtonSecondary>
    </ListActions>
  </ListHeader>
  
  <ListFilters>
    <FilterGroup>
      <FilterLabel>Type</FilterLabel>
      <Select value={typeFilter} onChange={setTypeFilter}>
        <Option value="all">Tous les types</Option>
        <Option value="championship">Championnat</Option>
        <Option value="cup">Coupe</Option>
        <Option value="friendly">Amical</Option>
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
      <FilterLabel>Statut</FilterLabel>
      <Select value={statusFilter} onChange={setStatusFilter}>
        <Option value="all">Tous les statuts</Option>
        <Option value="pending">En préparation</Option>
        <Option value="registration">Inscription ouverte</Option>
        <Option value="in_progress">En cours</Option>
        <Option value="completed">Terminé</Option>
      </Select>
    </FilterGroup>
  </ListFilters>
  
  <TournamentsTable>
    <TableHeader>
      <TableHeaderCell>Nom</TableHeaderCell>
      <TableHeaderCell>Type</TableHeaderCell>
      <TableHeaderCell>Catégorie</TableHeaderCell>
      <TableHeaderCell textAlign="center">Date</TableHeaderCell>
      <TableHeaderCell textAlign="center">Inscrits</TableHeaderCell>
      <TableHeaderCell textAlign="center">Statut</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {tournaments.map(t => (
        <TournamentRow key={t.id} tournament={t}>
          <TableCell>
            <TournamentName>{t.name}</TournamentName>
            <TournamentLocation>
              <Icon name="location" size="xs" /> {t.location}
            </TournamentLocation>
          </TableCell>
          <TableCell><TournamentTypeBadge type={t.type} /></TableCell>
          <TableCell><CategoryBadge category={t.category} /></TableCell>
          <TableCell textAlign="center">{formatDate(t.date)}</TableCell>
          <TableCell textAlign="center">
            {t.registeredTeams} / {t.maxTeams || '∞'}
          </TableCell>
          <TableCell textAlign="center">
            <StatusBadge status={t.status} />
          </TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => viewDetails(t.id)}>
                <Icon name="eye" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => editTournament(t.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => manageRegistrations(t.id)}>
                <Icon name="users" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => enterResults(t.id)}>
                <Icon name="pencil" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => duplicateTournament(t.id)}>
                <Icon name="copy" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => deleteTournament(t.id)}>
                <Icon name="trash" size="sm" />
              </ButtonGhost>
            </ActionButtons>
          </TableCell>
        </TournamentRow>
      ))}
    </TableBody>
  </TournamentsTable>
</TournamentsList>
```

---

### 4. Results Management

#### Match Results Entry

```jsx
<MatchResultsEntry>
  <ResultsHeader>
    <ResultsTitle>Saisie des résultats</ResultsTitle>
    <ResultsSubtitle>
      {tournamentName} - {phaseName} - {groupName}
    </ResultsSubtitle>
    <ResultsActions>
      <ButtonSecondary onClick={autoCalculate}>
        <Icon name="calculator" size="sm" /> Calculer automatiquement
      </ButtonSecondary>
      <ButtonPrimary onClick={saveResults}>
        <Icon name="check" size="sm" /> Enregistrer les résultats
      </ButtonPrimary>
    </ResultsActions>
  </ResultsHeader>
  
  <ResultsSteps>
    <Step active={step === 'selection'}>1. Sélection</Step>
    <Step active={step === 'scores'}>2. Scores</Step>
    <Step active={step === 'validation'}>3. Validation</Step>
    <Step active={step === 'confirmation'}>4. Confirmation</Step>
  </ResultsSteps>
  
  {step === 'selection' && (
    <MatchSelection 
      tournament={tournament} 
      onSelectMatch={selectMatch} 
      selectedMatch={selectedMatch}
    />
  )}
  
  {step === 'scores' && selectedMatch && (
    <ScoreEntry match={selectedMatch} onChange={handleScoreChange}>
      <MatchInfo>
        <MatchTeams>
          <MatchTeam 
            team={selectedMatch.team1} 
            isHome={true}
            editableTeam={allowTeamEdit}
            onTeamChange={handleTeamChange}
          />
          <MatchVs>VS</MatchVs>
          <MatchTeam 
            team={selectedMatch.team2} 
            isHome={false}
            editableTeam={allowTeamEdit}
            onTeamChange={handleTeamChange}
          />
        </MatchTeams>
        <MatchDetails>
          <DetailItem label="Date">{formatDate(selectedMatch.date)}</DetailItem>
          <DetailItem label="Heure">{selectedMatch.time || 'À définir'}</DetailItem>
          <DetailItem label="Terrain">{selectedMatch.location || 'À définir'}</DetailItem>
          <DetailItem label="Type"><MatchTypeBadge type={selectedMatch.type} /></DetailItem>
        </MatchDetails>
      </MatchInfo>
      
      <ScoreInput>
        <TeamScoreInput 
          team="team1" 
          players={selectedMatch.team1.players} 
          onScoreChange={handleTeamScoreChange}
          onPlayerScoreChange={handlePlayerScoreChange}
          mancheCount={selectedMatch.mancheCount}
        />
        <ScoreSeparator />
        <TeamScoreInput 
          team="team2" 
          players={selectedMatch.team2.players} 
          onScoreChange={handleTeamScoreChange}
          onPlayerScoreChange={handlePlayerScoreChange}
          mancheCount={selectedMatch.mancheCount}
        />
      </ScoreInput>
      
      <MancheScoreInput 
        manches={mancheScores} 
        onChange={handleMancheChange}
        onAddManche={addManche}
        onRemoveManche={removeManche}
        totalManches={selectedMatch.mancheCount}
      />
      
      <MatchOptions>
        <FormField label="Match forfait">
          <Checkbox 
            checked={matchOptions.forfait} 
            onChange={() => toggleOption('forfait')}
          />
        </FormField>
        <FormField label="Match abandonné">
          <Checkbox 
            checked={matchOptions.abandoned} 
            onChange={() => toggleOption('abandoned')}
          />
        </FormField>
        <FormField label="Résultat contesté">
          <Checkbox 
            checked={matchOptions.disputed} 
            onChange={() => toggleOption('disputed')}
          />
        </FormField>
        <FormField label="Notes">
          <Input type="textarea" value={notes} onChange={setNotes} />
        </FormField>
      </MatchOptions>
    </ScoreEntry>
  )}
  
  {step === 'validation' && (
    <ResultsValidation 
      match={selectedMatch} 
      scores={currentScores} 
      onConfirm={handleValidationConfirm}
      validationErrors={validationErrors}
    />
  )}
  
  {step === 'confirmation' && (
    <ResultsConfirmation 
      match={selectedMatch} 
      scores={currentScores} 
      onSubmit={handleFinalSubmit}
      onBack={goBackToValidation}
    />
  )}
</MatchResultsEntry>
```

---

### 5. Player Management

#### Player List

```jsx
<PlayersList>
  <ListHeader>
    <ListTitle>Joueurs</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Ajouter un joueur
      </ButtonPrimary>
      <ButtonSecondary onClick={importPlayers}>
        <Icon name="upload" size="sm" /> Importer
      </ButtonSecondary>
      <Dropdown label="Exporter">
        <DropdownItem onClick={exportCSV}>CSV</DropdownItem>
        <DropdownItem onClick={exportExcel}>Excel</DropdownItem>
      </Dropdown>
    </ListActions>
  </ListHeader>
  
  <ListFilters>
    <FilterGroup>
      <FilterLabel>Club</FilterLabel>
      <ClubSelector 
        value={clubFilter} 
        onChange={setClubFilter} 
        clubs={clubs}
      />
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Catégorie</FilterLabel>
      <Select value={categoryFilter} onChange={setCategoryFilter}>
        <Option value="all">Toutes les catégories</Option>
        <Option value="senior">Sénior</Option>
        <Option value="junior">Junior</Option>
        <Option value="veteran">Vétéran</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Statut</FilterLabel>
      <Select value={statusFilter} onChange={setStatusFilter}>
        <Option value="all">Tous les statuts</Option>
        <Option value="active">Actif</Option>
        <Option value="inactive">Inactif</Option>
        <Option value="suspended">Suspendu</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Rechercher</FilterLabel>
      <Input type="search" placeholder="Nom, prénom, licence..." />
    </FilterGroup>
  </ListFilters>
  
  <PlayersTable>
    <TableHeader>
      <TableHeaderCell>Joueur</TableHeaderCell>
      <TableHeaderCell>Licence</TableHeaderCell>
      <TableHeaderCell>Club</TableHeaderCell>
      <TableHeaderCell textAlign="center">Catégorie</TableHeaderCell>
      <TableHeaderCell textAlign="center">Statut</TableHeaderCell>
      <TableHeaderCell textAlign="center">Matchs</TableHeaderCell>
      <TableHeaderCell textAlign="center">Victoires</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {players.map(p => (
        <PlayerRow key={p.id} player={p}>
          <TableCell>
            <PlayerInfo>
              <PlayerAvatar src={p.photo} size="sm" />
              <PlayerNames>
                <PlayerName>{p.firstName} {p.lastName}</PlayerName>
                <PlayerAlias>{p.alias}</PlayerAlias>
              </PlayerNames>
            </PlayerInfo>
          </TableCell>
          <TableCell>
            <LicenseInfo>
              {p.licenseNumber}
              <LicenseStatus status={p.licenseStatus} />
            </LicenseInfo>
          </TableCell>
          <TableCell>
            <ClubInfo>
              <ClubLogo club={p.club} size="xs" />
              <ClubName>{p.club.name}</ClubName>
            </ClubInfo>
          </TableCell>
          <TableCell textAlign="center"><CategoryBadge category={p.category} /></TableCell>
          <TableCell textAlign="center"><StatusBadge status={p.status} /></TableCell>
          <TableCell textAlign="center">{p.stats.matches}</TableCell>
          <TableCell textAlign="center">{p.stats.wins}</TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => viewProfile(p.id)}>
                <Icon name="eye" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => editPlayer(p.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => suspendPlayer(p.id)}>
                <Icon name="pause" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => deletePlayer(p.id)}>
                <Icon name="trash" size="sm" />
              </ButtonGhost>
            </ActionButtons>
          </TableCell>
        </PlayerRow>
      ))}
    </TableBody>
  </PlayersTable>
  
  <BulkActions>
    <BulkSelectActions>
      <ButtonSecondary onClick={bulkSuspend}>
        Suspendre les sélectionnés
      </ButtonSecondary>
      <ButtonSecondary onClick={bulkActivate}>
        Activer les sélectionnés
      </ButtonSecondary>
      <ButtonSecondary onClick={bulkExport}>
        Exporter les sélectionnés
      </ButtonSecondary>
    </BulkSelectActions>
  </BulkActions>
  
  <Pagination 
    current={currentPage} 
    total={totalPages} 
    onChange={setCurrentPage}
  />
</PlayersList>
```

---

### 6. Club Management

#### Club List

```jsx
<ClubsList>
  <ListHeader>
    <ListTitle>Clubs</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Ajouter un club
      </ButtonPrimary>
      <ButtonSecondary onClick={importClubs}>
        <Icon name="upload" size="sm" /> Importer
      </ButtonSecondary>
    </ListActions>
  </ListHeader>
  
  <ClubsTable>
    <TableHeader>
      <TableHeaderCell>Club</TableHeaderCell>
      <TableHeaderCell>Code</TableHeaderCell>
      <TableHeaderCell>Ligue</TableHeaderCell>
      <TableHeaderCell>Ville</TableHeaderCell>
      <TableHeaderCell textAlign="center">Joueurs</TableHeaderCell>
      <TableHeaderCell textAlign="center">Équipes</TableHeaderCell>
      <TableHeaderCell textAlign="center">Statut</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {clubs.map(c => (
        <ClubRow key={c.id} club={c}>
          <TableCell>
            <ClubInfo>
              <ClubLogo src={c.logo} size="sm" />
              <ClubNames>
                <ClubName>{c.name}</ClubName>
                <ClubNickname>{c.nickname}</ClubNickname>
              </ClubNames>
            </ClubInfo>
          </TableCell>
          <TableCell>{c.code}</TableCell>
          <TableCell>{c.league}</TableCell>
          <TableCell>{c.city}</TableCell>
          <TableCell textAlign="center">{c.playersCount}</TableCell>
          <TableCell textAlign="center">{c.teamsCount}</TableCell>
          <TableCell textAlign="center"><StatusBadge status={c.status} /></TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => viewClub(c.id)}>
                <Icon name="eye" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => editClub(c.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => manageClubMembers(c.id)}>
                <Icon name="users" size="sm" />
              </ButtonGhost>
            </ActionButtons>
          </TableCell>
        </ClubRow>
      ))}
    </TableBody>
  </ClubsTable>
</ClubsList>
```

---

### 7. User & Permissions Management

#### User List

```jsx
<UsersList>
  <ListHeader>
    <ListTitle>Utilisateurs</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Ajouter un utilisateur
      </ButtonPrimary>
    </ListActions>
  </ListHeader>
  
  <ListFilters>
    <FilterGroup>
      <FilterLabel>Rôle</FilterLabel>
      <Select value={roleFilter} onChange={setRoleFilter}>
        <Option value="all">Tous les rôles</Option>
        <Option value="super_admin">Super Admin</Option>
        <Option value="commission_admin">Admin Commission</Option>
        <Option value="club_admin">Admin Club</Option>
        <Option value="arbitre">Arbitre</Option>
        <Option value="joueur">Joueur</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Statut</FilterLabel>
      <Select value={statusFilter} onChange={setStatusFilter}>
        <Option value="all">Tous les statuts</Option>
        <Option value="active">Actif</Option>
        <Option value="inactive">Inactif</Option>
      </Select>
    </FilterGroup>
    <FilterGroup>
      <FilterLabel>Rechercher</FilterLabel>
      <Input type="search" placeholder="Nom, email..." />
    </FilterGroup>
  </ListFilters>
  
  <UsersTable>
    <TableHeader>
      <TableHeaderCell>Utilisateur</TableHeaderCell>
      <TableHeaderCell>Email</TableHeaderCell>
      <TableHeaderCell>Rôle</TableHeaderCell>
      <TableHeaderCell>Club</TableHeaderCell>
      <TableHeaderCell textAlign="center">Dernière connexion</TableHeaderCell>
      <TableHeaderCell textAlign="center">Statut</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {users.map(u => (
        <UserRow key={u.id} user={u}>
          <TableCell>
            <UserInfo>
              <UserAvatar src={u.avatar} size="sm" />
              <UserNames>
                <UserName>{u.firstName} {u.lastName}</UserName>
                <UserUsername>{u.username}</UserUsername>
              </UserNames>
            </UserInfo>
          </TableCell>
          <TableCell>{u.email}</TableCell>
          <TableCell><RoleBadge role={u.role} /></TableCell>
          <TableCell>{u.club?.name || '-'}</TableCell>
          <TableCell textAlign="center">{formatDate(u.lastLogin)}</TableCell>
          <TableCell textAlign="center"><StatusBadge status={u.status} /></TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => viewUser(u.id)}>
                <Icon name="eye" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => editUser(u.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => resetPassword(u.id)}>
                <Icon name="key" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => deactivateUser(u.id)}>
                <Icon name="ban" size="sm" />
              </ButtonGhost>
            </ActionButtons>
          </TableCell>
        </UserRow>
      ))}
    </TableBody>
  </UsersTable>
</UsersList>
```

---

### 8. Role & Permissions Management

```jsx
<RolesList>
  <ListHeader>
    <ListTitle>Rôles et Permissions</ListTitle>
    <ListActions>
      <ButtonPrimary onClick={handleCreate}>
        <Icon name="plus" size="sm" /> Nouveau rôle
      </ButtonPrimary>
    </ListActions>
  </ListHeader>
  
  <RolesTable>
    <TableHeader>
      <TableHeaderCell>Rôle</TableHeaderCell>
      <TableHeaderCell>Description</TableHeaderCell>
      <TableHeaderCell textAlign="center">Utilisateurs</TableHeaderCell>
      <TableHeaderCell>Permissions</TableHeaderCell>
      <TableHeaderCell>Actions</TableHeaderCell>
    </TableHeader>
    <TableBody>
      {roles.map(role => (
        <RoleRow key={role.id} role={role}>
          <TableCell>
            <RoleName>{role.name}</RoleName>
            <RoleLevel>{role.level}</RoleLevel>
          </TableCell>
          <TableCell>{role.description}</TableCell>
          <TableCell textAlign="center">{role.usersCount}</TableCell>
          <TableCell>
            <PermissionTags>
              {role.permissions.map(p => (
                <PermissionTag key={p.id}>{p.name}</PermissionTag>
              ))}
            </PermissionTags>
          </TableCell>
          <TableCell>
            <ActionButtons>
              <ButtonGhost size="sm" onClick={() => editRole(role.id)}>
                <Icon name="edit" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => manageUsers(role.id)}>
                <Icon name="users" size="sm" />
              </ButtonGhost>
              <ButtonGhost size="sm" onClick={() => deleteRole(role.id)} disabled={role.id === 'super_admin'}>
                <Icon name="trash" size="sm" />
              </ButtonGhost>
            </ActionButtons>
          </TableCell>
        </RoleRow>
      ))}
    </TableBody>
  </RolesTable>
</RolesList>
```

---

## Data Requirements

### Core Entities

```typescript
// Admin User Types
interface AdminUser {
  id: string;
  firstName: string;
  lastName: string;
  email: string;
  username: string;
  role: UserRole;
  status: 'active' | 'inactive' | 'suspended';
  permissions: Permission[];
  clubId: string | null;
  lastLogin: Date;
  createdAt: Date;
  updatedAt: Date;
}

// User Roles
interface UserRole {
  id: string;
  name: string;
  description: string;
  level: number; // 1=super_admin, 2=commission_admin, 3=club_admin, 4=arbitre, 5=joueur
  permissions: Permission[];
  usersCount: number;
}

// Permissions
interface Permission {
  id: string;
  name: string;
  code: string;
  category: 'competition' | 'player' | 'club' | 'system' | 'report';
  description: string;
}
```

---

## API Endpoints

### Admin Endpoints

| Category | Endpoint | Method | Description |
|----------|----------|--------|-------------|
| Dashboard | `/api/admin/dashboard` | GET | Get dashboard metrics |
| Dashboard | `/api/admin/activity` | GET | Get recent activity |
| Dashboard | `/api/admin/alerts` | GET | Get system alerts |

### Competition Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/championships` | GET | List championships |
| `/api/admin/championships` | POST | Create championship |
| `/api/admin/championships/{id}` | GET | Get championship |
| `/api/admin/championships/{id}` | PATCH | Update championship |
| `/api/admin/championships/{id}` | DELETE | Delete championship |
| `/api/admin/championships/{id}/publish` | POST | Publish championship |
| `/api/admin/championships/{id}/complete` | POST | Complete championship |
| `/api/admin/championships/{id}/classification` | PUT | Update classification |

### Tournament Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/tournaments` | GET | List tournaments |
| `/api/admin/tournaments` | POST | Create tournament |
| `/api/admin/tournaments/{id}` | GET | Get tournament |
| `/api/admin/tournaments/{id}` | PATCH | Update tournament |
| `/api/admin/tournaments/{id}` | DELETE | Delete tournament |
| `/api/admin/tournaments/{id}/registrations` | GET | List registrations |
| `/api/admin/tournaments/{id}/registrations/{regId}` | PATCH | Update registration |

### Results Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/results` | GET | List results |
| `/api/admin/results` | POST | Create result |
| `/api/admin/results/{id}` | GET | Get result |
| `/api/admin/results/{id}` | PATCH | Update result |
| `/api/admin/results/{id}` | DELETE | Delete result |
| `/api/admin/results/bulk` | POST | Bulk create/update results |
| `/api/admin/results/validate` | POST | Validate result |
| `/api/admin/results/{id}/dispute` | POST | Create dispute |

### Player Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/players` | GET | List players |
| `/api/admin/players` | POST | Create player |
| `/api/admin/players/{id}` | GET | Get player |
| `/api/admin/players/{id}` | PATCH | Update player |
| `/api/admin/players/{id}` | DELETE | Delete player |
| `/api/admin/players/import` | POST | Import players |
| `/api/admin/players/export` | GET | Export players |

### Team Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/teams` | GET | List teams |
| `/api/admin/teams` | POST | Create team |
| `/api/admin/teams/{id}` | GET | Get team |
| `/api/admin/teams/{id}` | PATCH | Update team |
| `/api/admin/teams/{id}` | DELETE | Delete team |

### Club Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/clubs` | GET | List clubs |
| `/api/admin/clubs` | POST | Create club |
| `/api/admin/clubs/{id}` | GET | Get club |
| `/api/admin/clubs/{id}` | PATCH | Update club |
| `/api/admin/clubs/{id}` | DELETE | Delete club |

### User Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/users` | GET | List users |
| `/api/admin/users` | POST | Create user |
| `/api/admin/users/{id}` | GET | Get user |
| `/api/admin/users/{id}` | PATCH | Update user |
| `/api/admin/users/{id}` | DELETE | Delete user |
| `/api/admin/users/{id}/reset-password` | POST | Reset password |
| `/api/admin/users/{id}/activate` | POST | Activate user |
| `/api/admin/users/{id}/deactivate` | POST | Deactivate user |

### Role & Permission Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/roles` | GET | List roles |
| `/api/admin/roles` | POST | Create role |
| `/api/admin/roles/{id}` | GET | Get role |
| `/api/admin/roles/{id}` | PATCH | Update role |
| `/api/admin/roles/{id}` | DELETE | Delete role |
| `/api/admin/roles/{id}/users` | GET | Get role users |
| `/api/admin/permissions` | GET | List permissions |

---

## File Structure

```
web/src/pages/admin/
├── index.jsx                          # Admin dashboard
├── AdminProvider.jsx                  # Admin context
├── layouts/
│   ├── AdminLayout.jsx               # Layout with sidebar
│   ├── AdminHeader.jsx               # Header with user info
│   └── AdminSidebar.jsx              # Navigation sidebar
├── dashboard/
│   ├── index.jsx                     # Dashboard overview
│   ├── DashboardMetrics.jsx
│   ├── UpcomingCompetitions.jsx
│   └── RecentActivity.jsx
├── competitions/
│   ├── ChampionshipsList.jsx
│   ├── ChampionshipForm.jsx
│   ├── TournamentsList.jsx
│   ├── TournamentForm.jsx
│   └── ResultsEntry.jsx
├── players/
│   ├── PlayersList.jsx
│   ├── PlayerForm.jsx
│   └── PlayerImport.jsx
├── clubs/
│   ├── ClubsList.jsx
│   └── ClubForm.jsx
├── users/
│   ├── UsersList.jsx
│   ├── UserForm.jsx
│   └── RolesList.jsx
├── results/
│   ├── ResultsList.jsx
│   ├── ScoreEntry.jsx
│   └── ResultsValidation.jsx
├── reports/
│   ├── ReportsList.jsx
│   ├── ReportBuilder.jsx
│   └── ExportManager.jsx
├── settings/
│   ├── SystemSettings.jsx
│   └── SeasonSettings.jsx
├── components/
│   ├── AdminTable.jsx
│   ├── AdminFilters.jsx
│   ├── AdminActions.jsx
│   ├── StatusBadge.jsx
│   ├── RoleBadge.jsx
│   ├── PermissionTag.jsx
│   ├── FormField.jsx
│   └── ConfirmationDialog.jsx
└── hooks/
    ├── useAdmin.js
    ├── useCompetitions.js
    ├── usePlayers.js
    ├── useClubs.js
    └── useUsers.js
```

---

## Authentication & Authorization

### Route Protection
```jsx
// AdminRoute.jsx
const AdminRoute = ({ children, requiredRole }) => {
  const { user, hasPermission } = useAuth();
  
  if (!user) return <Navigate to="/login" />;
  if (!user.isAdmin) return <Navigate to="/" />;
  if (requiredRole && !hasPermission(requiredRole)) return <Forbidden />;
  
  return children;
};

// Usage
<Route 
  path="/admin/*" 
  element={
    <AdminRoute requiredRole="admin">
      <AdminLayout />
    </AdminRoute>
  }
/>
```

### Permission Checking
```jsx
// usePermission.js
const usePermission = (permissionCode) => {
  const { user } = useAuth();
  
  return useMemo(() => {
    if (!user) return false;
    if (user.role === 'super_admin') return true;
    return user.permissions.some(p => p.code === permissionCode);
  }, [user, permissionCode]);
};

// Usage
const canEditResults = usePermission('results:edit');
```

---

## Error States

### Not Authorized
```jsx
<ForbiddenPage>
  <Icon name="lock" size="xl" />
  <Title>Accès interdit</Title>
  <Text>Vous n'avez pas les droits nécessaires pour accéder à cette page.</Text>
  <ButtonPrimary onClick={goBack}>Retour</ButtonPrimary>
</ForbiddenPage>
```

### Not Found
```jsx
<NotFoundPage>
  <Icon name="search" size="xl" />
  <Title>Page non trouvée</Title>
  <Text>La ressource demandée n'existe pas ou a été supprimée.</Text>
  <ButtonPrimary onClick={goToDashboard}>Tableau de bord</ButtonPrimary>
</NotFoundPage>
```

### Validation Error
```jsx
<ValidationError>
  <ErrorTitle>Erreur de validation</ErrorTitle>
  <ErrorList>{errors.map(e => <ErrorItem>{e.message}</ErrorItem>)}</ErrorList>
  <ButtonPrimary onClick={retry}>Réessayer</ButtonPrimary>
</ValidationError>
```

---

## Performance

### Data Fetching Strategy
```jsx
// Use React Query with smart caching
const { data: championships } = useQuery({
  queryKey: ['admin-championships', filters],
  queryFn: () => fetchAdminChampionships(filters),
  staleTime: 30 * 60 * 1000, // 30 minutes
  cacheTime: 60 * 60 * 1000, // 1 hour
  keepPreviousData: true,
});

// Prefetch related data
const { data: players } = useQuery({
  queryKey: ['admin-players'],
  queryFn: fetchAdminPlayers,
  staleTime: Infinity,
});
```

### Pagination & Infinite Scroll
```jsx
const { 
  data, 
  fetchNextPage, 
  hasNextPage,
  isFetchingNextPage 
} = useInfiniteQuery({
  queryKey: ['admin-players-paginated'],
  queryFn: ({ pageParam }) => fetchPlayersPage(pageParam),
  getNextPageParam: (lastPage) => lastPage.nextPage,
});
```

---

## SEO

```jsx
// No-index for admin pages
<Helmet>
  <meta name="robots" content="noindex, nofollow" />
  <title>Administration - Palet Vendéen</title>
  <meta name="description" content="Interface d'administration du Palet Vendéen" />
</Helmet>
```

---

## Accessibility

### Keyboard Navigation
- All interactive elements accessible via keyboard
- Tab order follows logical sequence
- Focus visible on all interactive elements

### Screen Reader Support
- ARIA labels for all actions and statuses
- Live regions for dynamic updates
- Form validation announcements

### Color Contrast
- WCAG AA compliance
- High contrast mode support

---

## Implementation Checklist

- [ ] Admin layout with sidebar navigation
- [ ] Authentication and authorization system
- [ ] Dashboard overview with metrics
- [ ] Championship management (CRUD)
- [ ] Tournament management (CRUD)
- [ ] Results entry and validation
- [ ] Player management (CRUD)
- [ ] Team management
- [ ] Club management
- [ ] User management
- [ ] Role and permission management
- [ ] Reporting and export functionality
- [ ] System settings
- [ ] Responsive design for admin interface
- [ ] Error handling and validation
- [ ] Performance optimization
- [ ] Accessibility features

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)
- [Player Page](player.md)
- [Club Page](club.md)

---

*Page specification - Admin Dashboard*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Pages**: [Results](results.md) | [News](news.md) | [Login](login.md)