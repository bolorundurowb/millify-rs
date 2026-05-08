const DEFAULT_UNITS: [&str; 9] = ["", "k", "m", "g", "t", "p", "e", "z", "y"];
const DEFAULT_BINARY_UNITS: [&str; 9] = ["",  "ki", "mi", "gi", "ti", "pi", "ei", "zi", "yi"];

#[derive(Debug)]
pub struct MillifiedNumber {
    scaled_value: f64,
    unit_index: usize
}

impl MillifiedNumber {
    pub fn new(scaled_value: f64, unit_index: usize) -> Self {
        Self { scaled_value, unit_index }
    }

    pub fn to_formatted_string(&self) -> String {
        format!("{:.2} {}", self.scaled_value, DEFAULT_UNITS[self.unit_index])
    }
}

/// Represents the scaling bases used for millifying numbers.
///
/// This enum defines the two different scaling systems that can be used to
/// abbreviate large numbers into human-readable formats:
///
/// - `Decimal`: Uses a base of 1000 for scaling (e.g., kilo, mega, giga).
///   Commonly used in contexts like metric systems and general-purpose
///   numeric formatting.
///
/// - `Binary`: Uses a base of 1024 for scaling (e.g., kibibyte, mebibyte, gibibyte).
///   Typically used in computing contexts, particularly for memory and data
///   storage representation.
///
/// # Variants
///
/// - `Decimal = 1000`: Represents the base-1000 (metric) scaling system.
/// - `Binary = 1024`: Represents the base-1024 (binary) scaling system.
///
/// # Example
/// ```rust
/// use your_module::MillifyScaleBase;
///
/// let scale = MillifyScaleBase::Decimal;
/// match scale {
///     MillifyScaleBase::Decimal => println!("Using decimal scaling (1000)."),
///     MillifyScaleBase::Binary => println!("Using binary scaling (1024)."),
/// }
/// ```
pub enum MillifyScaleBase {
    Decimal = 1000,
    Binary = 1024,
}

pub struct MillifyOptions {
    precision: u32,
    lowercase: bool,
    space_before_unit: bool,
    units: Vec<String>,
scale_base: MillifyScaleBase,
    
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
