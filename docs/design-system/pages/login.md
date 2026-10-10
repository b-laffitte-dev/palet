# Login & Authentication - Palet Vendeen

> User authentication system including login, registration, password recovery, and profile management

---

## Overview

The **Authentication System** provides complete user management for the Palet Vendéen platform:

- **Login**: User authentication and session creation
- **Registration**: New user account creation
- **Password Recovery**: Forgotten password reset flow
- **Email Verification**: Account activation and email confirmation
- **Social Login**: Integration with third-party providers
- **Profile Management**: User account settings and preferences
- **Role-Based Access**: Different experiences for different user types

**Business Context**: The platform supports multiple user types:
- **Guest**: Limited access, can view public content
- **Player**: Full player features (profile, registration, results)
- **Club Admin**: Club management features
- **Commission Admin**: Regional competition management
- **Super Admin**: Full system access

---

## Authentication Flow

```
┌─────────────────────────┐     ┌─────────────────────────┐     ┌─────────────────────────┐
│   NOT LOGGED IN          │     │   LOGGING IN             │     │   LOGGED IN              │
│                         │     │                         │     │                         │
│  ┌─────────────────┐   │─────▶│  ┌─────────────────┐   │─────▶│  ┌─────────────────┐   │
│  │ Homepage         │   │     │  │ Login Form         │   │     │  │ Homepage         │   │
│  │ (Public content)  │   │     │  │ Email + Password   │   │     │  │ (Full access)    │   │
│  └─────────────────┘   │     │  │ Social Buttons     │   │     │  │ Dashboard        │   │
│                         │     │  │ Forgot Password    │   │     │  │ Profile          │   │
│  ┌─────────────────┐   │     │  └─────────────────┘   │     │  │ Settings         │   │
│  │ Login Button     │   │     │                         │     │  │ Admin Features   │   │
│  └─────────────────┘   │     │  ┌─────────────────┐   │     │  └─────────────────┘   │
│                         │     │  │ Loading State     │   │     │                         │
│  ┌─────────────────┐   │     │  │ Validating...      │   │     └─────────────────────────┘
│  │ Register Option   │   │     │  └─────────────────┘   │
│  └─────────────────┘   │     └─────────────────────────┘
│                         │
│  ┌─────────────────┐   │
│  │ Forgot Password   │───┐
│  └─────────────────┘   │
└─────────────────────────┘     │
                                  │
                                  ▼
                         ┌─────────────────────────┐
                         │   VERIFICATION           │
                         │                         │
                         │  ┌─────────────────┐   │
                         │  │ Success           │   │
                         │  │ Welcome back!     │   │
                         │  └─────────────────┘   │
                         │                         │
                         │  ┌─────────────────┐   │
                         │  │ 2FA Required      │   │
                         │  │ Code Input        │   │
                         │  └─────────────────┘   │
                         │                         │
                         │  ┌─────────────────┐   │
                         │  │ Error            │   │
                         │  │ Invalid creds    │   │
                         │  │ Try again        │   │
                         │  └─────────────────┘   │
                         └─────────────────────────┘
```

---

## Page Structure

### Login Page

