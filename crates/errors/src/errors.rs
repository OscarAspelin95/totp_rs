use std::{array::TryFromSliceError, env::VarError};

use hmac::digest::InvalidLength;
use sqlx;
use thiserror::Error;

/// Rework this mod to avoid including big crates.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid timestamp")]
    TimeStampError(String),
    #[error("Failed to truncate timestamp")]
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
    #[error("")]
    CrudError(String),
    #[error("")]
    SqlxError(#[from] sqlx::Error),
}
