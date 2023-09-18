#[derive(Debug)]
pub enum NTRUErrors<'a> {
    ParamsError(&'a str),
    KeyGenError(&'a str),
    PrivateKeyImport(&'a str),
    PubKeyKeyImport(&'a str),
    KeyExportError(&'a str),
    ThreadError(&'a str),
    R3EncodeError(&'a str),
    SliceError(&'a str),
}
