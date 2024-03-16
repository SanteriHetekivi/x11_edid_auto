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
            ConnectionNewError::ConnectError(e) => write!(f, "Connect error:\n{}", e),
            ConnectionNewError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            ConnectionNewError::NoRootForScreenNumberError(e) => {
                write!(f, "No root for screen number error:\n{}", e)
            }
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

// No free Crtc error.
#[derive(Debug)]
pub(crate) struct NoFreeCrtcError {}
impl NoFreeCrtcError {
    pub fn new() -> NoFreeCrtcError {
        NoFreeCrtcError {}
    }
}
impl std::error::Error for NoFreeCrtcError {}
impl std::fmt::Display for NoFreeCrtcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No free Crtc found!")
    }
}

// Collects all of the errors that can occur when getting free Crtc.
#[derive(Debug)]
pub(crate) enum GetFreeCrtcError {
    ReplyError(x11rb::errors::ReplyError),
    NoFreeCrtcError(NoFreeCrtcError),
}
impl std::fmt::Display for GetFreeCrtcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetFreeCrtcError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            GetFreeCrtcError::NoFreeCrtcError(e) => write!(f, "No free Crtc error:\n{}", e),
        }
    }
}
impl From<x11rb::errors::ReplyError> for GetFreeCrtcError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        GetFreeCrtcError::ReplyError(err)
    }
}
impl From<NoFreeCrtcError> for GetFreeCrtcError {
    fn from(err: NoFreeCrtcError) -> Self {
        GetFreeCrtcError::NoFreeCrtcError(err)
    }
}

// Failed to transform to f64 error.
#[derive(Debug)]
pub(crate) struct TryIntoF64Error {
    name: String,
    infallible: std::convert::Infallible,
}
impl TryIntoF64Error {
    pub fn new(name: String, infallible: std::convert::Infallible) -> TryIntoF64Error {
        TryIntoF64Error { name, infallible }
    }
}
impl std::error::Error for TryIntoF64Error {}
impl std::fmt::Display for TryIntoF64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Failed to tranform {} to f64 and gotted infallible error {:?}!",
            self.name, self.infallible
        )
    }
}

// Collects all of the errors that can occur when updating screen size.
#[derive(Debug)]
pub(crate) enum UpdateScreenSizeError {
    ReplyError(x11rb::errors::ReplyError),
    TryIntoF64Error(TryIntoF64Error),
    ConnectionError(x11rb::errors::ConnectionError),
}
impl std::fmt::Display for UpdateScreenSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UpdateScreenSizeError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            UpdateScreenSizeError::TryIntoF64Error(e) => write!(f, "Try into f64 error:\n{}", e),
            UpdateScreenSizeError::ConnectionError(e) => write!(f, "Connection error:\n{}", e),
        }
    }
}
impl From<x11rb::errors::ReplyError> for UpdateScreenSizeError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        UpdateScreenSizeError::ReplyError(err)
    }
}
impl From<TryIntoF64Error> for UpdateScreenSizeError {
    fn from(err: TryIntoF64Error) -> Self {
        UpdateScreenSizeError::TryIntoF64Error(err)
    }
}
impl From<x11rb::errors::ConnectionError> for UpdateScreenSizeError {
    fn from(err: x11rb::errors::ConnectionError) -> Self {
        UpdateScreenSizeError::ConnectionError(err)
    }
}

// Collects all of the errors that can occur when ending connection.
#[derive(Debug)]
pub(crate) enum ConnectionEndError {
    ConnectionError(x11rb::errors::ConnectionError),
    UpdateScreenSizeError(UpdateScreenSizeError),
}
impl std::fmt::Display for ConnectionEndError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionEndError::ConnectionError(e) => write!(f, "Connect error:\n{}", e),
            ConnectionEndError::UpdateScreenSizeError(e) => {
                write!(f, "Update screen size error:\n{}", e)
            }
        }
    }
}
impl From<x11rb::errors::ConnectionError> for ConnectionEndError {
    fn from(err: x11rb::errors::ConnectionError) -> Self {
        ConnectionEndError::ConnectionError(err)
    }
}
impl From<UpdateScreenSizeError> for ConnectionEndError {
    fn from(err: UpdateScreenSizeError) -> Self {
        ConnectionEndError::UpdateScreenSizeError(err)
    }
}

