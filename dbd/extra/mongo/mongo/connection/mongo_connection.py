from pymongo import MongoClient
from pymongo.errors import ConnectionFailure
from dotenv import load_dotenv
import os

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(ROOT_DIR)
ROOT_DIR = os.path.dirname(ROOT_DIR)
ENV_FILE_PATH = ROOT_DIR + "/.env"

# Considering containerized solution to solve portability issues


def mongo_connection_string(containerized=False):
    print(ENV_FILE_PATH)
    if not containerized:
        load_dotenv(dotenv_path=ENV_FILE_PATH)
        conn_str = f'mongodb://{os.getenv("MONGO_USERNAME")}:{os.getenv("MONGO_PASSWORD")}@{os.getenv("MONGO_HOST")}:{os.getenv("MONGO_PORT")}'
        return conn_str
    else:
        print("Containerized conn string not implemented. Aborting.")
        raise Exception

test = {
    "thing":"stuff"
}


def get_mongo_connection(containerized=False):
    conn_str = mongo_connection_string(containerized)
    print(conn_str)
    
    client = MongoClient(conn_str)
    try:
        db = client[os.getenv("MONGO_DATABASE")]
        things = db.things
        thing_id = things.insert_one(test).inserted_id
        print(thing_id)
    except ConnectionFailure:
        print("Server not available")
    
