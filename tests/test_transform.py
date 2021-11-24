"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import Transform


class TransformCase(unittest.TestCase):
    """
    pyplanetarium.Transform class unit tests
    """

    def test_init(self) -> None:
        """
        Transform instantiation test
        """

        tr1 = Transform()
        self.assertIsInstance(tr1, Transform)
        self.assertEqual(str(tr1), "[[1, 0, 0], [0, 1, 0]]")
        self.assertEqual(
            repr(tr1),
            "Transform { xx: 1.0, xy: 0.0, yx: 0.0, yy: 1.0, tx: 0.0, ty: 0.0 }",
        )

        tr2 = tr1.scale(3.5)
        self.assertNotEqual(tr1, tr2)

        tr3 = Transform(3.5)
        self.assertEqual(str(tr3), "[[3.5, 0, 0], [0, 3.5, 0]]")
        self.assertEqual(
            repr(tr3),
            "Transform { xx: 3.5, xy: 0.0, yx: 0.0, yy: 3.5, tx: 0.0, ty: 0.0 }",
        )

        tr4 = Transform((3.5, 2.5))
        self.assertEqual(str(tr4), "[[1, 0, 3.5], [0, 1, 2.5]]")
        self.assertEqual(
            repr(tr4),
            "Transform { xx: 1.0, xy: 0.0, yx: 0.0, yy: 1.0, tx: 3.5, ty: 2.5 }",
        )

        tr5 = Transform([[3.5, 0.5], [-0.5, 2.5]])
        self.assertEqual(str(tr5), "[[3.5, 0.5, 0], [-0.5, 2.5, 0]]")

        tr6 = Transform([[3.5, 0.5, 5.25], [-0.5, 2.5, -14.75]])
        self.assertEqual(str(tr6), "[[3.5, 0.5, 5.25], [-0.5, 2.5, -14.75]]")

        tr7 = Transform(3)
        self.assertIsInstance(tr7, Transform)

        tr8 = Transform((3, 2))
        self.assertIsInstance(tr8, Transform)

        tr9 = Transform([[3, 0], [0, 2]])
        self.assertIsInstance(tr9, Transform)

        tr10 = Transform([[3, 0, 10], [0, 2, 5]])
        self.assertIsInstance(tr10, Transform)

    def test_init_err(self) -> None:
        """
        Transform instantiation errors test
        """

        with self.assertRaises(TypeError):
            Transform({})  # type: ignore

        with self.assertRaises(TypeError):
            Transform("1")  # type: ignore

        with self.assertRaises(TypeError):
            Transform((1,))  # type: ignore

        with self.assertRaises(TypeError):
            Transform([])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([1])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([1, 2])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([1, 2, 3])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([[1, 2], 2])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([[1, 2], [2]])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([[1, 2], [2, 3, 4]])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([[1, 2], [2, 3], 4])  # type: ignore

        with self.assertRaises(TypeError):
            Transform([[1, 2, 3, 4], [2, 3, 4]])  # type: ignore

    def test_ops(self) -> None:
        """
        Transform operations test
        """

        tr1 = Transform()
        self.assertEqual(str(tr1), "[[1, 0, 0], [0, 1, 0]]")

        tr2 = tr1.scale(2.5)
        self.assertEqual(str(tr2), "[[2.5, 0, 0], [0, 2.5, 0]]")

        tr3 = tr2.translate((5.5, -4.25))
        self.assertEqual(str(tr3), "[[2.5, 0, 5.5], [0, 2.5, -4.25]]")

        tr4 = tr3.stretch(2.0, 1.5)
        self.assertEqual(str(tr4), "[[5, 0, 11], [0, 3.75, -6.375]]")

        tr5 = Transform((5.0, -10.0))

        tr6 = tr5.scale(2.0).rotate(45)

        tr7 = tr6.compose(tr4).translate((10, -10))

        tr8 = tr7.compose(tr3).compose(Transform())
        self.assertIsInstance(tr8, Transform)


if __name__ == "__main__":
    unittest.main()
