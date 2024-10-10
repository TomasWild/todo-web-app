use std::net::IpAddr;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[clap(long, env = "SERVER_HOST", default_value = "0.0.0.0")]
    pub server_host: IpAddr,
    #[clap(long, env = "SERVER_PORT", default_value = "3000")]
    pub server_port: u16,
}