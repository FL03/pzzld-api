/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::collect_configurations;
use config::{Config, ConfigError, Environment};
use core::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ServerConfig {
    pub addr: ServerAddr,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ServerAddr {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            addr: ServerAddr::default(),
        }
    }

    pub fn builder() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .add_source(Environment::default().separator("_").prefix("NETWORK"))
            .add_source(collect_configurations("**/*.config.*", false))
            .build()?
            .try_deserialize()
    }

    pub const fn addr(&self) -> &ServerAddr {
        &self.addr
    }

    pub fn addr_mut(&mut self) -> &mut ServerAddr {
        &mut self.addr
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        self.addr().addr()
    }

    pub fn set_addr(&mut self, addr: ServerAddr) {
        self.addr = addr;
    }

    pub fn set_server_host(&mut self, host: impl ToString) {
        self.addr_mut().set_host(host);
    }

    pub fn set_server_port(&mut self, port: u16) {
        self.addr_mut().set_port(port);
    }

    pub fn with_server(self, server: ServerAddr) -> Self {
        Self { addr: server }
    }

    pub fn with_server_host(self, host: impl ToString) -> Self {
        Self {
            addr: self.addr.with_host(host),
            ..self
        }
    }

    pub fn with_server_port(self, port: u16) -> Self {
        Self {
            addr: self.addr.with_port(port),
            ..self
        }
    }
}

impl ServerAddr {
    pub fn new(host: impl ToString, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn builder() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("host", "127.0.0.1")?
            .set_default("port", 8080)?
            .add_source(Environment::default().separator("_").prefix("SERVER"))
            .add_source(collect_configurations("**/*.config.*", false))
            .build()?
            .try_deserialize()
    }

    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }

    pub async fn bind(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(self.addr()).await
    }

    pub fn ip_addr(&self) -> IpAddr {
        self.addr().ip()
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }

    pub const fn port(&self) -> u16 {
        self.port
    }

    pub fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }

    pub fn set_host(&mut self, host: impl ToString) {
        self.host = host.to_string();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn with_host(self, host: impl ToString) -> Self {
        Self {
            host: host.to_string(),
            ..self
        }
    }

    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }
}

/*
 ************* Implementations *************
*/
impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}
unsafe impl core::marker::Send for ServerConfig {}

unsafe impl core::marker::Sync for ServerConfig {}

mod impl_server {
    use super::*;

    impl Default for ServerAddr {
        fn default() -> Self {
            Self::new("0.0.0.0".to_string(), 8080)
        }
    }

    impl core::fmt::Display for ServerAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}:{}", self.host, self.port)
        }
    }

    unsafe impl core::marker::Send for ServerAddr {}

    unsafe impl core::marker::Sync for ServerAddr {}

    impl From<SocketAddr> for ServerAddr {
        fn from(addr: SocketAddr) -> Self {
            Self::new(addr.ip().to_string(), addr.port())
        }
    }

    impl From<ServerAddr> for SocketAddr {
        fn from(config: ServerAddr) -> Self {
            config.addr()
        }
    }
}
