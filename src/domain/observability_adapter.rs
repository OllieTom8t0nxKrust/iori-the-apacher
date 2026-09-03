use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FOSSPluginConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub plugged_in: bool,
}

pub struct ObservabilityHubAdapter {
    plugins: Vec<FOSSPluginConfig>,
}

impl ObservabilityHubAdapter {
    pub fn new() -> Self {
        let plugins = vec![
            FOSSPluginConfig {
                id: "grafana".into(),
                name: "Grafana Dashboards".into(),
                description: "Enterprise metric visualization & alerts".into(),
                category: "Visualization".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "prometheus".into(),
                name: "Prometheus Exporter".into(),
                description: "Time-series metrics & scraping engine".into(),
                category: "Metrics".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "jaeger".into(),
                name: "Jaeger APM Tracing".into(),
                description: "Distributed transaction tracing & latency analysis".into(),
                category: "Tracing".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "opentelemetry".into(),
                name: "OpenTelemetry Collector".into(),
                description: "Vendor-agnostic telemetry ingestion pipeline".into(),
                category: "Telemetry".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "elk".into(),
                name: "ELK Stack / Kibana".into(),
                description: "Elasticsearch log aggregation & search".into(),
                category: "Logging".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "zabbix".into(),
                name: "Zabbix Enterprise Monitor".into(),
                description: "Infrastructure & network health monitoring".into(),
                category: "Monitoring".into(),
                plugged_in: false,
            },
            FOSSPluginConfig {
                id: "netdata".into(),
                name: "Netdata Realtime Agent".into(),
                description: "High-resolution node performance monitoring".into(),
                category: "Metrics".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "loki".into(),
                name: "Grafana Loki Logs".into(),
                description: "Cost-effective log aggregation system".into(),
                category: "Logging".into(),
                plugged_in: true,
            },
            FOSSPluginConfig {
                id: "victoriametrics".into(),
                name: "VictoriaMetrics Cluster".into(),
                description: "Scalable long-term time-series database".into(),
                category: "Storage".into(),
                plugged_in: false,
            },
            FOSSPluginConfig {
                id: "fluentd".into(),
                name: "Fluentd Log Collector".into(),
                description: "Unified logging layer for container streams".into(),
                category: "Pipeline".into(),
                plugged_in: true,
            },
        ];
        Self { plugins }
    }

    pub fn list_plugins(&self) -> &[FOSSPluginConfig] {
        &self.plugins
    }

    pub fn toggle_plugin(&mut self, id: &str) -> Option<bool> {
        if let Some(p) = self.plugins.iter_mut().find(|p| p.id == id) {
            p.plugged_in = !p.plugged_in;
            Some(p.plugged_in)
        } else {
            None
        }
    }

    pub fn active_count(&self) -> usize {
        self.plugins.iter().filter(|p| p.plugged_in).count()
    }
}
