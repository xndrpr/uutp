use std::fmt;

pub struct Error {}

impl Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {{ }}")
    }
}
