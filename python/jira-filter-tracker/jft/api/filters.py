from flask import Blueprint, request, jsonify
from flask_cors import CORS

from jft.jira import favorite_filters, filter_count
from jft.db import store_filter, retrieve_filters

filters_api = Blueprint(
    'filters_api', 'filters_api', url_prefix='/api/filters')

CORS(filters_api)

@filters_api.route('/', methods=['GET'])
def api_hello_world():
    print("HERE I AM")
    return "Hello, world"

@filters_api.route('/record', methods=['POST'])
def api_record_filter_sizes():
    # 1. get favorite filters
    filters = favorite_filters()

    # 2. for each filter, get the length
    for filter in filters:
        id = filter.id
        count = filter_count(id)

        # 3. store it in the collection
        store_filter(id, count)

    return "done"

@filters_api.route('/data', methods=['GET'])
def api_get_filters():
    filters = favorite_filters()
    ids = list(map(lambda x: x.id, filters))
    return jsonify(partition_filters(retrieve_filters(ids)))

def partition_filters(filters):
    res = {}
    for filter in filters:
        id = filter['metadata']['id']
        res[id] = res.get(id, []) + [ filter ]
    return res
