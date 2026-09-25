// irig106-tmats/src/error.rs
//
// # Error Types
//
// Structured, actionable error and diagnostic types for all crate operations.
//
// ## Traceability:
//   L1-ERR → L2-ERR-001..005 → L3-ERR-001..007

use crate::types_bridge::GroupPrefix;
use core::fmt;

// ─── Error Code Convention (L3-ERR-007) ──────────────────────────────────────
// TMATS-P### : Parse errors
// TMATS-V### : Validation diagnostics
// TMATS-S### : Serialization errors
// TMATS-C### : Ch10 payload errors
// TMATS-G### : Generation errors
// TMATS-R### : Repair errors
// TMATS-X### : XML errors

/// Top-level error enum for all crate operations.
///
/// **Requirement:** L3-ERR-001
#[derive(Debug)]
pub enum TmatsError {
    /// Error during ASCII tokenization or structuring.
    Parse(ParseError),
    /// Validation produced errors (may also contain warnings/info).
    Validate(Vec<Diagnostic>),
    /// Error during ASCII or XML serialization.
    Serialize(SerializeError),
    /// Error during Ch10 payload decode/encode.
    Ch10(Ch10Error),
    /// Error during XML parsing or serialization.
    #[cfg(feature = "xml")]
    Xml(XmlError),
    /// Error during TMATS generation.
    #[cfg(feature = "generate")]
    Generate(GenerateError),
    /// Error during TMATS repair.
    #[cfg(feature = "repair")]
    Repair(RepairError),
}

impl fmt::Display for TmatsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "{e}"),
            Self::Validate(diags) => {
                let errors = diags.iter().filter(|d| d.severity == Severity::Error).count();
                write!(f, "validation failed with {errors} error(s)")
            }
            Self::Serialize(e) => write!(f, "{e}"),
            Self::Ch10(e) => write!(f, "{e}"),
            #[cfg(feature = "xml")]
            Self::Xml(e) => write!(f, "{e}"),
            #[cfg(feature = "generate")]
            Self::Generate(e) => write!(f, "{e}"),
            #[cfg(feature = "repair")]
            Self::Repair(e) => write!(f, "{e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TmatsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(e) => Some(e),
            Self::Serialize(e) => Some(e),
            Self::Ch10(e) => Some(e),
            _ => None,
        }
    }
}

/// Collection of errors from parsing or validation.
///
/// **Requirement:** L3-ERR-004
#[derive(Debug)]
pub struct TmatsErrors {
    pub errors: Vec<TmatsError>,
}

impl TmatsErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(&mut self, error: TmatsError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }
}

impl Default for TmatsErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TmatsErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} error(s)", self.errors.len())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TmatsErrors {}

impl IntoIterator for TmatsErrors {
    type Item = TmatsError;
    type IntoIter = std::vec::IntoIter<TmatsError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

// ─── Parse Errors (L3-ERR-002, L3-ERR-003) ──────────────────────────────────

/// A specific parse error with location context.
///
/// **Requirement:** L3-ERR-002
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Classification of the error.
    pub kind: ParseErrorKind,
    /// Byte offset in the source input.
    pub byte_offset: usize,
    /// 1-based line number (if determinable).
    pub line: u32,
    /// The code-name that triggered the error, if applicable.
    pub code_name: Option<String>,
    /// Human-readable description.
    pub message: String,
}

/// **Requirement:** L3-ERR-003
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Missing or invalid colon/semicolon delimiter.
    InvalidDelimiter,
    /// Code-name does not match expected `GROUP\ATTR` pattern.
    MalformedCodeName,
    /// Attribute value could not be parsed to expected type.
    InvalidValue,
    /// Group prefix character is not recognized.
    UnknownGroup,
    /// Same code-name appears more than once.
    DuplicateAttribute,
    /// Exceeded configured maximum attribute count (DoS protection).
    MaxAttributesExceeded,
    /// Attribute value exceeds configured maximum size (DoS protection).
    MaxValueSizeExceeded,
    /// Input contains non-7-bit-ASCII bytes outside CR/LF.
    InvalidAscii,
    /// Unexpected end of input during tokenization.
    UnexpectedEof,
}

