import os
from dotenv import load_dotenv

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(ROOT_DIR)
ROOT_DIR = os.path.dirname(ROOT_DIR)

ENVIRONMENT_VARIABLE_FILE = ROOT_DIR + "/.env.development"

if not os.getenv("CONTAINERIZED_VARIABLES"):
    load_dotenv(ENVIRONMENT_VARIABLE_FILE)
