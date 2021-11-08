//! Planetarium light spot rendering library bindings for Python.
//!
//! The Python bindings are implemented entirely in Rust using PyO3.

use pyo3::prelude::*;

/// Planetarium light spot rendering library bindings for Python.
///
/// This module provides a complete Python programming interface
/// for the Planetarium light spot rendering library crate
/// implemented in Rust.
///
/// See the Rust library crate documentation for the complete
/// public interface description.
#[pymodule]
fn pyplanetarium(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add module version attributes.
    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", env!("CARGO_PKG_AUTHORS"))?;

    Ok(())
}
