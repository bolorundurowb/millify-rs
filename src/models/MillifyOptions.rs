use crate::constants::{DEFAULT_BINARY_UNITS, DEFAULT_UNITS};
use crate::enums::MillifyScaleBase;

#[derive(Debug, Clone)]
pub struct MillifyOptions {
    pub  precision: u32,
    pub  lowercase: bool,
    pub  space_before_unit: bool,
    pub  units: Vec<String>,
    pub  trim_insignificant_zeros: bool,
    pub  smart_precision: bool,
    pub  scale_base: MillifyScaleBase,
    // local_key: Locale
}

impl MillifyOptions {
    pub  fn default(scale_base: Option<MillifyScaleBase>) -> Self {
        let base = scale_base.unwrap_or(MillifyScaleBase::Decimal);
let units = (if base == MillifyScaleBase::Decimal { DEFAULT_UNITS } else { DEFAULT_BINARY_UNITS } ).map(String::from).to_vec();
        
        MillifyOptions {
            precision: 1,
            lowercase: false,
            space_before_unit: false,
            scale_base: base,
            units,
            trim_insignificant_zeros: true,
            smart_precision: false
        }
    }
}