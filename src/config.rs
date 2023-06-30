use anyhow::Context;
use std::collections::{HashMap, HashSet};
use sysinfo::SystemExt;

struct Configuration {
    compilers: HashMap<String, (String, String)>,
    git_tokens: HashSet<String>,
    cpu_flags: Vec<String>,
    cpu_name: String,
    os_name: String,
    os_version: String,
    mac_address: String,
    logical_cores: usize,
    physical_cores: usize,
    ram_total_mb: usize,
    machine_name: String,
    machine_id: String,
    secret_token: String,
    syzygy_max: u32,

    concurrency: u32,
    sockets: u32,
    client_version: String,
}

impl Configuration {
    fn new() -> Result<Self, anyhow::Error> {
        #![allow(clippy::many_single_char_names)]
        let mac_address = mac_address::get_mac_address()
            .with_context(|| "Failed to get MAC address")?
            .with_context(|| "Got empty MAC address")?;
        let [a, b, c, d, e, f] = mac_address.bytes();
        let mac_address = [0, 0, a, b, c, d, e, f];
        let mac_address = format!("{:016x}", u64::from_be_bytes(mac_address));

        let sys = sysinfo::System::new_all();

        let mut out = Self {
            compilers: HashMap::new(),
            git_tokens: HashSet::new(),
            cpu_flags: Vec::new(),
            cpu_name: String::new(),
            os_name: std::env::consts::OS.to_string(),
            os_version: String::new(),
            mac_address,
            logical_cores: num_cpus::get(),
            physical_cores: num_cpus::get_physical(),
            ram_total_mb: (sys.total_memory() / 1024 / 1024) as usize,
            machine_name: "None".to_string(),
            machine_id: "None".to_string(),
            secret_token: "None".to_string(),
            syzygy_max: 2,

            concurrency: 1,
            sockets: 1,
            client_version: "0.0.0".to_string(),
        };

        Ok(out)
    }
}
