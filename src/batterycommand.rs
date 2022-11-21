use crate::config::{AMD_GPU_GOVERNOR, ENERGY_PERFORMANCE_PRE, SCALING_GOVERNOR};
use anyhow::{anyhow, Result};
use std::io::Write;
use std::process::{Command, Stdio};

// TODO: FREQUEQ
#[allow(dead_code)]
const CPUFREQ: &str = "/sys/devices/system/cpu/cpufreq";
#[allow(dead_code)]
const ENERGY: &str = "energy_performance_preference";
#[allow(dead_code)]
const SCALLING: &str = "scaling_governor";

pub fn set_battery(name: &str, tochange: &str) -> Result<()> {
    let path = match name {
        "IntelPstate" => ENERGY_PERFORMANCE_PRE,
        "AMDGPU" => AMD_GPU_GOVERNOR,
        "ScalingGovernor" => SCALING_GOVERNOR,
        _ => return Err(anyhow!("MisMatch, Cannot find Battery Performance")),
    };
    let mut child = Command::new("pkexec")
        .stdin(Stdio::piped())
        .arg("tee")
        .arg("-a")
        .arg(path)
        .spawn()?;
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(tochange.as_bytes())?;
    child.wait()?;
    Ok(())
}

// TODO: Battery Groups
#[allow(dead_code)]
pub fn set_battery_v2(name: &str, _tochange: &str) -> Result<()> {
    match name {
        "IntelPstate" => {
            if let Ok(paths) = glob::glob(CPUFREQ) {
                for path in paths.flatten() {
                    let nextpath = path.join(CPUFREQ);
                    if nextpath.exists() {}
                }
            }
            Ok(())
        }
        "AMDGPU" => Ok(()),
        "ScalingGovernor" => {
            if let Ok(paths) = glob::glob(CPUFREQ) {
                for path in paths.flatten() {
                    let nextpath = path.join(SCALLING);
                    if nextpath.exists() {}
                }
            }
            Ok(())
        }
        _ => Err(anyhow!("MisMatch, Cannot find Battery Performance"))
    }
}
