use serde::{Deserialize, Serialize};
use std::{fs, io};

const HISTORY_FILE: &str = "conversion.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub original_unit: String,
    pub target_unit: String,
    pub original_value: f64,
    pub target_value: f64,
}

pub fn load_history() -> Result<Vec<HistoryEntry>, io::Error> {
    match fs::read_to_string(HISTORY_FILE) {
        Ok(data) => {
            serde_json::from_str(&data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse JSON: {}", e)))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub fn save_history(history: &[HistoryEntry]) -> Result<(), io::Error> {
    let json_data = serde_json::to_string_pretty(history)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to serialize history: {}", e)))?;
    
    fs::write(HISTORY_FILE, json_data)
}

pub fn add_and_save_entry(entry: HistoryEntry) -> Result<(), io::Error> {
    let mut history = load_history()?;
    
    history.push(entry);
    
    save_history(&history)
}

/// Prints the formatted history list to the console.
pub fn display_history() -> Result<(), io::Error> {
    let history = load_history()?;
    
    if history.is_empty() {
        println!("Conversion History is empty.");
        return Ok(());
    }

    println!("Conversion History:");
    for (i, entry) in history.iter().enumerate() {
        // Format the output using the stored fields
        println!(
            "{}. {:.4} {} = {:.4} {}", 
            i + 1, 
            entry.original_value, 
            entry.original_unit, 
            entry.target_value, 
            entry.target_unit
        );
    }
    
    Ok(())
}