"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import SpotShape, SpotId, Transform, Canvas, ImageFormat, Window


class CanvasCase(unittest.TestCase):
    """
    pyplanetarium.Canvas class unit tests
    """

    def test_init(self) -> None:
        """
        Canvas object instantiation test
        """

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        self.assertIsInstance(canvas, Canvas)
        self.assertEqual(canvas.dimensions(), (width, height))

        self.assertEqual(str(canvas), "Canvas(1024, 768)")
        self.assertEqual(repr(canvas), "Canvas(1024, 768)")

        canvas.set_background(1000)
        canvas.clear()

    def test_init_err(self) -> None:
        """
        Canvas instantiation errors test
        """

        with self.assertRaises(TypeError):
            Canvas.new()  # type: ignore

        with self.assertRaises(TypeError):
            Canvas.new(100)  # type: ignore

        with self.assertRaises(TypeError):
            Canvas.new((100, 200))  # type: ignore

    def test_enum_repr(self) -> None:
        """
        Enum variants string representation test
        """

        self.assertEqual(str(ImageFormat.PngGamma8Bpp), "PngGamma8Bpp")
        self.assertEqual(repr(ImageFormat.PngGamma8Bpp), "PngGamma8Bpp")
        self.assertEqual(repr(ImageFormat.PngLinear16Bpp), "PngLinear16Bpp")
        self.assertEqual(repr(ImageFormat.RawGamma8Bpp), "RawGamma8Bpp")
        self.assertEqual(repr(ImageFormat.RawLinear10BppLE), "RawLinear10BppLE")
        self.assertEqual(repr(ImageFormat.RawLinear12BppLE), "RawLinear12BppLE")

    def test_enum_hash(self) -> None:
        """
        Enum variants as dict keys test
        """

        self.assertEqual(hash(ImageFormat.PngGamma8Bpp), 3)

        formats = {}
        formats[ImageFormat.PngGamma8Bpp] = "PNG8"
        formats[ImageFormat.PngLinear16Bpp] = "PNG16"

        self.assertEqual(formats[ImageFormat.PngGamma8Bpp], "PNG8")
        self.assertEqual(formats[ImageFormat.PngLinear16Bpp], "PNG16")

    def test_draw_spots(self) -> None:
        """
        Light spots image drawing test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape(5.5).stretch(1.5, 1.0).rotate(45)

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        self.assertEqual(canvas.dimensions(), (width, height))

        spot1 = canvas.add_spot((100.5, 200.7), shape1, 0.8)
        self.assertIsInstance(spot1, SpotId)
        self.assertEqual(str(spot1), "SpotId(0)")

        spot2 = canvas.add_spot((400.5, 600.7), shape2, 0.6)
        self.assertIsInstance(spot2, SpotId)
        self.assertEqual(repr(spot2), "SpotId(1)")

        self.assertNotEqual(spot1, spot2)

        canvas.set_background(int(0.1 * Canvas.PIXEL_MAX))

        canvas.draw()

    def test_move_spots(self) -> None:
        """
        Light spots moving test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape(5.5).stretch(1.0, 1.5).rotate(30)

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        spot1 = canvas.add_spot((100.5, 200.7), shape1, 0.8)
        spot2 = canvas.add_spot((400.5, 600.7), shape2, 0.6)

        pos1 = canvas.spot_position(spot1)
        pos2 = canvas.spot_position(spot2)
        assert pos1 is not None
        assert pos2 is not None
        self.assertAlmostEqual(pos1[0], 100.5, 4)
        self.assertAlmostEqual(pos2[1], 600.7, 4)

        int1 = canvas.spot_intensity(spot1)
        int2 = canvas.spot_intensity(spot2)
        assert int1 is not None
        assert int2 is not None
        self.assertAlmostEqual(int1, 0.8, 4)
        self.assertAlmostEqual(int2, 0.6, 4)

        canvas.set_spot_offset(spot2, (5.5, -7.0))
        canvas.set_spot_illumination(spot2, 0.5)

        canvas.set_background(int(0.2 * Canvas.PIXEL_MAX))
        canvas.set_brightness(1.3)

        pos2 = canvas.spot_position(spot2)
        assert pos2 is not None
        self.assertAlmostEqual(pos2[0], 400.5 + 5.5, 4)
        self.assertAlmostEqual(pos2[1], 600.7 - 7.0, 4)

        int1 = canvas.spot_intensity(spot1)
        int2 = canvas.spot_intensity(spot2)
        assert int1 is not None
        assert int2 is not None
        self.assertAlmostEqual(int1, 0.8 * 1.3, 4)
        self.assertAlmostEqual(int2, 0.6 * 0.5 * 1.3, 4)

        canvas.draw()

    def test_view_transform(self) -> None:
        """
        Setting the canvas view transform test
        """

        shape1 = SpotShape([[1, -0.5], [0.5, 1.5]])
        shape2 = SpotShape(5.5).stretch(1.0, 1.5).rotate(30)

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        spot1 = canvas.add_spot((100.5, 200.25), shape1, 0.8)
        spot2 = canvas.add_spot((400.5, 600.75), shape2, 0.6)

        pos1 = canvas.spot_position(spot1)
        pos2 = canvas.spot_position(spot2)
        assert pos1 is not None
        assert pos2 is not None
        self.assertAlmostEqual(pos1[0], 100.5, 4)
        self.assertAlmostEqual(pos1[1], 200.25, 4)
        self.assertAlmostEqual(pos2[0], 400.5, 4)
        self.assertAlmostEqual(pos2[1], 600.75, 4)

        canvas.set_view_transform(Transform())

        canvas.set_spot_offset(spot2, (5.5, -7.0))
        pos2 = canvas.spot_position(spot2)
        assert pos2 is not None
        self.assertAlmostEqual(pos2[0], 400.5 + 5.5, 4)
        self.assertAlmostEqual(pos2[1], 600.75 - 7.0, 4)

        canvas.set_view_transform(Transform((-10, 25)))

        pos1 = canvas.spot_position(spot1)
        pos2 = canvas.spot_position(spot2)
        assert pos1 is not None
        assert pos2 is not None
        self.assertAlmostEqual(pos1[0], 100.5 - 10, 4)
        self.assertAlmostEqual(pos1[1], 200.25 + 25, 4)
        self.assertAlmostEqual(pos2[0], 400.5 + 5.5 - 10, 4)
        self.assertAlmostEqual(pos2[1], 600.75 - 7.0 + 25, 4)

        xfrm = Transform((-100, 200)).rotate(45).compose(Transform([[-1, 0], [0, 1]]))

        canvas.set_view_transform(xfrm)
        pos1 = canvas.spot_position(spot1)
        pos2 = canvas.spot_position(spot2)
        assert pos1 is not None
        assert pos2 is not None
        self.assertAlmostEqual(pos1[0], 282.6659, 4)
        self.assertAlmostEqual(pos1[1], 283.373, 4)
        self.assertAlmostEqual(pos2[0], 344.8913, 4)
        self.assertAlmostEqual(pos2[1], 777.6407, 4)

        canvas.draw()

    def test_spot_hash(self) -> None:
        """
        Opaque spot identifiers as dict keys test
        """

        canvas = Canvas.new(10, 10)
        spot1 = canvas.add_spot((1.0, 1.0), SpotShape(), 0.8)
        spot2 = canvas.add_spot((2.0, 2.0), SpotShape(2.0), 0.6)

        self.assertEqual(hash(spot1), 0)
        self.assertEqual(hash(spot2), 1)

        spots = {}
        spots[spot1] = "Spot1"
        spots[spot2] = "Spot2"

        self.assertEqual(spots[spot1], "Spot1")
        self.assertEqual(spots[spot2], "Spot2")

    def test_export_images(self) -> None:
        """
        Canvas image export test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape().scale(5.5)

        width = 256
        height = 256

        canvas = Canvas.new(width, height)

        spot1 = canvas.add_spot((180.5, 150.7), shape1, 0.8)
        spot2 = canvas.add_spot((100.5, 110.7), shape2, 0.6)

        self.assertNotEqual(spot1, spot2)

        canvas.set_background(5000)

        canvas.draw()

        raw8_bytes = canvas.export_image(ImageFormat.RawGamma8Bpp)
        self.assertIsInstance(raw8_bytes, bytes)
        self.assertEqual(len(raw8_bytes), 65536)
        self.assertEqual(raw8_bytes[0], 78)
        self.assertEqual(raw8_bytes[65535], 78)

        raw10_bytes = canvas.export_image(ImageFormat.RawLinear10BppLE)
        self.assertIsInstance(raw10_bytes, bytes)
        self.assertEqual(len(raw10_bytes), 2 * 65536)
        self.assertEqual(raw10_bytes[0], 78)
        self.assertEqual(raw10_bytes[1], 0)

        raw12_bytes = canvas.export_image(ImageFormat.RawLinear12BppLE)
        self.assertIsInstance(raw12_bytes, bytes)
        self.assertEqual(len(raw12_bytes), 2 * 65536)
        self.assertEqual(raw12_bytes[0], 56)
        self.assertEqual(raw12_bytes[1], 1)

        png8_bytes = canvas.export_image(ImageFormat.PngGamma8Bpp)
        self.assertIsInstance(png8_bytes, bytes)
        self.assertEqual(len(png8_bytes), 949)

        png16_bytes = canvas.export_image(ImageFormat.PngLinear16Bpp)
        self.assertIsInstance(png16_bytes, bytes)
        self.assertEqual(len(png16_bytes), 1816)

        # with open("image8.raw", "wb") as f:
        #     f.write(raw8_bytes)

        # with open("image10.raw", "wb") as f:
        #     f.write(raw10_bytes)

        # with open("image12.raw", "wb") as f:
        #     f.write(raw12_bytes)

        # with open("image8.png", "wb") as f:
        #     f.write(png8_bytes)

        # with open("image16.png", "wb") as f:
        #     f.write(png16_bytes)

    def test_export_window_images(self) -> None:
        """
        Windowed canvas image export test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape().scale(5.5)

        width = 256
        height = 256

        canvas = Canvas.new(width, height)

        spot1 = canvas.add_spot((180.5, 150.7), shape1, 0.8)
        spot2 = canvas.add_spot((100.5, 110.7), shape2, 0.6)

        self.assertNotEqual(spot1, spot2)

        canvas.set_background(5000)

        canvas.draw()

        wnd1 = Window.new(32, 16).at(170, 140)
        wnd2 = Window.new(32, 16).at(90, 100)

        raw8_bytes = canvas.export_window_image(wnd1, ImageFormat.RawGamma8Bpp)
        self.assertIsInstance(raw8_bytes, bytes)
        self.assertEqual(len(raw8_bytes), 32 * 16)

        raw10_bytes = canvas.export_window_image(wnd1, ImageFormat.RawLinear10BppLE)
        self.assertIsInstance(raw10_bytes, bytes)
        self.assertEqual(len(raw10_bytes), 2 * 32 * 16)

        raw12_bytes = canvas.export_window_image(wnd2, ImageFormat.RawLinear12BppLE)
        self.assertIsInstance(raw12_bytes, bytes)
        self.assertEqual(len(raw12_bytes), 2 * 32 * 16)

        png8_bytes = canvas.export_window_image(wnd1, ImageFormat.PngGamma8Bpp)
        self.assertIsInstance(png8_bytes, bytes)
        self.assertEqual(len(png8_bytes), 250)

        png16_bytes = canvas.export_window_image(wnd2, ImageFormat.PngLinear16Bpp)
        self.assertIsInstance(png16_bytes, bytes)
        self.assertEqual(len(png16_bytes), 664)


if __name__ == "__main__":
    unittest.main()
