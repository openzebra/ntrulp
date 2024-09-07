use crate::poly::error::PolyErrors;

#[derive(Debug, PartialEq, Eq)]
pub enum KemErrors {
    PolyErrors(PolyErrors),
    InvalidR3GInvrBytes,
    InvalidR3FBytes,
}
