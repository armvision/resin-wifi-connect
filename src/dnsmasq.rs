use std::process::{Child, Command};

use network_manager::Device;

use config::Config;

pub fn start_dnsmasq(config: &Config, device: &Device) -> Result<Child, String> {
    let args = [
        &format!("--address=/#/{}", config.gateway),
        &format!("--dhcp-range={}", config.dhcp_range),
        &format!("--dhcp-option=option:router,{}", config.gateway),
        &format!("--interface={}", device.interface()),
        "--keep-in-foreground",
        "--bind-interfaces",
        "--except-interface=lo",
        "--conf-file",
        "--no-hosts",
    ];

    Command::new("dnsmasq")
        .args(&args)
        .spawn()
        .map_err(|_| "Error spawning dnsmasq".to_string())
}
