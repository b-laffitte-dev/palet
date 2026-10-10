# News Page - Palet Vendeen

> News and announcements page for the Palet Vendeen platform

---

## Overview

The **News Page** serves as the central hub for platform communication, featuring:

- **Announcements**: Official communications from FNSMR, CVDP, and clubs
- **Competition News**: Updates on championships, tournaments, and cup events
- **Player Spotlights**: Features on outstanding players and achievements
- **Club News**: Updates from individual clubs
- **Tournament Previews**: Announcements and previews of upcoming events
- **Results Summaries**: Recap of completed competitions
- **Media Gallery**: Photos and videos from events
- **Newsletter Archive**: Past newsletter issues

**Business Context**: News keeps the community engaged and informed. Different types of news exist:
- **Official**: FNSMR and commission announcements (highest priority)
- **Competition**: Tournament and championship updates
- **Club**: Local club news and events
- **Feature**: Player/team spotlights and interviews
- **Media**: Photo galleries and videos

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
│  │ ACTUALITÉS                                                  ││
│  │ Restez informé des dernières nouvelles du palet vendéen   ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ CATEGORY FILTERS (50px)                                        │
│  [Toutes] [Officiel] [Compétitions] [Clubs] [À la une] [Média]│
├─────────────────────────────────────────────────────────────┤
│ FEATURED NEWS (300px)                                         │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  ┌─────────────────┐  ┌─────────────────────────────┐  ││
│  │  │ [Featured Image] │  │ [Featured News Title]        │  ││
│  │  │ 400px x 250px    │  │ [Featured News Excerpt]      │  ││
│  │  │                 │  │ [Featured News Meta]         │  ││
│  │  └─────────────────┘  └─────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ MAIN CONTENT                                                 │
│  ┌─────────────────────┬─────────────────────────────────┐│
│  │                     │ NEWS SIDEBAR                      ││
│  │ NEWS FEED            │  ┌─────────────────────────────┐ ││
│  │                     │  │ [Search...]                   │ ││
│  │ [News Card 1]        │  │                             │ ││
│  │ [News Card 2]        │  │ [Recent News]               │ ││
│  │ [News Card 3]        │  │ ┌─────────────────────────┐ │ ││
│  │ ...                 │  │ │ [Small News Card]       │ │ ││
│  │                     │  │ │ [Small News Card]       │ │ ││
│  │                     │  │ │ [Small News Card]       │ │ ││
│  │                     │  │ └─────────────────────────┘ │ ││
│  │                     │  │                             │ ││
│  │                     │  │ [Categories]                 │ ││
│  │                     │  │ ┌─────────────────────────┐ │ ││
│  │                     │  │ │ Officiel (12)           │ │ ││
│  │                     │  │ │ Compétitions (24)       │ │ ││
│  │                     │  │ │ Clubs (15)              │ │ ││
│  │                     │  │ │ À la une (8)           │ │ ││
│  │                     │  │ │ Média (5)              │ │ ││
│  │                     │  │ └─────────────────────────┘ │ ││
│  │                     │  │                             │ ││
│  │                     │  │ [Newsletter Signup]         │ ││
│  │                     │  └─────────────────────────────┘ ││
│  └─────────────────────┴─────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│ PAGINATION                                                    │
│  [1] [2] [3] ... [10] >                                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Page Header

```jsx
<PageHeader>
  <Breadcrumb>
    <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
    <BreadcrumbItem active>Actualités</BreadcrumbItem>
  </Breadcrumb>
  
  <PageTitle size="display-l" weight="bold">
    ACTUALITÉS
  </PageTitle>
  
  <PageSubtitle size="body-l" color="gray-500">
    Restez informé des dernières nouvelles, annonces et résultats du palet vendéen
  </PageSubtitle>
  
  <PageActions>
    {isAdmin && (
      <ButtonPrimary onClick={createNews}>
        <Icon name="plus" size="sm" /> Nouvelle actualité
      </ButtonPrimary>
    )}
    <ButtonSecondary onClick={subscribeNewsletter}>
      <Icon name="mail" size="sm" /> S'abonner à la newsletter
    </ButtonSecondary>
  </PageActions>
</PageHeader>
```

