// src/storage.rs
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

pub fn endurance_test(duration: Duration, file_path: &str) -> Result<String, String> {
    // Open or create the file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;

    // Prepare a buffer of data (e.g., 1 MB of zero bytes)
    let buffer_size = 1 * 1024 * 1024; // 1 MB
    let buffer = vec![0u8; buffer_size];

    let start_time = Instant::now();
    let total_time = duration.as_secs();

    println!("Starting storage endurance test for {} seconds...", total_time);

    while start_time.elapsed() < duration {
        // Write the buffer to the file
        if let Err(e) = file.write_all(&buffer) {
            let error_msg = format!("Error writing to file '{}': {}", file_path, e);
            eprintln!("{}", error_msg);
            return Err(error_msg);
        }
    }

    println!("Storage endurance test completed.");
    Ok("Endurance test completed successfully.".to_string())
}
