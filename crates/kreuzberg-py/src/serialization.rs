//! Wire format serialization functions
//!
//! Exposes serialize_to_toon() and serialize_to_json() to Python,
//! using the cached JSON from ExtractionResult to avoid lossy round-trips.

use crate::types::ExtractionResult;
use pyo3::prelude::*;

/// Serialize an ExtractionResult to TOON (Token-Oriented Object Notation).
///
/// TOON is a token-efficient alternative to JSON for LLM prompts.
/// Losslessly convertible to/from JSON but uses ~30-50% fewer tokens.
///
/// Args:
///     result: An ExtractionResult from any extract function
///
/// Returns:
///     str: TOON-serialized string
///
/// Raises:
///     RuntimeError: If serialization fails
///
/// Example:
///     >>> from kreuzberg import extract_file_sync, serialize_to_toon
///     >>> result = extract_file_sync("document.pdf")
///     >>> toon_str = serialize_to_toon(result)
///     >>> print(toon_str)  # compact, token-efficient format
#[pyfunction]
pub fn serialize_to_toon(result: &ExtractionResult) -> PyResult<String> {
    // Deserialize cached JSON back to Rust ExtractionResult, then serialize to TOON
    let rust_result: kreuzberg::ExtractionResult =
        serde_json::from_str(&result.result_json).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to deserialize cached result: {}",
                e
            ))
        })?;

    kreuzberg::serialize_to_toon(&rust_result).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("TOON serialization failed: {}", e))
    })
}

/// Serialize an ExtractionResult to pretty-printed JSON.
///
/// Args:
///     result: An ExtractionResult from any extract function
///
/// Returns:
///     str: Pretty-printed JSON string
///
/// Raises:
///     RuntimeError: If serialization fails
///
/// Example:
///     >>> from kreuzberg import extract_file_sync, serialize_to_json
///     >>> result = extract_file_sync("document.pdf")
///     >>> json_str = serialize_to_json(result)
///     >>> print(json_str)
#[pyfunction]
pub fn serialize_to_json(result: &ExtractionResult) -> PyResult<String> {
    // Deserialize and re-serialize to get pretty-printed output
    let rust_result: kreuzberg::ExtractionResult =
        serde_json::from_str(&result.result_json).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to deserialize cached result: {}",
                e
            ))
        })?;

    kreuzberg::serialize_to_json(&rust_result).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("JSON serialization failed: {}", e))
    })
}
