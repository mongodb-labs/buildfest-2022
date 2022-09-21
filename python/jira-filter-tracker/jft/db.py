from itertools import count
import bson

from flask import current_app, g
from werkzeug.local import LocalProxy
from flask_pymongo import PyMongo

from pymongo.errors import DuplicateKeyError, OperationFailure
from bson.objectid import ObjectId
from bson.errors import InvalidId

from datetime import datetime

def get_db():
    """
    Configuration method to return db instance
    """
    db = getattr(g, "_database", None)

    if db is None:
        db = g._database = PyMongo(current_app).db

    return db

# Use LocalProxy to read the global db instance with just `db`
db = LocalProxy(get_db)

def store_filter(id, count):
    filter_col = db.filter_sizes
    filter_col.insert_one({
        'timestamp': datetime.now(),
        'metadata': { 'id': id, 'count': count }
    })

def retrieve_filters(ids):
    filter_col = db.filter_sizes
    return filter_col.aggregate([
        {
            '$match': {
                'metadata.id': {
                    '$in': ids
                }
            }
        }, {
            '$group': {
                '_id': '$metadata.id',
                'data': {
                    '$push': '$metadata'
                }
            }
        }
    ])