```
┌─────────────────────────────────────────────────────────────┐
│ TOP BAR (40px) - Minimal                                    │
│  [Logo] Palet Vendéen                                     │
├─────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                                                                 ││
│  │    ┌─────────────────┐      ┌─────────────────┐          ││
│  │    │                 │      │                 │          ││
│  │    │   [LOGO]        │      │   Bienvenue !    │          ││
│  │    │                 │      │                 │          ││
│  │    │   Palet         │      │ Connectez-vous   │          ││
│  │    │   Vendéen       │      │ à votre compte   │          ││
│  │    │                 │      │                 │          ││
│  │    └─────────────────┘      └─────────────────┘          ││
│  │                                                                 ││
│  │    ┌─────────────────────────────────────────────────┐   ││
│  │    │                                                             │   ││
│  │    │  [Email]                                                 │   ││
│  │    │  ┌─────────────────────────────────────────────┐│   ││
│  │    │  │                                             ││   ││
│  │    │  └─────────────────────────────────────────────┘│   ││
│  │    │                                                             │   ││
│  │    │  [Password]                                              │   ││
│  │    │  ┌─────────────────────────────────────────────┐│   ││
│  │    │  │ ● ● ● ● ● ● ● ●                             ││   ││
│  │    │  └─────────────────────────────────────────────┘│   ││
│  │    │                                                             │   ││
│  │    │  [Show Password]     [Forgot Password?]              │   ││
│  │    │                                                             │   ││
│  │    │  [Se connecter]                                       │   ││
│  │    │  ┌─────────────────────────────────────────────┐│   ││
│  │    │  │                   SE CONNECTER                    ││   ││
│  │    │  └─────────────────────────────────────────────┘│   ││
│  │    │                                                             │   ││
│  │    │                         OU                               │   ││
│  │    │                                                             │   ││
│  │    │  ┌─────────┐  ┌─────────┐  ┌─────────┐                 │   ││
│  │    │  │ [Google]│  │ [FB]    │  │[Twitter] │                 │   ││
│  │    │  └─────────┘  └─────────┘  └─────────┘                 │   ││
│  │    │                                                             │   ││
│  │    │  Pas de compte? [S'inscrire]                              │   ││
│  │    │                                                             │   ││
│  │    └─────────────────────────────────────────────────┘   ││
│  │                                                                 ││
│  └─────────────────────────────────────────────────────────┘│
│                                                                 │
├─────────────────────────────────────────────────────────────┤
│ FOOTER (40px)                                                  │
│  © 2026 Palet Vendéen - [Liens] [Politique] [Conditions]       │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Login Form

```jsx
<LoginForm onSubmit={handleLogin} loading={loading}>
  <FormHeader>
    <Logo src="/logo.svg" alt="Palet Vendéen" size="large" />
    <FormTitle size="heading-l" weight="bold">
      Bienvenue !
    </FormTitle>
    <FormSubtitle size="body-m" color="gray-500">
      Connectez-vous à votre compte
    </FormSubtitle>
  </FormHeader>
  
  <FormFields>
    <FormField label="Adresse email" required>
      <Input 
        type="email" 
        value={email} 
        onChange={setEmail}
        placeholder="votre@email.com"
        autoComplete="email"
        autoFocus={true}
        icon="mail"
      />
      {errors.email && (
        <ErrorMessage size="body-xs">{errors.email}</ErrorMessage>
      )}
    </FormField>
    
    <FormField label="Mot de passe" required>
      <Input 
        type={showPassword ? "text" : "password"} 
        value={password} 
        onChange={setPassword}
        placeholder="Votre mot de passe"
        autoComplete="current-password"
        icon="lock"
        actionIcon={showPassword ? "eye-off" : "eye"} 
        onActionClick={toggleShowPassword}
      />
      {errors.password && (
        <ErrorMessage size="body-xs">{errors.password}</ErrorMessage>
      )}
    </FormField>
    
    <FormOptions>
      <RememberMe>
        <Checkbox 
          id="remember" 
          checked={rememberMe} 
          onChange={setRememberMe}
          label="Se souvenir de moi"
        />
      </RememberMe>
      
      <ForgotPassword>
        <ButtonLink to="/mot-de-passe-oublie">
          Mot de passe oublié ?
        </ButtonLink>
      </ForgotPassword>
    </FormOptions>
  </FormFields>
  
  <FormActions>
    <ButtonPrimary 
      type="submit" 
      loading={loading} 
      fullWidth
      size="lg"
    >
      Se connecter
    </ButtonPrimary>
  </FormActions>
  
  <FormDivider>
    <DividerText>OU</DividerText>
  </FormDivider>
  
  <SocialLogin>
    <SocialButtons>
      <SocialButton 
        provider="google" 
        onClick={loginWithGoogle}
        loading={googleLoading}
      >
        <Icon name="google" size="sm" /> Continuer avec Google
      </SocialButton>
      <SocialButton 
        provider="facebook" 
        onClick={loginWithFacebook}
        loading={facebookLoading}
      >
        <Icon name="facebook" size="sm" /> Continuer avec Facebook
      </SocialButton>
      <SocialButton 
        provider="twitter" 
        onClick={loginWithTwitter}
        loading={twitterLoading}
      >
        <Icon name="twitter" size="sm" /> Continuer avec Twitter
      </SocialButton>
    </SocialButtons>
  </SocialLogin>
  
  <FormFooter>
    <Text size="body-s" color="gray-500">
      Pas de compte ?
    </Text>
    <Link to="/inscription">S'inscrire</Link>
  </FormFooter>
