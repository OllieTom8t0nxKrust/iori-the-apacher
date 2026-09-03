use iori_the_apacher::domain::failover::{CircuitBreaker, FailoverOrchestrator, FailoverState};
use iori_the_apacher::domain::telemetry::TelemetryMetrics;

#[tokio::test]
async fn test_circuit_breaker_transitions() {
    let cb = CircuitBreaker::new(2);
    assert_eq!(cb.get_state().await, FailoverState::Healthy);

    let state1 = cb.record_failure().await;
    assert_eq!(state1, FailoverState::Degraded);

    let state2 = cb.record_failure().await;
    assert_eq!(state2, FailoverState::FailoverActive);

    cb.record_success().await;
    assert_eq!(cb.get_state().await, FailoverState::Healthy);
}

#[tokio::test]
async fn test_failover_orchestrator_success_and_fallback() {
    let orchestrator = FailoverOrchestrator::new(2);

    // Primary succeeds
    let res = orchestrator.execute_with_failover(
        || async { Ok::<&str, &str>("primary_ok") },
        || async { Ok::<&str, &str>("secondary_ok") },
    ).await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "primary_ok");

    // Primary fails, secondary succeeds
    let res_failover = orchestrator.execute_with_failover(
        || async { Err::<&str, &str>("primary_down") },
        || async { Ok::<&str, &str>("secondary_fallback_ok") },
    ).await;
    assert!(res_failover.is_ok());
    assert_eq!(res_failover.unwrap(), "secondary_fallback_ok");
}

#[test]
fn test_telemetry_metrics_collection_and_prometheus() {
    let metrics = TelemetryMetrics::collect();
    assert!(metrics.uptime_seconds > 0);
    assert!(metrics.active_apache_tools.len() >= 10);
    assert!(metrics.observability_plugins.len() >= 5);

    let prom = metrics.to_prometheus();
    assert!(prom.contains("iori_uptime_seconds"));
    assert!(prom.contains("iori_active_tunnels"));
    assert!(prom.contains("iori_total_requests"));
}