---

### 2. Category Filters

```jsx
<NewsCategories>
  <CategoryFilter 
    categories={categories} 
    activeCategory={activeCategory} 
    onSelectCategory={setActiveCategory}
  >
    <CategoryItem value="all">Toutes les actualités</CategoryItem>
    <CategoryItem value="official" badge={officialCount}>
      Officiel
    </CategoryItem>
    <CategoryItem value="competitions" badge={competitionsCount}>
      Compétitions
    </CategoryItem>
    <CategoryItem value="clubs" badge={clubsCount}>
      Clubs
    </CategoryItem>
    <CategoryItem value="featured" badge={featuredCount}>
      À la une
    </CategoryItem>
    <CategoryItem value="media" badge={mediaCount}>
      Média
    </CategoryItem>
  </CategoryFilter>
</NewsCategories>
```

---

### 3. Featured News

```jsx
<FeaturedNews featured={featuredNews}>
  {featuredNews && (
    <FeaturedCard>
      <FeaturedImage 
        src={featuredNews.image} 
        alt={featuredNews.title}
        size="large"
      />
      <FeaturedContent>
        <FeaturedBadge type={featuredNews.category} />
        <FeaturedTitle size="heading-xl" weight="bold">
          {featuredNews.title}
        </FeaturedTitle>
        <FeaturedExcerpt size="body-l" color="gray-600">
          {featuredNews.excerpt}
        </FeaturedExcerpt>
        <FeaturedMeta>
          <MetaItem>
            <Icon name="calendar" size="xs" />
            <Text size="body-xs">{formatDate(featuredNews.publishedAt)}</Text>
          </MetaItem>
          <MetaItem>
            <Icon name="user" size="xs" />
            <Text size="body-xs">{featuredNews.author.name}</Text>
          </MetaItem>
          <MetaItem>
            <Icon name="eye" size="xs" />
            <Text size="body-xs">{featuredNews.views} vues</Text>
          </MetaItem>
        </FeaturedMeta>
        <FeaturedActions>
          <ButtonPrimary onClick={() => readFullArticle(featuredNews.id)}>
            Lire l'article <Icon name="arrow-right" size="sm" />
          </ButtonPrimary>
          <ButtonGhost onClick={() => shareArticle(featuredNews.id)}>
            <Icon name="share" size="sm" /> Partager
          </ButtonGhost>
        </FeaturedActions>
      </FeaturedContent>
    </FeaturedCard>
  )}
</FeaturedNews>
```

---

### 4. News Card

#### Large Card (Main Feed)

```jsx
<NewsCard news={news} size="large">
  <CardHeader>
    <CategoryBadge type={news.category} />
    <NewsTitle size="heading-m" weight="semibold">
      {news.title}
    </NewsTitle>
    <NewsMeta>
      <MetaItem>
        <Icon name="calendar" size="xs" />
        <Text size="body-xs">{formatDate(news.publishedAt)}</Text>
      </MetaItem>
      <MetaItem>
        <Icon name="user" size="xs" />
        <Text size="body-xs">{news.author.name}</Text>
      </MetaItem>
      <MetaItem>
        <Icon name="eye" size="xs" />
        <Text size="body-xs">{news.views} vues</Text>
      </MetaItem>
    </NewsMeta>
  </CardHeader>
  
  {news.image && (
    <CardImage src={news.image} alt={news.title} />
  )}
  
  <CardContent>
    <NewsExcerpt size="body-m" color="gray-600">
      {news.excerpt}
    </NewsExcerpt>
    
    <NewsTags>
      {news.tags.map(tag => (
        <Tag key={tag.id} variant="subtle">
          {tag.name}
        </Tag>
      ))}
    </NewsTags>
  </CardContent>
  
  <CardFooter>
    <NewsActions>
      <ButtonGhost size="sm" onClick={() => readFullArticle(news.id)}>
        Lire la suite <Icon name="arrow-right" size="xs" />
      </ButtonGhost>
      <ButtonGhost size="sm" onClick={() => shareArticle(news.id)}>
        <Icon name="share" size="sm" />
      </ButtonGhost>
      <ButtonGhost size="sm" onClick={() => likeArticle(news.id)}>
        <Icon name="heart" size="sm" filled={news.isLiked} />
        <Text size="body-xs">{news.likes}</Text>
      </ButtonGhost>
    </NewsActions>
  </CardFooter>
</NewsCard>
```