</LoginForm>
```

---

### 2. Registration Form

```jsx
<RegistrationForm onSubmit={handleRegister} loading={loading}>
  <FormHeader>
    <Logo src="/logo.svg" alt="Palet Vendéen" size="medium" />
    <FormTitle size="heading-l" weight="bold">
      Créer un compte
    </FormTitle>
    <FormSubtitle size="body-m" color="gray-500">
      Rejoignez la communauté du palet vendéen
    </FormSubtitle>
  </FormHeader>
  
  <RegistrationSteps currentStep={currentStep} steps={steps} />
  
  {currentStep === 1 && (
    <PersonalInfoStep>
      <StepTitle size="heading-s">Informations personnelles</StepTitle>
      <FormFields>
        <FormRow>
          <FormField label="Prénom" required>
            <Input 
              value={firstName} 
              onChange={setFirstName}
              placeholder="Jean"
              autoComplete="given-name"
            />
            {errors.firstName && <ErrorMessage>{errors.firstName}</ErrorMessage>}
          </FormField>
          <FormField label="Nom" required>
            <Input 
              value={lastName} 
              onChange={setLastName}
              placeholder="Dupont"
              autoComplete="family-name"
            />
            {errors.lastName && <ErrorMessage>{errors.lastName}</ErrorMessage>}
          </FormField>
        </FormRow>
        
        <FormField label="Adresse email" required>
          <Input 
            type="email" 
            value={email} 
            onChange={setEmail}
            placeholder="votre@email.com"
            autoComplete="email"
          />
          {errors.email && <ErrorMessage>{errors.email}</ErrorMessage>}
        </FormField>
        
        <FormField label="Date de naissance" required>
          <DatePicker 
            value={birthDate} 
            onChange={setBirthDate}
            maxDate={new Date()}
            autoComplete="bday"
          />
          {errors.birthDate && <ErrorMessage>{errors.birthDate}</ErrorMessage>}
        </FormField>
        
        <FormField label="Pseudo" optional>
          <Input 
            value={username} 
            onChange={setUsername}
            placeholder="Jeandupont"
            autoComplete="username"
            helpText="Utilisé pour votre profil public"
          />
          {errors.username && <ErrorMessage>{errors.username}</ErrorMessage>}
        </FormField>
      </FormFields>
    </PersonalInfoStep>
  )}
  
  {currentStep === 2 && (
    <PaletInfoStep>
      <StepTitle size="heading-s">Informations Palet Vendéen</StepTitle>
      <FormFields>
        <FormField label="N° de licence FNSMR" optional>
          <Input 
            value={licenseNumber} 
            onChange={setLicenseNumber}
            placeholder="123456"
            helpText="Si vous avez déjà une licence"
          />
        </FormField>
        
        <FormField label="Club" required>
          <ClubSelector 
            value={clubId} 
            onChange={setClubId}
            clubs={clubs}
            allowNone={true}
          />
          {errors.clubId && <ErrorMessage>{errors.clubId}</ErrorMessage>}
        </FormField>
        
        <FormField label="Catégorie" required>
          <Select 
            value={category} 
            onChange={setCategory}
            options={categoryOptions}
          />
          {errors.category && <ErrorMessage>{errors.category}</ErrorMessage>}
        </FormField>
        
        <FormField label="Main dominante">
          <Select 
            value={dominantHand} 
            onChange={setDominantHand}
            options={handOptions}
          />
        </FormField>
        
        <FormField label="Style de jeu" optional>
          <Select 
            value={playStyle} 
            onChange={setPlayStyle}
            options={styleOptions}
            allowNone={true}
          />
        </FormField>
      </FormFields>
    </PaletInfoStep>
  )}
  
  {currentStep === 3 && (
    <AccountInfoStep>
      <StepTitle size="heading-s">Informations de compte</StepTitle>
      <FormFields>
        <FormField label="Mot de passe" required>
          <Input 
            type={showPassword ? "text" : "password"} 
            value={password} 
            onChange={setPassword}
            placeholder="Créez un mot de passe"
            autoComplete="new-password"
            icon="lock"
            actionIcon={showPassword ? "eye-off" : "eye"} 
            onActionClick={toggleShowPassword}
            helpText="Minimum 8 caractères"
          />
          {errors.password && <ErrorMessage>{errors.password}</ErrorMessage>}
          <PasswordStrength strength={passwordStrength} />
        </FormField>
        
        <FormField label="Confirmer le mot de passe" required>
          <Input 
            type={showConfirmPassword ? "text" : "password"} 
            value={confirmPassword} 
            onChange={setConfirmPassword}
            placeholder="Confirmez votre mot de passe"
            autoComplete="new-password"
            icon="lock"
            actionIcon={showConfirmPassword ? "eye-off" : "eye"} 
            onActionClick={toggleShowConfirmPassword}
          />
          {errors.confirmPassword && (
            <ErrorMessage>{errors.confirmPassword}</ErrorMessage>
          )}
        </FormField>
        
        <FormField>
          <Checkbox 
            checked={acceptTerms} 
            onChange={setAcceptTerms}
            required
            label="J'accepte les conditions générales d'utilisation"
          />
          {errors.acceptTerms && (
            <ErrorMessage>{errors.acceptTerms}</ErrorMessage>
          )}
        </FormField>
        
        <FormField>
          <Checkbox 
            checked={acceptPrivacy} 
            onChange={setAcceptPrivacy}
            required
            label="J'accepte la politique de confidentialité"
          />
          {errors.acceptPrivacy && (
            <ErrorMessage>{errors.acceptPrivacy}</ErrorMessage>
          )}
        </FormField>
        
        <FormField>
          <Checkbox 
            checked={receiveNewsletter} 
            onChange={setReceiveNewsletter}
            label="S'abonner à la newsletter"
          />
        </FormField>
      </FormFields>
    </AccountInfoStep>
  )}
  
  {currentStep === 4 && (
    <ReviewStep>
      <StepTitle size="heading-s">Vérifiez vos informations</StepTitle>
      <ReviewInfo>
        <ReviewSection title="Informations personnelles">
          <ReviewItem label="Prénom">{firstName}</ReviewItem>
          <ReviewItem label="Nom">{lastName}</ReviewItem>
          <ReviewItem label="Email">{email}</ReviewItem>
          <ReviewItem label="Date de naissance">{formatDate(birthDate)}</ReviewItem>
          <ReviewItem label="Pseudo">{username || 'Non spécifié'}</ReviewItem>
        </ReviewSection>
        
        <ReviewSection title="Informations Palet">
          <ReviewItem label="Licence FNSMR">{licenseNumber || 'Non spécifiée'}</ReviewItem>
          <ReviewItem label="Club">{clubName}</ReviewItem>
          <ReviewItem label="Catégorie">{categoryLabel}</ReviewItem>
          <ReviewItem label="Main dominante">{dominantHandLabel}</ReviewItem>
          <ReviewItem label="Style de jeu">{playStyleLabel || 'Non spécifié'}</ReviewItem>
        </ReviewSection>
      </ReviewInfo>
      
      <ReviewDisclaimer>
        <Text size="body-s" color="gray-600">
          Vous recevrez un email de confirmation pour activer votre compte.
        </Text>
      </ReviewDisclaimer>
    </ReviewStep>
  )}
  
  <FormActions>
    {currentStep > 1 && (
      <ButtonSecondary onClick={prevStep}>
        Précédent
      </ButtonSecondary>
    )}
    
    {currentStep < 4 ? (
      <ButtonPrimary onClick={nextStep} disabled={!isStepValid()}>
        Continuer
      </ButtonPrimary>
    ) : (
      <ButtonPrimary 
        type="submit" 
        loading={loading}
        disabled={!isFormValid()}
      >
        Créer mon compte
      </ButtonPrimary>
    )}
  </FormActions>
