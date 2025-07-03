#[derive(Debug)]
pub struct Error {
    pos: ErrorPosition,
    msg: String,
}
#[derive(Debug)]
pub struct ErrorPosition {
    col:  usize,
    line: usize,
}
