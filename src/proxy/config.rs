use std::{path::Path, time::Duration};

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::error::config::ConfigError;
use crate::error::Result;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;
const TB: u64 = 1024 * GB;
const PB: u64 = 1024 * TB;

/// configuration for session
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SessionConfig {
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub recv_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub recv_timeout: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub send_bufsize: u64,
    pub auth: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub send_timeout: Duration,
    pub max_pipeline: u32,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub keepalive_timeout: Duration,
    pub break_on_failure: bool,
}

/// configuration for proxy
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub enum ProxyProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "tcp4")]
    Tcp4,
    #[serde(rename = "tcp6")]
    Tcp6,
    #[serde(rename = "unix")]
    Unix,
    #[serde(rename = "unix_packet")]
    UnixPacket,
}

impl Default for ProxyProtocol {
    fn default() -> Self {
        ProxyProtocol::Tcp
    }
}

/// configuration for proxy
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ProxyConfig {
    pub protocol_type: ProxyProtocol,
    pub addr: String,
    pub admin_addr: String,
    pub host_proxy: String,
    pub host_admin: String,
    pub product_name: String,
    pub product_auth: String,
    pub data_center: String,
    pub max_clients: u32,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub max_offheap_bytes: u64,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub heap_place_holder: u64,
}

/// configuration for metrics
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct MetricsConfig {
    pub report_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub report_period: Duration,
    pub report_influxdb_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub report_influxdb_period: Duration,
    pub report_influxdb_username: String,
    pub report_influxdb_password: String,
    pub report_influxdb_database: String,
    pub report_stats_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub report_stats_period: Duration,
    pub report_stats_prefix: String,
}

/// configuration for backend
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct BackendConfig {
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub ping_period: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub recv_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub recv_timeout: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    pub send_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub send_timeout: Duration,
    pub max_pipeline: u32,
    pub primary_only: bool,
    pub primary_parallel: u32,
    pub replica_parallel: u32,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    pub keepalive_period: Duration,
    pub number_databases: u32,
}

/// all config
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub backend: BackendConfig,
    pub proxy: ProxyConfig,
    pub session: SessionConfig,
    pub metrics: MetricsConfig,
}

fn deserialize_string_to_size<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let (size_num, unit) = parse_string_to_num_and_unit(&s);
    match unit {
        "kb" => Ok(size_num * KB),
        "mb" => Ok(size_num * MB),
        "gb" => Ok(size_num * GB),
        "tb" => Ok(size_num * TB),
        "pb" => Ok(size_num * PB),
        &_ => Err(de::Error::missing_field("size")),
    }
}

fn deserialize_string_to_duration<'de, D>(
    deserializer: D,
) -> std::result::Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let (time_num, unit) = parse_string_to_num_and_unit(&s);
    match unit {
        "m" => Ok(Duration::from_secs(time_num * 60)),
        "s" => Ok(Duration::from_secs(time_num)),
        "ms" => Ok(Duration::from_millis(time_num)),
        "us" => Ok(Duration::from_micros(time_num)),
        "ns" => Ok(Duration::from_nanos(time_num)),
        &_ => Err(de::Error::missing_field("duration")),
    }
}

fn parse_string_to_num_and_unit(str: &str) -> (u64, &str) {
    let mut num_length = 0;
    let mut num_str = String::new();
    for c in str.chars() {
        match c {
            '0'..='9' => {
                num_str.push(c);
                num_length += 1;
            }
            _ => {}
        }
    }
    let num = num_str.parse::<u64>().unwrap();
    let unit = &str[num_length..];
    (num, unit)
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content).map_err(ConfigError::ParseToml)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config() {
        let mut config_path = project_root::get_project_root().unwrap();
        config_path.push("config/proxy.toml");
        let config = Config::from_path(config_path).unwrap();
        assert_eq!(config.proxy.addr, "127.0.0.1:19000");
        assert_eq!(config.backend.recv_bufsize, 128 * 1024);
    }
}
