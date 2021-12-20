//! Planetarium light spot rendering library bindings for Python.
//!
//! The Python bindings are implemented entirely in Rust using PyO3.

// FIXME: Add a workaround for rustc-1.57 and pyo3-0.15.1 combination.
#![allow(clippy::needless_option_as_deref)]

use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError};
use pyo3::types::PyBytes;

use pyo3::prelude::*;

use planetarium::{EncoderError, Matrix, Matrix23, Pixel, Point, Vector};

use planetarium::{
    Canvas as RsCanvas, ImageFormat as RsImageFormat, SpotId as RsSpotId, SpotShape as RsSpotShape,
    Transform as RsTransform, Window as RsWindow,
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
///
/// Example usage:
///
/// ```python
/// from pyplanetarium import SpotShape
///
/// # Create a unit-sized circular spot.
/// s1 = SpotShape()
///
/// # Upscale 2x
/// s2 = s1.scale(2.0);
///
/// # Stretch by 1.5 in the X direction and rotate clockwise by 45 degrees.
/// s3 = s2.stretch(1.5, 1.0).rotate(-45.0)
/// ```
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct SpotShape(RsSpotShape);

/// Light spot descriptor type
///
/// This class can not be instantiated by Python code.
///
/// `SpotId` objects are created by calling `add_spot()`
/// method of a `Canvas` object.
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct SpotId(RsSpotId);

/// 2D affine transformation definition matrix
///
/// Contains a 2x3 linear transform matrix to be applied
/// to homogenous coordinates internally.
///
/// The Python objects can be created as either:
///
/// - `Transform()` -- the default identity transform
/// - `Transform((sx, sy))` -- the translation transform defined by a vector `(sx, sy)`
/// - `Transform(k)` -- the scaling transform defined by a factor `k`
/// - `Transform([[xx, xy], [yx, yy]])` -- explicit linear transform matrix initialization
/// - `Transform([[xx, xy, sx], [yx, yy, sy]])` -- explicit affine transform matrix initialization
///
/// Example usage:
///
/// ```python
/// from pyplanetarium import Transform
///
/// # Create an identity tranformation.
/// t1 = Transform()
///
/// # Upscale 2x
/// t2 = t1.scale(2.0)
///
/// # Stretch by 1.5 in the X direction and rotate clockwise by 45 degrees.
/// t3 = t2.stretch(1.5, 1.0).rotate(-45.0)
///
/// # Translate by a vector (10, 25)
/// t4 = t3.translate((10, 25))
///
/// # Compose t4 and t2 as [t2][t4]
/// t5 = t4.compose(t2)
/// ```
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct Transform(RsTransform);

/// Canvas image window coordinates
///
/// Defines a rectangular window on the canvas to export the image from.
///
/// The window origin is defined by its upper left corner.
///
/// The window object can be constructed from a nested tuple `((x, y), (w, h))`,
/// where `(w, h)` is the window rectangle dimensions and `(x, y)` is
/// the window origin coordinates.
///
/// Example usage:
///
/// ```python
/// from pyplanetarium import Window
///
/// # Create a new 128x64 window with origin at (100, 200).
/// wnd1 = Window(((100, 200), (128, 64)));
///
/// # Create a new rectangular window with origin at (0, 0).
/// wnd2 = Window.new(128, 64);
///
/// # Move the window origin to (250, 150).
/// wnd3 = wnd2.at(250, 150);
/// ```
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct Window(RsWindow);

/// Exportable canvas image formats
///
/// Python class wrapper for Rust `ImageFormat` enum:
///
///   `RawGamma8Bpp`     8-bit gamma-compressed grayscale RAW
///
///   `RawLinear10BppLE` 10-bit linear light grayscale little-endian RAW
///
///   `RawLinear12BppLE` 12-bit linear light grayscale little-endian RAW
///
///   `PngGamma8Bpp`     8-bit gamma-compressed grayscale PNG
///
///   `PngLinear16Bpp`   16-bit linear light grayscale PNG
#[pyclass(module = "pyplanetarium", freelist = 8)]
struct ImageFormat(RsImageFormat);

/// Opaque light spots drawing canvas object
///
/// Generates the synthesized image containing multiple light spots.
///
/// `Canvas` objects can only be created via a static constructor
/// method `new(width, height)`.
///
/// Example usage:
///
/// ```python
/// from pyplanetarium import Canvas
///
/// # Draw on a square 256x256 pixel canvas.
/// c = Canvas.new(256, 256)
///
/// # Set the canvas background pixel value.
/// c.set_background(100)
///
/// ...
///
/// # Clear the canvas and paint the light spots.
/// c.draw()
/// ```
#[pyclass(module = "pyplanetarium")]
struct Canvas(RsCanvas);

#[pymethods]
impl SpotShape {
    #[new]
    fn new(src: Option<&PyAny>) -> PyResult<Self> {
        if let Some(src) = src {
            if let Ok(k) = src.extract::<f32>() {
                Ok(SpotShape(k.into()))
            } else if let Ok(kxy) = src.extract::<(f32, f32)>() {
                Ok(SpotShape(kxy.into()))
            } else if let Ok(mat) = src.extract::<Matrix>() {
                Ok(SpotShape(mat.into()))
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

    /// Linearly stretches the spot shape in X and Y directions.
    #[pyo3(text_signature = "(kx, ky, /)")]
    fn stretch(&self, kx: f32, ky: f32) -> SpotShape {
        SpotShape(self.0.stretch(kx, ky))
    }

    /// Rotates the spot shape counter-clockwise by `phi` degrees.
    #[pyo3(text_signature = "(phi, /)")]
    fn rotate(&self, phi: f32) -> SpotShape {
        SpotShape(self.0.rotate(phi))
    }

    /// Implements `str(x)` in Python.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pymethods]
impl SpotId {
    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        format!("SpotId({})", self.0)
    }

    /// Implements `hash(x)` in Python.
    fn __hash__(&self) -> usize {
        self.0 as usize
    }
}

#[pymethods]
impl Transform {
    #[new]
    fn new(src: Option<&PyAny>) -> PyResult<Self> {
        if let Some(src) = src {
            if let Ok(k) = src.extract::<f32>() {
                Ok(Transform(k.into()))
            } else if let Ok(shift) = src.extract::<Vector>() {
                Ok(Transform(shift.into()))
            } else if let Ok(mat) = src.extract::<Matrix>() {
                Ok(Transform(mat.into()))
            } else if let Ok(mat) = src.extract::<Matrix23>() {
                Ok(Transform(mat.into()))
            } else {
                Err(PyTypeError::new_err(format!(
                    "Unexpected initializer type: '{}'",
                    src.get_type().name().unwrap()
                )))
            }
        } else {
            Ok(Transform(RsTransform::default()))
        }
    }

    /// Linearly translates the output coordinates by a shift vector.
    #[pyo3(text_signature = "(shift, /)")]
    fn translate(&self, shift: Vector) -> Transform {
        Transform(self.0.translate(shift))
    }

    /// Linearly scales the spot shape by a single scalar factor.
    #[pyo3(text_signature = "(k, /)")]
    fn scale(&self, k: f32) -> Transform {
        Transform(self.0.scale(k))
    }

    /// Linearly stretches the spot shape in X and Y directions.
    #[pyo3(text_signature = "(kx, ky, /)")]
    fn stretch(&self, kx: f32, ky: f32) -> Transform {
        Transform(self.0.stretch(kx, ky))
    }

    /// Rotates the spot shape counter-clockwise by `phi` degrees.
    #[pyo3(text_signature = "(phi, /)")]
    fn rotate(&self, phi: f32) -> Transform {
        Transform(self.0.rotate(phi))
    }

    /// Composes the coordinate transformation with an outer transformation.
    ///
    /// In the matrix multiplication form: `[t][self]`
    #[pyo3(text_signature = "(t, /)")]
    fn compose(&self, t: &Transform) -> Transform {
        Transform(self.0.compose(t.0))
    }

    /// Implements `str(x)` in Python.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    /// Implements `repr(x)` in Python.
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pymethods]
impl Window {
    #[new]
    fn new_(src: ((u32, u32), (u32, u32))) -> Self {
        Window(src.into())
    }

    /// Creates a new window with given dimensions located at the origin.
    #[staticmethod]
    #[pyo3(text_signature = "(width, height, /)")]
    fn new(width: u32, height: u32) -> Self {
        Window(RsWindow::new(width, height))
    }

    /// Moves the window origin to the given origin coordinates.
    #[pyo3(text_signature = "(x, y, /)")]
    fn at(&self, x: u32, y: u32) -> Window {
        Window(self.0.at(x, y))
    }

    /// Implements `str(x)` in Python.
    fn __str__(&self) -> String {
        self.0.to_string()
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

    /// Implements `hash(x)` in Python.
    fn __hash__(&self) -> usize {
        self.0 as usize
    }
}

/// Converts `EncoderError` to Python exceptions.
fn my_to_pyerr(err: EncoderError) -> PyErr {
    match err {
        EncoderError::BrokenWindow => PyValueError::new_err("window is out of bounds".to_string()),
        _ => PyNotImplementedError::new_err(err.to_string()),
    }
}

#[pymethods]
impl Canvas {
    /// `Pixel::MAX` alias
    #[classattr]
    const PIXEL_MAX: Pixel = Pixel::MAX;

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
    ///
    /// The dark pixel value must be in the range
    /// 0 to `Canvas.PIXEL_MAX` inclusive.
    #[pyo3(text_signature = "($self, level, /)")]
    fn set_background(&mut self, level: Pixel) {
        self.0.set_background(level);
    }

    /// Sets the world coordinates to canvas coordinates transformation.
    ///
    /// The light spot coordinates are defined in the world coordinate system only.
    #[pyo3(text_signature = "($self, transform, /)")]
    fn set_view_transform(&mut self, transform: &Transform) {
        self.0.set_view_transform(transform.0)
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
            Err(e) => Err(my_to_pyerr(e)),
        }
    }

    /// Exports the canvas window contents in the requested image format.
    #[pyo3(text_signature = "($self, window, format, /)")]
    fn export_window_image(
        &self,
        window: &Window,
        format: &ImageFormat,
        py: Python,
    ) -> PyResult<Py<PyBytes>> {
        match self.0.export_window_image(window.0, format.0) {
            Ok(b) => Ok(PyBytes::new(py, b.as_slice()).into()),
            Err(e) => Err(my_to_pyerr(e)),
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
/// from pyplanetarium import Canvas, SpotShape, ImageFormat, Window
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
/// c.set_background(int(0.1 * Canvas.PIXEL_MAX))
///
/// # Clear the canvas and paint the light spots.
/// c.draw()
///
/// # Export to a 8-bit gamma-compressed grayscale PNG image.
/// png_8bpp_bytes = c.export_image(ImageFormat.PngGamma8Bpp)
///
/// # Export to a 16-bit linear light grayscale PNG image.
/// png_16bpp_bytes = c.export_image(ImageFormat.PngLinear16Bpp)
///
/// # Export a rectangular canvas window to a 8-bit gamma-compressed RAW image.
/// wnd = Window.new(64, 32).at(90, 120)
/// raw_window_8bpp_bytes = c.export_window_image(wnd, ImageFormat.PngGamma8Bpp)
/// ```
#[pymodule]
fn pyplanetarium(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add module version attributes.
    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", env!("CARGO_PKG_AUTHORS"))?;

    m.add_class::<SpotShape>()?;
    m.add_class::<SpotId>()?;
    m.add_class::<Transform>()?;
    m.add_class::<Window>()?;
    m.add_class::<ImageFormat>()?;
    m.add_class::<Canvas>()?;

    Ok(())
}
