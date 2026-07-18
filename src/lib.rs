pub mod models;
pub mod constants;

use std::fmt::Error;
use rust_decimal::prelude::*;
use icu::locale::Locale;




#[derive(PartialEq, Debug)]
pub enum MillifyScaleBase {
    Decimal = 1000,
    Binary = 1024,
}




pub trait Millify {
    fn shorten(&self, options: Option<MillifyOptions>) -> String;

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber;

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, std::fmt::Error>;
}

pub fn format_scaled(millified_number: &MillifiedNumber, options: &MillifyOptions) -> String {
    todo!()
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
