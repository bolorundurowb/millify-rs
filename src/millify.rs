use crate::models::MillifiedNumber::MillifiedNumber;
use crate::models::MillifyOptions::MillifyOptions;
use num_traits::Num;
use rust_decimal::Decimal;
use std::fmt::{Debug, Display, Error};
use num_traits::real::Real;
use crate::enums::MillifyScaleBase;

pub trait Millify {
    fn shorten(&self, options: impl Into<Option<MillifyOptions>>) -> String;

    fn decompose(&self, options: impl Into<Option<MillifyOptions>>) -> MillifiedNumber;

    fn try_format(&self, options: impl Into<Option<MillifyOptions>>) -> Result<String, Error>;
}

impl<T> Millify for T
where
    T: Num + Display + Clone,
{
    fn shorten(&self, options: impl Into<Option<MillifyOptions>>) -> String {
        todo!()
    }

    fn decompose(&self, options: impl Into<Option<MillifyOptions>>) -> MillifiedNumber {
        let options = options.into();
        let millify_options = options.unwrap_or_default();
        
        let divisor: i32 = if (millify_options.scale_base == MillifyScaleBase::Decimal) {1000} else {1024};
        let is_negative = self < 0;
        let mut absolute_value = self.abs();
        let mut unit_index: usize = 0;

        while absolute_value >= divisor && unit_index < millify_options.units.len() - 1 {
            absolute_value /= divisor;
            unit_index += 1;
        }

        if is_negative {
            absolute_value *= -1;
        }

        MillifiedNumber {
            scaled_value: absolute_value,
            unit_index,
        }
    }

    fn try_format(&self, options: impl Into<Option<MillifyOptions>>) -> Result<String, Error> {
        todo!()
    }
}

pub fn format_scaled(
    millified_number: &MillifiedNumber,
    options: impl Into<Option<MillifyOptions>>,
) -> String {
    let options = options.into().unwrap_or_default();
    format_scaled_core(millified_number, &options)
}

fn format_scaled_core(millified_number: &MillifiedNumber, options: &MillifyOptions) -> String {
    let is_negative = millified_number.scaled_value < 0.0;
    let absolute_value = millified_number.scaled_value.abs();
    let effective_precision =
        get_effective_precision(absolute_value, options.precision, options.smart_precision);

    let mut formatted_number = format!(
        "{:.precision$}",
        absolute_value,
        precision = effective_precision as usize
    );

    if options.trim_insignificant_zeros {
        formatted_number = formatted_number.trim_end_matches('0').to_string();
    }

    if (is_negative) {
        formatted_number.insert(0, '-');
    }

    let mut unit = options.units[millified_number.unit_index].clone();

    if options.lowercase {
        unit = unit.to_lowercase();
    }

    format!(
        "{}{}{}",
        formatted_number,
        if options.space_before_unit { " " } else { "" },
        unit
    )
}

fn get_effective_precision(
    absolute_scaled_value: f64,
    max_precision: u32,
    use_smart_precision: bool,
) -> u32 {
    if use_smart_precision {
        if absolute_scaled_value >= 100f64 {
            return 0;
        }

        if absolute_scaled_value >= 10f64 {
            return 1.min(max_precision);
        }

        if absolute_scaled_value >= 1f64 {
            return max_precision;
        }
    }

    max_precision
}
