# Registration Page - Palet Vendeen

> Tournament registration flow for players and teams

---

## Overview

The **Registration Page** provides a complete multi-step flow for users to register for tournaments:

1. **Tournament Selection** - Browse and select from open tournaments
2. **Tournament Details** - View tournament info and verify eligibility
3. **Team Formation** - Form team (for Doublette/Triplette)
4. **Registration Form** - Enter contact info and preferences
5. **Review & Confirm** - Verify all information
6. **Payment** - Process payment (if applicable)
7. **Confirmation** - Registration complete

**Business Context**: Registration can be individual (INDIVIDUEL) or team-based (DOUBLETTE, TRIPLETTE). Each tournament has specific requirements (license, club affiliation, category). Some have fees.

---

## Page Structure

### Step 1: Tournament Selection

Displays available tournaments with filters (category, type, dates, search). Shows eligibility status for each.

### Step 2: Tournament Details & Eligibility

Shows tournament info with eligibility checklist. Users must pass all checks to proceed.

### Step 3: Team Formation (for team tournaments)

- Captain is the logged-in user
- Search and select teammates from club or invite external players
- Name the team
- Must have required number of players

### Step 4: Registration Form

Collects:
- Contact information (email, phone, address)
- Tournament-specific preferences
- Custom questions from tournament organizer
- Additional comments

### Step 5: Review & Confirm

Displays all entered information for verification. Requires accepting terms and conditions.

### Step 6: Payment

- Shows payment summary
- Multiple payment methods (card, PayPal, bank transfer, on-site)
- Payment form for selected method

### Step 7: Confirmation

- Registration reference number
- Payment confirmation (if applicable)
- Next steps
- Download options (receipt, summary)

---

## Components

### 1. Tournament Selection

```jsx
<TournamentSelection>
  <TournamentFilters>
    <FilterGroup><FilterLabel>Catégorie</FilterLabel><Select options={[...]} /></FilterGroup>
    <FilterGroup><FilterLabel>Type</FilterLabel><Select options={[...]} /></FilterGroup>
    <FilterGroup><FilterLabel>Dates</FilterLabel><DateRangePicker /></FilterGroup>
    <FilterGroup><FilterLabel>Rechercher</FilterLabel><Input type="search" /></FilterGroup>
  </TournamentFilters>
  
  <TournamentGrid>
    {tournaments.map(t => <RegistrationTournamentCard tournament={t} onRegisterClick={...} />)}
  </TournamentGrid>
</TournamentSelection>
```

### 2. Eligibility Check

```jsx
<EligibilityCheck tournament={tournament} user={user}>
  <EligibilityList>
    <EligibilityItem label="Licence FNSMR valide" status={checkLicense(user)} />
    <EligibilityItem label="Club affilié" status={checkClubAffiliation(user)} />
    <EligibilityItem label={`Catégorie: ${category}`} status={checkCategory(user, category)} />
    <EligibilityItem label="Places disponibles" status={tournament.availableSlots > 0} />
    <EligibilityItem label="Non déjà inscrit" status={!isRegistered(user, tournament)} />
  </EligibilityList>
  <EligibilitySummary>
    {eligibleCount}/{totalChecks} critères validés
    {isEligible ? '✓ Vous êtes éligible' : '✗ Non éligible'}
  </EligibilitySummary>
  <ButtonPrimary disabled={!isEligible}>Démarrer l'inscription</ButtonPrimary>
</EligibilityCheck>
```

### 3. Progress Bar

```jsx
<RegistrationProgress currentStep={currentStep} steps={steps} />

const steps = [
  { id: 'team', label: 'Équipe', icon: 'users' },
  { id: 'info', label: 'Infos', icon: 'file-text' },
  { id: 'confirmation', label: 'Confirmation', icon: 'check-circle' },
  { id: 'payment', label: 'Paiement', icon: 'credit-card' }
];
```

### 4. Team Formation

