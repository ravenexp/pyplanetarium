//! Planetarium light spot rendering library bindings for Python.
//!
//! The Python bindings are implemented entirely in Rust using PyO3.

use pyo3::exceptions::PyNotImplementedError;
use pyo3::types::PyBytes;

use pyo3::prelude::*;

use planetarium::{Pixel, Point, Vector};

use planetarium::{
    Canvas as RsCanvas, ImageFormat as RsImageFormat, SpotId as RsSpotId, SpotShape as RsSpotShape,
};

/// Spot shape definition matrix
///
/// A unit sized circular spot is scaled
/// using the 2x2 transform matrix.
#[pyclass(module = "pyplanetarium", freelist = 8)]
#[pyo3(text_signature = "()")]
struct SpotShape(RsSpotShape);

/// Light spot descriptor type
///
/// This class can not be instantiated by Python code.
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct SpotId(RsSpotId);

/// Exportable canvas image formats
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct ImageFormat(RsImageFormat);

/// Generates the synthesized image containing multiple light spots
#[pyclass(module = "pyplanetarium")]
struct Canvas(RsCanvas);

#[pymethods]
impl SpotShape {
    // TODO: Accept more initializers like `k`, `[kx, ky]` or `[[xx, xy], [yx, yy]]`
    #[new]
    fn new() -> Self {
        SpotShape(RsSpotShape::default())
    }

    /// Linearly scales the spot shape by a single scalar factor.
    #[pyo3(text_signature = "(k, /)")]
    fn scale(&self, k: f32) -> SpotShape {
        SpotShape(self.0.scale(k))
    }
}

#[allow(non_upper_case_globals)]
#[pymethods]
impl ImageFormat {
    /// `ImageFormat::PngGamma8Bpp` enum variant singleton.
    #[classattr]
    const PngGamma8Bpp: ImageFormat = ImageFormat(RsImageFormat::PngGamma8Bpp);

    /// `ImageFormat::PngLinear16Bpp` enum variant singleton.
    #[classattr]
    const PngLinear16Bpp: ImageFormat = ImageFormat(RsImageFormat::PngLinear16Bpp);
}

#[pymethods]
impl Canvas {
    /// Creates a new clear canvas to render light spots on.
    #[staticmethod]
    #[pyo3(text_signature = "(width, height, /)")]
    fn new(width: u32, height: u32) -> Self {
        Canvas(RsCanvas::new(width, height))
    }

    /// Creates a new light spot on the canvas.
    #[pyo3(text_signature = "($self, position, shape, intensity, /)")]
    fn add_spot(&mut self, position: Point, shape: &SpotShape, intensity: f32) -> SpotId {
        let id = self.0.add_spot(position, shape.0, intensity);
        SpotId(id)
    }

    /// Sets the internal light spot position offset vector.
    ///
    /// The position offset vector is added to the immutable spot position
    /// to calculate the spot rendering coordinates on the canvas.
    #[pyo3(text_signature = "($self, spot, offset, /)")]
    fn set_spot_offset(&mut self, spot: &SpotId, offset: Vector) {
        self.0.set_spot_offset(spot.0, offset)
    }

    /// Sets the internal light spot illumination state.
    ///
    /// The spot illumination factor is multiplied with the immutable spot
    /// intensity factor to calculate the rendered peak intensity.
    #[pyo3(text_signature = "($self, spot, illumination, /)")]
    fn set_spot_illumination(&mut self, spot: &SpotId, illumination: f32) {
        self.0.set_spot_illumination(spot.0, illumination)
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

    /// Exports the canvas contents in the requested image format.
    #[pyo3(text_signature = "($self, format, /)")]
    fn export_image(&self, format: &ImageFormat, py: Python) -> PyResult<Py<PyBytes>> {
        match self.0.export_image(format.0) {
            Ok(b) => Ok(PyBytes::new(py, b.as_slice()).into()),
            Err(e) => Err(PyNotImplementedError::new_err(e.to_string())),
        }
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

    m.add_class::<SpotShape>()?;
    m.add_class::<SpotId>()?;
    m.add_class::<ImageFormat>()?;
    m.add_class::<Canvas>()?;

    Ok(())
}