</RegistrationForm>
```

---

### 3. Forgot Password Form

```jsx
<ForgotPasswordForm onSubmit={handleSubmit} loading={loading}>
  <FormHeader>
    <Logo src="/logo.svg" alt="Palet Vendéen" size="medium" />
    <FormTitle size="heading-l" weight="bold">
      Mot de passe oublié ?
    </FormTitle>
    <FormSubtitle size="body-m" color="gray-500">
      Nous allons vous envoyer un lien de réinitialisation
    </FormSubtitle>
  </FormHeader>
  
  <FormFields>
    <FormField label="Adresse email" required>
      <Input 
        type="email" 
        value={email} 
        onChange={setEmail}
        placeholder="votre@email.com"
        autoComplete="email"
        autoFocus={true}
        icon="mail"
      />
      {errors.email && <ErrorMessage>{errors.email}</ErrorMessage>}
      {success && (
        <SuccessMessage>
          <Icon name="check-circle" size="sm" />
          <Text>Si un compte existe avec cette adresse, vous recevrez un email.</Text>
        </SuccessMessage>
      )}
    </FormField>
  </FormFields>
  
  <FormActions>
    <ButtonPrimary 
      type="submit" 
      loading={loading} 
      fullWidth
      disabled={!email}
    >
      Envoyer le lien de réinitialisation
    </ButtonPrimary>
  </FormActions>
  
  <FormFooter>
    <Text size="body-s" color="gray-500">
      Vous avez déjà un compte ?
    </Text>
    <Link to="/connexion">Se connecter</Link>
  </FormFooter>
</ForgotPasswordForm>
```

---

### 4. Reset Password Form

```jsx
<ResetPasswordForm 
  onSubmit={handleReset} 
  loading={loading} 
  token={token}
>
  <FormHeader>
    <Logo src="/logo.svg" alt="Palet Vendéen" size="medium" />
    <FormTitle size="heading-l" weight="bold">
      Réinitialiser le mot de passe
    </FormTitle>
    <FormSubtitle size="body-m" color="gray-500">
      Créez un nouveau mot de passe pour votre compte
    </FormSubtitle>
  </FormHeader>
  
  <FormFields>
    <FormField label="Nouveau mot de passe" required>
      <Input 
        type={showPassword ? "text" : "password"} 
        value={password} 
        onChange={setPassword}
        placeholder="Nouveau mot de passe"
        autoComplete="new-password"
        icon="lock"
        actionIcon={showPassword ? "eye-off" : "eye"} 
        onActionClick={toggleShowPassword}
        helpText="Minimum 8 caractères"
      />
      {errors.password && <ErrorMessage>{errors.password}</ErrorMessage>}
      <PasswordStrength strength={passwordStrength} />
    </FormField>
    
    <FormField label="Confirmer le nouveau mot de passe" required>
      <Input 
        type={showConfirmPassword ? "text" : "password"} 
        value={confirmPassword} 
        onChange={setConfirmPassword}
        placeholder="Confirmez le nouveau mot de passe"
        autoComplete="new-password"
        icon="lock"
        actionIcon={showConfirmPassword ? "eye-off" : "eye"} 
        onActionClick={toggleShowConfirmPassword}
      />
      {errors.confirmPassword && (
        <ErrorMessage>{errors.confirmPassword}</ErrorMessage>
      )}
    </FormField>
  </FormFields>
  
  <FormActions>
    <ButtonPrimary 
      type="submit" 
      loading={loading} 
      fullWidth
      disabled={!isFormValid()}
    >
      Réinitialiser le mot de passe
    </ButtonPrimary>
  </FormActions>
  
  {resetSuccess && (
    <SuccessMessage>
      <Icon name="check-circle" size="sm" />
      <Text>Votre mot de passe a été réinitialisé avec succès.</Text>
      <Text>Vous pouvez maintenant vous connecter.</Text>
      <ButtonPrimary onClick={goToLogin} fullWidth>
        Se connecter
      </ButtonPrimary>
    </SuccessMessage>
  )}
</ResetPasswordForm>
```

---

### 5. Email Verification Form

```jsx
<EmailVerificationForm 
  onSubmit={handleVerify} 
  loading={loading} 
  token={token}
>
  <FormHeader>
    <Logo src="/logo.svg" alt="Palet Vendéen" size="medium" />
    {verified ? (
      <>
        <FormTitle size="heading-l" weight="bold">
          Email vérifié !
        </FormTitle>
        <FormSubtitle size="body-m" color="gray-500">
          Votre adresse email a été confirmée avec succès
        </FormSubtitle>
      </>
    ) : (
      <>
        <FormTitle size="heading-l" weight="bold">
          Vérification de l'email
        </FormTitle>
        <FormSubtitle size="body-m" color="gray-500">
          Cliquez sur le bouton pour confirmer votre adresse email
        </FormSubtitle>
      </>
    )}
  </FormHeader>
  
  {verified ? (
    <SuccessContent>
      <Icon name="check-circle" size="xl" color="success" />
      <SuccessTitle size="heading-m">
        Compte activé
      </SuccessTitle>
      <SuccessText size="body-m" color="gray-600">
        Votre compte est maintenant activé. Vous pouvez vous connecter.
      </SuccessText>
      <ButtonPrimary onClick={goToLogin} fullWidth>
        Se connecter
      </ButtonPrimary>
    </SuccessContent>
  ) : (
    <FormFields>
      {expired && (
        <ErrorMessage>
          <Icon name="alert-circle" size="sm" />
          <Text>Le lien de vérification a expiré. Veuillez en demander un nouveau.</Text>
          <ButtonSecondary onClick={resendVerification}>
            Renvoyer le lien de vérification
          </ButtonSecondary>
        </ErrorMessage>
      )}
    </FormFields>
  )}
  
  {!verified && !expired && (
    <FormActions>
      <ButtonPrimary 
        type="submit" 
        loading={loading} 
        fullWidth
      >
        Vérifier l'email
      </ButtonPrimary>
    </FormActions>
  )}
