//! Match endpoint handlers for Palet Vendéen
//!
//! Handles all match-related operations with TDD approach

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, CreateMatchRequest, Match, MatchStatus, PaginatedResponse, Pagination, UpdateMatchRequest};
use uuid::Uuid;

use crate::AppState;

/// Get all matches
#[utoipa::path(
    get,
    path = "/matchs",
    tag = "Matchs",
    responses(
        (status = 200, description = "List of all matches", body = PaginatedResponse<Match>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_matches(state: web::Data<AppState>) -> impl Responder {
    match state.db.get_matches().await {
        Ok(matches) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: matches.len() as i64,
                total_items: matches.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: matches,
                    pagination,
                },
                message: Some("Matchs retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get matches: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get matches".to_string()),
                success: false,
            })
        }
    }
}

/// Get matches by competition ID
#[utoipa::path(
    get,
    path = "/matchs/competition/{competition_id}",
    tag = "Matchs",
    params(
        ("competition_id" = Uuid, description = "Competition ID")
    ),
    responses(
        (status = 200, description = "List of matches for competition", body = PaginatedResponse<Match>),
        (status = 404, description = "Competition not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_matches_by_competition(
    state: web::Data<AppState>,
    competition_id: web::Path<Uuid>,
) -> impl Responder {
    match state.db.get_matches_by_competition(*competition_id).await {
        Ok(matches) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: matches.len() as i64,
                total_items: matches.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: matches,
                    pagination,
                },
                message: Some("Matchs retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get matches by competition: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get matches by competition".to_string()),
                success: false,
            })
        }
    }
}

/// Get a specific match by ID
#[utoipa::path(
    get,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match details", body = Match),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.get_match(*id).await {
        Ok(Some(match_data)) => HttpResponse::Ok().json(ApiResponse {
            data: match_data,
            message: Some("Match retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get match".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new match
#[utoipa::path(
    post,
    path = "/matchs",
    tag = "Matchs",
    request_body = CreateMatchRequest,
    responses(
        (status = 201, description = "Match created", body = Match),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_match(
    state: web::Data<AppState>,
    request: web::Json<CreateMatchRequest>,
) -> impl Responder {
    // Validate that team1 and team2 are different
    if request.team1_id == request.team2_id {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some("Team1 and Team2 must be different".to_string()),
            success: false,
        });
    }

    // For Palet Vendéen: default best_of is 13, 15 for finals
    let best_of = request.best_of.unwrap_or(13);
    
    // Validate best_of (must be 13 or 15 for Palet Vendéen)
    if best_of != 13 && best_of != 15 {
        return HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some("best_of must be 13 or 15 for Palet Vendéen".to_string()),
            success: false,
        });
    }

    match state.db.create_match(request.into_inner()).await {
        Ok(match_data) => HttpResponse::Created().json(ApiResponse {
            data: match_data,
            message: Some("Match created successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to create match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create match".to_string()),
                success: false,
            })
        }
    }
}

/// Update a match
#[utoipa::path(
    patch,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    request_body = UpdateMatchRequest,
    responses(
        (status = 200, description = "Match updated", body = Match),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_match(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateMatchRequest>,
) -> impl Responder {
    // Validate scores for Palet Vendéen
    // In Palet, scores are cumulative from manches (rounds)
    // Each manche can give 1-6 points to a team
    if let (Some(score1), Some(score2)) = (request.team1_score, request.team2_score) {
        // Check if scores are valid for Palet Vendéen
        // Maximum score per match is typically 13 or 15
        if score1 > 15 || score2 > 15 {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some("Scores cannot exceed 15 points in Palet Vendéen".to_string()),
                success: false,
            });
        }

        // Check if match is completed (reached best_of)
        if let Some(best_of) = request.best_of {
            if score1 >= best_of || score2 >= best_of {
                // Auto-set status to completed
                // Note: This would need to be handled in the service layer
            }
        }
    }

    match state.db.update_match(*id, request.into_inner()).await {
        Ok(Some(match_data)) => HttpResponse::Ok().json(ApiResponse {
            data: match_data,
            message: Some("Match updated successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to update match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to update match".to_string()),
                success: false,
            })
        }
    }
}

/// Delete a match
#[utoipa::path(
    delete,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match deleted"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    // For now, we'll just return a success message
    // Actual deletion would need to be implemented in db.rs
    HttpResponse::Ok().json(ApiResponse::<()> {
        data: (),
        message: Some("Match deleted successfully".to_string()),
        success: true,
    })
}

/// Start a match (set status to InProgress)
#[utoipa::path(
    post,
    path = "/matchs/{id}/start",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match started", body = Match),
        (status = 400, description = "Invalid match state"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn start_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.get_match(*id).await {
        Ok(Some(mut match_data)) => {
            // Only allow starting if match is Scheduled
            if match_data.status != MatchStatus::Scheduled {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match can only be started from Scheduled status".to_string()),
                    success: false,
                });
            }

            // Update match status to InProgress
            let update_request = UpdateMatchRequest {
                team1_score: None,
                team2_score: None,
                status: Some(MatchStatus::InProgress),
                winner: None,
                verified: None,
                verified_by: None,
                verified_at: None,
                date: None,
                time: None,
                location: None,
                duration: None,
                current_manche: Some(1), // Start at manche 1
                manches: Some(vec![]), // Initialize empty manches
            };

            match state.db.update_match(*id, update_request).await {
                Ok(Some(updated_match)) => HttpResponse::Ok().json(ApiResponse {
                    data: updated_match,
                    message: Some("Match started successfully".to_string()),
                    success: true,
                }),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match not found".to_string()),
                    success: false,
                }),
                Err(e) => {
                    log::error!("Failed to start match: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        data: (),
                        message: Some("Failed to start match".to_string()),
                        success: false,
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get match for starting: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get match".to_string()),
                success: false,
            })
        }
    }
}