#### Small Card (Sidebar)

```jsx
<NewsCard news={news} size="small">
  <SmallCardContent>
    <SmallTitle size="body-m" weight="semibold">
      {news.title}
    </SmallTitle>
    <SmallMeta>
      <Text size="body-xs" color="gray-500">
        {formatDate(news.publishedAt)}
      </Text>
    </SmallMeta>
    {news.image && (
      <SmallImage src={news.image} alt={news.title} />
    )}
  </SmallCardContent>
</NewsCard>
```

---

### 5. News Feed

```jsx
<NewsFeed news={newsItems} loading={loading} hasMore={hasMore}>
  <FeedLayout>
    <MainFeed>
      {newsItems.map(news => (
        <NewsCard 
          key={news.id} 
          news={news} 
          size="large" 
          onClick={() => readArticle(news.id)}
        />
      ))}
      
      {loading && (
        <NewsCardSkeleton count={3} />
      )}
      
      {hasMore && !loading && (
        <LoadMore onClick={loadMore} />
      )}
      
      {newsItems.length === 0 && !loading && (
        <EmptyState type="no-news">
          <Icon name="newspaper" size="xl" />
          <Text size="heading-l">Aucune actualité trouvée</Text>
          <Text size="body-m" color="gray-500">
            Essayez de modifier vos filtres ou revenez plus tard
          </Text>
        </EmptyState>
      )}
    </MainFeed>
    
    <Sidebar>
      <SidebarSearch onSearch={setSearchQuery} />
      
      <SidebarSection title="Récent">
        <RecentNewsList 
          recentNews={recentNews} 
          onSelect={readArticle}
        />
      </SidebarSection>
      
      <SidebarSection title="Catégories">
        <CategoryList 
          categories={categories} 
          activeCategory={activeCategory} 
          onSelect={setActiveCategory}
        />
      </SidebarSection>
      
      <SidebarSection title="Newsletter">
        <NewsletterSignup 
          onSubscribe={handleNewsletterSubscribe}
          loading={newsletterLoading}
        />
      </SidebarSection>
      
      <SidebarSection title="Archives">
        <ArchiveList 
          archives={archives} 
          onSelect={viewArchive}
        />
      </SidebarSection>
    </Sidebar>
  </FeedLayout>
</NewsFeed>
```

---

### 6. News Detail Page