// Collects all of the errors that can occur when getting monitor's name.
#[derive(Debug)]
pub(crate) enum MonitorNameError {
    ReplyError(x11rb::errors::ReplyError),
    FromUtf8Error(std::string::FromUtf8Error),
}
impl std::fmt::Display for MonitorNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MonitorNameError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            MonitorNameError::FromUtf8Error(e) => write!(f, "From UTF-8 error:\n{}", e),
        }
    }
}
impl From<x11rb::errors::ReplyError> for MonitorNameError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        MonitorNameError::ReplyError(err)
    }
}
impl From<std::string::FromUtf8Error> for MonitorNameError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        MonitorNameError::FromUtf8Error(err)
    }
}

// No modes found error.
#[derive(Debug)]
pub(crate) struct NoModesError {}
impl NoModesError {
    pub fn new() -> NoModesError {
        NoModesError {}
    }
}
impl std::error::Error for NoModesError {}
impl std::fmt::Display for NoModesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No modes found!")
    }
}

// Collects all of the errors that can occur when mode info for monitor.
#[derive(Debug)]
pub(crate) enum MonitorModeInfoError {
    ReplyError(x11rb::errors::ReplyError),
    NoModesError(NoModesError),
}
impl std::fmt::Display for MonitorModeInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MonitorModeInfoError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            MonitorModeInfoError::NoModesError(e) => write!(f, "No modes error:\n{}", e),
        }
    }
}
impl From<x11rb::errors::ReplyError> for MonitorModeInfoError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        MonitorModeInfoError::ReplyError(err)
    }
}
impl From<NoModesError> for MonitorModeInfoError {
    fn from(err: NoModesError) -> Self {
        MonitorModeInfoError::NoModesError(err)
    }
}

// Collects all of the errors that can occur when enabling monitor.
#[derive(Debug)]
pub(crate) enum MonitorEnableError {
    ReplyError(x11rb::errors::ReplyError),
    GetFreeCrtcError(GetFreeCrtcError),
    MonitorModeInfoError(MonitorModeInfoError),
}
impl std::fmt::Display for MonitorEnableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MonitorEnableError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            MonitorEnableError::GetFreeCrtcError(e) => write!(f, "Get free Crtc {:?}", e),
            MonitorEnableError::MonitorModeInfoError(e) => write!(f, "Monitor mode info {:?}", e),
        }
    }
}
impl From<x11rb::errors::ReplyError> for MonitorEnableError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        MonitorEnableError::ReplyError(err)
    }
}
impl From<GetFreeCrtcError> for MonitorEnableError {
    fn from(err: GetFreeCrtcError) -> Self {
        MonitorEnableError::GetFreeCrtcError(err)
    }
}
impl From<MonitorModeInfoError> for MonitorEnableError {
    fn from(err: MonitorModeInfoError) -> Self {
        MonitorEnableError::MonitorModeInfoError(err)
    }
}

// Invalid arguments error.
#[derive(Debug)]
pub(crate) struct InvalidArgumentsError {
    script: String,
}
impl InvalidArgumentsError {
    pub fn new(script: String) -> InvalidArgumentsError {
        InvalidArgumentsError { script }
    }
}
impl std::error::Error for InvalidArgumentsError {}
impl std::fmt::Display for InvalidArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Usage: {} <config file>", self.script)
    }
}

// No monitor groups given error.
#[derive(Debug)]
pub(crate) struct NoMonitorGroupsGivenError {
    config_file_path: String,
}
impl NoMonitorGroupsGivenError {
    pub fn new(config_file_path: String) -> NoMonitorGroupsGivenError {
        NoMonitorGroupsGivenError { config_file_path }
    }
}
impl std::error::Error for NoMonitorGroupsGivenError {}
impl std::fmt::Display for NoMonitorGroupsGivenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "No monitor groups given in config file: {}",
            self.config_file_path
        )
    }
}

