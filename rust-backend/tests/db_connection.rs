//! Database Connection Tests with Mock
//!
//! This module contains isolated tests for database connectivity using MockDatabase.
//! These tests can run without a real PostgreSQL connection.

mod mock;
use mock::{MockDatabase, MockDatabaseError};
use uuid::Uuid;

// ============================================
// CONNECTION TESTS
// ============================================

#[tokio::test]
async fn test_mock_database_connection_establishment() {
    // This test verifies that the mock database connection works
    let db = MockDatabase::new();

    // Verify the mock connection is alive
    let is_connected = db.check_connection().await;
    assert!(is_connected, "Mock database connection should be established");
}

#[tokio::test]
async fn test_mock_database_connection_failure() {
    // This test verifies that we can simulate a connection failure
    let db = MockDatabase::new();
    db.set_connected(false);

    // Verify the connection is not alive
    let is_connected = db.check_connection().await;
    assert!(!is_connected, "Mock database connection should fail when set to false");
}

#[tokio::test]
async fn test_mock_database_connection_pool() {
    // This test verifies that multiple mock database instances can be created
    let db1 = MockDatabase::new();
    let db2 = MockDatabase::new();

    let is_connected1 = db1.check_connection().await;
    let is_connected2 = db2.check_connection().await;

    assert!(is_connected1, "First mock database instance should be connected");
    assert!(is_connected2, "Second mock database instance should be connected");
}

// ============================================
// SCHEMA INITIALIZATION TESTS
// ============================================

#[tokio::test]
async fn test_mock_database_schema_initialization() {
    // This test verifies that the mock database schema can be initialized
    let db = MockDatabase::new();

    // Initialize the schema
    let result = db.initialize().await;
    assert!(result.is_ok(), "Mock database schema initialization should succeed");
}

#[tokio::test]
async fn test_mock_database_schema_initialization_failure() {
    // This test verifies that schema initialization fails when not connected
    let db = MockDatabase::new();
    db.set_connected(false);

    // Initialize the schema - should fail
    let result = db.initialize().await;
    assert!(result.is_err(), "Mock database schema initialization should fail when not connected");
    assert!(matches!(result.unwrap_err(), MockDatabaseError::ConnectionFailed));
}

#[tokio::test]
async fn test_mock_database_schema_initialization_idempotent() {
    // This test verifies that schema initialization is idempotent
    let db = MockDatabase::new();

    // Initialize the schema multiple times
    let result1 = db.initialize().await;
    let result2 = db.initialize().await;
    let result3 = db.initialize().await;

    assert!(result1.is_ok(), "First mock schema initialization should succeed");
    assert!(result2.is_ok(), "Second mock schema initialization should succeed");
    assert!(result3.is_ok(), "Third mock schema initialization should succeed");
}

// ============================================
// MIGRATION TESTS
// ============================================

#[tokio::test]
async fn test_mock_database_run_migrations() {
    // This test verifies that mock database migrations can be run
    let db = MockDatabase::new();

    // Initialize first
    db.initialize().await.expect("Schema initialization should succeed");

    // Run migrations
    let result = db.run_migrations().await;
    assert!(result.is_ok(), "Mock database migrations should run successfully");
}

#[tokio::test]
async fn test_mock_database_run_migrations_without_init() {
    // This test verifies that migrations fail without schema initialization
    let db = MockDatabase::new();

    // Run migrations without initializing - should fail
    let result = db.run_migrations().await;
    assert!(result.is_err(), "Mock database migrations should fail without schema initialization");
    assert!(matches!(result.unwrap_err(), MockDatabaseError::SchemaInitializationFailed));
}

#[tokio::test]
async fn test_mock_database_run_migrations_connection_failure() {
    // This test verifies that migrations fail when not connected
    let db = MockDatabase::new();
    db.set_connected(false);

    // Run migrations - should fail
    let result = db.run_migrations().await;
    assert!(result.is_err(), "Mock database migrations should fail when not connected");
    assert!(matches!(result.unwrap_err(), MockDatabaseError::ConnectionFailed));
}

// ============================================
// FEDERATION TESTS
// ============================================

#[tokio::test]
async fn test_mock_database_create_federation() {
    let db = MockDatabase::new();

    let federation = db
        .create_federation(
            "Test Federation".to_string(),
            "test-federation".to_string(),
            Some("Test description".to_string()),
        )
        .await
        .expect("Should create federation");

    assert_eq!(federation.name, "Test Federation");
    assert_eq!(federation.slug, "test-federation");
    assert_eq!(federation.description, Some("Test description".to_string()));
}

