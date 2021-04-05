from testing_covid_hypothesis import __version__
import unittest
from testing_covid_hypothesis.aliases import REGIONAL_FILE_TREE, COMMUNAL_FILE_TREE


def check_file_exists(file):
    try:
        f = open(REGIONAL_FILE_TREE.get("noegle_tal"))
    except IOError:
        print("File not found..")
        raise Exception
    finally:
        f.close()


class Tests(unittest.TestCase):
    def test_version(self):
        assert __version__ == '0.1.0'

    def test_get_file(self):
        assert(check_file_exists("noegle_tal"), not None)
