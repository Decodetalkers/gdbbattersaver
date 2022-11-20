use crate::config::{AMD_GPU_GOVERNOR, ENERGY_PERFORMANCE_PRE, SCALING_GOVERNOR};
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};
pub fn set_battery(name: &str, tochange: &str) -> Result<(), Box<dyn Error>> {
    let path = match name {
        "IntelPstate" => ENERGY_PERFORMANCE_PRE,
        "AMDGPU" => AMD_GPU_GOVERNOR,
        "ScalingGovernor" => SCALING_GOVERNOR,
        _ => return Ok(()),
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
