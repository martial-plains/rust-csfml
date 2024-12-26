use csfml_sys::{
    sfSocketDisconnected, sfSocketDone, sfSocketError, sfSocketNotReady, sfSocketPartial,
    sfSocketStatus,
};

/// Miscellaneous errors that can occur when handling sockets (connecting, sending, receiving, etc...)
#[derive(Debug)]
pub enum Error {
    NotReady,
    Partial,
    Disconnected,
    OtherError,
}

/// Turns a CSFML error code into a proper Rust error
/// For this wrapper's internal workings
#[allow(non_snake_case)]
pub const fn code_to_err(code: sfSocketStatus) -> Result<(), Error> {
    match code {
        x if x == sfSocketDone => Ok(()),
        x if x == sfSocketNotReady => Err(Error::NotReady),
        x if x == sfSocketPartial => Err(Error::Partial),
        x if x == sfSocketDisconnected => Err(Error::Disconnected),
        x if x == sfSocketError => Err(Error::OtherError),
        _ => panic!("Unexpected socket status"),
    }
}
