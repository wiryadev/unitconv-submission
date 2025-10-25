use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use crate::unit::{AnyUnit, LengthUnit, TempUnit, UnitDimension};

// Implement FromStr so `clap` can parse the input strings into the AnyUnit wrapper
impl FromStr for AnyUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as Temperature
        if let Ok(temp_unit) = TempUnit::from_str(s, true) {
            return Ok(AnyUnit::Temperature(temp_unit));
        }
        // Try parsing as Length
        if let Ok(length_unit) = LengthUnit::from_str(s, true) {
            return Ok(AnyUnit::Length(length_unit));
        }

        // If neither matches
        Err(format!("Invalid unit: {}", s))
    }
}

/// A simple command-line unit conversion tool.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert a value from one unit to another
    Convert {
        /// The unit to convert from (e.g., celsius, cm)
        #[arg(long)]
        from: AnyUnit,

        /// The unit to convert to (e.g., fahrenheit, km)
        #[arg(long)]
        to: AnyUnit,

        /// The numeric value to convert
        #[arg(long)]
        value: f64,
    },
    List,
    History,
}

// Function to generate the list output
pub fn display_all_units() -> String {
    let mut output = String::from("Supported Units:\n");

    // Define a list of references to our unit dimensions.
    // The type is a 'vector of trait objects' (&dyn UnitDimension).
    // Note: We use one arbitrary variant of the enum just to call the trait method.
    let dimensions: Vec<&dyn UnitDimension> = vec![
        &TempUnit::Celsius,
        &LengthUnit::Cm,
        // FUTURE: If you add WeightUnit, you just add: &WeightUnit::Kg, here.
    ];

    for dimension in dimensions {
        output.push_str(&format!("\n=== {} ===\n", dimension.dimension_name()));
        for unit_name in dimension.list_units() {
            output.push_str(&format!("  {}\n", unit_name));
        }
    }

    output
}