</EmailVerificationForm>
```

---

### 6. Profile Settings Form

```jsx
<ProfileSettingsForm 
  user={user} 
  onSubmit={handleSave} 
  loading={loading}
>
  <FormHeader>
    <FormTitle size="heading-l" weight="bold">
      Mon profil
    </FormTitle>
    <FormSubtitle size="body-m" color="gray-500">
      Gérer vos informations personnelles
    </FormSubtitle>
  </FormHeader>
  
  <FormTabs defaultActive="profile">
    <Tab id="profile">Profil</Tab>
    <Tab id="account">Compte</Tab>
    <Tab id="preferences">Préférences</Tab>
    <Tab id="security">Sécurité</Tab>
    {user.isAdmin && <Tab id="admin">Admin</Tab>}
  </FormTabs>
  
  <TabPanel id="profile">
    <FormSection title="Informations personnelles">
      <FormFields>
        <FormRow>
          <FormField label="Photo de profil">
            <ProfilePicture 
              src={user.photo} 
              onUpload={handlePhotoUpload}
              onRemove={handlePhotoRemove}
              loading={photoLoading}
            />
          </FormField>
        </FormRow>
        
        <FormRow>
          <FormField label="Prénom" required>
            <Input 
              value={firstName} 
              onChange={setFirstName}
              placeholder="Jean"
            />
          </FormField>
          <FormField label="Nom" required>
            <Input 
              value={lastName} 
              onChange={setLastName}
              placeholder="Dupont"
            />
          </FormField>
        </FormRow>
        
        <FormField label="Pseudo">
          <Input 
            value={username} 
            onChange={setUsername}
            placeholder="Jeandupont"
          />
        </FormField>
        
        <FormField label="Email" required>
          <Input 
            type="email" 
            value={email} 
            onChange={setEmail}
            placeholder="votre@email.com"
            disabled={true}
            helpText="L'email ne peut pas être modifié"
          />
        </FormField>
        
        <FormRow>
          <FormField label="Téléphone">
            <Input 
              type="tel" 
              value={phone} 
              onChange={setPhone}
              placeholder="06 12 34 56 78"
            />
          </FormField>
          <FormField label="Date de naissance">
            <DatePicker 
              value={birthDate} 
              onChange={setBirthDate}
              maxDate={new Date()}
            />
          </FormField>
        </FormRow>
      </FormFields>
    </FormSection>
    
    <FormSection title="Informations sportives">
      <FormFields>
        <FormField label="N° de licence FNSMR">
          <Input 
            value={licenseNumber} 
            onChange={setLicenseNumber}
            placeholder="123456"
          />
        </FormField>
        
        <FormField label="Club">
          <ClubSelector 
            value={clubId} 
            onChange={setClubId}
            clubs={clubs}
          />
        </FormField>
        
        <FormRow>
          <FormField label="Catégorie">
            <Select 
              value={category} 
              onChange={setCategory}
              options={categoryOptions}
            />
          </FormField>
          <FormField label="Main dominante">
            <Select 
              value={dominantHand} 
              onChange={setDominantHand}
              options={handOptions}
            />
          </FormField>
        </FormRow>
        
        <FormField label="Style de jeu">
          <Select 
            value={playStyle} 
            onChange={setPlayStyle}
            options={styleOptions}
            allowNone={true}
          />
        </FormField>
      </FormFields>
    </FormSection>
    
    <FormSection title="Biographie">
      <FormField label="À propos de moi">
        <Input 
          type="textarea" 
          value={bio} 
          onChange={setBio}
          placeholder="Décrivez-vous..."
          rows={4}
        />
      </FormField>
    </FormSection>
    
    <FormActions>
      <ButtonPrimary type="submit" loading={loading}>
        Enregistrer les modifications
      </ButtonPrimary>
    </FormActions>
  </TabPanel>
  
  <TabPanel id="account">
    <FormSection title="Paramètres du compte">
      <FormFields>
        <FormField label="Langue">
          <Select 
            value={language} 
            onChange={setLanguage}
            options={languageOptions}
          />
        </FormField>
        
        <FormField label="Fuseau horaire">
          <Select 
            value={timezone} 
            onChange={setTimezone}
            options={timezoneOptions}
          />
        </FormField>
        
        <FormField label="Unité de distance">
          <Select 
            value={distanceUnit} 
            onChange={setDistanceUnit}
            options={[{ value: 'metric', label: 'Métrique' }, { value: 'imperial', label: 'Impérial' }]}
          />
        </FormField>
      </FormFields>
    </FormSection>
  </TabPanel>
  
  <TabPanel id="preferences">
    <FormSection title="Préférences de notification">
      <FormFields>
        <FormField>
          <Checkbox 
            checked={notifications.email.results} 
            onChange={() => toggleNotification('email.results')}
            label="Recevoir les résultats par email"
          />
        </FormField>
        <FormField>
          <Checkbox 
            checked={notifications.email.news} 
            onChange={() => toggleNotification('email.news')}
            label="Recevoir les actualités par email"
          />
        </FormField>
        <FormField>
          <Checkbox 
            checked={notifications.email.reminders} 
            onChange={() => toggleNotification('email.reminders')}
            label="Recevoir les rappels par email"
          />
        </FormField>
        
        <FormField>
          <Checkbox 
            checked={notifications.push.results} 
            onChange={() => toggleNotification('push.results')}
            label="Notifications push pour les résultats"
          />
        </FormField>
        <FormField>
          <Checkbox 
            checked={notifications.push.matches} 
            onChange={() => toggleNotification('push.matches')}
            label="Notifications push pour les matchs"
          />
        </FormField>
      </FormFields>
    </FormSection>
  </TabPanel>
  
  <TabPanel id="security">
    <FormSection title="Sécurité">
      <FormFields>
        <FormField label="Mot de passe actuel">
          <Input 
            type={showCurrentPassword ? "text" : "password"} 
            value={currentPassword} 
            onChange={setCurrentPassword}
            placeholder="Mot de passe actuel"
            icon="lock"
            actionIcon={showCurrentPassword ? "eye-off" : "eye"} 
            onActionClick={toggleShowCurrentPassword}
          />
        </FormField>
        
        <FormField label="Nouveau mot de passe">
          <Input 
            type={showNewPassword ? "text" : "password"} 
            value={newPassword} 
            onChange={setNewPassword}
            placeholder="Nouveau mot de passe"
            icon="lock"
            actionIcon={showNewPassword ? "eye-off" : "eye"} 
            onActionClick={toggleShowNewPassword}
          />
          <PasswordStrength strength={passwordStrength} />
        </FormField>
        
        <FormField label="Confirmer le nouveau mot de passe">
          <Input 
            type={showConfirmNewPassword ? "text" : "password"} 
            value={confirmNewPassword} 
            onChange={setConfirmNewPassword}
            placeholder="Confirmez le nouveau mot de passe"
            icon="lock"
            actionIcon={showConfirmNewPassword ? "eye-off" : "eye"} 
            onActionClick={toggleShowConfirmNewPassword}
          />
        </FormField>
        
        <FormActions>
          <ButtonPrimary onClick={handlePasswordChange} loading={passwordLoading}>
            Changer le mot de passe
          </ButtonPrimary>
        </FormActions>
      </FormFields>
    </FormSection>
    
    <FormSection title="Sesion active">
      <ActiveSessions 
        sessions={activeSessions} 
        onTerminate={terminateSession}
      />
    </FormSection>
    
    <FormSection title="Suppression du compte">
      <DeleteAccount 
        onDelete={handleDeleteAccount}
        loading={deleteLoading}
      />
    </FormSection>
  </TabPanel>
  
  {user.isAdmin && (
    <TabPanel id="admin">
      <AdminSettings user={user} />
    </TabPanel>
  )}
