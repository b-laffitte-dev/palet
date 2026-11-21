//! News Repository - actualités (table news)

use crate::db::DatabaseError;
use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{slugify, CreateNewsRequest, NewsArticle, UpdateNewsRequest};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait NewsRepository: Send + Sync {
    async fn create_article(
        &self,
        request: CreateNewsRequest,
        author_id: Uuid,
    ) -> Result<NewsArticle, DatabaseError>;
    async fn get_article(&self, id: Uuid) -> Result<Option<NewsArticle>, DatabaseError>;
    async fn get_article_by_slug(&self, slug: &str) -> Result<Option<NewsArticle>, DatabaseError>;
    /// Articles publiés uniquement (API publique), filtre catégorie/recherche optionnel.
    async fn get_published_articles(
        &self,
        category: Option<String>,
        query: Option<String>,
    ) -> Result<Vec<NewsArticle>, DatabaseError>;
    /// Tous les articles (brouillons compris) pour l'espace admin.
    async fn get_all_articles(&self) -> Result<Vec<NewsArticle>, DatabaseError>;
    async fn update_article(
        &self,
        id: Uuid,
        request: UpdateNewsRequest,
    ) -> Result<Option<NewsArticle>, DatabaseError>;
    async fn delete_article(&self, id: Uuid) -> Result<bool, DatabaseError>;
}

#[derive(Clone)]
pub struct PostgresNewsRepository {
    pool: Arc<PgPool>,
}

impl PostgresNewsRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn row_to_article(row: sqlx::postgres::PgRow) -> NewsArticle {
        NewsArticle {
            id: row.get("id"),
            title: row.get("title"),
            slug: row.get("slug"),
            excerpt: row.get("excerpt"),
            content: row.get("content"),
            image: row.get("image"),
            category: row.get("category"),
            author_id: row.get("author_id"),
            published: row.get("published"),
            published_at: row.get("published_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[async_trait]
impl NewsRepository for PostgresNewsRepository {
    async fn create_article(
        &self,
        request: CreateNewsRequest,
        author_id: Uuid,
    ) -> Result<NewsArticle, DatabaseError> {
        let now = Utc::now();
        let slug = slugify(&request.title);
        let published_at = if request.published { Some(now) } else { None };
        let row = sqlx::query(
            r#"
            INSERT INTO news (id, title, slug, excerpt, content, image, category, author_id, published, published_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&request.title)
        .bind(&slug)
        .bind(&request.excerpt)
        .bind(&request.content)
        .bind(&request.image)
        .bind(&request.category)
        .bind(author_id)
        .bind(request.published)
        .bind(published_at)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("news_slug_key") => {
                DatabaseError::ValidationError(format!(
                    "Le slug \"{slug}\" est déjà utilisé (titre proche d'un article existant)"
                ))
            }
            other => DatabaseError::from(other),
        })?;
        Ok(Self::row_to_article(row))
    }

    async fn get_article(&self, id: Uuid) -> Result<Option<NewsArticle>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM news WHERE id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(Self::row_to_article))
    }

    async fn get_article_by_slug(&self, slug: &str) -> Result<Option<NewsArticle>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM news WHERE slug = $1")
            .bind(slug)
            .fetch_optional(&*self.pool)
            .await?;
        Ok(row.map(Self::row_to_article))
    }

    async fn get_published_articles(
        &self,
        category: Option<String>,
        query: Option<String>,
    ) -> Result<Vec<NewsArticle>, DatabaseError> {
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
            sqlx::QueryBuilder::new("SELECT * FROM news WHERE published = true");
        if let Some(c) = &category {
            qb.push(" AND category = ").push_bind(c.clone());
        }
        if let Some(search) = &query {
            let pattern = format!("%{search}%");
            qb.push(" AND (title ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR content ILIKE ")
                .push_bind(pattern)
                .push(")");
        }
        qb.push(" ORDER BY published_at DESC NULLS LAST, created_at DESC");
        let rows = qb.build().fetch_all(&*self.pool).await?;
        Ok(rows.into_iter().map(Self::row_to_article).collect())
    }

    async fn get_all_articles(&self) -> Result<Vec<NewsArticle>, DatabaseError> {
        let rows = sqlx::query("SELECT * FROM news ORDER BY created_at DESC")
            .fetch_all(&*self.pool)
            .await?;
        Ok(rows.into_iter().map(Self::row_to_article).collect())
    }

    async fn update_article(
        &self,
        id: Uuid,
        request: UpdateNewsRequest,
    ) -> Result<Option<NewsArticle>, DatabaseError> {
        let now = Utc::now();
        let new_slug = request.title.as_deref().map(slugify);
        let row = sqlx::query(
            r#"
            UPDATE news SET
                title = COALESCE($1, title),
                slug = COALESCE($2, slug),
                excerpt = COALESCE($3, excerpt),
                content = COALESCE($4, content),
                image = COALESCE($5, image),
                category = COALESCE($6, category),
                published = COALESCE($7, published),
                published_at = CASE WHEN $7 = true AND published_at IS NULL THEN $8 ELSE published_at END,
                updated_at = $8
            WHERE id = $9
            RETURNING *
            "#,
        )
        .bind(&request.title)
        .bind(&new_slug)
        .bind(&request.excerpt)
        .bind(&request.content)
        .bind(&request.image)
        .bind(&request.category)
        .bind(request.published)
        .bind(now)
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;
        Ok(row.map(Self::row_to_article))
    }

    async fn delete_article(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query("DELETE FROM news WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
