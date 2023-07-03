use crate::utils::error::{ProxyError, ProxyResult};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::Debug;
use std::{fs::File, io::Read, time::Duration};

const MAX_INT: u64 = u64::MAX;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;
const TB: u64 = 1024 * GB;
const PB: u64 = 1024 * TB;

const DEFAULT_CONFIG_PATH: &str = "config/proxy.toml";

// 定义 Proxy 启动时候的参数列表
#[derive(Serialize, Deserialize, Debug)]
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
    proxy_heap_placeholder: u64,

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
    session_keepalive_period: Duration,
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
    metrics_report_statsd_server: String,
    #[serde(deserialize_with = "deserialize_string_to_duration")]
    metrics_report_statsd_period: Duration,
    metrics_report_statsd_prefix: String,
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

impl Default for Config {
    fn default() -> Self {
        Config::read_from_file(DEFAULT_CONFIG_PATH).unwrap()
    }
}

impl Config {
    fn read_from_file(path: &str) -> ProxyResult<Config> {
        let mut content = String::new();
        let mut file = File::open(path).map_err(|e| ProxyError::ConfigIO(e.to_string()))?;
        file.read_to_string(&mut content)
            .map_err(|e| ProxyError::ConfigIO(e.to_string()))?;
        let config: Config =
            toml::from_str(&content).map_err(|e| ProxyError::ConfigParse(e.to_string()))?;
        Ok(config)
    }

    pub fn from_path(path: &str) -> ProxyResult<Config> {
        Config::read_from_file(path)
    }

    pub fn proxy_addr(&self) -> &str {
        &self.proxy_addr
    }

    pub fn admin_addr(&self) -> &str {
        &self.admin_addr
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn validate(&self) -> ProxyResult<()> {
        if self.proto_type.is_empty() {
            return Err(ProxyError::ConfigInvalid("proto_type".into()));
        }
        if self.proxy_addr.is_empty() {
            return Err(ProxyError::ConfigInvalid("proxy_addr".into()));
        }
        if self.admin_addr.is_empty() {
            return Err(ProxyError::ConfigInvalid("admin_addr".into()));
        }
        if !self.jodis_name.is_empty() {
            if self.jodis_addr.is_empty() {
                return Err(ProxyError::ConfigInvalid("jodis_addr".into()));
            }
            if self.jodis_timeout < Duration::from_secs(0 as u64) {
                return Err(ProxyError::ConfigInvalid("jodis_timeout".into()));
            }
        }
        if self.product_name.is_empty() {
            return Err(ProxyError::ConfigInvalid("product_name".into()));
        }
        if self.proxy_max_clients < 0 as u32 {
            return Err(ProxyError::ConfigInvalid("proxy_max_clients".into()));
        }
        if self.proxy_max_offheap_bytes < 0 as u64 || self.proxy_max_offheap_bytes > MAX_INT {
            return Err(ProxyError::ConfigInvalid("proxy_max_offheap_size".into()));
        }
        if self.proxy_heap_placeholder < 0 as u64 || self.proxy_heap_placeholder > MAX_INT {
            return Err(ProxyError::ConfigInvalid("proxy_heap_placeholder".into()));
        }
        if self.backend_ping_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("backend_ping_period".into()));
        }
        if self.backend_recv_bufsize < 0 as u64 || self.backend_recv_bufsize > MAX_INT {
            return Err(ProxyError::ConfigInvalid("backend_recv_bufsize".into()));
        }
        if self.backend_recv_timeout < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("backend_recv_timeout".into()));
        }
        if self.backend_send_bufsize < 0 as u64 || self.backend_send_bufsize > MAX_INT {
            return Err(ProxyError::ConfigInvalid("backend_send_bufsize".into()));
        }
        if self.backend_send_timeout < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("backend_send_timeout".into()));
        }
        if self.backend_max_pipeline < 0 as u32 {
            return Err(ProxyError::ConfigInvalid("backend_max_pipeline".into()));
        }
        if self.backend_primary_parallel < 0 as u32 {
            return Err(ProxyError::ConfigInvalid("backend_primary_parallel".into()));
        }
        if self.backend_replica_parallel < 0 as u32 {
            return Err(ProxyError::ConfigInvalid("backend_replica_parallel".into()));
        }
        if self.backend_keepalive_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("backend_keepalive_period".into()));
        }
        if self.backend_number_databases < 1 as u32 {
            return Err(ProxyError::ConfigInvalid("backend_number_databases".into()));
        }
        if self.session_recv_bufsize < 0 as u64 || self.session_recv_bufsize > MAX_INT {
            return Err(ProxyError::ConfigInvalid("session_recv_bufsize".into()));
        }
        if self.session_recv_timeout < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("session_recv_timeout".into()));
        }
        if self.session_send_bufsize < 0 as u64 || self.session_send_bufsize > MAX_INT {
            return Err(ProxyError::ConfigInvalid("session_send_bufsize".into()));
        }
        if self.session_send_timeout < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("session_send_timeout".into()));
        }
        if self.session_max_pipeline < 0 as u32 {
            return Err(ProxyError::ConfigInvalid("session_max_pipeline".into()));
        }
        if self.session_keepalive_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("session_keepalive_period".into()));
        }
        if self.metrics_report_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid("metrics_report_period".into()));
        }
        if self.metrics_report_influxdb_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid(
                "metrics_report_influxdb_period".into(),
            ));
        }
        if self.metrics_report_statsd_period < Duration::from_secs(0 as u64) {
            return Err(ProxyError::ConfigInvalid(
                "metrics_report_statsd_period".into(),
            ));
        }
        Ok(())
    }
}

mod tests {
    use crate::proxy::config::Config;

    #[test]
    fn test_config() {
        let path = "config/proxy.toml";
        let mut root_path = project_root::get_project_root().unwrap();
        root_path.push(path);
        let config_path = root_path.to_str().unwrap();
        let config = Config::from_path(config_path);
    }
}
