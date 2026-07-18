use crate::constants::{DEFAULT_BINARY_UNITS, DEFAULT_UNITS};
use crate::enums::MillifyScaleBase;

pub struct MillifyOptions {
    precision: u32,
    lowercase: bool,
    space_before_unit: bool,
    units: Vec<String>,
    trim_insignificant_zeros: bool,
    smart_precision: bool,
    // local_key: Locale
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
            smart_precision: false
        }
    }
}