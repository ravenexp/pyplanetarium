"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import SpotShape, SpotId, Canvas


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

        canvas.set_spot_offset(spot2, (5.5, -7.0))
        canvas.set_spot_illumination(spot2, 0.5)

        canvas.set_background(6000)
        canvas.set_brightness(1.3)

        self.assertNotEqual(spot1, spot2)

        canvas.draw()


if __name__ == "__main__":
    unittest.main()
