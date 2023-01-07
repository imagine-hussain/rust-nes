use sdl2::{video::WindowBuildError, IntegerOrSdlError};

/// Error type for the render module.
#[derive(Debug)]
pub enum RenderError {
    SdlError(String),
    WindowError(WindowBuildError),
    IntegerOrSdlError(IntegerOrSdlError),
    // InternalError(String),
    Quit,
}

impl From<String> for RenderError {
    fn from(err: String) -> Self {
        Self::SdlError(err)
    }
}

impl From<WindowBuildError> for RenderError {
    fn from(err: WindowBuildError) -> Self {
        Self::WindowError(err)
    }
}

impl From<IntegerOrSdlError> for RenderError {
    fn from(err: IntegerOrSdlError) -> Self {
        Self::IntegerOrSdlError(err)
    }
}