```jsx
<TeamFormation tournament={tournament} user={user} onTeamComplete={...}>
  <CaptainSection>
    <CaptainCard>
      <PlayerInfo player={user} />
      <CaptainBadge />
      <VerificationStatus status={user.verified ? 'verified' : 'pending'} />
    </CaptainCard>
  </CaptainSection>
  
  <TeammatesSection>
    <TeammateSearch>
      <SearchInput placeholder="Rechercher un joueur..." />
      <SearchResults>{results.map(p => <TeammateOption player={p} onSelect={...} />)}</SearchResults>
    </TeammateSearch>
    <ClubTeammates>{clubPlayers.map(p => <TeammateOption player={p} onSelect={...} />)}</ClubTeammates>
    <ExternalTeammateButton onClick={...}>Inviter un joueur externe</ExternalTeammateButton>
    
    <SelectedTeam>
      {selectedTeammates.map((p, i) => <SelectedTeammate player={p} isCaptain={p.id === user.id} />)}
      {selectedTeammates.length < requiredPlayers && <IncompleteMessage>Il manque X joueur(s)</IncompleteMessage>}
    </SelectedTeam>
  </TeammatesSection>
  
  <TeamNameSection>
    <Input value={teamName} onChange={setTeamName} placeholder="Nom de l'équipe" />
    <Checkbox checked={customName} onChange={setCustomName}>Utiliser un nom personnalisé</Checkbox>
  </TeamNameSection>
</TeamFormation>
```

### 5. Registration Form

```jsx
<RegistrationForm tournament={tournament} team={team} onSubmit={...}>
  <FormSection title="Coordonnées">
    <Input label="Email" type="email" required />
    <Input label="Téléphone" type="tel" />
    <Input label="Adresse" />
  </FormSection>
  
  <FormSection title="Préférences">
    <Select label="Catégorie" options={tournament.categories} />
    <Input label="Distance" value={`${tournament.distance}m`} disabled />
  </FormSection>
  
  <FormSection title="Questions spécifiques">
    {tournament.registrationQuestions.map(q => (
      <FormField key={q.id} field={q} />
    ))}
  </FormSection>
  
  <FormSection title="Commentaires">
    <Input type="textarea" label="Informations supplémentaires" />
  </FormSection>
</RegistrationForm>
```

### 6. Review & Confirm

```jsx
<RegistrationReview tournament={tournament} team={team} formData={formData}>
  <ReviewSection title="Tournoi">
    <ReviewTournamentInfo tournament={tournament} />
  </ReviewSection>
  <ReviewSection title="Équipe">
    <ReviewTeamInfo team={team} />
  </ReviewSection>
  <ReviewSection title="Informations de contact">
    <ReviewContactInfo contact={formData.contact} />
  </ReviewSection>
  
  <TermsAndConditions>
    <Checkbox id="terms">J'accepte les conditions générales</Checkbox>
    <Checkbox id="privacy">J'accepte la politique de confidentialité</Checkbox>
    <Checkbox id="eligibility">Je confirme mon éligibilité</Checkbox>
  </TermsAndConditions>
  
  <TotalDisplay>TOTAL: {formatCurrency(tournament.fee)}</TotalDisplay>
  
  <ButtonPrimary disabled={!allTermsAccepted}>Confirmer l'inscription</ButtonPrimary>
</RegistrationReview>
```

### 7. Payment

```jsx
<PaymentPage tournament={tournament} registration={registration}>
  <PaymentSummary>
    <SummaryItem label="Tournoi">{tournament.name}</SummaryItem>
    <SummaryItem label="Équipe">{team.name}</SummaryItem>
    <SummaryItem label="Participants">{team.players.length}</SummaryItem>
    <SummaryItem label="TOTAL" variant="total">{formatCurrency(tournament.fee)}</SummaryItem>
  </PaymentSummary>
  
  <PaymentMethodSelector>
    <PaymentMethod id="card" label="Carte bancaire" icon="credit-card" />
    <PaymentMethod id="paypal" label="PayPal" icon="paypal" />
    <PaymentMethod id="transfer" label="Virement bancaire" icon="bank" />
    {tournament.allowLaterPayment && <PaymentMethod id="later" label="Paiement sur place" icon="calendar" />}
  </PaymentMethodSelector>
  
  {selectedMethod === 'card' && <CardPaymentForm onSubmit={...} />}
  {selectedMethod === 'paypal' && <PayPalPayment onSuccess={...} />}
  
  <ButtonPrimary onClick={handlePayment}>
    {selectedMethod === 'later' ? 'Confirmer sans paiement' : `Payer ${formatCurrency(tournament.fee)}`}
  </ButtonPrimary>
</PaymentPage>
```

### 8. Confirmation

