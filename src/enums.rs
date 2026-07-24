#[derive(PartialEq, Debug, Clone, Default)]
pub enum MillifyScaleBase {
    Decimal = 1000,
    #[default]
    Binary = 1024,
}