```jsx
<NewsDetail news={news} related={relatedNews}>
  <DetailHeader>
    <Breadcrumb>
      <BreadcrumbItem href="/">Accueil</BreadcrumbItem>
      <BreadcrumbItem href="/actualites">Actualités</BreadcrumbItem>
      <BreadcrumbItem active>{news.title}</BreadcrumbItem>
    </Breadcrumb>
    
    <DetailTitle size="display-m" weight="bold">
      {news.title}
    </DetailTitle>
    
    <DetailMeta>
      <MetaGroup>
        <MetaItem>
          <Icon name="calendar" size="sm" />
          <Text size="body-m">{formatDate(news.publishedAt)}</Text>
        </MetaItem>
        <MetaItem>
          <Icon name="user" size="sm" />
          <AuthorInfo author={news.author} />
        </MetaItem>
      </MetaGroup>
      <MetaGroup>
        <MetaItem>
          <Icon name="eye" size="sm" />
          <Text size="body-m">{news.views} vues</Text>
        </MetaItem>
        <MetaItem>
          <Icon name="heart" size="sm" />
          <Text size="body-m">{news.likes} j'aime</Text>
        </MetaItem>
      </MetaGroup>
    </DetailMeta>
    
    <DetailActions>
      <ButtonPrimary onClick={() => shareArticle(news.id)}>
        <Icon name="share" size="sm" /> Partager
      </ButtonPrimary>
      <ButtonGhost onClick={() => likeArticle(news.id)}>
        <Icon name="heart" size="sm" filled={news.isLiked} />
        <Text size="body-s">{news.likes} j'aime</Text>
      </ButtonGhost>
      <ButtonGhost onClick={printArticle}>
        <Icon name="printer" size="sm" /> Imprimer
      </ButtonGhost>
    </DetailActions>
  </DetailHeader>
  
  {news.image && (
    <DetailImage src={news.image} alt={news.title} caption={news.imageCaption} />
  )}
  
  <DetailContent>
    <ContentHTML html={news.content} />
    
    <ContentTags>
      <TagsTitle>Mots-clés :</TagsTitle>
      {news.tags.map(tag => (
        <Tag key={tag.id} variant="filled">
          {tag.name}
        </Tag>
      ))}
    </ContentTags>
  </DetailContent>
  
  <DetailAuthorSection>
    <AuthorCard author={news.author} />
    <AuthorDescription>{news.author.bio}</AuthorDescription>
    <AuthorSocial social={news.author.social} />
  </DetailAuthorSection>
  
  <RelatedNewsSection>
    <SectionTitle size="heading-m">Articles similaires</SectionTitle>
    <RelatedNewsGrid>
      {relatedNews.map(related => (
        <NewsCard 
          key={related.id} 
          news={related} 
          size="medium" 
          onClick={() => readArticle(related.id)}
        />
      ))}
    </RelatedNewsGrid>
  </RelatedNewsSection>
  
  <CommentsSection>
    <SectionTitle size="heading-m">
      Commentaires ({news.commentsCount})
    </SectionTitle>
    <CommentsList 
      comments={news.comments} 
      onAddComment={addComment} 
      loading={commentsLoading}
    />
  </CommentsSection>
</NewsDetail>
```

---

### 7. Newsletter Signup

```jsx
<NewsletterSignup onSubscribe={handleSubscribe} loading={loading}>
  <SignupTitle size="heading-s">Abonnez-vous à notre newsletter</SignupTitle>
  <SignupDescription size="body-s" color="gray-600">
    Recevez les dernières actualités et résultats directement dans votre boîte mail
  </SignupDescription>
  
  <SignupForm onSubmit={handleSubmit}>
    <FormField label="Adresse email" required>
      <Input 
        type="email" 
        value={email} 
        onChange={setEmail}
        placeholder="votre@email.com"
      />
    </FormField>
    
    <FormField label="Fréquence">
      <Select value={frequency} onChange={setFrequency}>
        <Option value="daily">Quotidienne</Option>
        <Option value="weekly">Hebdomadaire</Option>
        <Option value="monthly">Mensuelle</Option>
      </Select>
    </FormField>
    
    <FormField>
      <Checkbox 
        checked={receiveResults} 
        onChange={setReceiveResults}
        label="Recevoir les résultats des compétitions"
      />
    </FormField>
    
    <FormField>
      <Checkbox 
        checked={receiveAnnouncements} 
        onChange={setReceiveAnnouncements}
        label="Recevoir les annonces importantes"
      />
    </FormField>
    
    <FormField>
      <Checkbox 
        checked={receiveTips} 
        onChange={setReceiveTips}
        label="Recevoir des conseils et astuces"
      />
    </FormField>
    
    <ButtonPrimary type="submit" loading={loading} fullWidth>
      S'abonner
    </ButtonPrimary>
  </SignupForm>
  
  <SignupDisclaimer size="body-xs" color="gray-500">
    En vous abonnant, vous acceptez notre politique de confidentialité.
    Vous pouvez vous désabonner à tout moment.
  </SignupDisclaimer>
</NewsletterSignup>
```

