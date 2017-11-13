"""Flask app handler."""
from flask import g, request
from datetime import datetime
from server.extensions import db
from server.models.user import User
from server.responses import (
    bad_request,
    unauthorized,
)


def register_handlers(app):
    """Register app handlers."""
    @app.before_request
    def require_json_input():
        """Require JSON input.

        If the request's method is either 'POST' or 'PUT', require the
        'Content-Type' to be JSON.
        """
        if request.method in ['POST', 'PUT']:
            if request.headers['Content-Type'] != 'application/json':
                return bad_request("'Content-Type' must be 'application/json'.")

    @app.before_request
    def default_login_required():
        # If this is an error or something else without a proper endpoint, just
        # return straight away.
        if not request.endpoint:
            return

        view = app.view_functions[request.endpoint]

        if getattr(view, 'login_exempt', False):
            return

        token_header = request.headers.get('Authorization')
        if not token_header:
            return bad_request("'Authorization' not found in headers.")

        token = token_header
        user = User.get_user_from_login_token(token)

        if not user:
            return bad_request("Invalid login token.")

        # Require user to re-login if the last action is too long ago.
        if not user.has_valid_auth_token:
            return unauthorized("Auth token too old, please log in again.")

        # At this point the user is considered successfully authenticated.
        user.last_action = datetime.utcnow()
        db.session.add(user)
        db.session.commit()
        g.current_user = db.session.query(User).get(user.id)

    @app.after_request
    def add_cors_headers(response):
        """Add CORS to the headers of this request."""
        response.headers['Access-Control-Allow-Origin'] = app.config['CORS_ALLOW_ORIGIN']
        response.headers['Access-Control-Allow-Methods'] = app.config['CORS_ALLOW_METHODS']
        response.headers['Access-Control-Allow-Headers'] = app.config['CORS_ALLOW_HEADERS']
        return response
