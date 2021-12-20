"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import SpotShape


class ShapeCase(unittest.TestCase):
    """
    pyplanetarium.SpotShape class unit tests
    """

    def test_init(self) -> None:
        """
        Python object instantiation test
        """

        shape1 = SpotShape()
        self.assertIsInstance(shape1, SpotShape)
        self.assertEqual(str(shape1), "[[1, 0], [0, 1]]")
        self.assertEqual(
            repr(shape1), "SpotShape { xx: 1.0, xy: 0.0, yx: 0.0, yy: 1.0 }"
        )

        shape2 = shape1.scale(3.5)
        self.assertNotEqual(shape1, shape2)

        shape3 = SpotShape(3.5)
        self.assertIsInstance(shape3, SpotShape)
        self.assertEqual(str(shape3), "[[3.5, 0], [0, 3.5]]")
        self.assertEqual(
            repr(shape3), "SpotShape { xx: 3.5, xy: 0.0, yx: 0.0, yy: 3.5 }"
        )

        shape4 = SpotShape((3.5, 2.5))
        self.assertIsInstance(shape4, SpotShape)
        self.assertEqual(str(shape4), "[[3.5, 0], [0, 2.5]]")
        self.assertEqual(
            repr(shape4), "SpotShape { xx: 3.5, xy: 0.0, yx: 0.0, yy: 2.5 }"
        )

        shape5 = SpotShape([[3.5, 0.5], [-0.5, 2.5]])
        self.assertIsInstance(shape5, SpotShape)
        self.assertEqual(str(shape5), "[[3.5, 0.5], [-0.5, 2.5]]")
        self.assertEqual(
            repr(shape5), "SpotShape { xx: 3.5, xy: 0.5, yx: -0.5, yy: 2.5 }"
        )

        shape6 = SpotShape(3)
        self.assertIsInstance(shape6, SpotShape)

        shape7 = SpotShape((3, 2))
        self.assertIsInstance(shape7, SpotShape)

        shape8 = SpotShape([[3, 0], [0, 2]])
        self.assertIsInstance(shape8, SpotShape)

    def test_init_err(self) -> None:
        """
        SpotShape instantiation errors test
        """

        with self.assertRaises(TypeError):
            SpotShape({})  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape("1")  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape((1,))  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([1])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([1, 2])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([1, 2, 3])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([[1, 2], 2])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([[1, 2], [2]])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([[1, 2], [2, 3, 4]])  # type: ignore

        with self.assertRaises(TypeError):
            SpotShape([[1, 2], [2, 3], 4])  # type: ignore


if __name__ == "__main__":
    unittest.main()