---

### 8. Media Gallery

```jsx
<MediaGallery media={mediaItems}>
  <GalleryHeader>
    <GalleryTitle size="heading-m">Galerie média</GalleryTitle>
    <GalleryFilters>
      <FilterChip 
        active={mediaType === 'all'} 
        onClick={() => setMediaType('all')}
      >
        Tout
      </FilterChip>
      <FilterChip 
        active={mediaType === 'photos'} 
        onClick={() => setMediaType('photos')}
      >
        Photos
      </FilterChip>
      <FilterChip 
        active={mediaType === 'videos'} 
        onClick={() => setMediaType('videos')}
      >
        Vidéos
      </FilterChip>
    </GalleryFilters>
  </GalleryHeader>
  
  <GalleryGrid>
    {mediaItems.map(item => (
      <MediaCard 
        key={item.id} 
        media={item} 
        onClick={() => openMedia(item.id)}
      >
        {item.type === 'photo' && (
          <PhotoThumbnail src={item.thumbnail} alt={item.title} />
        )}
        {item.type === 'video' && (
          <VideoThumbnail 
            src={item.thumbnail} 
            alt={item.title} 
            duration={item.duration}
          />
        )}
        <MediaInfo>
          <MediaTitle size="body-s" weight="semibold">{item.title}</MediaTitle>
          <MediaMeta size="body-xs" color="gray-500">
            {formatDate(item.date)} - {item.views} vues
          </MediaMeta>
        </MediaInfo>
      </MediaCard>
    ))}
  </GalleryGrid>
  
  {hasMore && (
    <LoadMore onClick={loadMoreMedia} />
  )}
</MediaGallery>
```

---

## Data Requirements

### News Article

```typescript
interface NewsArticle {
  id: string;
  title: string;
  slug: string;
  excerpt: string;
  content: string;
  
  // Category & Type
  category: NewsCategory;
  type: NewsType;
  
  // Media
  image: string | null;
  imageCaption: string | null;
  thumbnail: string | null;
  
  // Author
  author: NewsAuthor;
  authorId: string;
  
  // Metadata
  status: 'draft' | 'published' | 'archived' | 'deleted';
  visibility: 'public' | 'members' | 'admin';
  priority: number; // For sorting
  
  // Stats
  views: number;
  likes: number;
  shares: number;
  commentsCount: number;
  
  // Classification
  tags: NewsTag[];
  
  // Publishing
  publishedAt: Date;
  scheduledAt: Date | null;
  createdAt: Date;
  updatedAt: Date;
  
  // SEO
  metaTitle: string | null;
  metaDescription: string | null;
  
  // Comments
  comments: NewsComment[];
  
  // Related
  relatedArticles: string[] | NewsArticle[];
}

// Types
type NewsCategory = 'official' | 'competitions' | 'clubs' | 'featured' | 'media';
type NewsType = 'announcement' | 'news' | 'preview' | 'recap' | 'interview' | 'feature' | 'gallery';

interface NewsTag {
  id: string;
  name: string;
  slug: string;
}

interface NewsAuthor {
  id: string;
  name: string;
  avatar: string | null;
  role: string;
  bio: string | null;
  social: {
    facebook: string | null;
    twitter: string | null;
    instagram: string | null;
  };
}

interface NewsComment {
  id: string;
  content: string;
  author: NewsCommentAuthor;
  createdAt: Date;
  updatedAt: Date;
  likes: number;
  isLiked: boolean;
  replies: NewsComment[];
}

interface NewsCommentAuthor {
  id: string;
  name: string;
  avatar: string | null;
}
```

---

### Media Item

