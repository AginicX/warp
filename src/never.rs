use std::error::Error as StdError;
use std::fmt;

/// Rejection of a request by a [`Filter`](::Filter).
///
/// This rejection can never happen.
#[derive(Debug)]
pub enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl StdError for Never {
    fn description(&self) -> &str {
        match *self {}
    }
}
