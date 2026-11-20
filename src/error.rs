/// Error that can happen for various frogquad operations
#[derive(Debug)]
pub enum Error {
    /// Error when trying to load a font
    FontError(&'static str),
    /// Error when trying to load a file
    FileError {
        /// What kind of filesystem error happened
        kind: miniquad::fs::Error,
        /// Path of the file we were trying to access
        path: String,
    },
    /// Error when trying to compile a shader
    ShaderError(miniquad::ShaderError),
    /// Error when trying to load an image
    ImageError(image::ImageError),
    /// Unknown error, with possibly an info string
    UnknownError(&'static str),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::UnknownError(s)
    }
}

impl From<miniquad::ShaderError> for Error {
    fn from(s: miniquad::ShaderError) -> Self {
        Error::ShaderError(s)
    }
}

impl From<image::ImageError> for Error {
    fn from(s: image::ImageError) -> Self {
        Error::ImageError(s)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {self:?}")
    }
}

impl std::error::Error for Error {}