</ProfileSettingsForm>
```

---

## Data Requirements

### User Object

```typescript
interface User {
  id: string;
  email: string;
  username: string | null;
  firstName: string;
  lastName: string;
  
  // Profile
  photo: string | null;
  bio: string | null;
  phone: string | null;
  birthDate: Date | null;
  
  // Palet Info
  licenseNumber: string | null;
  clubId: string | null;
  category: string;
  dominantHand: 'right' | 'left' | 'ambidextrous';
  playStyle: string | null;
  
  // Account
  role: UserRole;
  permissions: Permission[];
  status: 'active' | 'inactive' | 'suspended' | 'banned';
  emailVerified: boolean;
  
  // Preferences
  preferences: UserPreferences;
  
  // Security
  passwordLastChanged: Date | null;
  mfaEnabled: boolean;
  lastLogin: Date | null;
  
  // Social
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
  
  // Metadata
  createdAt: Date;
  updatedAt: Date;
}

interface UserPreferences {
  language: string;
  timezone: string;
  distanceUnit: 'metric' | 'imperial';
  notifications: {
    email: {
      results: boolean;
      news: boolean;
      reminders: boolean;
    };
    push: {
      results: boolean;
      matches: boolean;
    };
  };
}

type UserRole = 'player' | 'club_admin' | 'commission_admin' | 'super_admin' | 'arbitre';
```

---

### Auth Tokens

```typescript
interface AuthToken {
  accessToken: string;
  refreshToken: string;
  expiresIn: number; // seconds
  tokenType: 'Bearer';
}

interface AuthState {
  user: User | null;
  token: AuthToken | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  
  // 2FA
  requires2FA: boolean;
  twoFactorPending: boolean;
}
```

---

## API Endpoints

### Authentication Endpoints

| Endpoint | Method | Description | Request | Response |
|----------|--------|-------------|---------|----------|
| `/api/auth/login` | POST | Login with email/password | `{ email, password, rememberMe }` | `{ user, token }` |
| `/api/auth/logout` | POST | Logout | `{ refreshToken }` | `{ success }` |
| `/api/auth/refresh` | POST | Refresh access token | `{ refreshToken }` | `{ token }` |
| `/api/auth/me` | GET | Get current user | - | `{ user }` |

### Registration Endpoints
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/auth/register` | POST | Register new user | User registration data |
| `/api/auth/register/verify` | POST | Verify registration | `{ token }` |
| `/api/auth/register/resend` | POST | Resend verification email | `{ email }` |

### Password Recovery
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/auth/forgot-password` | POST | Request password reset | `{ email }` |
| `/api/auth/reset-password` | POST | Reset password | `{ token, password, confirmPassword }` |
| `/api/auth/verify-reset-token` | GET | Verify reset token | `{ token }` |

### Email Verification
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/auth/verify-email` | POST | Verify email address | `{ token }` |
| `/api/auth/resend-verification` | POST | Resend verification email | `{ email }` |

