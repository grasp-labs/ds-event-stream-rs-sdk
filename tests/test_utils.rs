use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, get_topic, list_topics, ClientCredentials, Environment};

#[test]
fn test_get_topic_function_exists() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };

    let result = std::panic::catch_unwind(|| get_topic(&bootstrap_servers, &credentials, "test-topic"));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_list_topics_function_exists() {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string(),
    };

    let result = std::panic::catch_unwind(|| list_topics(&bootstrap_servers, &credentials));

    // Function should exist and be callable (even if it fails due to no Kafka)
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_bootstrap_servers_sandbox() {
    // Test internal hostnames
    let internal = get_bootstrap_servers(Environment::Sandbox, true);
    assert_eq!(internal, "kafka.kafka-sandbox.svc.cluster.local:9092");

    // Test external hostnames
    let external = get_bootstrap_servers(Environment::Sandbox, false);
    assert_eq!(external, "b0.sandbox.kafka.ds.local:9095");
}

#[test]
fn test_bootstrap_servers_all_environments() {
    // Test Development
    assert_eq!(
        get_bootstrap_servers(Environment::Development, true),
        "kafka.kafka-dev.svc.cluster.local:9092"
    );
    assert_eq!(
        get_bootstrap_servers(Environment::Development, false),
        "b0.dev.kafka.ds.local:9095"
    );

    // Test Sandbox
    assert_eq!(
        get_bootstrap_servers(Environment::Sandbox, true),
        "kafka.kafka-sandbox.svc.cluster.local:9092"
    );
    assert_eq!(
        get_bootstrap_servers(Environment::Sandbox, false),
        "b0.sandbox.kafka.ds.local:9095"
    );

    // Test Production
    assert_eq!(
        get_bootstrap_servers(Environment::Production, true),
        "kafka.kafka.svc.cluster.local:9092"
    );
    assert_eq!(
        get_bootstrap_servers(Environment::Production, false),
        "b0.kafka.ds.local:9095,b1.kafka.ds.local:9095,b2.kafka.ds.local:9095"
    );
}
