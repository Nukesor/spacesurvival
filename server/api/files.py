from flask import request, send_from_directory, url_for

from server import user_bp

@user_bp.route('/', methods = ['GET'])
def index():
    return app.send_static_file('index.html')


@user_bp.route('/static/<path:path>')
def send_js(path):
    return app.send_static_file(path)
