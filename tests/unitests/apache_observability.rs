use iori_the_apacher::domain::apache_tools::ApacheEcosystemManager;
use iori_the_apacher::domain::observability_adapter::ObservabilityHubAdapter;

#[test]
fn test_apache_ecosystem_manager_comprehensive() {
    let manager = ApacheEcosystemManager::new();
    let tools = manager.list_tools();
    assert!(tools.len() >= 18);
    assert!(manager.verify_pqc_compliance());

    let kafka = manager.get_tool("Kafka");
    assert!(kafka.is_some());
    assert_eq!(kafka.unwrap().name, "Apache Kafka");

    let spark = manager.get_tool("Spark");
    assert!(spark.is_some());
    assert_eq!(spark.unwrap().name, "Apache Spark");
}

#[test]
fn test_observability_hub_adapter_comprehensive() {
    let mut hub = ObservabilityHubAdapter::new();
    let plugins = hub.list_plugins();
    assert!(plugins.len() >= 10);
    assert!(hub.active_count() >= 8);

    let toggled = hub.toggle_plugin("grafana");
    assert_eq!(toggled, Some(false));
    assert!(hub.active_count() < 8);

    let toggled_back = hub.toggle_plugin("grafana");
    assert_eq!(toggled_back, Some(true));
}
