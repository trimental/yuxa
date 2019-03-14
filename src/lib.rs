#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[path = "platform/linux/mod.rs"]
mod platform;

pub use self::platform::*;

pub use winit::{
    dpi, AvailableMonitorsIter, AxisId, ButtonId, ControlFlow,
    CreationError as WindowCreationError, DeviceEvent, DeviceId, ElementState, Event, EventsLoop,
    EventsLoopClosed, EventsLoopProxy, Icon, KeyboardInput, ModifiersState, MonitorId, MouseButton,
    MouseCursor, MouseScrollDelta, ScanCode, Touch, TouchPhase, VirtualKeyCode, Window,
    WindowAttributes, WindowBuilder, WindowEvent, WindowId,
};

pub use platform::YuxaWindow;

impl std::ops::Deref for YuxaWindow {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window()
    }
}

#[derive(Debug)]
pub enum CreationError {
    OsError(String),
    /// TODO: remove this error
    NotSupported(&'static str),
    NoBackendAvailable(Box<std::error::Error + Send>),
    RobustnessNotSupported,
    OpenGlVersionNotSupported,
    NoAvailablePixelFormat,
    PlatformSpecific(String),
    Window(WindowCreationError),
    /// We received two errors, instead of one.
    CreationErrorPair(Box<CreationError>, Box<CreationError>),
}

impl CreationError {
    fn to_string(&self) -> &str {
        match *self {
            CreationError::OsError(ref text) => &text,
            CreationError::NotSupported(text) => &text,
            CreationError::NoBackendAvailable(_) => "No backend is available",
            CreationError::RobustnessNotSupported => {
                "You requested robustness, but it is \
                 not supported."
            }
            CreationError::OpenGlVersionNotSupported => {
                "The requested OpenGL version is not \
                 supported."
            }
            CreationError::NoAvailablePixelFormat => {
                "Couldn't find any pixel format that matches \
                 the criteria."
            }
            CreationError::PlatformSpecific(ref text) => &text,
            CreationError::Window(ref err) => std::error::Error::description(err),
            CreationError::CreationErrorPair(ref _err1, ref _err2) => "Received two errors.",
        }
    }
}

impl std::fmt::Display for CreationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_str(self.to_string())?;

        if let CreationError::CreationErrorPair(ref e1, ref e2) = *self {
            write!(formatter, " Error 1: \"")?;
            e1.fmt(formatter)?;
            write!(formatter, "\"")?;
            write!(formatter, " Error 2: \"")?;
            e2.fmt(formatter)?;
            write!(formatter, "\"")?;
        }

        if let CreationError::NotSupported(msg) = self {
            write!(formatter, ": {}", msg)?;
        }
        Ok(())
    }
}

impl std::error::Error for CreationError {
    fn description(&self) -> &str {
        self.to_string()
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            CreationError::NoBackendAvailable(ref err) => Some(&**err),
            CreationError::Window(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<WindowCreationError> for CreationError {
    fn from(err: WindowCreationError) -> Self {
        CreationError::Window(err)
    }
}