```typescript
interface MediaItem {
  id: string;
  title: string;
  description: string | null;
  type: 'photo' | 'video';
  
  // Media URLs
  url: string;
  thumbnail: string;
  
  // Metadata
  category: MediaCategory;
  tags: string[];
  
  // Stats
  views: number;
  likes: number;
  
  // Details
  date: Date;
  uploadedBy: NewsAuthor;
  
  // For videos
  duration: number | null; // in seconds
  
  // For photos
  width: number | null;
  height: number | null;
}

type MediaCategory = 'competition' | 'player' | 'club' | 'event' | 'behind-the-scenes';
```

---

### Newsletter

```typescript
interface Newsletter {
  id: string;
  email: string;
  firstName: string | null;
  lastName: string | null;
  
  // Preferences
  frequency: 'daily' | 'weekly' | 'monthly';
  categories: NewsCategory[];
  receiveResults: boolean;
  receiveAnnouncements: boolean;
  receiveTips: boolean;
  
  // Stats
  sentCount: number;
  openRate: number;
  clickRate: number;
  
  // Status
  status: 'active' | 'unsubscribed' | 'bounced';
  verified: boolean;
  
  // Timestamps
  createdAt: Date;
  updatedAt: Date;
  unsubscribedAt: Date | null;
}
```

---

## API Endpoints

### News Endpoints

| Endpoint | Method | Description | Parameters |
|----------|--------|-------------|------------|
| `/api/news` | GET | List news articles | category, type, author, tags, search, page, limit |
| `/api/news` | POST | Create news article | news data |
| `/api/news/{id}` | GET | Get news article | id |
| `/api/news/{id}` | PATCH | Update news article | id, news data |
| `/api/news/{id}` | DELETE | Delete news article | id |
| `/api/news/{id}/publish` | POST | Publish news article | id |
| `/api/news/{id}/archive` | POST | Archive news article | id |
| `/api/news/{id}/like` | POST | Like news article | id |
| `/api/news/{id}/unlike` | POST | Unlike news article | id |
| `/api/news/{id}/views` | POST | Increment view count | id |
| `/api/news/{id}/comments` | GET | Get article comments | id, page, limit |
| `/api/news/{id}/comments` | POST | Add comment to article | id, comment data |

### Category Endpoints
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/news/categories` | GET | List news categories |
| `/api/news/tags` | GET | List news tags |
| `/api/news/tags` | POST | Create news tag |

### Featured News
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/news/featured` | GET | Get featured news |
| `/api/news/{id}/featured` | POST | Set as featured |
| `/api/news/{id}/featured` | DELETE | Remove from featured |

### Media Endpoints
| Endpoint | Method | Description | Parameters |
|----------|--------|-------------|------------|
| `/api/media` | GET | List media items | type, category, tags, page, limit |
| `/api/media` | POST | Upload media | media file + metadata |
| `/api/media/{id}` | GET | Get media item | id |
| `/api/media/{id}` | PATCH | Update media item | id, metadata |
| `/api/media/{id}` | DELETE | Delete media item | id |
| `/api/media/{id}/like` | POST | Like media item | id |
| `/api/media/{id}/views` | POST | Increment view count | id |

### Newsletter Endpoints
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/newsletter/subscribe` | POST | Subscribe to newsletter | email, preferences |
| `/api/newsletter/unsubscribe` | POST | Unsubscribe from newsletter | email/token |
| `/api/newsletter/verify` | POST | Verify email address | token |
| `/api/newsletter/subscriptions` | GET | List subscriptions (admin) | page, limit |
| `/api/newsletter/campaigns` | GET | List newsletter campaigns (admin) |
| `/api/newsletter/send` | POST | Send newsletter campaign (admin) |

---

## Page States

### Loading States

#### News Feed Loading
```jsx
<NewsFeedSkeleton>
  <FeaturedSkeleton />
  <NewsCardsSkeleton count={6} />
  <SidebarSkeleton />
