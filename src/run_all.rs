use std::{collections::HashMap, fs::File, io::Write, time::Duration};
use serde::{Serialize, Deserialize};
use std::error::Error;

use crate::{cpu, memory, storage};

#[derive(Serialize, Deserialize)]
pub struct CommandReport {
    command: String,
    output: String,
    status: String,
}

pub fn run_all_tests(duration: Duration) -> Result<(), Box<dyn Error>> {
    let mut report: HashMap<String, CommandReport> = HashMap::new();

    let cpu_result = cpu::stress_cpu(Duration::from_secs(10), 80.0);
    let status = if cpu_result.is_ok() { "Success".to_string() } else { "Failed".to_string() };
    let output = cpu_result.map(|_| "CPU test completed successfully".to_string()).unwrap_or_else(|e| e.to_string());
    
    report.insert(
        "CPU Test".to_string(),
        CommandReport {
            command: format!("CPU stress for {} seconds", duration.as_secs()),
            output,
            status,
        },
    );

    let memory_result = memory::stress_memory(Duration::from_secs(10), 1024 * 1024);
    let status = if memory_result.is_ok() { "Success".to_string() } else { "Failed".to_string() };
    let output = memory_result.map(|_| "Memory test completed successfully".to_string()).unwrap_or_else(|e| e.to_string());
    report.insert(
        "Memory Test".to_string(),
        CommandReport {
            command: format!("Memory stress for {} seconds", duration.as_secs()),
            output,
            status,
        },
    );

    let storage_result = storage::endurance_test(Duration::from_secs(10), "test_file.bin");
    let status = if storage_result.is_ok() { "Success".to_string() } else { "Failed".to_string() };
    let output = storage_result.map(|_| "Storage test completed successfully".to_string()).unwrap_or_else(|e| e.to_string());
    report.insert(
        "Storage Test".to_string(),
        CommandReport {
            command: format!("Storage endurance for {} seconds", duration.as_secs()),
            output,
            status,
        },
    );

    // Write the report to a YAML file
    let yaml = serde_yaml::to_string(&report)?;
    let mut file = File::create("report.yaml")?;
    file.write_all(yaml.as_bytes())?;

    println!("All tests completed. Report generated at report.yaml");
    Ok(())
}