// No monitor groups given error.
#[derive(Debug)]
pub(crate) struct NoMonitorsFoundError {}
impl NoMonitorsFoundError {
    pub fn new() -> NoMonitorsFoundError {
        NoMonitorsFoundError {}
    }
}
impl std::error::Error for NoMonitorsFoundError {}
impl std::fmt::Display for NoMonitorsFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No monitors found!")
    }
}

// Monitor not found error.
#[derive(Debug)]
pub(crate) struct MonitorNotFoundError {
    edid: String,
}
impl MonitorNotFoundError {
    pub fn new(edid: String) -> MonitorNotFoundError {
        MonitorNotFoundError { edid }
    }
}
impl std::error::Error for MonitorNotFoundError {}
impl std::fmt::Display for MonitorNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Monitor not found for EDID {}!", self.edid)
    }
}

// Failed to transform to i16 error.
#[derive(Debug)]
pub(crate) struct TryIntoI16Error {
    name: String,
    try_from_int_error: std::num::TryFromIntError,
}
impl TryIntoI16Error {
    pub fn new(name: String, try_from_int_error: std::num::TryFromIntError) -> TryIntoI16Error {
        TryIntoI16Error {
            name,
            try_from_int_error,
        }
    }
}
impl std::error::Error for TryIntoI16Error {}
impl std::fmt::Display for TryIntoI16Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Failed to tranform {} to i16 and gotted try from int error {:?}!",
            self.name, self.try_from_int_error
        )
    }
}

// No monitor groups with all monitors present error.
#[derive(Debug)]
pub(crate) struct NoMonitorGroupWithAllMonitorsPresentError {
    monitor_infos_for_monitors: Vec<Vec<String>>,
}
impl NoMonitorGroupWithAllMonitorsPresentError {
    pub fn new(
        monitor_infos_for_monitors: Vec<Vec<String>>,
    ) -> NoMonitorGroupWithAllMonitorsPresentError {
        NoMonitorGroupWithAllMonitorsPresentError {
            monitor_infos_for_monitors,
        }
    }
}
impl std::error::Error for NoMonitorGroupWithAllMonitorsPresentError {}
impl std::fmt::Display for NoMonitorGroupWithAllMonitorsPresentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "No monitor group with all of it's monitors present found!\n{}",
            (if self.monitor_infos_for_monitors.is_empty() {
                "No monitors!".to_string()
            } else {
                format!(
                    "Available monitors:\nMonitor:\n\t{}",
                    self.monitor_infos_for_monitors
                        .iter()
                        .map(|monitor_info| monitor_info.join("\n\t"))
                        .collect::<Vec<String>>()
                        .join("\nMonitor:\n\t")
                )
            })
        )
    }
}

