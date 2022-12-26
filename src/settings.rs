use std::fs;
use std::path::Path;

slint::include_modules!();
// PSTATE INTEL
pub const ENERGY_PERFORMANCE_PRE: &str =
    "/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference";
const ENERGY_PERFORMANCE_AVALABLE: &str =
    "/sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences";

// ORIGIN

pub const SCALING_GOVERNOR: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
const SCALING_GOVERNOR_AVALABLE: &str =
    "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors";

// AMD
pub const AMD_GPU_GOVERNOR: &str = "/sys/class/drm/card0/device/power_dpm_force_performance_level";
const AMD_GPU_GOVERNOR_AVALABLE: [&str; 8] = [
    "auto",
    "low",
    "high",
    "manual",
    "profile_standard",
    "profile_min_sclk",
    "profile_min_mclk",
    "profile_peak",
];

#[derive(Debug)]
pub struct AvailableSetting {
    pub name: String,
    pub currentselected: String,
    pub selects: Vec<String>,
    pub doc: String,
}

pub fn get_all_settings() -> Vec<AvailableSetting> {
    let mut output = vec![];
    let mut output_append = |path: &str, current: &str, name: &str, doc: &str| {
        if let (Ok(content), Ok(content_current)) = (
            fs::read_to_string(Path::new(path)),
            fs::read_to_string(current),
        ) {
            let selects: Vec<String> = content
                .trim()
                .split(' ')
                .map(|unit| unit.to_string())
                .collect();
            let currentselected = content_current.trim().to_string();
            output.push(AvailableSetting {
                name: name.to_string(),
                currentselected,
                selects,
                doc: doc.to_string(),
            });
        };
    };

    if Path::new(SCALING_GOVERNOR).exists() {
        output_append(
            SCALING_GOVERNOR_AVALABLE,
            SCALING_GOVERNOR,
            "ScalingGovernor",
            include_str!("../misc/about/kernel_cpu.md"),
        );
    }

    if Path::new(ENERGY_PERFORMANCE_PRE).exists() {
        output_append(
            ENERGY_PERFORMANCE_AVALABLE,
            ENERGY_PERFORMANCE_PRE,
            "IntelPstate",
            include_str!("../misc/about/intel_pstate.md"),
        );
    }

    if Path::new(AMD_GPU_GOVERNOR).exists() {
        if let Ok(content) = fs::read_to_string(Path::new(AMD_GPU_GOVERNOR)) {
            output.push(AvailableSetting {
                name: "AMDGPU".to_string(),
                currentselected: content.trim().to_string(),
                selects: AMD_GPU_GOVERNOR_AVALABLE
                    .map(|unit| unit.to_string())
                    .to_vec(),
                doc: include_str!("../misc/about/amd_performance.md").to_string(),
            });
        }
    }
    output
}
