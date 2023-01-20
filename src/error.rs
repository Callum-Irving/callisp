#[derive(Debug)]
pub enum LispError {
    IOError,
    ParseError,
    TypeError,
}
