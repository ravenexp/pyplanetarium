//! Planetarium light spot rendering library bindings for Python.
//!
//! The Python bindings are implemented entirely in Rust using PyO3.

use pyo3::exceptions::{PyNotImplementedError, PyTypeError};
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
///
/// The Python objects can be created as either:
///
/// - `SpotShape()` -- the default unit-sized shape
/// - `SpotShape(k)` -- the default shape scaled by factor `k`
/// - `SpotShape((kx, ky))` -- the default shape XY stretched by `kx` and `ky` factors
/// - `SpotShape([[xx, xy], [yx, yy]])` -- explicit transform matrix initialization
#[pyclass(module = "pyplanetarium", freelist = 8)]
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
    #[new]
    fn new(src: Option<&PyAny>) -> PyResult<Self> {
        if let Some(src) = src {
            if let Ok(k) = src.extract::<f32>() {
                Ok(SpotShape(RsSpotShape::default().scale(k)))
            } else if let Ok((kx, ky)) = src.extract::<(f32, f32)>() {
                Ok(SpotShape(RsSpotShape {
                    xx: kx,
                    xy: 0.0,
                    yx: 0.0,
                    yy: ky,
                }))
            } else if let Ok(mat) = src.extract::<Vec<Vec<f32>>>() {
                if mat.len() == 2 && mat[0].len() == 2 && mat[1].len() == 2 {
                    Ok(SpotShape(RsSpotShape {
                        xx: mat[0][0],
                        xy: mat[0][1],
                        yx: mat[1][0],
                        yy: mat[1][1],
                    }))
                } else {
                    Err(PyTypeError::new_err(
                        "Invalid initializer dimensions: must be 2x2",
                    ))
                }
            } else {
                Err(PyTypeError::new_err(format!(
                    "Unexpected initializer type: '{}'",
                    src.get_type().name().unwrap()
                )))
            }
        } else {
            Ok(SpotShape(RsSpotShape::default()))
        }
    }

    /// Linearly scales the spot shape by a single scalar factor.
    #[pyo3(text_signature = "(k, /)")]
    fn scale(&self, k: f32) -> SpotShape {
        SpotShape(self.0.scale(k))
    }

    /// Implements `str(x)` in Python.
    fn __str__(&self) -> String {
        format!(
            "[[{}, {}], [{}, {}]]",
            self.0.xx, self.0.xy, self.0.yx, self.0.yy
        )
    }

    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[allow(non_upper_case_globals)]
#[pymethods]
impl ImageFormat {
    /// `ImageFormat::RawGamma8Bpp` enum variant singleton.
    #[classattr]
    const RawGamma8Bpp: ImageFormat = ImageFormat(RsImageFormat::RawGamma8Bpp);

    /// `ImageFormat::RawLinear10BppLE` enum variant singleton.
    #[classattr]
    const RawLinear10BppLE: ImageFormat = ImageFormat(RsImageFormat::RawLinear10BppLE);

    /// `ImageFormat::RawLinear12BppLE` enum variant singleton.
    #[classattr]
    const RawLinear12BppLE: ImageFormat = ImageFormat(RsImageFormat::RawLinear12BppLE);

    /// `ImageFormat::PngGamma8Bpp` enum variant singleton.
    #[classattr]
    const PngGamma8Bpp: ImageFormat = ImageFormat(RsImageFormat::PngGamma8Bpp);

    /// `ImageFormat::PngLinear16Bpp` enum variant singleton.
    #[classattr]
    const PngLinear16Bpp: ImageFormat = ImageFormat(RsImageFormat::PngLinear16Bpp);

    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
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

    /// Calculates the canvas coordinates of the light spot.
    ///
    /// The canvas coordinates are calculated as the immutable spot position coordinates
    /// shifted by the variable spot offset vector and transformed using the canvas
    /// world transform.
    #[pyo3(text_signature = "($self, spot, /)")]
    fn spot_position(&self, spot: &SpotId) -> Option<Point> {
        self.0.spot_position(spot.0)
    }

    /// Calculates the effective peak intensity of the light spot.
    ///
    /// The effective peak intensity is calculated as the product of the immutable spot
    /// intensity factor, the variable spot illumination factor
    /// and the global brightness level.
    #[pyo3(text_signature = "($self, spot, /)")]
    fn spot_intensity(&self, spot: &SpotId) -> Option<f32> {
        self.0.spot_intensity(spot.0)
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

    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        let (w, h) = self.0.dimensions();
        format!("Canvas({}, {})", w, h)
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
///
/// Example usage
/// -------------
///
/// ```python
/// from pyplanetarium import Canvas, SpotShape, ImageFormat
///
/// # Draw on a square 256x256 pixel canvas.
/// c = Canvas.new(256, 256)
///
/// # Define a round spot shape with diffraction radius of 2.5 pixels.
/// shape = SpotShape().scale(2.5)
///
/// # Add some spots at random positions with varying shape size
/// # and peak intensity.
/// spot1 = c.add_spot((100.3, 130.8), shape, 0.5)
/// spot2 = c.add_spot((80.6, 200.2), shape.scale(0.5), 0.9)
///
/// # Note: Out of range position coordinates and peak intensities are fine.
/// #       The resulting spot image is clipped into the canvas rectangle.
/// #       Peak intensity > 1.0 leads to saturation to the maximum pixel value.
/// spot3 = c.add_spot((256.1, 3.5), shape.scale(10.0), 1.1)
///
/// # Set the canvas background pixel value.
/// c.set_background(100)
///
/// # Clear the canvas and paint the light spots.
/// c.draw()
///
/// # Export to a 8-bit gamma-compressed grayscale PNG image.
/// png_8bpp_bytes = c.export_image(ImageFormat.PngGamma8Bpp)
///
/// # Export to a 16-bit linear light grayscale PNG image.
/// png_16bpp_bytes = c.export_image(ImageFormat.PngLinear16Bpp)
/// ```
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
