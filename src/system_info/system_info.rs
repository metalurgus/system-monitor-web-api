use std::time::{Duration, Instant};
use sysinfo::{Networks, System};
use serde::{Serialize, Deserialize};

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct CPU {
    pub cores: u32,
    pub usage: f32,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub ip: String,
    pub mac: String,
    pub sent: u64,
    pub received: u64,
    pub usage: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub memory: Memory,
    pub swap: Memory,
    pub cpu: CPU,
    pub networks: Vec<Network>,
}

pub trait ToSize {
    fn to_giga(self) -> f32;
    fn to_mega(self) -> f32;
    fn to_kilo(self) -> f32;
    fn auto_size(self) -> String;
    fn auto_bits(self) -> String;
}

impl ToSize for u64 {
    fn to_giga(self) -> f32 {
        (self as f32 / GB as f32 * 100.0).round() / 100.0
    }
    fn to_mega(self) -> f32 {
        (self as f32 / MB as f32 * 100.0).round() / 100.0
    }
    fn to_kilo(self) -> f32 {
        (self as f32 / KB as f32 * 100.0).round() / 100.0
    }
    fn auto_size(self) -> String {
        if self >= GB {
            format!("{} GB", self.to_giga())
        } else if self >= MB {
            format!("{} MB", self.to_mega())
        } else if self >= KB {
            format!("{} KB", self.to_kilo())
        } else {
            format!("{} B", self)
        }
    }
   fn auto_bits(self) -> String {
        if self >= GB {
            format!("{} Gbps", self.to_giga())
        } else if self >= MB {
            format!("{} Mbps", self.to_mega())
        } else if self >= KB {
            format!("{} Kbps", self.to_kilo())
        } else {
            format!("{} bps", self)
        }
    } 
}


fn calc_network_usage(received: u64, transmitted: u64, elapsed_secs: f64) -> u64 {
    let total_bits = (received + transmitted) * 8;
    if elapsed_secs > 0.0 {
        (total_bits as f64 / elapsed_secs) as u64
    } else {
        0
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let memory = Memory {
            total: sys.total_memory(),
            used: sys.used_memory(),
            free: sys.total_memory() - sys.used_memory(),
            text: format!("Memory: {}/{} free", (sys.total_memory() - sys.used_memory()).auto_size(), sys.total_memory().auto_size()),
        };

        let swap = Memory {
            total: sys.total_swap(),
            used: sys.used_swap(),
            free: sys.total_swap() - sys.used_swap(),
            text: format!("Swap: {}/{} free", (sys.total_swap() - sys.used_swap()).auto_size(), sys.total_swap().auto_size()),
        };

        let last_refresh = Instant::now();
        let mut network_interfaces = Networks::new_with_refreshed_list();

        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(Duration::from_secs(1));

        sys.refresh_all();
        network_interfaces.refresh(true);

        let cpu = CPU {
            cores: sys.cpus().len() as u32,
            usage: sys.global_cpu_usage(),
            text: format!("CPU Usage: {:.2}%", sys.global_cpu_usage()),
        };

        let elapsed = last_refresh.elapsed().as_secs_f64();
        let mut networks = Vec::new();
        for (interface_name, data) in network_interfaces.iter() {
            let is_enabled = !data.ip_networks().is_empty()
                && (data.total_received() > 0 || data.total_transmitted() > 0)
                && !data.mac_address().is_unspecified();
            if !is_enabled {
                continue;
            }

            let ip = data.ip_networks()
                .iter()
                .find(|ip| ip.addr.is_ipv4())
                .or_else(|| data.ip_networks().iter().find(|ip| ip.addr.is_ipv6()))
                .map_or_else(|| "Unknown".to_string(), |ip| ip.addr.to_string());

            let usage = calc_network_usage(data.received(), data.transmitted(), elapsed);

            networks.push(Network {
                name: interface_name.clone(),
                ip: ip.clone(),
                mac: data.mac_address().to_string(),
                sent: data.total_transmitted(),
                received: data.total_received(),
                usage,
                text: format!("Network: {}\nIP: {}\nMAC: {}\nSent: {}\nReceived: {}\nUsage: {}",
                    interface_name,
                    ip,
                    data.mac_address(),
                    data.total_transmitted().auto_size(),
                    data.total_received().auto_size(),
                    usage.auto_bits()),
            });
        }

        networks.sort_by(|a, b| a.name.cmp(&b.name));
        SystemInfo { memory, swap, cpu, networks }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("Memory: {}/{} free\n", self.memory.free.auto_size(), self.memory.total.auto_size()));
        result.push_str(&format!("Swap: {}/{} free\n", self.swap.free.auto_size(), self.swap.total.auto_size()));
        result.push_str(&format!("CPU Cores: {}\n", self.cpu.cores));
        result.push_str(&format!("CPU Usage: {:.2}%\n", self.cpu.usage));
        for network in &self.networks {
            result.push_str("-----------------------------------------------------\n");
            result.push_str(&format!("Network: {}\n", network.name));
            result.push_str(&format!("IP: {}\n", network.ip));
            result.push_str(&format!("MAC: {}\n", network.mac));
            result.push_str(&format!("Sent: {}\n", network.sent.auto_size()));
            result.push_str(&format!("Received: {}\n", network.received.auto_size()));
            result.push_str(&format!("Usage: {}\n", network.usage.auto_bits()));            
        }
        result
    }
}
