use std::net::Ipv4Addr;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Configuration {
    #[envconfig(from = "PODINFORS_BIND_IP", default = "127.0.0.1")]
    pub bind_ip: Ipv4Addr,
    #[envconfig(from = "PODINFORS_BIND_PORT", default = "20000")]
    pub bind_port: u16,
}
