use crate::config::HELPER_PATH;
use crate::settings::AMD_GPU_GOVERNOR;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;
// TODO: FREQUEQ
#[allow(dead_code)]
const CPUFREQ: &str = "/sys/devices/system/cpu/cpufreq/*";
#[allow(dead_code)]
const ENERGY: &str = "energy_performance_preference";
#[allow(dead_code)]
const SCALLING: &str = "scaling_governor";

// TODO: Battery Groups

pub fn set_battery(name: &str, tochange: &str) -> Result<()> {
    match name {
        "IntelPstate" => {
            if let Ok(paths) = glob::glob(CPUFREQ) {
                let paths = paths
                    .flatten()
                    .map(|path| path.join(ENERGY))
                    .filter(|path| path.exists())
                    .map(|path| path.to_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                Command::new("pkexec")
                    .arg(HELPER_PATH)
                    .arg(tochange)
                    .args(paths)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
        "AMDGPU" => {
            if Path::new(AMD_GPU_GOVERNOR).exists() {
                Command::new("pkexec")
                    .arg(HELPER_PATH)
                    .arg(tochange)
                    .arg(AMD_GPU_GOVERNOR)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
        "ScalingGovernor" => {
            if let Ok(paths) = glob::glob(CPUFREQ) {
                let paths = paths
                    .flatten()
                    .map(|path| path.join(SCALLING))
                    .filter(|path| path.exists())
                    .map(|path| path.to_str().unwrap().to_string())
                    .collect::<Vec<String>>();
                Command::new("pkexec")
                    .arg(HELPER_PATH)
                    .arg(tochange)
                    .args(paths)
                    .spawn()?
                    .wait()?;
            }
            Ok(())
        }
        _ => Err(anyhow!("MisMatch, Cannot find Battery Performance")),
    }
}
