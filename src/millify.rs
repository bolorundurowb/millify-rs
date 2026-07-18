use std::fmt::Error;
use rust_decimal::Decimal;
use crate::models::MillifiedNumber::MillifiedNumber;
use crate::models::MillifyOptions::MillifyOptions;

pub trait Millify {
    fn shorten(&self, options: Option<MillifyOptions>) -> String;

    fn decompose(&self, options: Option<MillifyOptions>) -> MillifiedNumber;

    fn try_format(&self, options: Option<MillifyOptions>) -> Result<String, Error>;
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