```jsx
<RegistrationConfirmation registration={registration} tournament={tournament} payment={payment}>
  <ConfirmationHeader>
    <CheckCircleIcon />
    <ConfirmationTitle>Inscription confirmée !</ConfirmationTitle>
    <ConfirmationSubtitle>Votre numéro d'inscription: {registration.reference}</ConfirmationSubtitle>
  </ConfirmationHeader>
  
  <ConfirmationDetails>
    <DetailsCard title="Votre inscription">
      <DetailRow label="Tournoi">{tournament.name}</DetailRow>
      <DetailRow label="Équipe">{registration.team.name}</DetailRow>
      <DetailRow label="Date">{formatDate(registration.createdAt)}</DetailRow>
      <DetailRow label="Statut"><StatusBadge status={registration.status} /></DetailRow>
      <ButtonSecondary onClick={handleDownloadSummary}>Télécharger le récapitulatif</ButtonSecondary>
    </DetailsCard>
    
    {payment && (
      <DetailsCard title="Paiement">
        <DetailRow label="Méthode">{payment.method}</DetailRow>
        <DetailRow label="Montant">{formatCurrency(payment.amount)}</DetailRow>
        <DetailRow label="Statut"><StatusBadge status={payment.status} /></DetailRow>
        <ButtonSecondary onClick={handleDownloadReceipt}>Télécharger le reçu</ButtonSecondary>
      </DetailsCard>
    )}
    
    <DetailsCard title="Prochaines étapes">
      <NextStepsList>
        <NextStep><CheckIcon /> Vous recevrez un email de confirmation</NextStep>
        <NextStep><CheckIcon /> Votre équipe est inscrite</NextStep>
        <NextStep><CheckIcon /> Consultez votre tableau après le {formatDate(tournament.registrationDeadline)}</NextStep>
      </NextStepsList>
    </DetailsCard>
  </ConfirmationDetails>
  
  <ConfirmationActions>
    <ButtonSecondary onClick={handleShare}>Partager par email</ButtonSecondary>
    <ButtonSecondary onClick={handleViewTournament}>Voir le tournoi</ButtonSecondary>
    <ButtonPrimary onClick={handleReturnHome}>Retour à l'accueil</ButtonPrimary>
  </ConfirmationActions>
</RegistrationConfirmation>
```

---

## Data Requirements

### API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/api/tournaments` | GET | List open tournaments | Tournament[] |
| `/api/tournaments/{id}` | GET | Get tournament details | Tournament |
| `/api/tournaments/{id}/eligibility` | GET | Check user eligibility | Eligibility |
| `/api/registrations` | POST | Create registration | Registration |
| `/api/registrations/{id}` | GET | Get registration | Registration |
| `/api/registrations/{id}/payment` | POST | Process payment | Payment |
| `/api/clubs/{id}/players` | GET | Get club players | Player[] |
| `/api/players/search` | GET | Search players | Player[] |

### Registration Object

```typescript
interface Registration {
  id: string;
  reference: string;
  user: User;
  tournament: Tournament;
  tournamentId: string;
  team: RegistrationTeam;
  isIndividual: boolean;
  status: 'pending' | 'confirmed' | 'cancelled' | 'waitlisted';
  payment: RegistrationPayment | null;
  paymentStatus: 'pending' | 'paid' | 'waived' | 'failed';
  fee: number;
  email: string;
  phone: string | null;
  address: string | null;
  answers: RegistrationAnswer[];
  createdAt: Date;
  updatedAt: Date;
  confirmedAt: Date | null;
}

interface RegistrationTeam {
  id: string;
  name: string;
  captain: Player;
  players: Player[];
}
```

---

## Page States

### Eligibility States
- ✓ All checks passed - Can proceed
- ✗ Some checks failed - Show reasons, cannot proceed
- ⚠ Pending verification - Show loading state

### Payment States
- **Free tournament**: Skip payment step
- **Paid tournament**: Require payment
- **Payment processing**: Show loading state
- **Payment success**: Proceed to confirmation
- **Payment failed**: Show error, allow retry or change method

### Registration States
- **Draft**: Incomplete, saved but not submitted
- **Pending**: Submitted, waiting for payment or admin approval
- **Confirmed**: Payment received or waived
- **Cancelled**: User cancelled
- **Waitlisted**: Tournament full, on waitlist

