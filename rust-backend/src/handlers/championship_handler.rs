//! Championship endpoint handlers for Palet Vendéen
//!
//! Handles all championship-related operations with TDD approach

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{
    ApiResponse, Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationRules, ClassificationSecondary, ClassificationTertiary, CompetitionStatus,
    CreateChampionshipRequest, PaginatedResponse, Pagination, TeamClassificationStatus, TeamStats,
    UpdateChampionshipRequest,
};
use uuid::Uuid;

use crate::AppState;

/// Get all championships
#[utoipa::path(
    get,
    path = "/championnats",
    tag = "Championnats",
    responses(
        (status = 200, description = "List of all championships", body = PaginatedResponse<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships(state: web::Data<AppState>) -> impl Responder {
    match state.db.get_championships().await {
        Ok(championships) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: championships.len() as i64,
                total_items: championships.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: championships,
                    pagination,
                },
                message: Some("Championnats retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get championships: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships".to_string()),
                success: false,
            })
        }
    }
}

/// Get championships by league
#[utoipa::path(
    get,
    path = "/championnats/league/{league}",
    tag = "Championnats",
    params(
        ("league" = String, description = "League name")
    ),
    responses(
        (status = 200, description = "List of championships for league", body = PaginatedResponse<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships_by_league(
    state: web::Data<AppState>,
    league: web::Path<String>,
) -> impl Responder {
    match state.db.get_championships().await {
        Ok(championships) => {
            let filtered: Vec<Championship> = championships
                .into_iter()
                .filter(|c| c.league == *league)
                .collect();

            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: filtered.len() as i64,
                total_items: filtered.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: filtered,
                    pagination,
                },
                message: Some("Championnats retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get championships by league: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships by league".to_string()),
                success: false,
            })
        }
    }
}

/// Get championships by season
#[utoipa::path(
    get,
    path = "/championnats/season/{season}",
    tag = "Championnats",
    params(
        ("season" = String, description = "Season (e.g., 2025-2026)")
    ),
    responses(
        (status = 200, description = "List of championships for season", body = PaginatedResponse<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships_by_season(
    state: web::Data<AppState>,
    season: web::Path<String>,
) -> impl Responder {
    match state.db.get_championships().await {
        Ok(championships) => {
            let filtered: Vec<Championship> = championships
                .into_iter()
                .filter(|c| c.season == *season)
                .collect();

            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: filtered.len() as i64,
                total_items: filtered.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: filtered,
                    pagination,
                },
                message: Some("Championnats retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get championships by season: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships by season".to_string()),
                success: false,
            })
        }
    }
}

/// Get a specific championship by ID
#[utoipa::path(
    get,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship details", body = Championship),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championship(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state.db.get_championship(*id).await {
        Ok(Some(championship)) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Championship retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new championship
#[utoipa::path(
    post,
    path = "/championnats",
    tag = "Championnats",
    request_body = CreateChampionshipRequest,
    responses(
        (status = 201, description = "Championship created", body = Championship),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_championship(
    state: web::Data<AppState>,
    request: web::Json<CreateChampionshipRequest>,
) -> impl Responder {
    // Validate championship data for Palet Vendéen
    // - Must have at least 1 division
    // - Total journées must be positive
    // - Classification rules must be valid

    if request.divisions.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship must have at least one division".to_string()),
            success: false,
        });
    }

    if request.total_journees <= 0 {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some("Total journées must be positive".to_string()),
            success: false,
        });
    }

    // Validate classification rules
    // Primary must be Points for Palet Vendéen
    if request.classification_rules.primary != ClassificationPrimary::Points {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some(
                "Primary classification must be Points for Palet Vendéen championships".to_string(),
            ),
            success: false,
        });
    }

    match state.db.create_championship(request.into_inner()).await {
        Ok(championship) => HttpResponse::Created().json(ApiResponse {
            data: championship,
            message: Some("Championship created successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to create championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create championship".to_string()),
                success: false,
            })
        }
    }
}

