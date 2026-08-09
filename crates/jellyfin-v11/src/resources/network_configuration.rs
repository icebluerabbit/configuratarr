use core_macros::resource;

/// `/System/Configuration/network` — HTTP(S), ports, and network access.
#[resource(
    sync = singleton,
    case = pascal,
    read = get("/System/Configuration/network"),
    update = post("/System/Configuration/network"),
)]
pub struct NetworkConfiguration {
    /// Base Url
    pub base_url: Option<String>,
    /// Enable Https
    pub enable_https: bool,
    /// Require Https
    pub require_https: bool,
    /// Certificate Path
    pub certificate_path: Option<String>,
    /// Certificate Password
    pub certificate_password: Option<String>,
    /// Internal Http Port
    pub internal_http_port: i32,
    /// Internal Https Port
    pub internal_https_port: i32,
    /// Public Http Port
    pub public_http_port: i32,
    /// Public Https Port
    pub public_https_port: i32,
    /// Auto Discovery
    pub auto_discovery: bool,
    /// Enable U Pn P
    pub enable_u_pn_p: bool,
    /// Enable I Pv 4
    pub enable_i_pv4: bool,
    /// Enable I Pv 6
    pub enable_i_pv6: bool,
    /// Enable Remote Access
    pub enable_remote_access: bool,
    /// Local Network Subnets
    pub local_network_subnets: Vec<String>,
    /// Local Network Addresses
    pub local_network_addresses: Vec<String>,
    /// Known Proxies
    pub known_proxies: Vec<String>,
    /// Ignore Virtual Interfaces
    pub ignore_virtual_interfaces: bool,
    /// Virtual Interface Names
    pub virtual_interface_names: Vec<String>,
    /// Enable Published Server Uri By Request
    pub enable_published_server_uri_by_request: bool,
    /// Published Server Uri By Subnet
    pub published_server_uri_by_subnet: Vec<String>,
    /// Remote IP Filter
    pub remote_i_p_filter: Vec<String>,
    /// Is Remote IP Filter Blacklist
    pub is_remote_i_p_filter_blacklist: bool,
}
