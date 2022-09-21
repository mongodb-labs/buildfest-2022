from flask import Blueprint, request, jsonify
from flask_cors import CORS

from jft.jira import favorite_filters, filter_count
from jft.db import store_filter, retrieve_filters

from functools import reduce

filters_api = Blueprint(
    'filters_api', 'filters_api', url_prefix='/api/filters')

CORS(filters_api)

@filters_api.route('/', methods=['GET'])
def api_hello_world():
    print("HERE I AM")
    return "Hello, world"

@filters_api.route('/record', methods=['POST'])
def api_record_filter_sizes():
    print("recording filter sizes...")

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

    ids_to_names = reduce(update_ids_to_names, filters, {})
    results = list(retrieve_filters(list(ids_to_names.keys())))

    return jsonify(reduce(lambda acc, x: update_results(acc, x, ids_to_names[x['_id']]), results, []))

def update_ids_to_names(acc, x):
    acc[x.id] = x.name
    return acc

def update_results(acc, x, name):
    x['name'] = name
    return acc + [x]