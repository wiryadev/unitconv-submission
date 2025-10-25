use crate::history::HistoryEntry;
use crate::unit::{AnyUnit, LengthUnit, TempUnit};

pub struct ConversionResult {
    pub display_string: String,
    pub history_entry: HistoryEntry,
}

fn get_unit_name(unit: &AnyUnit) -> String {
    match unit {
        AnyUnit::Temperature(temp_unit) => format!("{:?}", temp_unit),
        AnyUnit::Length(length_unit) => format!("{:?}", length_unit),
    }
}

/// Determines the symbol for a given unit for display purposes.
fn get_unit_symbol(unit: &AnyUnit) -> &str {
    match unit {
        AnyUnit::Temperature(temp_unit) => match temp_unit {
            TempUnit::Celsius => "°C",
            TempUnit::Fahrenheit => "°F",
            TempUnit::Kelvin => "K",
        },
        AnyUnit::Length(length_unit) => match length_unit {
            LengthUnit::Cm => "cm",
            LengthUnit::Inch => "inch",
            LengthUnit::Km => "km",
            LengthUnit::Miles => "miles",
        },
    }
}

pub fn convert_value(from: AnyUnit, to: AnyUnit, value: f64) -> Result<ConversionResult, String> {
    let from_type = match &from {
        AnyUnit::Temperature(u) => u.get_type(),
        AnyUnit::Length(u) => u.get_type(),
    };
    let to_type = match &to {
        AnyUnit::Temperature(u) => u.get_type(),
        AnyUnit::Length(u) => u.get_type(),
    };

    let from_symbol = get_unit_symbol(&from);
    let to_symbol = get_unit_symbol(&to);

    if from_type != to_type {
        return Err(format!(
            "[ERROR] Cannot convert between different dimensions: {:?} ({}) to {:?} ({})",
            from_type,
            get_unit_name(&from),
            to_type,
            get_unit_name(&to)
        ));
    }

    // Clone the values here because the `match` arms need to consume the
    // `AnyUnit` to access the inner `TempUnit` or `LengthUnit`.
    let result = match (from.clone(), to.clone()) {
        (AnyUnit::Temperature(from_temp), AnyUnit::Temperature(to_temp)) => {
            let base_celsius = to_celsius(value, &from_temp);
            from_celsius(base_celsius, &to_temp)
        }
        (AnyUnit::Length(from_length), AnyUnit::Length(to_length)) => {
            let base_cm = to_cm(value, &from_length);
            from_cm(base_cm, &to_length)
        }
        _ => unreachable!(),
    };

    let display_string = format!("{} {} = {} {}", format_flexible_precision(value), from_symbol, format_flexible_precision(result), to_symbol);

    let history_entry = HistoryEntry {
        original_unit: String::from(from_symbol),
        target_unit: String::from(to_symbol),
        original_value: value,
        target_value: result,
    };

    Ok(ConversionResult {
        display_string,
        history_entry,
    })
}

pub fn format_flexible_precision(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        return format!("{}", value.round());
    }
    let rounded_value = (value * 1000.0).round() / 1000.0;
    format!("{}", rounded_value)
}

fn to_celsius(value: f64, from: &TempUnit) -> f64 {
    match from {
        TempUnit::Celsius => value,
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TempUnit::Kelvin => value - 273.15,
    }
}

fn from_celsius(celsius: f64, to: &TempUnit) -> f64 {
    match to {
        TempUnit::Celsius => celsius,
        TempUnit::Fahrenheit => (celsius * 9.0 / 5.0) + 32.0,
        TempUnit::Kelvin => celsius + 273.15,
    }
}

// --- Length Conversion Logic ---

fn to_cm(value: f64, from: &LengthUnit) -> f64 {
    match from {
        LengthUnit::Cm => value,
        LengthUnit::Inch => value * 2.54,
        LengthUnit::Km => value * 100_000.0,
        LengthUnit::Miles => value * 160_934.4,
    }
}

fn from_cm(cm: f64, to: &LengthUnit) -> f64 {
    match to {
        LengthUnit::Cm => cm,
        LengthUnit::Inch => cm / 2.54,
        LengthUnit::Km => cm / 100_000.0,
        LengthUnit::Miles => cm / 160_934.4,
    }
}
