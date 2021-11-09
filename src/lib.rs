//! Planetarium light spot rendering library bindings for Python.
//!
//! The Python bindings are implemented entirely in Rust using PyO3.

use pyo3::prelude::*;

use planetarium::Canvas as RsCanvas;
use planetarium::Pixel;

/// Generates the synthesized image containing multiple light spots
#[pyclass(module = "pyplanetarium")]
struct Canvas(RsCanvas);

#[pymethods]
impl Canvas {
    /// Creates a new clear canvas to render light spots on.
    #[staticmethod]
    #[pyo3(text_signature = "(width, height, /)")]
    fn new(width: u32, height: u32) -> Self {
        Canvas(RsCanvas::new(width, height))
    }

    /// Clears the canvas image (fills with background pixels).
    #[pyo3(text_signature = "($self, /)")]
    fn clear(&mut self) {
        self.0.clear()
    }

    /// Draws the light spots onto the canvas image.
    #[pyo3(text_signature = "($self, /)")]
    fn draw(&mut self) {
        self.0.draw();
    }

    /// Returns the canvas dimensions as `(width, height)`.
    #[pyo3(text_signature = "($self, /)")]
    fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }

    /// Sets the background light level (dark pixel value).
    #[pyo3(text_signature = "($self, level, /)")]
    fn set_background(&mut self, level: Pixel) {
        self.0.set_background(level);
    }

    /// Sets the global brightness level (light spot intensity adjustment).
    #[pyo3(text_signature = "($self, brightness, /)")]
    fn set_brightness(&mut self, brightness: f32) {
        self.0.set_brightness(brightness);
    }
}

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

    m.add_class::<Canvas>()?;

    Ok(())
}
