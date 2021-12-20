PyPlanetarium
=============

Python bindings for **Planetarium** sub-pixel precision light spot rendering
library for astronomy and video tracking applications.

Example usage
-------------

```python
from pyplanetarium import Canvas, SpotShape, ImageFormat, Window

# Draw on a square 256x256 pixel canvas.
c = Canvas.new(256, 256)

# Define a round spot shape with diffraction radius of 2.5 pixels.
shape = SpotShape().scale(2.5)

# Add some spots at random positions with varying shape size
# and peak intensity.
spot1 = c.add_spot((100.3, 130.8), shape, 0.5)
spot2 = c.add_spot((80.6, 200.2), shape.scale(0.5), 0.9)

# Note: Out of range position coordinates and peak intensities are fine.
#       The resulting spot image is clipped into the canvas rectangle.
#       Peak intensity > 1.0 leads to saturation to the maximum pixel value.
spot3 = c.add_spot((256.1, 3.5), shape.scale(10.0), 1.1)

# Set the canvas background pixel value.
c.set_background(int(0.05 * Canvas.PIXEL_MAX))

# Clear the canvas and paint the light spots.
c.draw()

# Export to a 8-bit gamma-compressed grayscale PNG image.
png_8bpp_bytes = c.export_image(ImageFormat.PngGamma8Bpp)

# Export to a 16-bit linear light grayscale PNG image.
png_16bpp_bytes = c.export_image(ImageFormat.PngLinear16Bpp)

# Export a rectangular canvas window to a 8-bit gamma-compressed RAW image.
wnd = Window.new(64, 32).at(90, 120)
raw_window_8bpp_bytes = c.export_window_image(wnd, ImageFormat.PngGamma8Bpp)
```
