use std::{process, time::UNIX_EPOCH};

use caps::CapSet;
use serde::Serialize;
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Serialize)]
pub struct RuntimeInfo {
    pid: u32,
    hostname: Option<String>,
    cores: Option<usize>,
    uptime: u64,
    system_time: u128,
    caps_ambient_set: String,
    caps_bounding_set: String,
    caps_effective_set: String,
    caps_inherited_set: String,
    caps_permitted_set: String,
}

pub struct RuntimeInfoService {
    system: System,
}

impl RuntimeInfoService {
    pub fn get_runtime_info(&mut self) -> RuntimeInfo {
        self.system.refresh_specifics(RefreshKind::everything());

        let hostname = self.system.host_name();
        let cores = self.system.physical_core_count();
        let uptime = self.system.uptime();
        let system_time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Could not determine current time.")
            .as_millis();

        // FIXME: error handling
        let caps_ambient_set = format!(
            "{:?}",
            caps::read(None, CapSet::Ambient).expect("Could not get ambient capabilities set.")
        );
        let caps_bounding_set = format!(
            "{:?}",
            caps::read(None, CapSet::Bounding).expect("Could not get bounding capabilities set.")
        );
        let caps_effective_set = format!(
            "{:?}",
            caps::read(None, CapSet::Effective).expect("Could not get effective capabilities set.")
        );
        let caps_inherited_set = format!(
            "{:?}",
            caps::read(None, CapSet::Inheritable)
                .expect("Could not get inherited capabilities set.")
        );
        let caps_permitted_set = format!(
            "{:?}",
            caps::read(None, CapSet::Permitted).expect("Could not get permitted capabilities set.")
        );

        RuntimeInfo {
            pid: process::id(),
            hostname,
            cores,
            uptime,
            system_time,
            caps_ambient_set,
            caps_bounding_set,
            caps_effective_set,
            caps_inherited_set,
            caps_permitted_set,
        }
    }
}

impl Default for RuntimeInfoService {
    fn default() -> Self {
        let system = System::default();
        Self { system }
    }
}