/// **Requirement:** L3-ERR-005
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ERROR] TMATS-P{:03}: {} (at byte {}, line {})",
            self.kind as u8,
            self.message,
            self.byte_offset,
            self.line,
        )?;
        if let Some(ref cn) = self.code_name {
            write!(f, " [code-name: {cn}]")?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

// ─── Validation Diagnostics (L3-VALID-005, L3-VALID-006) ────────────────────

/// Severity level for validation findings.
///
/// **Requirement:** L3-VALID-006
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Severity {
    /// Advisory information, not a spec violation.
    Info,
    /// Suspicious but technically allowed by spec.
    Warning,
    /// Definite spec violation.
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Error => write!(f, "ERROR"),
        }
    }
}

/// A single validation finding.
///
/// **Requirement:** L3-VALID-005
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostic {
    /// Severity of this finding.
    pub severity: Severity,
    /// Unique rule identifier (e.g., "TMATS-V001").
    pub rule_id: String,
    /// Human-readable description of the issue.
    pub message: String,
    /// Spec section reference (e.g., "Ch9 §9.5.2 Table 9-1").
    pub spec_reference: String,
    /// TMATS attribute path if applicable (e.g., "R-1\\TK1-1").
    pub attribute_path: Option<String>,
    /// What the spec requires.
    pub expected: Option<String>,
    /// What was actually found.
    pub actual: Option<String>,
    /// Suggested corrective action.
    pub suggested_fix: Option<String>,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.rule_id, self.message)?;
        if let Some(ref path) = self.attribute_path {
            write!(f, " [{path}]")?;
        }
        write!(f, " → {}", self.spec_reference)?;
        Ok(())
    }
}

// ─── Serialization Errors ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SerializeError {
    pub message: String,
    pub attribute_path: Option<String>,
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ERROR] TMATS-S: {}", self.message)?;
        if let Some(ref path) = self.attribute_path {
            write!(f, " [{path}]")?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SerializeError {}

// ─── Ch10 Errors ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Ch10Error {
    pub message: String,
}

impl fmt::Display for Ch10Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ERROR] TMATS-C: {}", self.message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Ch10Error {}

// ─── XML Errors ──────────────────────────────────────────────────────────────

#[cfg(feature = "xml")]
#[derive(Debug, Clone)]
pub struct XmlError {
    pub message: String,
}

#[cfg(feature = "xml")]
impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ERROR] TMATS-X: {}", self.message)
    }
}

#[cfg(feature = "xml")]
#[cfg(feature = "std")]
impl std::error::Error for XmlError {}

// ─── Generate Errors ─────────────────────────────────────────────────────────

#[cfg(feature = "generate")]
#[derive(Debug, Clone)]
pub struct GenerateError {
    pub message: String,
}

#[cfg(feature = "generate")]
impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ERROR] TMATS-G: {}", self.message)
    }
}

#[cfg(feature = "generate")]
#[cfg(feature = "std")]
impl std::error::Error for GenerateError {}

// ─── Repair Errors ───────────────────────────────────────────────────────────

#[cfg(feature = "repair")]
#[derive(Debug, Clone)]
pub struct RepairError {
    pub message: String,
}

#[cfg(feature = "repair")]
impl fmt::Display for RepairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ERROR] TMATS-R: {}", self.message)
    }
}

#[cfg(feature = "repair")]
#[cfg(feature = "std")]
impl std::error::Error for RepairError {}

// ─── Conversion impls ───────────────────────────────────────────────────────

impl From<ParseError> for TmatsError {
    fn from(e: ParseError) -> Self {
        TmatsError::Parse(e)
    }
}

impl From<SerializeError> for TmatsError {
    fn from(e: SerializeError) -> Self {
        TmatsError::Serialize(e)
    }
}

impl From<Ch10Error> for TmatsError {
    fn from(e: Ch10Error) -> Self {
        TmatsError::Ch10(e)
    }
}
