#[derive(Debug)]
pub enum ConversionError {
    Overflow,
    NoInvSolution,
}

pub trait TryFrom: Sized {
    fn try_from_usize(num: usize) -> Result<Self, ConversionError>;
    fn try_from_u32(num: u32) -> Result<Self, ConversionError>;
}

impl TryFrom for u16 {
    fn try_from_usize(num: usize) -> Result<Self, ConversionError> {
        if num <= u8::MAX as usize {
            Ok(num as u16)
        } else {
            Err(ConversionError::Overflow)
        }
    }

    fn try_from_u32(num: u32) -> Result<Self, ConversionError> {
        if num <= u16::MAX as u32 {
            Ok(num as u16)
        } else {
            Err(ConversionError::Overflow)
        }
    }
}

impl TryFrom for u8 {
    fn try_from_usize(num: usize) -> Result<Self, ConversionError> {
        if num <= u8::MAX as usize {
            Ok(num as u8)
        } else {
            Err(ConversionError::Overflow)
        }
    }

    fn try_from_u32(num: u32) -> Result<Self, ConversionError> {
        if num <= u8::MAX as u32 {
            Ok(num as u8)
        } else {
            Err(ConversionError::Overflow)
        }
    }
}
