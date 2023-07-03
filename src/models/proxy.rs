use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    token: String,
    start_time: String,
    admin_addr: String,
    proto_type: String,
    proxy_addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    jodis_path: Option<String>,
    product_name: String,
    pid: i32,
    pwd: String,
    sys: String,
    hostname: String,
    datacenter: String,
}

impl Default for Proxy {
    fn default() -> Self {
        Proxy {
            id: None,
            token: String::new(),
            start_time: String::new(),
            admin_addr: String::new(),
            proto_type: String::new(),
            proxy_addr: String::new(),
            jodis_path: None,
            product_name: String::new(),
            pid: 0,
            pwd: String::new(),
            sys: String::new(),
            hostname: String::new(),
            datacenter: String::new(),
        }
    }
}

impl Proxy {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
