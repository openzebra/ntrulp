#[derive(Debug)]
pub enum ConversionError {
    Overflow,
}

pub trait TryFrom: Sized {
    fn try_from_usize(num: usize) -> Result<Self, ConversionError>;
    fn try_from_i32(num: i32) -> Result<Self, ConversionError>;
}

impl TryFrom for i16 {
    fn try_from_usize(num: usize) -> Result<Self, ConversionError> {
        if num <= usize::MAX {
            Ok(num as i16)
        } else {
            Err(ConversionError::Overflow)
        }
    }

    fn try_from_i32(num: i32) -> Result<Self, ConversionError> {
        if num <= i16::MAX as i32 {
            Ok(num as i16)
        } else {
            Err(ConversionError::Overflow)
        }
    }
}
