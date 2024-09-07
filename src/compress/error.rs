#[derive(Debug, PartialEq, Eq)]
pub enum CompressError {
    SeedSliceError,
    SizeSliceError,
    ByteslengthError,
}
