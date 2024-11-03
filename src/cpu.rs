use rayon::prelude::*;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt};

pub fn stress_cpu(duration: Duration, threshold: f32) -> Result<String, String> {
    let mut system = System::new_all();
    let start_time = Instant::now();
    let end_time = start_time + duration;

    let cpu_count = system.cpus().len();

    // Simulate CPU load
    (0..cpu_count).into_par_iter().for_each(|_| {
        let mut load = 0;
        while Instant::now() < end_time {
            load += 1;
            if load % 1_000_000 == 0 {
                std::thread::yield_now();
            }
        }
    });

    std::thread::sleep(duration);
    system.refresh_cpu();

    let cpus = system.cpus();
    let mut alert_triggered = false;

    for (i, cpu) in cpus.iter().enumerate() {
        let usage = cpu.cpu_usage();
        if usage > threshold {
            println!("Alert: CPU {} is using more than {:.2}%!", i, threshold);
            alert_triggered = true;
        }
        println!("CPU {}: usage: {:.2}%", i, usage);
    }

    if alert_triggered {
        Err("CPU usage exceeded threshold".to_string())
    } else {
        Ok("CPU stress test completed successfully".to_string())
    }
}