### Social Login
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/auth/social/google` | POST | Login with Google | `{ token }` |
| `/api/auth/social/facebook` | POST | Login with Facebook | `{ token }` |
| `/api/auth/social/twitter` | POST | Login with Twitter | `{ token }` |
| `/api/auth/social/connect` | POST | Connect social account | `{ provider, token }` |
| `/api/auth/social/disconnect` | POST | Disconnect social account | `{ provider }` |

### User Profile
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/users/me` | GET | Get my profile | - |
| `/api/users/me` | PATCH | Update my profile | Profile data |
| `/api/users/me/photo` | POST | Upload profile photo | FormData with photo |
| `/api/users/me/photo` | DELETE | Remove profile photo | - |
| `/api/users/me/password` | PATCH | Change password | `{ currentPassword, newPassword }` |

### 2FA Endpoints
| Endpoint | Method | Description | Request |
|----------|--------|-------------|---------|
| `/api/auth/2fa/setup` | POST | Setup 2FA | `{ method }` |
| `/api/auth/2fa/verify` | POST | Verify 2FA code | `{ code, method }` |
| `/api/auth/2fa/disable` | POST | Disable 2FA | `{ password }` |
| `/api/auth/2fa/backup` | POST | Generate backup codes | `{ password }` |

---

## Authentication Flow

### Login Flow
```
1. User enters email and password
2. Client sends POST /api/auth/login
3. Server validates credentials
4. If valid, server returns JWT tokens
5. Client stores tokens (accessToken in memory, refreshToken in httpOnly cookie)
6. Client sets isAuthenticated = true
7. Redirect to intended page or homepage
```

### Registration Flow
```
1. User fills registration form
2. Client sends POST /api/auth/register
3. Server creates user with status = 'pending'
4. Server sends verification email
5. User clicks email link with token
6. Client sends POST /api/auth/register/verify
7. Server activates user account
8. User is redirected to login page
```

### Password Recovery Flow
```
1. User enters email on forgot password page
2. Client sends POST /api/auth/forgot-password
3. Server generates reset token and sends email
4. User clicks email link with token
5. User enters new password
6. Client sends POST /api/auth/reset-password
7. Server updates password and invalidates tokens
8. User is redirected to login page
```

---

## Page States

### Loading States

#### Login Loading
```jsx
<LoginForm loading={true}>
  <ButtonPrimary loading>Se connecter</ButtonPrimary>
</LoginForm>
```

#### Social Login Loading
```jsx
<SocialButton provider="google" loading={true}>
  Continuer avec Google
</SocialButton>
```

### Error States

#### Invalid Credentials
```jsx
<ErrorState type="invalid-credentials">
  <Icon name="alert-circle" size="sm" />
  <Text>Email ou mot de passe incorrect</Text>
  <ButtonSecondary onClick={forgotPassword}>
    Mot de passe oublié ?
  </ButtonSecondary>
</ErrorState>
```

#### Account Locked
```jsx
<ErrorState type="account-locked">
  <Icon name="lock" size="sm" />
  <Text>Votre compte est verrouillé</Text>
  <Text size="body-s" color="gray-500">
    Contactez l'administrateur pour débloquer votre compte
  </Text>
  <ButtonSecondary onClick={contactSupport}>
    Contacter le support
  </ButtonSecondary>
</ErrorState>
```

#### Email Not Verified
```jsx
<ErrorState type="email-not-verified">
  <Icon name="mail" size="sm" />
  <Text>Votre email n'est pas vérifié</Text>
  <Text size="body-s" color="gray-500">
    Un lien de vérification a été envoyé à {email}
  </Text>
  <ButtonPrimary onClick={resendVerification}>
    Renvoyer le lien
  </ButtonPrimary>
</ErrorState>
```

#### Registration Error
```jsx
<ErrorState type="registration-error">
  <Icon name="alert-circle" size="sm" />
  <Text>Erreur lors de l'inscription</Text>
  {errors.map(error => (
    <ErrorMessage key={error.field}>{error.message}</ErrorMessage>
  ))}
  <ButtonPrimary onClick={retryRegistration}>
    Réessayer
  </ButtonPrimary>
</ErrorState>
```

---

## Responsive Design

### Mobile (< 640px)
- Single column layout
- Full width form
- Stacked form fields
- Simplified social login buttons
- Minimal header

### Tablet (640px - 1023px)
- Two column layout for form fields
- Side by side form fields where appropriate
- Full width social login buttons
- Compact header

### Desktop (1024px+)
- Full layout as shown in diagrams
- Form centered with max-width
- Social login buttons in row
- Complete header with all elements

---

## Accessibility

### Semantic HTML
```jsx
<main>
  <form aria-labelledby="login-title">
    <h1 id="login-title">Se connecter</h1>
    <div>
      <label htmlFor="email">Adresse email</label>
      <input 
        id="email" 
        type="email" 
        aria-required="true"
        aria-invalid={!!errors.email}
      />
      <span id="email-error" role="alert">{errors.email}</span>
    </div>
    <div>
      <label htmlFor="password">Mot de passe</label>
      <input 
        id="password" 
        type="password" 
        aria-required="true"
        aria-invalid={!!errors.password}
      />
    </div>
    <button type="submit">Se connecter</button>
  </form>
</main>
```

### Keyboard Navigation
- All form fields keyboard accessible
- Tab order follows logical sequence
- Focus visible on all interactive elements
- Enter key submits form

