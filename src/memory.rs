use std::time::Duration;
use sysinfo::{System, SystemExt};

pub fn stress_memory(duration: Duration, allocation_size: usize) -> Result<String, String> {
    let mut system = System::new_all();
    let start_time = std::time::Instant::now();

    println!("Lancement du test de stress mémoire pendant {:?} secondes...", duration);

    // Boucle d'allocation de mémoire
    let mut memory_load = Vec::new();

    while start_time.elapsed() < duration {
        // Attempt to allocate the specified amount of memory
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            memory_load.push(vec![0u8; allocation_size])
        })) {
            Ok(_) => {}
            Err(_) => return Err("Memory allocation failed.".to_string()),
        }

        // Refresh and display memory info
        system.refresh_memory();
        println!(
            "Mémoire totale : {} Mo, Mémoire utilisée : {} Mo",
            system.total_memory() / 1024,
            system.used_memory() / 1024
        );

        std::thread::sleep(Duration::from_secs(1));
    }

    println!("Test de stress mémoire terminé.");
    Ok("Test de stress mémoire réussi.".to_string())
}

