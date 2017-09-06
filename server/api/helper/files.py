from flask import request, send_from_directory, url_for
from server import app


@app.route('/', methods = ['GET'])
def index():
    return app.send_static_file('index.html')


@app.route('/static/<path:path>')
def send_js(path):
    return app.send_static_file(path)