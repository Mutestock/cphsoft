from logic.redis_interactions import make_redis_pool
import os

from flask import Flask

app = Flask(__name__)


@app.route("/")
def index():
    return "Hi from the redis example thing"


@app.route("/get/<string:get_value>")
def redis_get_example(get_value):
    return make_redis_pool().get(get_value)


@app.route("/set/<string:key>=<string:value>")
def redis_set_example(key, value):
    make_redis_pool().set(key, value)
    return f"{key} set to {value}"


if __name__ == "__main__":
    # Without docker
    if not os.getenv("CONTAINERIZED_VARIABLES"):
        app.run(host="0.0.0.0", port="9123", debug=True)
    # With docker
    else:
        app.run(host="0.0.0.0", debug=True)
