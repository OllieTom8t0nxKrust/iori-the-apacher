use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailoverState {
    Healthy,
    Degraded,
    FailoverActive,
    EmergencyQuarantine,
}

#[derive(Clone, Debug)]
pub struct FailoverMetrics {
    pub state: FailoverState,
    pub primary_failures: u64,
    pub last_failure_timestamp: String,
    pub active_circuit_breaker: bool,
}

pub struct CircuitBreaker {
    failure_threshold: u32,
    _failure_count: u32,
    state: RwLock<FailoverState>,
    primary_failures: RwLock<u64>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32) -> Self {
        Self {
            failure_threshold,
            _failure_count: 0,
            state: RwLock::new(FailoverState::Healthy),
            primary_failures: RwLock::new(0),
        }
    }

    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        *state = FailoverState::Healthy;
        let mut failures = self.primary_failures.write().await;
        *failures = 0;
    }

    pub async fn record_failure(&self) -> FailoverState {
        let mut failures = self.primary_failures.write().await;
        *failures += 1;
        
        let mut state = self.state.write().await;
        if *failures >= self.failure_threshold as u64 {
            *state = FailoverState::FailoverActive;
            eprintln!("Circuit breaker tripped! Transitioned to FailoverActive state after {} consecutive failures.", *failures);
        } else {
            *state = FailoverState::Degraded;
            eprintln!("Service degraded. Failure count: {}", *failures);
        }
        state.clone()
    }

    pub async fn get_state(&self) -> FailoverState {
        let state = self.state.read().await;
        state.clone()
    }

    pub async fn get_metrics(&self) -> FailoverMetrics {
        let state = self.state.read().await.clone();
        let primary_failures = *self.primary_failures.read().await;
        FailoverMetrics {
            state,
            primary_failures,
            last_failure_timestamp: chrono::Utc::now().to_rfc3339(),
            active_circuit_breaker: primary_failures >= self.failure_threshold as u64,
        }
    }
}

pub struct FailoverOrchestrator {
    primary_circuit: Arc<CircuitBreaker>,
    secondary_circuit: Arc<CircuitBreaker>,
}

impl FailoverOrchestrator {
    pub fn new(threshold: u32) -> Self {
        Self {
            primary_circuit: Arc::new(CircuitBreaker::new(threshold)),
            secondary_circuit: Arc::new(CircuitBreaker::new(threshold)),
        }
    }

    pub async fn execute_with_failover<F1, F2, Fut1, Fut2, T, E>(
        &self,
        primary_op: F1,
        secondary_op: F2,
    ) -> Result<T, String>
    where
        F1: FnOnce() -> Fut1,
        F2: FnOnce() -> Fut2,
        Fut1: std::future::Future<Output = Result<T, E>>,
        Fut2: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        // Try primary
        match primary_op().await {
            Ok(res) => {
                self.primary_circuit.record_success().await;
                Ok(res)
            }
            Err(e) => {
                let state = self.primary_circuit.record_failure().await;
                eprintln!("Primary operation failed: {:?}. State: {:?}", e, state);
                
                println!("Initiating seamless failover to secondary path...");
                match secondary_op().await {
                    Ok(res) => {
                        self.secondary_circuit.record_success().await;
                        println!("Secondary failover path succeeded successfully.");
                        Ok(res)
                    }
                    Err(sec_err) => {
                        let sec_state = self.secondary_circuit.record_failure().await;
                        eprintln!("Secondary failover path also failed: {:?}. State: {:?}", sec_err, sec_state);
                        Err(format!("Critical Failover Failure. Primary error: {:?}, Secondary error: {:?}", e, sec_err))
                    }
                }
            }
        }
    }
}