#[tokio::test]
async fn test_mock_database_get_federations() {
    let db = MockDatabase::new();

    // Create some federations
    db.create_federation("Fed 1".to_string(), "fed1".to_string(), None)
        .await
        .ok();
    db.create_federation("Fed 2".to_string(), "fed2".to_string(), None)
        .await
        .ok();

    let federations = db.get_federations().await.expect("Should get federations");
    assert_eq!(federations.len(), 2);
}

#[tokio::test]
async fn test_mock_database_get_federation() {
    let db = MockDatabase::new();

    // Create a federation
    let created = db
        .create_federation("Test Fed".to_string(), "test-fed".to_string(), None)
        .await
        .expect("Should create federation");

    // Get it back
    let retrieved = db
        .get_federation(created.id)
        .await
        .expect("Should get federation")
        .expect("Federation should exist");
    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.name, "Test Fed");
}

// ============================================
// CLUB TESTS
// ============================================

#[tokio::test]
async fn test_mock_database_create_club() {
    let db = MockDatabase::new();

    // First create a federation
    let federation = db
        .create_federation("Test Fed".to_string(), "test-fed".to_string(), None)
        .await
        .expect("Should create federation");

    // Now create a club
    let club_request = palet_shared::CreateClubRequest {
        name: "Test Club".to_string(),
        nickname: Some("TC".to_string()),
        code: "TC001".to_string(),
        city: "Test City".to_string(),
        postal_code: Some("12345".to_string()),
        league: "Deux-Sèvres".to_string(),
        division: "D1".to_string(),
        federation_id: federation.id,
        contact_email: Some("test@test.com".to_string()),
        contact_phone: Some("1234567890".to_string()),
        website: Some("https://test.com".to_string()),
    };

    let club = db.create_club(club_request).await.expect("Should create club");
    assert_eq!(club.name, "Test Club");
    assert_eq!(club.code, "TC001");
    assert_eq!(club.city, "Test City");
}

#[tokio::test]
async fn test_mock_database_create_club_without_federation() {
    let db = MockDatabase::new();

    // Try to create a club without a federation
    let club_request = palet_shared::CreateClubRequest {
        name: "Test Club".to_string(),
        nickname: Some("TC".to_string()),
        code: "TC001".to_string(),
        city: "Test City".to_string(),
        postal_code: Some("12345".to_string()),
        league: "Deux-Sèvres".to_string(),
        division: "D1".to_string(),
        federation_id: Uuid::new_v4(), // Non-existent federation
        contact_email: Some("test@test.com".to_string()),
        contact_phone: Some("1234567890".to_string()),
        website: Some("https://test.com".to_string()),
    };

    let result = db.create_club(club_request).await;
    assert!(result.is_err(), "Should fail to create club without federation");
    assert!(matches!(result.unwrap_err(), MockDatabaseError::FederationRequired));
}

#[tokio::test]
async fn test_mock_database_get_clubs() {
    let db = MockDatabase::new();

    // Create a federation
    let federation = db
        .create_federation("Test Fed".to_string(), "test-fed".to_string(), None)
        .await
        .expect("Should create federation");

    // Create some clubs
    let club_request = palet_shared::CreateClubRequest {
        name: "Club 1".to_string(),
        nickname: None,
        code: "C1".to_string(),
        city: "City 1".to_string(),
        postal_code: None,
        league: "Deux-Sèvres".to_string(),
        division: "D1".to_string(),
        federation_id: federation.id,
        contact_email: None,
        contact_phone: None,
        website: None,
    };

    db.create_club(club_request.clone()).await.ok();

    let club_request2 = palet_shared::CreateClubRequest {
        name: "Club 2".to_string(),
        ..club_request
    };
    db.create_club(club_request2).await.ok();

    let clubs = db.get_clubs().await.expect("Should get clubs");
    assert_eq!(clubs.len(), 2);
}

#[tokio::test]
async fn test_mock_database_get_club() {
    let db = MockDatabase::new();

    // Create a federation
    let federation = db
        .create_federation("Test Fed".to_string(), "test-fed".to_string(), None)
        .await
        .expect("Should create federation");

    // Create a club
    let club_request = palet_shared::CreateClubRequest {
        name: "Test Club".to_string(),
        nickname: None,
        code: "TC001".to_string(),
        city: "Test City".to_string(),
        postal_code: None,
        league: "Deux-Sèvres".to_string(),
        division: "D1".to_string(),
        federation_id: federation.id,
        contact_email: None,
        contact_phone: None,
        website: None,
    };

    let created_club = db.create_club(club_request).await.expect("Should create club");

    // Get it back
    let retrieved_club = db
        .get_club(created_club.id)
        .await
        .expect("Should get club")
        .expect("Club should exist");
    assert_eq!(retrieved_club.id, created_club.id);
    assert_eq!(retrieved_club.name, "Test Club");
}
