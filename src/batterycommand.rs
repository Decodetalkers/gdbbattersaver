use crate::config::{AMD_GPU_GOVERNOR, ENERGY_PERFORMANCE_PRE, SCALING_GOVERNOR};
use anyhow::{anyhow, Result};
use std::io::Write;
use std::process::{Command, Stdio};

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
