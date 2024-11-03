pub struct TemperatureMonitor;

impl TemperatureMonitor {
    pub fn new() -> Self {
        TemperatureMonitor
    }

    pub fn display_temperature(&self) {
        // Valeur factice
        println!("Current CPU Temperature: {} °C", 70);
    }
}
