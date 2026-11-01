use std::{array::TryFromSliceError, env::VarError};

use hmac::digest::InvalidLength;
use thiserror::Error;

/// Rework this mod to avoid including big crates.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("")]
    TimeStampError(String),
    #[error("")]
    TimeStampTruncError(#[from] chrono::RoundingError),
    #[error("")]
    SliceError(#[from] TryFromSliceError),
    #[error("")]
    MissingEnvVarError(#[from] VarError),
    // Hmac
    #[error("")]
    HmacLengthError(#[from] InvalidLength),
    #[error("")]
    IoError(#[from] std::io::Error),
    // Base32
    #[error("")]
    Base32Error,
    //
    #[error("")]
    FileDoesNotExistError(String),
    // turso
    #[error("")]
    TursoError(#[from] turso::Error),
}