</NewsFeedSkeleton>
```

#### Article Loading
```jsx
<NewsDetailSkeleton />
```

### Error States

#### Not Found
```jsx
<EmptyState type="not-found">
  <Icon name="newspaper" size="xl" />
  <Title>Article introuvable</Title>
  <Text>L'article que vous cherchez n'existe pas ou a été supprimé.</Text>
  <ButtonPrimary onClick={goToNews}>
    Retour aux actualités
  </ButtonPrimary>
</EmptyState>
```

#### No News
```jsx
<EmptyState type="no-news">
  <Icon name="newspaper" size="xl" />
  <Title>Aucune actualité disponible</Title>
  <Text>Il n'y a actuellement aucune actualité. Revenez plus tard !</Text>
  {isAdmin && (
    <ButtonPrimary onClick={createNews}>
      Créer une actualité
    </ButtonPrimary>
  )}
</EmptyState>
```

#### Subscription Error
```jsx
<SubscriptionError error={error}>
  <ErrorTitle>Erreur d'abonnement</ErrorTitle>
  <ErrorMessage>{error.message}</ErrorMessage>
  <ButtonPrimary onClick={retry}>
    Réessayer
  </ButtonPrimary>
</SubscriptionError>
```

---

## Responsive Design

### Mobile (< 640px)
- Single column layout
- Stacked featured news
- Sidebar moves below main content
- Simplified news cards
- Full width images

### Tablet (640px - 1023px)
- Two column layout for main content
- Featured news full width
- Sidebar on the side
- Grid layout for news cards

### Desktop (1024px+)
- Full multi-column layout
- Featured news section at top
- Sidebar with all widgets
- Masonry grid for news cards

---

## Accessibility

### Semantic HTML
```jsx
<main>
  <article>
    <header>
      <h1>{news.title}</h1>
      <address>Par {news.author.name}</address>
      <time datetime={news.publishedAt}>{formatDate(news.publishedAt)}</time>
    </header>
    <section>
      {news.image && <figure>
        <img src={news.image} alt={news.title} />
        <figcaption>{news.imageCaption}</figcaption>
      </figure>}
      <div dangerouslySetInnerHTML={{ __html: news.content }} />
    </section>
    <footer>
      <section aria-label="Commentaires">...</section>
    </footer>
  </article>
</main>
```

### Keyboard Navigation
- All interactive elements keyboard accessible
- News cards focusable
- Form fields properly labeled
- Action buttons keyboard accessible

### Screen Reader Support
```jsx
// Announce new articles
useEffect(() => {
  if (newArticles.length > 0) {
    announce(`${newArticles.length} nouveaux articles disponibles`);
  }
}, [newArticles]);

// Article reading time
const readingTime = calculateReadingTime(news.content);
<MetaItem>
  <Icon name="clock" size="xs" />
  <Text size="body-xs">{readingTime} min de lecture</Text>
</MetaItem>
```

### Color Contrast
- All text meets WCAG AA standards
- Good contrast for content on images
- Readable text sizes

---

## Performance

### Data Fetching
```jsx
// News list with infinite scroll
const { data: news } = useInfiniteQuery({
  queryKey: ['news', filters],
  queryFn: ({ pageParam }) => fetchNews(filters, pageParam),
  getNextPageParam: (lastPage) => lastPage.nextPage,
  staleTime: 5 * 60 * 1000,
});

// Featured news with higher priority
const { data: featured } = useQuery({
  queryKey: ['featured-news'],
  queryFn: fetchFeaturedNews,
  staleTime: 15 * 60 * 1000,
  priority: 'high',
});

// Recent news for sidebar
const { data: recent } = useQuery({
  queryKey: ['recent-news'],
  queryFn: () => fetchNews({ limit: 5, sort: 'recent' }),
  staleTime: 30 * 60 * 1000,
});
```

### Image Optimization
```jsx
// Use optimized images
<Image 
  src={news.image} 
  alt={news.title}
  loading="lazy"
  sizes="(max-width: 640px) 100vw, (max-width: 1024px) 50vw, 33vw"
  quality={85}
  placeholder="blur"
  blurDataURL={news.thumbnail || placeholderBlur}
