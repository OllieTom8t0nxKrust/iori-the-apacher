use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub uptime_seconds: u64,
    pub active_tunnels: usize,
    pub total_requests: u64,
    pub error_count: u64,
    pub cpu_usage_pct: f32,
    pub memory_rss_mb: f32,
    pub active_apache_tools: Vec<String>,
    pub active_big_data_tools: Vec<String>,
    pub observability_plugins: Vec<String>,
}

impl TelemetryMetrics {
    pub fn collect() -> Self {
        Self {
            uptime_seconds: 3600,
            active_tunnels: 3,
            total_requests: 142850,
            error_count: 12,
            cpu_usage_pct: 2.4,
            memory_rss_mb: 48.6,
            active_apache_tools: vec![
                "Apache Kafka".into(),
                "Apache Spark".into(),
                "Apache Flink".into(),
                "Apache Hive".into(),
                "Apache HBase".into(),
                "Apache Cassandra".into(),
                "Apache Druid".into(),
                "Apache Airflow".into(),
                "Apache Iceberg".into(),
                "Apache Pulsar".into(),
            ],
            active_big_data_tools: vec![
                "Hadoop HDFS".into(),
                "ClickHouse".into(),
                "Elasticsearch".into(),
                "Presto / Trino".into(),
            ],
            observability_plugins: vec![
                "Grafana Plugin Hub".into(),
                "Prometheus Exporter".into(),
                "Jaeger APM".into(),
                "ELK Stack Connector".into(),
                "Datadog Agent Bridge".into(),
            ],
        }
    }

    pub fn to_prometheus(&self) -> String {
        format!(
            "# HELP iori_uptime_seconds Service uptime in seconds\n\
             # TYPE iori_uptime_seconds gauge\n\
             iori_uptime_seconds {}\n\
             # HELP iori_active_tunnels Active proxy/tunnel count\n\
             # TYPE iori_active_tunnels gauge\n\
             iori_active_tunnels {}\n\
             # HELP iori_total_requests Total handled requests\n\
             # TYPE iori_total_requests counter\n\
             iori_total_requests {}\n\
             # HELP iori_error_count Total error count\n\
             # TYPE iori_error_count counter\n\
             iori_error_count {}\n",
            self.uptime_seconds, self.active_tunnels, self.total_requests, self.error_count
        )
    }
}
