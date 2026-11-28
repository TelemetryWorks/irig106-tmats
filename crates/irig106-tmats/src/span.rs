//! Checked raw byte ranges; no TMATS framing or character interpretation.

/// A range descriptor with private bounds, not a document or revision identity.
///
/// Construction checks a supplied length; access independently checks the actual
/// byte slice. An empty range at the source's end is valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    start: usize,
    len: usize,
}

impl Span {
    /// Construct a span only when both bounds fit within `source_len`.
    pub fn new(start: usize, len: usize, source_len: usize) -> Option<Self> {
        let remaining = source_len.checked_sub(start)?;
        if len > remaining {
            return None;
        }
        Some(Self { start, len })
    }

    /// Borrow the raw range, or reject it if this actual source is too short.
    ///
    /// No source bytes are allocated, normalized, clamped or decoded.
    pub fn get(self, source: &[u8]) -> Option<&[u8]> {
        let end = self.start.checked_add(self.len)?;
        source.get(self.start..end)
    }
}
