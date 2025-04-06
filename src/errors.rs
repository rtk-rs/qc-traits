//! Error definitions
use thiserror::Error;

#[cfg(docs)]
use crate::{pipeline::QcPipeline, QcAngle, QcScope};

/// Errors while designing a [QcPipeline]
#[derive(Debug, Error)]
pub enum QcPipelineError {
    #[error("invalid pipeline scope: {0}")]
    InvalidScope(QcScopeError),
    #[error("invalid pipeline selection: {0}")]
    InvalidSelection(QcSelectionError),
    #[error("invalid pipeline operand")]
    InvalidOperand,
}

/// Errors while parsing a [QcAngle]
#[derive(Debug, Error)]
pub enum QcAngleParsingError {
    #[error("invalid unit")]
    InvalidUnit,
    #[error("invalid angle value")]
    InvalidValue,
}

/// Errors while parsing a [QcScope]
#[derive(Debug, Error)]
pub enum QcScopeError {
    #[error("Invalid scope")]
    InvalidScope,
    #[error("Unknown product type")]
    UnknownProductType,
}

/// Errors while describing a [QcSelection] method
#[derive(Debug, Error)]
pub enum QcSelectionError {
    #[error("Empty selection step")]
    EmptyStep,
    #[error("Invalid item")]
    InvalidItem,
    #[error("Invalid operand")]
    InvalidOperand,
}
