use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct Resp<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespCompany {
    pub name: String,
    pub zh_name: String,
    pub en_name: String,
    pub domain: String,
    pub enable_self_signed: bool,
    pub self_signed_cert: String,
    pub enable_public_key: bool,
    pub public_key: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespLoginMethod {
    pub login_enable_ldap: bool,
    pub login_enable: bool,
    pub login_orders: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespTpsLoginMethod {
    pub alias: String,
    pub login_url: String,
    pub token: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespCorplinkLoginMethod {
    pub mfa: bool,
    pub auth: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespLogin {
    #[serde(default)]
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespOtp {
    pub url: String,
    pub code: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnMtuDetectConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub interval: i32,
    #[serde(default)]
    pub timeout_max_count: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnAutoDisconnectIdleConfig {
    #[serde(default)]
    pub switch: bool,
    #[serde(default)]
    pub idle_time: i32,
    #[serde(default)]
    pub threshold: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnAutoDisconnectOvertimeConfig {
    #[serde(default)]
    pub switch: bool,
    #[serde(default)]
    pub max_connect_time: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnAutoDisconnectIntranetDetectConfig {
    #[serde(default)]
    pub switch: bool,
    #[serde(default)]
    pub action: i32,
    #[serde(default)]
    pub exemption_switch: bool,
    #[serde(default)]
    pub exemption: i32,
    #[serde(default)]
    pub intranet_switch: bool,
    #[serde(default)]
    pub detect_ip: String,
    #[serde(default)]
    pub detect_port: i32,
    #[serde(default)]
    pub timeout: i32,
    #[serde(default)]
    pub interval: i32,
    #[serde(default)]
    pub ssid_switch: bool,
    pub ssids: Option<Vec<String>>,
    #[serde(default)]
    pub ip_switch: bool,
    pub ips: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnAutoDisconnectSetting {
    pub idle_config: Option<RespVpnAutoDisconnectIdleConfig>,
    pub overtime_config: Option<RespVpnAutoDisconnectOvertimeConfig>,
    pub intranet_detect_config: Option<RespVpnAutoDisconnectIntranetDetectConfig>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnProtocolDetectConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub udp2tcp_timeout_count: i32,
    #[serde(default)]
    pub tcp2udp_available_count: i32,
    #[serde(default)]
    pub refresh_timeout_count: i32,
    #[serde(default)]
    pub bad_network_count: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnIpDelayRoutingPolicy {
    #[serde(default)]
    pub is_operator: bool,
    #[serde(default)]
    pub policy_type: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnInfo {
    #[serde(default)]
    pub id: i32,
    #[serde(default, rename = "type")]
    pub r#type: String,
    // server name; prefer this over en_name (which may be empty on some deployments)
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub en_name: String,
    #[serde(default)]
    pub tag_name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub internal_ip: String,
    #[serde(default)]
    pub domain_name: String,
    pub backup_ips: Option<Vec<String>>,
    pub api_port: u16,
    pub vpn_port: u16,
    // 1 for tcp, 2 for udp, we only support udp for now
    pub protocol_mode: i32,
    pub timeout: i32,
    #[serde(default)]
    pub mode: i32,
    #[serde(default)]
    pub reconnect: bool,
    #[serde(default)]
    pub dedicated: bool,
    #[serde(default)]
    pub connect_internal_ip: bool,
    #[serde(default)]
    pub virtual_dns: bool,
    #[serde(default)]
    pub exclude: bool,
    pub mtu_detect_config: Option<RespVpnMtuDetectConfig>,
    pub export: Option<serde_json::Value>,
    pub auto_disconnect_setting: Option<RespVpnAutoDisconnectSetting>,
    pub protocol_detect_config: Option<RespVpnProtocolDetectConfig>,
    pub ip_delay_routing_policy: Option<RespVpnIpDelayRoutingPolicy>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespCentralDns {
    #[serde(default)]
    pub cpe_id: i32,
    #[serde(default)]
    pub tenant_id: String,
    #[serde(default)]
    pub vip: String,
    pub dns: Option<serde_json::Value>,
    #[serde(default)]
    pub bloom_num: i64,
    #[serde(default)]
    pub bloom_fp: i64,
    #[serde(default)]
    pub second_bloom: String,
    #[serde(default)]
    pub third_bloom: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespVpnGlobalExclude {
    pub ips: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespWebFilter {
    #[serde(default)]
    pub file_url: String,
    #[serde(default)]
    pub file_hash: String,
    pub category_ids: Option<Vec<i32>>,
    pub ports: Option<Vec<u16>>,
    pub dips: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespForwardProxy {
    pub domains: Option<Vec<String>>,
    pub web_filter: Option<RespWebFilter>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespForwardProxyExclude {
    pub ips: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub cert_exclude_domains: Option<Vec<String>>,
    pub blocked_category_ids: Option<Vec<i32>>,
    pub blocked_domains: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespSwgTunnelMeta {
    pub ports: Vec<u16>,
    pub dips: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespWgExtraInfo {
    pub vpn_mtu: u32,
    pub vpn_dns: String,
    pub vpn_dns_backup: String,
    pub vpn_dns_domain_split: Option<Vec<String>>,
    pub vpn_route_full: Vec<String>,
    pub vpn_route_split: Vec<String>,
    pub v6_route_full: Option<Vec<String>>,
    pub v6_route_split: Option<Vec<String>>,
    pub vpn_dynamic_domain_route_split: Option<HashMap<String, Vec<String>>>,
    pub vpn_wildcard_dynamic_domain_route_split: Option<HashMap<String, Vec<String>>>,
    pub suffix_wildcard_dynamic_domain_route_split: Option<HashMap<String, Vec<String>>>,
    pub dynamic_domain: Option<serde_json::Value>,
    pub search_domain: Option<Vec<String>>,
    pub central_dns: Option<RespCentralDns>,
    pub ip_nats: Option<serde_json::Value>,
    pub exclude_static_domain: Option<Vec<String>>,
    pub exclude_route: Option<Vec<String>>,
    pub exclude_dynamic_domain: Option<HashMap<String, serde_json::Value>>,
    pub vpn_global_exclude: Option<RespVpnGlobalExclude>,
    pub forward_proxy: Option<RespForwardProxy>,
    pub forward_proxy_exclude: Option<RespForwardProxyExclude>,
    pub swg_tunnel_meta: Option<RespSwgTunnelMeta>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespWgInfo {
    pub ip: String,
    pub ipv6: String,
    pub ip_mask: String,
    pub public_key: String,
    pub setting: RespWgExtraInfo,
    pub mode: u32,
    pub protocol_version: Option<String>,
    pub sign_token: Option<String>,
}
