"""Routes for retrieving files. Mostly for dev purposes."""
from flask import current_app

from server import user_bp
from server.helpers.decorators import login_exempt


@user_bp.route('/', methods=['GET'])
@login_exempt
def index():
    """Get the index page."""
    return current_app.send_static_file('index.html')
