use crate::intcode;

pub type MyResult<T> = Result<T, MyError>;

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

impl From<intcode::Error> for MyError {
    fn from(e: intcode::Error) -> MyError {
        MyError::Intcode(e)
    }
}