---

## Error Handling

### Form Validation
```jsx
<Input error={errors.email} helperText={errors.email?.message} />
<ErrorSummary errors={errors} />
```

### Eligibility Errors
```jsx
<EligibilityError reasons={ineligibilityReasons}>
  <ErrorTitle>Non éligible pour ce tournoi</ErrorTitle>
  <ErrorList>{reasons.map(r => <ErrorItem>{r}</ErrorItem>)}</ErrorList>
  <ButtonPrimary onClick={contactOrganizer}>Contacter l'organisateur</ButtonPrimary>
</EligibilityError>
```

### Payment Errors
```jsx
<PaymentError error={paymentError}>
  <ErrorTitle>Erreur de paiement</ErrorTitle>
  <ErrorMessage>{paymentError.message}</ErrorMessage>
  {paymentError.code === 'card_declined' && <Info>Carte refusée par la banque</Info>}
  <ButtonPrimary onClick={retryPayment}>Réessayer</ButtonPrimary>
  <ButtonSecondary onClick={changePaymentMethod}>Changer de méthode</ButtonSecondary>
</PaymentError>
```

---

## Responsive Design

### Mobile (< 640px)
- Single column layout
- Progress bar scrollable or simplified
- Form fields full width
- Stacked actions

### Tablet (640px - 1023px)
- 2-column layout for some sections
- Progress bar visible without scrolling
- Form fields side-by-side where appropriate

### Desktop (1024px+)
- Full layouts as shown in diagrams

---

## Accessibility

### Semantic HTML
```jsx
<main>
  <header><h1>{currentStepTitle}</h1></header>
  <nav aria-label="Progression">...</nav>
  <form aria-labelledby="form-title">...</form>
</main>
```

### Form Accessibility
```jsx
<label htmlFor="email">Adresse email</label>
<input id="email" aria-required="true" aria-invalid={!!errors.email} />
<div id="email-error" role="alert">{errors.email?.message}</div>
```

### Screen Reader Announcements
```jsx
// Announce step changes
useEffect(() => {
  const stepName = steps[currentStep]?.label;
  announce(`Étape ${currentStep + 1} sur ${steps.length}: ${stepName}`);
}, [currentStep]);
```

---

## Performance

### Data Fetching
```jsx
// Prefetch tournaments
const { data: tournaments } = useQuery({
  queryKey: ['open-tournaments'],
  queryFn: fetchOpenTournaments,
  staleTime: 5 * 60 * 1000,
});

// Fetch eligibility on selection
const { data: eligibility } = useQuery({
  queryKey: ['eligibility', userId, tournamentId],
  queryFn: () => fetchEligibility(userId, tournamentId),
  enabled: !!tournamentId && !!userId,
});
```

### Form Optimization
```jsx
const { register, handleSubmit, formState: { errors } } = useForm({
  mode: 'onChange',
  reValidateMode: 'onChange',
  defaultValues: formDefaultValues,
});
```

---

## SEO

### Step 1
```jsx
<Helmet>
  <title>Inscription aux tournois - Palet Vendéen</title>
  <meta name="description" content="Inscription aux tournois de palet vendéen" />
  <link rel="canonical" href="https://palet-vendeen.fr/inscription" />
</Helmet>
```

### Steps 2-7
```jsx
<Helmet>
  <meta name="robots" content="noindex" />
</Helmet>
```

---

## File Structure

```
web/src/pages/registration/
├── index.jsx              # Flow wrapper
├── RegistrationProvider.jsx
├── steps/
│   ├── TournamentSelection.jsx
│   ├── TournamentDetails.jsx
│   ├── TeamFormation.jsx
│   ├── RegistrationForm.jsx
│   ├── Review.jsx
│   ├── Payment.jsx
│   └── Confirmation.jsx
├── components/
│   ├── ProgressBar.jsx
│   ├── TournamentCard.jsx
│   ├── EligibilityCheck.jsx
│   ├── TeamFormationForm.jsx
│   ├── PaymentForm.jsx
│   └── ConfirmationCard.jsx
└── hooks/
    ├── useRegistration.js
    └── useEligibility.js
```

---

## Next Steps

1. Create **Club page**
2. Create **Player page**
3. Create **Admin pages**
4. Create **News page**
5. Create **Login page**

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Tournament Page](tournament.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)
