#[derive(Debug)]
pub enum NTRUErrors<'a> {
    ParamsError(&'a str),
    KeyGenError(&'a str),
    KeyExportError(&'a str),
}
