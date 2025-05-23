/// This module provides types to represent text positions in tombi.
///
/// We maintain two forms of source code position information.
///
/// - [`Position`][crate::Position] represents an absolute position in terms of line and column.
/// - [`Offset`][crate::Offset] represents an absolute offset from the beginning of the text.
///
/// We also provide [`Range`] and [`Span`] to indicate text ranges.
///
/// - [`Range`][crate::Range] is a struct that represents a range of text as `(Position, Position)`.
/// - [`Span`][crate::Span] is a struct that represents a range of text as `(Offset, Offset)`.
///
/// The biggest difference from Rust Analyzer's Red-Green Tree is that we preserve two representations,
/// [`Position`][crate::Position] and [`Offset`][crate::Offset], in the tree.
/// This increases the memory size of the tree,
/// but makes it much easier to implement features that work with the tree.
///
mod features;
mod offset;
mod position;
mod range;
mod relative_position;
mod span;

type RawTextSize = u32;
pub type RawOffset = RawTextSize;
pub type RelativeOffset = RawTextSize;
pub type Line = RawTextSize;
pub type Column = RawTextSize;

pub use crate::{
    offset::Offset, position::Position, range::Range, relative_position::RelativePosition,
    span::Span,
};

#[cfg(target_pointer_width = "16")]
compile_error!("'text' crate assumes usize >= u32 and does not work on 16-bit targets");
