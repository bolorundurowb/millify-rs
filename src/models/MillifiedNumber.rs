use crate::constants::DEFAULT_UNITS;
#[cfg(feature = "rust_decimal")]
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct MillifiedNumber {
    #[cfg(feature = "rust_decimal")]
    pub scaled_value: Decimal,
    #[cfg(not(feature = "rust_decimal"))]
    pub scaled_value: f64,
    pub unit_index: usize,
}

impl MillifiedNumber {
    pub fn new(scaled_value: f64, unit_index: usize) -> Self {
        Self {
            scaled_value,
            unit_index,
        }
    }

    pub fn to_formatted_string(&self) -> String {
        format!(
            "{:.2} {}",
            self.scaled_value, DEFAULT_UNITS[self.unit_index]
        )
    }
}
