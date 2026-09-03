use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApacheToolCategory {
    StreamingAndMessaging,
    DistributedCompute,
    DistributedStorage,
    StreamProcessing,
    WorkflowOrchestration,
    DataWarehouse,
    NoSQLBigTable,
    DistributedNoSQL,
    DataIntegration,
    EnterpriseSearch,
    BusinessIntelligence,
    PubSubMessaging,
    TableFormat,
    ServletContainer,
    WebServer,
    InMemoryData,
    IntegrationFramework,
    RealtimeAnalytics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApacheToolConfig {
    pub name: String,
    pub category: ApacheToolCategory,
    pub status: String,
    pub throughput_metric: String,
    pub secure_pqc_enabled: bool,
}

pub struct ApacheEcosystemManager {
    tools: Vec<ApacheToolConfig>,
}

impl ApacheEcosystemManager {
    pub fn new() -> Self {
        let tools = vec![
            ApacheToolConfig {
                name: "Apache Kafka".into(),
                category: ApacheToolCategory::StreamingAndMessaging,
                status: "Active Cluster".into(),
                throughput_metric: "1.2M msgs/sec".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Spark".into(),
                category: ApacheToolCategory::DistributedCompute,
                status: "Worker Pool Ready".into(),
                throughput_metric: "64 Cores Active".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Hadoop HDFS".into(),
                category: ApacheToolCategory::DistributedStorage,
                status: "Replication 3x".into(),
                throughput_metric: "4.8 PB Capacity".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Flink".into(),
                category: ApacheToolCategory::StreamProcessing,
                status: "Low Latency Engine".into(),
                throughput_metric: "sub-ms latency".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Airflow".into(),
                category: ApacheToolCategory::WorkflowOrchestration,
                status: "DAG Scheduler Running".into(),
                throughput_metric: "142 Active DAGs".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Hive".into(),
                category: ApacheToolCategory::DataWarehouse,
                status: "Metastore Online".into(),
                throughput_metric: "SQL LLAP Ready".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache HBase".into(),
                category: ApacheToolCategory::NoSQLBigTable,
                status: "RegionServers Online".into(),
                throughput_metric: "99.99% Availability".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Cassandra".into(),
                category: ApacheToolCategory::DistributedNoSQL,
                status: "Ring Synchronized".into(),
                throughput_metric: "Multi-DC Replication".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache NiFi".into(),
                category: ApacheToolCategory::DataIntegration,
                status: "Flows Processing".into(),
                throughput_metric: "8.4 GB/min".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Solr".into(),
                category: ApacheToolCategory::EnterpriseSearch,
                status: "Cloud Shards Active".into(),
                throughput_metric: "Full-text Indexing".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Superset".into(),
                category: ApacheToolCategory::BusinessIntelligence,
                status: "Dashboards Serving".into(),
                throughput_metric: "Visual Analytics".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Pulsar".into(),
                category: ApacheToolCategory::PubSubMessaging,
                status: "BookKeeper Connected".into(),
                throughput_metric: "Georeplicated".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Iceberg".into(),
                category: ApacheToolCategory::TableFormat,
                status: "Catalog Synchronized".into(),
                throughput_metric: "ACID Lakehouse".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Tomcat".into(),
                category: ApacheToolCategory::ServletContainer,
                status: "Catalina Engine Running".into(),
                throughput_metric: "HTTP/2 Connector".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache HTTP Server".into(),
                category: ApacheToolCategory::WebServer,
                status: "Reverse Proxy Active".into(),
                throughput_metric: "mod_proxy_balancer".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Arrow".into(),
                category: ApacheToolCategory::InMemoryData,
                status: "Zero-Copy Shared".into(),
                throughput_metric: "Columnar Vector".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Camel".into(),
                category: ApacheToolCategory::IntegrationFramework,
                status: "Routes Active".into(),
                throughput_metric: "EIP Pattern Engine".into(),
                secure_pqc_enabled: true,
            },
            ApacheToolConfig {
                name: "Apache Druid".into(),
                category: ApacheToolCategory::RealtimeAnalytics,
                status: "Historical Nodes Online".into(),
                throughput_metric: "Sub-second OLAP".into(),
                secure_pqc_enabled: true,
            },
        ];
        Self { tools }
    }

    pub fn list_tools(&self) -> &[ApacheToolConfig] {
        &self.tools
    }

    pub fn get_tool(&self, name: &str) -> Option<&ApacheToolConfig> {
        self.tools.iter().find(|t| t.name.to_lowercase().contains(&name.to_lowercase()))
    }

    pub fn verify_pqc_compliance(&self) -> bool {
        self.tools.iter().all(|t| t.secure_pqc_enabled)
    }
}
