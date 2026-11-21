//! Notifications in-app — GET /notifications, GET /notifications/count,
//! POST /notifications/{id}/lu, POST /notifications/tout-lu (compte connecté).

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, AppNotification, UnreadCount};
use serde::Deserialize;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::AppState;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct NotificationsQuery {
    /// ne renvoyer que les non lues
    #[serde(default)]
    pub unread: bool,
}

/// Liste les notifications du compte connecté (les plus récentes d'abord).
#[utoipa::path(
    get,
    path = "/notifications",
    tag = "Notifications",
    params(("unread", Query, description = "Uniquement les non lues")),
    responses(
        (status = 200, description = "Notifications", body = Vec<AppNotification>),
        (status = 401, description = "Non authentifié")
    )
)]
pub async fn list_notifications(
    auth: AuthUser,
    state: web::Data<AppState>,
    query: web::Query<NotificationsQuery>,
) -> impl Responder {
    let user_id = match Uuid::parse_str(&auth.0.sub) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                data: (),
                message: Some("Claim sub invalide".to_string()),
                success: false,
            })
        }
    };
    match state.db.notifications.list(user_id, query.unread, 50).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            data: list,
            message: Some("Notifications récupérées".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to list notifications: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}

/// Nombre de notifications non lues (badge du header).
#[utoipa::path(
    get,
    path = "/notifications/count",
    tag = "Notifications",
    responses(
        (status = 200, description = "Compteur non lues", body = UnreadCount),
        (status = 401, description = "Non authentifié")
    )
)]
pub async fn unread_count(auth: AuthUser, state: web::Data<AppState>) -> impl Responder {
    let user_id = match Uuid::parse_str(&auth.0.sub) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                data: (),
                message: Some("Claim sub invalide".to_string()),
                success: false,
            })
        }
    };
    match state.db.notifications.count_unread(user_id).await {
        Ok(unread) => HttpResponse::Ok().json(ApiResponse {
            data: UnreadCount { unread },
            message: Some("Compteur récupéré".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to count notifications: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}

/// Marque une notification comme lue.
#[utoipa::path(
    post,
    path = "/notifications/{id}/lu",
    tag = "Notifications",
    params(("id", description = "Notification ID")),
    responses(
        (status = 200, description = "Notification marquée lue", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 404, description = "Notification introuvable")
    )
)]
pub async fn mark_read(
    auth: AuthUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let user_id = match Uuid::parse_str(&auth.0.sub) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                data: (),
                message: Some("Claim sub invalide".to_string()),
                success: false,
            })
        }
    };
    match state.db.notifications.mark_read(*id, user_id).await {
        Ok(true) => HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Notification marquée comme lue".to_string()),
            success: true,
        }),
        Ok(false) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Notification introuvable".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to mark notification read: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}

/// Marque toutes les notifications comme lues.
#[utoipa::path(
    post,
    path = "/notifications/tout-lu",
    tag = "Notifications",
    responses(
        (status = 200, description = "Nombre de notifications mises à jour", body = i64),
        (status = 401, description = "Non authentifié")
    )
)]
pub async fn mark_all_read(auth: AuthUser, state: web::Data<AppState>) -> impl Responder {
    let user_id = match Uuid::parse_str(&auth.0.sub) {
        Ok(u) => u,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                data: (),
                message: Some("Claim sub invalide".to_string()),
                success: false,
            })
        }
    };
    match state.db.notifications.mark_all_read(user_id).await {
        Ok(n) => HttpResponse::Ok().json(ApiResponse {
            data: n,
            message: Some(format!("{n} notification(s) marquée(s) comme lue(s)")),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to mark all notifications read: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}
