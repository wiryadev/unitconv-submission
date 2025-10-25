use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq)]
pub enum UnitType {
    Temperature,
    Length,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TempUnit {
    pub fn get_type(&self) -> UnitType {
        UnitType::Temperature
    }
}

impl UnitDimension for TempUnit {
    fn dimension_name(&self) -> &'static str {
        "Temperature"
    }

    fn list_units(&self) -> Vec<String> {
        // Use clap's ValueEnum methods to get all variants automatically
        TempUnit::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect()
    }
}

impl UnitDimension for LengthUnit {
    fn dimension_name(&self) -> &'static str {
        "Length"
    }

    fn list_units(&self) -> Vec<String> {
        // Use clap's ValueEnum methods to get all variants automatically
        LengthUnit::value_variants()
            .iter()
            .map(|v| v.to_possible_value().unwrap().get_name().to_string())
            .collect()
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LengthUnit {
    Cm,
    Inch,
    Km,
    Miles,
}

impl LengthUnit {
    pub fn get_type(&self) -> UnitType {
        UnitType::Length
    }
}

pub trait UnitDimension {
    fn dimension_name(&self) -> &'static str;
    fn list_units(&self) -> Vec<String>;
}

// This enum acts as a container to unify the two specific unit types
// so `clap` can accept one or the other for `--from` and `--to`.
#[derive(Debug, Clone)]
pub enum AnyUnit {
    Temperature(TempUnit),
    Length(LengthUnit),
}