/>
```

### Lazy Loading
- Images lazy loaded
- Media gallery loaded on demand
- Comments loaded when section visible
- Newsletter form loaded when visible

---

## SEO

### News List Page
```jsx
<Helmet>
  <title>Actualités - Palet Vendéen</title>
  <meta name="description" content="Toutes les actualités du palet vendéen : résultats, annonces, interviews" />
  <meta property="og:title" content="Actualités - Palet Vendéen" />
  <meta property="og:description" content="Toutes les actualités du palet vendéen" />
  <meta property="og:type" content="website" />
  <link rel="canonical" href="https://palet-vendeen.fr/actualites" />
</Helmet>
```

### News Article Page
```jsx
<Helmet>
  <title>{news.metaTitle || news.title} - Palet Vendéen</title>
  <meta name="description" content={news.metaDescription || news.excerpt} />
  <meta property="og:title" content={news.metaTitle || news.title} />
  <meta property="og:description" content={news.metaDescription || news.excerpt} />
  <meta property="og:image" content={news.image || defaultOgImage} />
  <meta property="og:type" content="article" />
  <meta property="article:published_time" content={news.publishedAt} />
  <meta property="article:author" content={news.author.name} />
  <meta name="twitter:card" content="summary_large_image" />
  <link rel="canonical" href={`https://palet-vendeen.fr/actualites/${news.slug}`} />
</Helmet>
```

---

## File Structure

```
web/src/pages/news/
├── index.jsx                      # News list page
├── [slug].jsx                    # News detail page (dynamic route)
├── NewsProvider.jsx              # News context
├── media/
│   ├── index.jsx                 # Media gallery page
│   └── [id].jsx                  # Media detail page
├── tags/
│   └── index.jsx                 # Tag page
├── categories/
│   └── index.jsx                 # Category page
├── newsletter/
│   ├── index.jsx                 # Newsletter signup page
│   ├── manage.jsx                # Newsletter preferences (user)
│   └── confirm.jsx               # Email confirmation page
├── components/
│   ├── NewsHeader.jsx
│   ├── NewsFilters.jsx
│   ├── FeaturedNews.jsx
│   ├── NewsCard.jsx
│   ├── NewsFeed.jsx
│   ├── NewsDetail.jsx
│   ├── NewsletterForm.jsx
│   ├── MediaGallery.jsx
│   ├── MediaCard.jsx
│   ├── CommentSection.jsx
│   ├── Sidebar.jsx
│   ├── CategoryList.jsx
│   ├── ArchiveList.jsx
│   └── EmptyState.jsx
├── hooks/
│   ├── useNews.js
│   ├── useNewsDetail.js
│   ├── useMedia.js
│   ├── useNewsletter.js
│   └── useComments.js
└── utils/
    ├── newsFormatting.js
    ├── mediaFormatting.js
    └── newsletterValidation.js
```

---

## Implementation Checklist

- [ ] Page Header with breadcrumb and title
- [ ] Category Filters
- [ ] Featured News section
- [ ] News Feed with cards
- [ ] Sidebar with search, recent news, categories, newsletter signup
- [ ] News Detail page
- [ ] Newsletter signup functionality
- [ ] Media Gallery
- [ ] Comment system
- [ ] Tag and category pages
- [ ] Related articles
- [ ] Featured news management
- [ ] Responsive design
- [ ] Error and loading states
- [ ] Data fetching with caching
- [ ] Accessibility features
- [ ] SEO optimization

---

## References

- [Design System - Colors](/docs/design-system/colors.md)
- [Design System - Typography](/docs/design-system/typography.md)
- [Design System - Components](/docs/design-system/components.md)
- [Business Context](/docs/contexte-metier/contexte_metier_palet_vendeen_resume.md)

---

*Page specification - News Page*
*Created: 09/10/2026*
*Version: 1.0*

---

**Next Pages**: [Login](login.md)