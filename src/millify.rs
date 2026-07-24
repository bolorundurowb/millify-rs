use crate::models::MillifiedNumber::MillifiedNumber;
use crate::models::MillifyOptions::MillifyOptions;
use num_traits::Num;
use rust_decimal::Decimal;
use std::fmt::{Debug, Display, Error};
use num_traits::real::Real;
use crate::enums::MillifyScaleBase;

pub fn decompose<T>(input: T, millify_options: Option<MillifyOptions>) -> MillifiedNumber
where
    T: Num + Display + Clone + PartialOrd,
{
    let millify_options = millify_options.unwrap_or(MillifyOptions::default(None));
    let divisor: i32 = if (millify_options.scale_base == MillifyScaleBase::Decimal) {1000} else {1024};
    let is_negative = input < 0;
    let mut absolute_value = input.abs();
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

pub fn shorten<T>(input: T, millify_options: Option<MillifyOptions>) -> String
where 
    T: Num + Display + Clone,
{
    let millified_number = decompose(input, millify_options.clone());
    format_scaled(&millified_number, millify_options)
}

pub trait Millify {
    fn shorten(&self, options: Option<MillifyOptions>) -> String;

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber;

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, Error>;
}

impl Millify for i64 {
    fn shorten(&self, options: Option<MillifyOptions>) -> String {
        todo!()
    }

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber {
        todo!()
    }

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, Error> {
        todo!()
    }
}

impl Millify for f64 {
    fn shorten(&self, options: Option<MillifyOptions>) -> String {
        todo!()
    }

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber {
        todo!()
    }

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, Error> {
        todo!()
    }
}

impl Millify for Decimal {
    fn shorten(&self, options: Option<MillifyOptions>) -> String {
        todo!()
    }

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber {
        todo!()
    }

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, Error> {
        todo!()
    }
}

pub fn format_scaled(
    millified_number: &MillifiedNumber,
    options: Option<MillifyOptions>,
) -> String {
    let options = options.unwrap_or(MillifyOptions::default(None));
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
