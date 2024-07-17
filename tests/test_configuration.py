# This file is part of GreenMoon2D, a 2D game engine for Python
# written by Willi Kappler, MIT license.
#
# See: https://github.com/willi-kappler/green_moon_2d

import unittest
import pathlib

from green_moon_2d.gm_configuration import GMConfiguration


class TestConfiguration(unittest.TestCase):
    def create_and_load_config(self, data: str) -> GMConfiguration:
        config_file_name: str = "test_config1.json"

        with open(config_file_name, "w") as f:
            f.write(data)

        cfg: GMConfiguration = GMConfiguration()
        cfg.load_config(config_file_name)
        p = pathlib.Path(config_file_name)
        p.unlink()

        return cfg

    def test_load_configuration1(self):
        """
        Test loading of all configuration values.
        """

        data = """
        {
          "screen_width": 678,
          "screen_height": 456,
          "fullscreen": true,
          "window_title": "Test1",
          "fps": 77,
          "resource_file": "test_resources.json"
        }
        """

        cfg = self.create_and_load_config(data)

        self.assertEqual(cfg.screen_width, 678)
        self.assertEqual(cfg.screen_height, 456)
        self.assertEqual(cfg.fullscreen, True)
        self.assertEqual(cfg.window_title, "Test1")
        self.assertEqual(cfg.fps, 77)
        self.assertEqual(cfg.resource_file, "test_resources.json")

    def test_load_configuration2(self):
        """
        Test loading of no configuration values.
        """

        data = """
        {
          "foo": "bar"
        }
        """

        cfg = self.create_and_load_config(data)

        self.assertEqual(cfg.screen_width, 800)
        self.assertEqual(cfg.screen_height, 600)
        self.assertEqual(cfg.fullscreen, False)
        self.assertEqual(cfg.window_title, "Made with GreenMoon2D")
        self.assertEqual(cfg.fps, 60)
        self.assertEqual(cfg.resource_file, "resources.json")

    def test_load_configuration3(self):
        """
        Test loading of some configuration values.
        """

        data = """
        {
          "screen_width": 1200,
          "resource_file": "foo_bar1.json"
        }
        """

        cfg = self.create_and_load_config(data)

        self.assertEqual(cfg.screen_width, 1200)
        self.assertEqual(cfg.screen_height, 600)
        self.assertEqual(cfg.fullscreen, False)
        self.assertEqual(cfg.window_title, "Made with GreenMoon2D")
        self.assertEqual(cfg.fps, 60)
        self.assertEqual(cfg.resource_file, "foo_bar1.json")


if __name__ == "__main__":
    unittest.main()


