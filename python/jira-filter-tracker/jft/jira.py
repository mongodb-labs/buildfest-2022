from jira import JIRA
import os

key_cert_data = None
with open('./key_cert_data', 'r') as key_cert_file:
    key_cert_data = key_cert_file.read()

config = {
    "access_token": os.getenv('ACCESS_TOKEN'),
    "access_token_secret": os.getenv('ACCESS_TOKEN_SECRET'),
    "consumer_key": os.getenv('CONSUMER_KEY'),
    "key_cert": key_cert_data,
}

jira = JIRA('https://jira.mongodb.org', oauth=config)

def favorite_filters():
    return jira.favourite_filters()

def filter_count(id):
  return jira.search_issues(f'filter={id}', startAt=0, maxResults=0, json_result=True)['total']
