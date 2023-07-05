use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fs::File, io::Read, path::Path, time::Duration};

use crate::utils::error::Result as PikaProxyResult;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;
const TB: u64 = 1024 * GB;
const PB: u64 = 1024 * TB;

// 定义 Proxy 启动时候的参数列表
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    proto_type: String,
    proxy_addr: String,
    admin_addr: String,

    host_proxy: String,
    host_admin: String,

    jodis_name: String,
    jodis_addr: String,
    jodis_auth: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    jodis_timeout: Duration,
    jodis_compatible: bool,

    product_name: String,
    product_auth: String,
    serssion_auth: String,

    proxy_data_center: String,
    proxy_max_clients: u32,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    proxy_max_offheap_bytes: u64,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    proxy_heap_place_holder: u64,

    #[serde(deserialize_with = "deserialize_string_to_duration")]
    backend_ping_period: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    backend_recv_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    backend_recv_timeout: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    backend_send_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    backend_send_timeout: Duration,
    backend_max_pipeline: u32,
    backend_primary_only: bool,
    backend_primary_parallel: u32,
    backend_replica_parallel: u32,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    backend_keepalive_period: Duration,
    backend_number_databases: u32,

    #[serde(deserialize_with = "deserialize_string_to_size")]
    session_recv_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    session_recv_timeout: Duration,
    #[serde(deserialize_with = "deserialize_string_to_size")]
    session_send_bufsize: u64,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    session_send_timeout: Duration,
    session_max_pipeline: u32,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    session_keepalive_timeout: Duration,
    session_break_on_failure: bool,

    metrics_report_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    metrics_report_period: Duration,
    metrics_report_influxdb_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    metrics_report_influxdb_period: Duration,
    metrics_report_influxdb_username: String,
    metrics_report_influxdb_password: String,
    metrics_report_influxdb_database: String,
    metrics_report_stats_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    metrics_report_stats_period: Duration,
    metrics_report_stats_prefix: String,
}

fn deserialize_string_to_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
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

fn deserialize_string_to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
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
    pub fn from_path<P: AsRef<Path>>(path: P) -> PikaProxyResult<Self> {
        let mut f = File::open(path)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn proxy_addr(&self) -> &str {
        &self.proxy_addr
    }

    pub fn admin_addr(&self) -> &str {
        &self.admin_addr
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config() {
        let path = "config/proxy.toml";
        let mut root_path = project_root::get_project_root().unwrap();
        root_path.push(path);
        let config = Config::from_path(root_path).unwrap();
        assert_eq!(config.admin_addr, "0.0.0.0:11080");
        assert_eq!(config.proxy_addr, "127.0.0.1:19000")
    }
}
