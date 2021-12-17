"""
pyplanetarium package integration tests
"""

import unittest

from pyplanetarium import Window


class WindowCase(unittest.TestCase):
    """
    pyplanetarium.Window class unit tests
    """

    def test_init(self) -> None:
        """
        Window instantiation test
        """

        wnd = Window(((100, 200), (128, 64)))
        self.assertIsInstance(wnd, Window)

        self.assertEqual(str(wnd), "(100, 200)+(128, 64)")
        self.assertEqual(
            repr(wnd),
            "Window { x: 100, y: 200, w: 128, h: 64 }",
        )

    def test_new_at(self) -> None:
        """
        Window instantiation via new() test
        """

        wnd1 = Window.new(128, 64)
        self.assertIsInstance(wnd1, Window)

        self.assertEqual(str(wnd1), "(0, 0)+(128, 64)")
        self.assertEqual(
            repr(wnd1),
            "Window { x: 0, y: 0, w: 128, h: 64 }",
        )

        wnd2 = wnd1.at(100, 200)
        self.assertIsInstance(wnd2, Window)

        self.assertEqual(str(wnd2), "(100, 200)+(128, 64)")
        self.assertEqual(
            repr(wnd2),
            "Window { x: 100, y: 200, w: 128, h: 64 }",
        )

    def test_init_err(self) -> None:
        """
        Window instantiation errors test
        """

        with self.assertRaises(TypeError):
            Window()  # type: ignore

        with self.assertRaises(TypeError):
            Window(10, 20)  # type: ignore


if __name__ == "__main__":
    unittest.main()
