const DEFAULT_UNITS: [&str; 9] = ["", "k", "m", "g", "t", "p", "e", "z", "y"];
const DEFAULT_BINARY_UNITS: [&str; 9] = ["", "ki", "mi", "gi", "ti", "pi", "ei", "zi", "yi"];

#[derive(Debug)]
pub struct MillifiedNumber {
    scaled_value: f64,
    unit_index: usize,
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

#[derive(PartialEq, Debug)]
pub enum MillifyScaleBase {
    Decimal = 1000,
    Binary = 1024,
}

pub struct MillifyOptions {
    precision: u32,
    lowercase: bool,
    space_before_unit: bool,
    units: Vec<String>,
    trim_insignificant_zeros: bool,
    smart_precision: bool,
}

impl MillifyOptions {
    pub  fn default(scale_base: Option<MillifyScaleBase>) -> Self {
        let base = scale_base.unwrap_or(MillifyScaleBase::Decimal);
        
        MillifyOptions {
            precision: 1,
            lowercase: false,
            space_before_unit: false,
            units: (if base == MillifyScaleBase::Decimal { DEFAULT_UNITS } else { DEFAULT_BINARY_UNITS } ).map(String::from).to_vec() ,
            trim_insignificant_zeros: true,
            smart_precision: false,
        }
    }
}

pub trait Millify {
    fn shorten(&self, options: Option<MillifyOptions>) -> String;

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber;

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, std::fmt::Error>;
}

pub fn format_scaled(millified_number: &MillifiedNumber, options: &MillifyOptions) -> String {}

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