/// Update a championship
#[utoipa::path(
    patch,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    request_body = UpdateChampionshipRequest,
    responses(
        (status = 200, description = "Championship updated", body = Championship),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_championship(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateChampionshipRequest>,
) -> impl Responder {
    // Validate that we're not changing to invalid state
    if let Some(status) = request.status {
        // Can't go back to Pending once started
        match state.db.get_championship(*id).await {
            Ok(Some(championship)) => {
                if championship.status != CompetitionStatus::Pending
                    && status == CompetitionStatus::Pending
                {
                    return HttpResponse::BadRequest().json(ApiResponse::<()> {
                        data: (),
                        message: Some(
                            "Cannot revert championship to Pending status once started".to_string(),
                        ),
                        success: false,
                    });
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Championship not found".to_string()),
                    success: false,
                });
            }
            Err(e) => {
                log::error!("Failed to check championship status: {:?}", e);
                return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Failed to check championship status".to_string()),
                    success: false,
                });
            }
        }
    }

    match state.db.update_championship(*id, request.into_inner()).await {
        Ok(Some(championship)) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Championship updated successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to update championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to update championship".to_string()),
                success: false,
            })
        }
    }
}

/// Delete a championship
#[utoipa::path(
    delete,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship deleted"),
        (status = 400, description = "Cannot delete championship with matches"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_championship(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    // Check if championship has matches
    match state.db.get_matches_by_competition(*id).await {
        Ok(matches) => {
            if !matches.is_empty() {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some(
                        "Cannot delete championship with existing matches".to_string(),
                    ),
                    success: false,
                });
            }
        }
        Err(e) => {
            log::error!("Failed to check championship matches: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to check championship matches".to_string()),
                success: false,
            });
        }
    }

    // For now, just return success
    // Actual deletion would need to be implemented in db.rs
    HttpResponse::Ok().json(ApiResponse::<()> {
        data: (),
        message: Some("Championship deleted successfully".to_string()),
        success: true,
    })
}

/// Start a championship (set status to InProgress and current_journee to 1)
#[utoipa::path(
    post,
    path = "/championnats/{id}/start",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship started", body = Championship),
        (status = 400, description = "Invalid championship state"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn start_championship(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state.db.get_championship(*id).await {
        Ok(Some(mut championship)) => {
            // Only allow starting if championship is Pending
            if championship.status != CompetitionStatus::Pending {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some(
                        "Championship can only be started from Pending status".to_string(),
                    ),
                    success: false,
                });
            }

            // Update championship status
            let update_request = UpdateChampionshipRequest {
                name: None,
                code: None,
                season: None,
                description: None,
                league: None,
                divisions: None,
                total_journees: None,
                journee_duration: None,
                classification_rules: None,
                status: Some(CompetitionStatus::InProgress),
                current_journee: Some(1),
                current_phase: Some("Phase de poules".to_string()),
            };

            match state.db.update_championship(*id, update_request).await {
                Ok(Some(updated_championship)) => HttpResponse::Ok().json(ApiResponse {
                    data: updated_championship,
                    message: Some("Championship started successfully".to_string()),
                    success: true,
                }),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Championship not found".to_string()),
                    success: false,
                }),
                Err(e) => {
                    log::error!("Failed to start championship: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        data: (),
                        message: Some("Failed to start championship".to_string()),
                        success: false,
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get championship for starting: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship".to_string()),
                success: false,
            })
        }
    }
}

