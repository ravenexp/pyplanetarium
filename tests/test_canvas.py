"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import SpotShape, SpotId, Canvas, ImageFormat


class CanvasCase(unittest.TestCase):
    """
    pyplanetarium.Canvas class unit tests
    """

    def test_init(self) -> None:
        """
        SpotShape and Canvas instantiation test
        """

        shape1 = SpotShape()
        self.assertIsInstance(shape1, SpotShape)

        shape2 = shape1.scale(3.5)
        self.assertNotEqual(shape1, shape2)

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        self.assertIsInstance(canvas, Canvas)
        self.assertEqual(canvas.dimensions(), (width, height))

        canvas.set_background(1000)
        canvas.clear()

    def test_draw_spots(self) -> None:
        """
        Light spots image drawing test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape().scale(5.5)

        width = 1024
        height = 768

        canvas = Canvas.new(width, height)
        self.assertEqual(canvas.dimensions(), (width, height))

        spot1 = canvas.add_spot((100.5, 200.7), shape1, 0.8)
        self.assertIsInstance(spot1, SpotId)

        spot2 = canvas.add_spot((400.5, 600.7), shape2, 0.6)
        self.assertIsInstance(spot2, SpotId)

        self.assertNotEqual(spot1, spot2)

        canvas.set_background(1000)

        canvas.draw()

    def test_move_spots(self) -> None:
        """
        Light spots moving test
        """

        shape1 = SpotShape().scale(3.5)
        shape2 = SpotShape().scale(5.5)

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

        canvas.set_background(6000)
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


if __name__ == "__main__":
    unittest.main()
