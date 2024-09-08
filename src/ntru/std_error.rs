use crate::compress::error::CompressError;

#[derive(Debug, PartialEq, Eq)]
pub enum CipherError {
    CompressError(CompressError),
    InvalidRqChunkSize,
    SyncThreadJoinError,
    SyncLockError,
}
