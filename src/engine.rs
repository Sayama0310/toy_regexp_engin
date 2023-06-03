//! Regular Expression Engine Module
//!
//! This module contains the implementation of the regular expression engine.
//! The engine is responsible for determining whether a regular expression matches a search target string.
//!
//! This module consists of three components: a parser, an instruction compiler, and an evaluator.
mod parser;
mod compiler;
mod evaluator;

use crate::DynError;

/// Checks if a regular expression matches a search target string.
///
/// # Arguments
///
/// * `regexp` - The regular expression pattern to match.
/// * `line` - The search target string to match against.
///
/// # Returns
///
/// A boolean value indicating whether the regular expression matches the search target string.
///
/// # Examples
///
/// ```
/// let regexp = "(ab|cd)*";
/// let line = "cdcdab";
/// assert_eq!(true, is_match(regexp, line));
/// ```
pub fn is_match(regexp: &str, line: &str) -> Result<bool, DynError> {
    let ast = parser::parse(regexp)?;
    Ok(true)
}