### Screen Reader Support
```jsx
// Form validation announcements
useEffect(() => {
  if (errors.email) {
    announce('Email: ' + errors.email);
  }
  if (errors.password) {
    announce('Mot de passe: ' + errors.password);
  }
}, [errors]);

// Loading announcements
const handleLogin = async () => {
  announce('Connexion en cours...');
  try {
    await login();
    announce('Connexion réussie');
  } catch (error) {
    announce('Erreur de connexion: ' + error.message);
  }
};
```

### Color Contrast
- All text meets WCAG AA standards
- Form labels have sufficient contrast
- Error messages are clearly visible
- Focus states are distinguishable

---

## Performance

### Form Optimization
```jsx
// Use React Hook Form for efficient form handling
const { register, handleSubmit, formState: { errors }, setValue } = useForm({
  mode: 'onChange',
  reValidateMode: 'onChange',
  defaultValues: formDefaultValues,
});

// Debounced validation
const { debouncedRegister, debouncedTrigger } = useDebouncedForm(register, trigger, 500);
```

### Authentication Context
```jsx
// Centralized auth state management
const { 
  user, 
  isAuthenticated, 
  isLoading, 
  error, 
  login, 
  logout, 
  register, 
  refreshToken 
} = useAuth();

// Optimized user data fetching
const { data: userProfile } = useQuery({
  queryKey: ['user-profile'],
  queryFn: fetchUserProfile,
  enabled: isAuthenticated,
  staleTime: 5 * 60 * 1000,
});
```

---

## SEO

### Login Page
```jsx
<Helmet>
  <title>Connexion - Palet Vendéen</title>
  <meta name="description" content="Connectez-vous à votre compte Palet Vendéen pour accéder à vos résultats et inscriptions" />
  <meta name="robots" content="noindex" />
  <link rel="canonical" href="https://palet-vendeen.fr/connexion" />
</Helmet>
```

### Registration Page
```jsx
<Helmet>
  <title>Inscription - Palet Vendéen</title>
  <meta name="description" content="Créez votre compte Palet Vendéen pour participer aux compétitions et suivre vos résultats" />
  <meta name="robots" content="noindex" />
  <link rel="canonical" href="https://palet-vendeen.fr/inscription" />
</Helmet>
```

### Forgot Password Page
```jsx
<Helmet>
  <title>Mot de passe oublié - Palet Vendéen</title>
  <meta name="description" content="Réinitialisez votre mot de passe Palet Vendéen" />
  <meta name="robots" content="noindex" />
  <link rel="canonical" href="https://palet-vendeen.fr/mot-de-passe-oublie" />
</Helmet>
```

---

## File Structure

```
web/src/pages/auth/
├── login/
│   ├── index.jsx                  # Login page
│   └── LoginForm.jsx             # Login form component
├── register/
│   ├── index.jsx                 # Registration page
│   ├── RegistrationForm.jsx       # Multi-step registration form
│   └── steps/
│       ├── PersonalInfo.jsx
│       ├── PaletInfo.jsx
│       ├── AccountInfo.jsx
│       └── Review.jsx
├── forgot-password/
│   ├── index.jsx                 # Forgot password page
│   └── ForgotPasswordForm.jsx
├── reset-password/
│   ├── index.jsx                 # Reset password page
│   ├── [token].jsx               # Reset with token (dynamic route)
│   └── ResetPasswordForm.jsx
├── verify-email/
│   ├── index.jsx                 # Verify email page
│   ├── [token].jsx               # Verify with token
│   └── EmailVerificationForm.jsx
├── profile/
│   ├── index.jsx                 # Profile settings page
│   └── ProfileSettingsForm.jsx
├── components/
│   ├── AuthLayout.jsx
│   ├── AuthHeader.jsx
│   ├── AuthFooter.jsx
│   ├── SocialLogin.jsx
│   ├── FormField.jsx
│   ├── PasswordStrength.jsx
│   ├── ErrorState.jsx
│   ├── SuccessState.jsx
│   └── LoadingState.jsx
├── hooks/
│   ├── useAuth.js
│   ├── useLogin.js
│   ├── useRegister.js
│   ├── useForgotPassword.js
│   ├── useResetPassword.js
│   └── useProfile.js
├── context/
│   └── AuthProvider.jsx
└── utils/
    ├── authValidation.js
    ├── passwordValidation.js
    └── tokenStorage.js
```

---

## Security Considerations

### Password Security
- Minimum 8 characters
- Strong password validation
- Secure hashing (bcrypt, Argon2)
- Rate limiting on login attempts

### Token Security
- JWT with short expiration (access tokens)
- Longer expiration for refresh tokens
- HttpOnly, Secure, SameSite cookies
- CSRF protection

### Session Security
- Automatic token refresh
- Session timeout handling
- Concurrent session management
- Activity logging

### Data Protection
- All sensitive data encrypted
- PII (Personally Identifiable Information) protection
- GDPR compliance
- Right to be forgotten

---

## Implementation Checklist

- [ ] Login page with email/password and social login
- [ ] Registration form with multi-step process
- [ ] Password recovery flow
- [ ] Email verification flow
- [ ] Profile settings page
- [ ] Authentication state management
- [ ] Token refresh mechanism
- [ ] Error handling for all auth scenarios
- [ ] Form validation and security
- [ ] Responsive design for all auth pages
- [ ] Accessibility features
- [ ] Performance optimization
- [ ] Security best practices

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)
- [Admin Dashboard](admin.md)

---

*Page specification - Login & Authentication*
*Created: 09/10/2026*
*Version: 1.0*