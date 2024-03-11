// No root for screen number error.
#[derive(Debug)]
pub(crate) struct NoRootForScreenNumberError {
    screen_number: usize,
}
impl NoRootForScreenNumberError {
    pub fn new(screen_number: usize) -> NoRootForScreenNumberError {
        NoRootForScreenNumberError { screen_number }
    }
}
impl std::error::Error for NoRootForScreenNumberError {}
impl std::fmt::Display for NoRootForScreenNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "No root screen for screen number: {}",
            self.screen_number
        )
    }
}

// Collects all of the errors that can occur when creating a new connection.
#[derive(Debug)]
pub(crate) enum ConnectionNewError {
    ConnectError(x11rb::errors::ConnectError),
    ReplyError(x11rb::errors::ReplyError),
    NoRootForScreenNumberError(NoRootForScreenNumberError),
}
impl std::fmt::Display for ConnectionNewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionNewError::ConnectError(e) => write!(f, "Connect error {:?}", e),
            ConnectionNewError::ReplyError(e) => write!(f, "Reply error {:?}", e),
            ConnectionNewError::NoRootForScreenNumberError(e) => write!(f, "{}", e),
        }
    }
}
impl From<x11rb::errors::ConnectError> for ConnectionNewError {
    fn from(err: x11rb::errors::ConnectError) -> Self {
        ConnectionNewError::ConnectError(err)
    }
}
impl From<x11rb::errors::ReplyError> for ConnectionNewError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        ConnectionNewError::ReplyError(err)
    }
}
impl From<NoRootForScreenNumberError> for ConnectionNewError {
    fn from(err: NoRootForScreenNumberError) -> Self {
        ConnectionNewError::NoRootForScreenNumberError(err)
    }
}