// Collects all of the errors that can occur when running main.
#[derive(Debug)]
pub(crate) enum X11EDIDAutoError {
    InvalidArgumentsError(InvalidArgumentsError),
    IoError(std::io::Error),
    TomlDeserializeError(toml::de::Error),
    NoMonitorGroupsGivenError(NoMonitorGroupsGivenError),
    ConnectionNewError(ConnectionNewError),
    ReplyError(x11rb::errors::ReplyError),
    NoMonitorsFoundError(NoMonitorsFoundError),
    MonitorNotFoundError(MonitorNotFoundError),
    MonitorEnableError(MonitorEnableError),
    ConnectionError(x11rb::errors::ConnectionError),
    MonitorModeInfoError(MonitorModeInfoError),
    TryIntoI16Error(TryIntoI16Error),
    ConnectionEndError(ConnectionEndError),
    NoMonitorGroupWithAllMonitorsPresentError(NoMonitorGroupWithAllMonitorsPresentError),
}
impl std::fmt::Display for X11EDIDAutoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            X11EDIDAutoError::InvalidArgumentsError(e) => {
                write!(f, "Invalid arguments error:\n{}", e)
            }
            X11EDIDAutoError::IoError(e) => write!(f, "IO error:\n{}", e),
            X11EDIDAutoError::TomlDeserializeError(e) => {
                write!(f, "Toml deserialize error:\n{}", e)
            }
            X11EDIDAutoError::NoMonitorGroupsGivenError(e) => {
                write!(f, "No monitor groups given error:\n{}", e)
            }
            X11EDIDAutoError::ConnectionNewError(e) => write!(f, "Connection new error:\n{}", e),
            X11EDIDAutoError::ReplyError(e) => write!(f, "Reply error:\n{}", e),
            X11EDIDAutoError::NoMonitorsFoundError(e) => {
                write!(f, "No monitors found error:\n{}", e)
            }
            X11EDIDAutoError::MonitorNotFoundError(e) => {
                write!(f, "Monitor not found error:\n{}", e)
            }
            X11EDIDAutoError::MonitorEnableError(e) => write!(f, "Monitor enable error:\n{}", e),
            X11EDIDAutoError::ConnectionError(e) => write!(f, "Connection error:\n{}", e),
            X11EDIDAutoError::MonitorModeInfoError(e) => {
                write!(f, "Monitor mode info error:\n{}", e)
            }
            X11EDIDAutoError::TryIntoI16Error(e) => write!(f, "Try into i16 error:\n{}", e),
            X11EDIDAutoError::ConnectionEndError(e) => write!(f, "Connection end error:\n{}", e),
            X11EDIDAutoError::NoMonitorGroupWithAllMonitorsPresentError(e) => {
                write!(
                    f,
                    "No monitor group with all monitors present error:\n{}",
                    e
                )
            }
        }
    }
}
impl From<InvalidArgumentsError> for X11EDIDAutoError {
    fn from(err: InvalidArgumentsError) -> Self {
        X11EDIDAutoError::InvalidArgumentsError(err)
    }
}
impl From<std::io::Error> for X11EDIDAutoError {
    fn from(err: std::io::Error) -> Self {
        X11EDIDAutoError::IoError(err)
    }
}
impl From<toml::de::Error> for X11EDIDAutoError {
    fn from(err: toml::de::Error) -> Self {
        X11EDIDAutoError::TomlDeserializeError(err)
    }
}
impl From<NoMonitorGroupsGivenError> for X11EDIDAutoError {
    fn from(err: NoMonitorGroupsGivenError) -> Self {
        X11EDIDAutoError::NoMonitorGroupsGivenError(err)
    }
}
impl From<ConnectionNewError> for X11EDIDAutoError {
    fn from(err: ConnectionNewError) -> Self {
        X11EDIDAutoError::ConnectionNewError(err)
    }
}
impl From<x11rb::errors::ReplyError> for X11EDIDAutoError {
    fn from(err: x11rb::errors::ReplyError) -> Self {
        X11EDIDAutoError::ReplyError(err)
    }
}
impl From<NoMonitorsFoundError> for X11EDIDAutoError {
    fn from(err: NoMonitorsFoundError) -> Self {
        X11EDIDAutoError::NoMonitorsFoundError(err)
    }
}
impl From<MonitorNotFoundError> for X11EDIDAutoError {
    fn from(err: MonitorNotFoundError) -> Self {
        X11EDIDAutoError::MonitorNotFoundError(err)
    }
}
impl From<MonitorEnableError> for X11EDIDAutoError {
    fn from(err: MonitorEnableError) -> Self {
        X11EDIDAutoError::MonitorEnableError(err)
    }
}
impl From<x11rb::errors::ConnectionError> for X11EDIDAutoError {
    fn from(err: x11rb::errors::ConnectionError) -> Self {
        X11EDIDAutoError::ConnectionError(err)
    }
}
impl From<MonitorModeInfoError> for X11EDIDAutoError {
    fn from(err: MonitorModeInfoError) -> Self {
        X11EDIDAutoError::MonitorModeInfoError(err)
    }
}
impl From<TryIntoI16Error> for X11EDIDAutoError {
    fn from(err: TryIntoI16Error) -> Self {
        X11EDIDAutoError::TryIntoI16Error(err)
    }
}
impl From<ConnectionEndError> for X11EDIDAutoError {
    fn from(err: ConnectionEndError) -> Self {
        X11EDIDAutoError::ConnectionEndError(err)
    }
}
impl From<NoMonitorGroupWithAllMonitorsPresentError> for X11EDIDAutoError {
    fn from(err: NoMonitorGroupWithAllMonitorsPresentError) -> Self {
        X11EDIDAutoError::NoMonitorGroupWithAllMonitorsPresentError(err)
    }
}
