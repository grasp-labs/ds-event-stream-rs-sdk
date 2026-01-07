// Note: These tests verify producer creation behavior
// Producer creation is lazy - it doesn't connect to Kafka until you send a message

use chrono::Utc;
use ds_event_stream_rs_sdk::error::SDKError;
use ds_event_stream_rs_sdk::model::topics::Topic;
use ds_event_stream_rs_sdk::model::EventStream;
use ds_event_stream_rs_sdk::producer::error::ProducerError;
use ds_event_stream_rs_sdk::producer::KafkaProducer;
use ds_event_stream_rs_sdk::utils::ClientCredentials;
use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment};
use uuid::Uuid;

#[test]
fn test_producer_new() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials);
    // Producer creation should succeed because it's lazy - no actual connection is made
    assert!(producer.is_ok());
}

#[test]
fn test_producer_empty_credentials() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "".to_string(),
        password: "".to_string(),
    };
    let result = KafkaProducer::default(&bootstrap_servers, &credentials);
    // Empty credentials should fail because Kafka validates SASL credentials during construction
    assert!(result.is_err());
    assert!(matches!(result, Err(SDKError::Producer(_))));
}

#[tokio::test]
async fn test_send_event_with_empty_event_source() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials).unwrap();

    let event = EventStream {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        event_source: "".to_string(), // Empty event_source should fail validation
        event_type: "test_event".to_string(),
        timestamp: Utc::now(),
        created_by: "test_user".to_string(),
        md5_hash: "".to_string(),
        request_id: None,
        owner_id: None,
        product_id: None,
        product_schema_uri: None,
        event_source_uri: None,
        affected_entity_uri: None,
        message: None,
        payload: None,
        payload_uri: None,
        context: None,
        context_uri: None,
        metadata: None,
        tags: None,
    };

    let result = producer
        .send_event(&Topic::DsPipelineJobRequested, "test_key", &event, None)
        .await;

    // Should fail with validation error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, SDKError::Producer(ProducerError::ValidationError(_))));

    // Check error message
    if let SDKError::Producer(ProducerError::ValidationError(msg)) = err {
        assert_eq!(msg, "event_source cannot be empty");
    }
}

#[tokio::test]
async fn test_send_event_with_valid_event_source() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials).unwrap();

    let event = EventStream {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        event_source: "valid_source".to_string(), // Valid event_source
        event_type: "test_event".to_string(),
        timestamp: Utc::now(),
        created_by: "test_user".to_string(),
        md5_hash: "".to_string(),
        request_id: None,
        owner_id: None,
        product_id: None,
        product_schema_uri: None,
        event_source_uri: None,
        affected_entity_uri: None,
        message: None,
        payload: None,
        payload_uri: None,
        context: None,
        context_uri: None,
        metadata: None,
        tags: None,
    };

    let result = producer
        .send_event(&Topic::DsPipelineJobRequested, "test_key", &event, None)
        .await;

    // Should fail because we can't connect to Kafka, but NOT with a validation error
    // It will be a Kafka connection error, not a validation error
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Make sure it's NOT a validation error - validation passed
    assert!(!matches!(err, SDKError::Producer(ProducerError::ValidationError(_))));
}

#[test]
fn test_serialize_message_with_valid_event() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials).unwrap();

    let event = EventStream {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        event_source: "test_source".to_string(),
        event_type: "test_event".to_string(),
        timestamp: Utc::now(),
        created_by: "test_user".to_string(),
        md5_hash: "test_hash".to_string(),
        request_id: Some(Uuid::new_v4()),
        owner_id: Some("test_owner".to_string()),
        product_id: None,
        product_schema_uri: None,
        event_source_uri: None,
        affected_entity_uri: None,
        message: Some("test message".to_string()),
        payload: None,
        payload_uri: None,
        context: None,
        context_uri: None,
        metadata: None,
        tags: None,
    };

    let result = producer.serialize_message(&event);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert!(!serialized.is_empty());

    // Verify we can deserialize it back
    let deserialized: EventStream = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(deserialized.event_source, event.event_source);
    assert_eq!(deserialized.event_type, event.event_type);
}
