use clap::{Parser, Subcommand};
use std::time::Duration;
use std::error::Error;

mod cpu;
mod memory;
mod storage;
mod run_all;

#[derive(Parser)]
#[command(name = "System Stress Monitor")]
#[command(about = "Effectue des tests de stress CPU et mémoire", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Effectue un test de stress sur le CPU
    Cpu {
        /// Durée du test de stress CPU en secondes
        #[arg(short, long, default_value_t = 10)]
        duration: u64,
        
        /// Seuil d'utilisation du CPU en pourcentage
        #[arg(short, long, default_value_t = 80.0)]
        threshold: f32,
    },
    /// Effectue un test de stress sur la mémoire
    Memory {
        /// Durée du test de stress mémoire en secondes
        #[arg(short, long, default_value_t = 10)]
        duration: u64,
        /// Taille d'allocation mémoire en Mo
        #[arg(short, long, default_value_t = 10)]
        allocation_size: usize,
    },
    /// Endurance test for storage
    Storage {
        /// Duration of the storage endurance test in seconds
        #[arg(short, long, default_value_t = 10)]
        duration: u64,

        /// Path to the file used for the endurance test
        #[arg(short, long = "file-path")]
        file_path: String,
    },
    /// Run All stress tests in one shot
    RunAll {
        #[arg(short, long, default_value_t = 10)]
        duration: u64,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cpu { duration, threshold } => {
            if let Err(e) = cpu::stress_cpu(Duration::from_secs(*duration), *threshold) {
                eprintln!("Error during CPU stress test: {}", e);
            }
        }
        Commands::Memory { duration, allocation_size } => {
            if let Err(e) = memory::stress_memory(Duration::from_secs(*duration), *allocation_size) {
                eprintln!("Error during Memory stress test: {}", e);
            }
        }
        Commands::Storage { duration, file_path } => {
            if let Err(e) = storage::endurance_test(Duration::from_secs(*duration), file_path) {
                eprintln!("Error running endurance test: {}", e);
            }
        }
        Commands::RunAll { duration } => {
            run_all::run_all_tests(Duration::from_secs(*duration))?;
        }
    }
    
    Ok(())
}
