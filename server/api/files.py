"""Routes for retrieving files. Mostly for dev purposes."""
from flask import current_app

from server import user_bp
from server.helpers.decorators import login_exempt


@user_bp.route('/', methods=['GET'])
@login_exempt
def index():
    """Get the index page."""
    print(current_app.static_folder)
    return current_app.send_static_file('index.html')


@user_bp.route('/static/<path:path>')
@login_exempt
def send_static(path):
    """Any static file."""
    return current_app.send_static_file(path)