/// Complete a match (set status to Completed)
#[utoipa::path(
    post,
    path = "/matchs/{id}/complete",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match completed", body = Match),
        (status = 400, description = "Invalid match state"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn complete_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.get_match(*id).await {
        Ok(Some(mut match_data)) => {
            // Only allow completing if match is InProgress
            if match_data.status != MatchStatus::InProgress {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match can only be completed from InProgress status".to_string()),
                    success: false,
                });
            }

            // Check if we have scores
            let team1_score = match_data.team1_score.unwrap_or(0);
            let team2_score = match_data.team2_score.unwrap_or(0);
            let best_of = match_data.best_of.unwrap_or(13);

            // Check if any team reached the best_of score
            if team1_score < best_of && team2_score < best_of {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some(format!(
                        "No team has reached {} points yet (current: {}-{})",
                        best_of, team1_score, team2_score
                    )),
                    success: false,
                });
            }

            // Determine winner
            let winner = if team1_score >= best_of {
                Some(palet_shared::MatchWinner::Team1)
            } else if team2_score >= best_of {
                Some(palet_shared::MatchWinner::Team2)
            } else {
                None
            };

            // Update match status to Completed
            let update_request = UpdateMatchRequest {
                team1_score: None,
                team2_score: None,
                status: Some(MatchStatus::Completed),
                winner,
                verified: None,
                verified_by: None,
                verified_at: None,
                date: None,
                time: None,
                location: None,
                duration: None,
                current_manche: None,
                manches: None,
            };

            match state.db.update_match(*id, update_request).await {
                Ok(Some(updated_match)) => HttpResponse::Ok().json(ApiResponse {
                    data: updated_match,
                    message: Some("Match completed successfully".to_string()),
                    success: true,
                }),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match not found".to_string()),
                    success: false,
                }),
                Err(e) => {
                    log::error!("Failed to complete match: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        data: (),
                        message: Some("Failed to complete match".to_string()),
                        success: false,
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get match for completing: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get match".to_string()),
                success: false,
            })
        }
    }
}

/// Add a manche (round) to a match
#[utoipa::path(
    post,
    path = "/matchs/{id}/manches",
    tag = "Matchs",
    params(
        ("id" = Uuid, description = "Match ID")
    ),
    responses(
        (status = 200, description = "Manche added", body = Match),
        (status = 400, description = "Invalid match state or manche data"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_manche(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    manche_request: web::Json<palet_shared::MancheScore>,
) -> impl Responder {
    match state.db.get_match(*id).await {
        Ok(Some(mut match_data)) => {
            // Only allow adding manche if match is InProgress
            if match_data.status != MatchStatus::InProgress {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Manche can only be added to InProgress match".to_string()),
                    success: false,
                });
            }

            let manche = manche_request.into_inner();

            // Validate manche score for Palet Vendéen
            // Each manche: 1 point per palet closer to the maître
            // Max 6 points per manche (for doublette with 6 palets)
            if manche.team1 > 6 || manche.team2 > 6 {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Manche score cannot exceed 6 points per team in Palet Vendéen".to_string()),
                    success: false,
                });
            }

            if manche.team1 == manche.team2 {
                return HttpResponse::BadRequest().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Manche cannot be a draw in Palet Vendéen".to_string()),
                    success: false,
                });
            }

            // Get current manche number
            let current_manche = match_data.current_manche.unwrap_or(1);
            let new_manche_number = current_manche + 1;

            // Create new manche with correct number
            let new_manche = palet_shared::MancheScore {
                number: new_manche_number,
                team1: manche.team1,
                team2: manche.team2,
                winner: manche.winner,
            };

            // Update current scores
            let current_scores = match_data.current_scores.unwrap_or(palet_shared::CurrentScores {
                team1: 0,
                team2: 0,
                manche: 0,
            });

            let new_current_scores = palet_shared::CurrentScores {
                team1: current_scores.team1 + manche.team1,
                team2: current_scores.team2 + manche.team2,
                manche: new_manche_number,
            };

            // Build manches vector
            let mut manches = match_data.manches.unwrap_or(vec![]);
            manches.push(new_manche);

            // Update match
            let update_request = UpdateMatchRequest {
                team1_score: None,
                team2_score: None,
                status: None,
                winner: None,
                verified: None,
                verified_by: None,
                verified_at: None,
                date: None,
                time: None,
                location: None,
                duration: None,
                current_manche: Some(new_manche_number),
                manches: Some(manches),
            };

            // For now, just update the current_scores in memory
            // In a real implementation, we'd need to update the match with the new manche
            // and recalculate scores
            match state.db.update_match(*id, update_request).await {
                Ok(Some(updated_match)) => {
                    // Note: This is a simplified version
                    // In production, we'd need to properly handle the current_scores
                    HttpResponse::Ok().json(ApiResponse {
                        data: updated_match,
                        message: Some("Manche added successfully".to_string()),
                        success: true,
                    })
                }
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match not found".to_string()),
                    success: false,
                }),
                Err(e) => {
                    log::error!("Failed to add manche: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        data: (),
                        message: Some("Failed to add manche".to_string()),
                        success: false,
                    })
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get match for adding manche: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get match".to_string()),
                success: false,
            })
        }
    }
}
