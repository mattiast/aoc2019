use crate::intcode;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Intcode(intcode::Error),
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> MyError {
        MyError::Io(e)
    }
}