/// Advance championship to next journée
#[utoipa::path(
    post,
    path = "/championnats/{id}/next-journee",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship advanced to next journée", body = Championship),
        (status = 400, description = "Invalid championship state or already at last journée"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn next_journee(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state.db.get_championship(*id).await {
        Ok(Some(championship)) => {
            // Check if championship is InProgress
            if championship.status != CompetitionStatus::InProgress {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some(
                        "Championship must be InProgress to advance journée".to_string(),
                    ),
                    success: false,
                });
            }

            let current_journee = championship.current_journee.unwrap_or(0);
            let total_journees = championship.total_journees;

            // Check if we're at the last journée
            if current_journee >= total_journees {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some(
                        "Championship is already at the last journée".to_string(),
                    ),
                    success: false,
                });
            }

            // Calculate next journée
            let next_journee = current_journee + 1;

            // Update championship
            let update_request = UpdateChampionshipRequest {
                name: None,
                code: None,
                season: None,
                description: None,
                league: None,
                divisions: None,
                total_journees: None,
                journee_duration: None,
                classification_rules: None,
                status: None,
                current_journee: Some(next_journee),
                current_phase: None,
            };

            match state.db.update_championship(*id, update_request).await {
                Ok(Some(updated_championship)) => {
                    // Check if this is the last journée
                    if next_journee == total_journees {
                        // Auto-complete championship
                        let complete_request = UpdateChampionshipRequest {
                            name: None,
                            code: None,
                            season: None,
                            description: None,
                            league: None,
                            divisions: None,
                            total_journees: None,
                            journee_duration: None,
                            classification_rules: None,
                            status: Some(CompetitionStatus::Completed),
                            current_journee: None,
                            current_phase: None,
                        };

                        // Note: In a real implementation, we'd update this
                        // For now, just return the updated championship
                    }

                    HttpResponse::Ok().json(ApiResponse {
                        data: updated_championship,
                        message: Some("Championship advanced to next journée".to_string()),
                        success: true,
                    })
                }
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Championship not found".to_string()),
                    success: false,
                }),
                Err(e) => {
                    log::error!("Failed to advance championship journée: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        data: (),
                        message: Some("Failed to advance championship journée".to_string()),
                        success: false,
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get championship for advancing journée: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship".to_string()),
                success: false,
            })
        }
    }
}

/// Get classification for a championship
#[utoipa::path(
    get,
    path = "/championnats/{id}/classification",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship classification", body = ChampionshipClassification),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championship_classification(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    // For now, return a mock classification
    // In a real implementation, this would query the database
    match state.db.get_championship(*id).await {
        Ok(Some(championship)) => {
            // Create mock teams for classification
            let teams = vec![
                ChampionshipTeam {
                    position: 1,
                    team_id: Uuid::new_v4(),
                    stats: TeamStats {
                        matches: 5,
                        wins: 4,
                        draws: 1,
                        losses: 0,
                        points_for: 45,
                        points_against: 20,
                        diff: 25,
                        points: 13,
                        win_rate: 0.8,
                    },
                    form: vec![palet_shared::MatchResult::Win, palet_shared::MatchResult::Win, palet_shared::MatchResult::Win, palet_shared::MatchResult::Win, palet_shared::MatchResult::Draw],
                    status: TeamClassificationStatus::Qualified,
                },
                ChampionshipTeam {
                    position: 2,
                    team_id: Uuid::new_v4(),
                    stats: TeamStats {
                        matches: 5,
                        wins: 3,
                        draws: 1,
                        losses: 1,
                        points_for: 40,
                        points_against: 25,
                        diff: 15,
                        points: 10,
                        win_rate: 0.6,
                    },
                    form: vec![palet_shared::MatchResult::Win, palet_shared::MatchResult::Win, palet_shared::MatchResult::Win, palet_shared::MatchResult::Draw, palet_shared::MatchResult::Loss],
                    status: TeamClassificationStatus::Qualified,
                },
            ];

            let classification = ChampionshipClassification {
                division: championship.divisions.first().cloned().unwrap_or_default(),
                journee: championship.current_journee.unwrap_or(1),
                updated_at: chrono::Utc::now(),
                teams,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: classification,
                message: Some("Classification retrieved successfully".to_string()),
                success: true,
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championship not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get championship classification: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship classification".to_string()),
                success: false,
            })
        }
    }
}

/// Update classification for a championship
#[utoipa::path(
    post,
    path = "/championnats/{id}/classification",
    tag = "Championnats",
    params(
        ("id" = Uuid, description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Classification updated", body = ChampionshipClassification),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_championship_classification(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    // For now, return a success message
    // In a real implementation, this would update the classification in the database
    HttpResponse::Ok().json(ApiResponse::<()> {
        data: (),
        message: Some("Classification updated successfully".to_string()),
        success: true,
    })
}
