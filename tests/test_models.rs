use chrono::Utc;
use ds_event_stream_rs_sdk::model::EventStream;
use uuid::Uuid;

#[test]
fn test_event_stream_serialization() {
    let event = EventStream {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        event_source: "test".to_string(),
        event_type: "test".to_string(),
        timestamp: Utc::now(),
        created_by: "test".to_string(),
        md5_hash: "test".to_string(),
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
    let serialized = serde_json::to_string(&event).unwrap();
    assert!(!serialized.is_empty());
    let deserialized: EventStream = serde_json::from_str(&serialized).unwrap();
    assert_eq!(event.id, deserialized.id);
    assert_eq!(event.event_source, deserialized.event_source);
    assert_eq!(event.event_type, deserialized.event_type);
    assert_eq!(event.timestamp, deserialized.timestamp);
    assert_eq!(event.created_by, deserialized.created_by);
    assert_eq!(event.md5_hash, deserialized.md5_hash);
    assert_eq!(event.request_id, deserialized.request_id);
    assert_eq!(event.owner_id, deserialized.owner_id);
    assert_eq!(event.product_id, deserialized.product_id);
    assert_eq!(event.product_schema_uri, deserialized.product_schema_uri);
    assert_eq!(event.event_source_uri, deserialized.event_source_uri);
    assert_eq!(event.affected_entity_uri, deserialized.affected_entity_uri);
    assert_eq!(event.message, deserialized.message);
    assert_eq!(event.payload, deserialized.payload);
    assert_eq!(event.payload_uri, deserialized.payload_uri);
    assert_eq!(event.metadata, deserialized.metadata);
    assert_eq!(event.tags, deserialized.tags);
}

#[test]
fn test_event_stream_new() {
    let session_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let event_source = "test_source".to_string();
    let event_type = "test_type".to_string();
    let created_by = "test_user".to_string();

    let event = EventStream::new(
        session_id,
        tenant_id,
        event_source.clone(),
        event_type.clone(),
        created_by.clone(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    assert_eq!(event.session_id, session_id);
    assert_eq!(event.tenant_id, tenant_id);
    assert_eq!(event.event_source, event_source);
    assert_eq!(event.event_type, event_type);
    assert_eq!(event.created_by, created_by);
    assert!(!event.id.is_nil());
    // md5_hash is empty when payload is None
    assert!(event.md5_hash.is_empty());
}
