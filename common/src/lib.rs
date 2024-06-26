// FIXME:  drop this crate
// - sharing same structs between server and daemon is not a good idea, especially if we want to
// support versioned API and allow older daemons to communicate with newer server
// - section configuration should be property of section library
use pipe::config::{Config as DynamicPipeConfig, Value as DynamicPipeValue};
use section::SectionError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Top level configuration object
// todo: should this be only on the client?
// todo: disallow clone
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub node: Node,
    pub server: Server,
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub destinations: Vec<Destination>,
}

/// Client-side server config
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub endpoint: String,
}

/// Generic node config
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    // FIXME: that's property of control plane
    pub display_name: String,
    // FIXME: that's property of control plane
    pub unique_id: String,
    pub storage_path: String,
    // FIXME: why auth_token is here and not in Server section?
    pub auth_token: String,
}

/// Internally-tagged type of a source needs to match the variant name
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    Sqlite_Connector(SqliteSourceConfig),
    Kafka(KafkaConfig),
    Snowflake(SnowflakeSourceConfig),
    Sqlite_Physical_Replication(SqlitePhysicalReplicationSourceConfig),
    Hello_World(HelloWorldSourceConfig),
    Excel_Connector(ExcelConfig),
    Postgres_Connector(PostgresConnectorConfig),
    // TODO: either we need to add another enum for transformers, or merge these two into "sections" and make the section itself know it's ability to source, destination, or transform
    Tagging_Transformer(TaggingTransformerConfig),
    Typecast_Transformer(TypecastTransformerConfig),
    Mysql_Connector(MysqlConnectorSourceConfig),
    File(FileSourceConfig),
}

/// Internally-tagged type of a source needs to match the variant name
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Destination {
    Sqlite_Connector(SqliteDestinationConfig),
    Snowflake(SnowflakeDestinationConfig),
    Sqlite_Physical_Replication(SqlitePhysicalReplicationDestinationConfig),
    Hello_World(HelloWorldDestinationConfig),
    Kafka(KafkaDestinationConfig),
    Postgres_Connector(PostgresConnectorDestinationConfig),
    Mysql_Connector(MysqlConnectorDestinationConfig),
    File(FileDestinationConfig),
}

// Shared between all source definitions
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonAttrs {
    // pub r#type: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SqliteSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub path: String,
    pub origin: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SqliteDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub path: String,
    pub truncate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostgresConnectorConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub url: String,
    pub origin: String,
    pub query: String,
    pub poll_interval: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExcelConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub path: String,
    pub sheets: String,
    pub strict: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KafkaConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    // comma-separated
    pub brokers: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SnowflakeSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub username: String,
    pub password: String,
    pub role: String,
    pub account_identifier: String,
    pub warehouse: String,
    pub database: String,
    pub schema: String,
    pub query: String,
    pub delay: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SnowflakeDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub username: String,
    pub password: String,
    pub role: String,
    pub account_identifier: String,
    pub warehouse: String,
    pub database: String,
    pub schema: String,
    pub truncate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SqlitePhysicalReplicationSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub journal_path: String,
    // database path
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HelloWorldSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub message: String,
    pub interval_milis: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TaggingTransformerConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub column: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TypecastTransformerConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub column: String,
    pub target_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HelloWorldDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KafkaDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub brokers: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SqlitePhysicalReplicationDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub journal_path: String,
    pub database_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostgresConnectorDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub url: String,
    pub schema: String,
    pub truncate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MysqlConnectorSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub url: String,
    pub origin: String,
    pub query: String,
    pub poll_interval: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileSourceConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MysqlConnectorDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub url: String,
    pub truncate: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileDestinationConfig {
    #[serde(flatten)]
    pub common_attrs: CommonAttrs,
    pub path: String,
}

// requests and responses
// todo: move to a module
// FIXME: don't share daemon & server internals.
// FIXME: redo provisioning
#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionDaemonRequest {
    // FIXME: unique_id/display_name properties of control plane
    pub unique_id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionDaemonResponse {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueTokenRequest {
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueTokenResponse {
    pub id: String,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PipeConfigs {
    pub configs: Vec<PipeConfig>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PipeConfig {
    /// Scheduler needs to maintain pipe processes:
    /// - start new pipes
    /// - restart pipes on configuration update
    /// - stop pipes, when pipe was removed from configuration list
    ///
    /// To do that - each pipe config needs to be uniquely identified, so here is i64 integer to
    /// help with that. Signed due to Sqlite backed storage
    #[serde(default)]
    // FIXME: try_from
    #[sqlx(try_from = "i32")]
    pub id: u64,

    /// # Example of config
    /// ```json
    /// {"configs": [
    ///     {
    ///         "id":1,
    ///         "pipe": [
    ///             {
    ///                 "name": "sqlite",
    ///                 "path": "/tmp/test.sqlite",
    ///                 "query": "select * from test"
    ///             },
    ///             {
    ///                 "endpoint": "http://localhost:8080/ingestion",
    ///                 "name": "mycelial_server",
    ///                 "token": "mycelial_server_token"
    ///             }
    ///         ]
    /// }]}
    /// ```
    // FIXME: renaming
    #[sqlx(rename = "raw_config")]
    pub pipe: serde_json::Value,
    // FIXME: default_id, try_from cast
    #[sqlx(try_from = "i64")]
    #[serde(default = "default_id")]
    pub workspace_id: u64,
}

// FIXME:
fn default_id() -> u64 {
    1
}

impl TryInto<DynamicPipeConfig> for PipeConfig {
    type Error = SectionError;

    fn try_into(self) -> Result<DynamicPipeConfig, Self::Error> {
        let value: DynamicPipeValue = self.pipe.try_into()?;
        DynamicPipeConfig::try_from(value)
    }
}
