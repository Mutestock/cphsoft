import importlib
import unittest
import os

SRC_PATH = "../src"

redis_interactions = importlib.import_module("logic.redis_interactions", SRC_PATH)
environment = importlib.import_module("misc.environment", SRC_PATH)


class TestStuff(unittest.TestCase):
    def test_environment_variable_file_found(self):

        self.assertTrue(os.path.isfile(environment.ENVIRONMENT_VARIABLE_FILE))


class TestRedis(unittest.TestCase):
    def test_redis_set(self):
        redis_interactions.make_redis_pool().set("test_key01", "test_value01")

    def test_redis_set_get(self):
        redis_interactions.make_redis_pool().set("test_key02", "test_value02")
        value = redis_interactions.make_redis_pool().get("test_key02")
        self.assertEqual(value.decode('UTF-8'), "test_value02")
