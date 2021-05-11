import redis
import os
import misc.environment

# By default, each Redis instance you create will in turn create its own connection pool

REDIS_HOST = os.getenv("REDIS_HOST")
REDIS_PORT = os.getenv("REDIS_PORT")
REDIS_DB = os.getenv("REDIS_DB")


def make_redis_pool():
    return redis.Redis(host=REDIS_HOST, port=REDIS_PORT, db=REDIS_DB)
