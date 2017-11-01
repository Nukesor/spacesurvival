"""Response helper for this app."""
from flask import jsonify


def json_response(code, data=None):
    """Create a simple json response."""
    response = jsonify(data)
    response.status_code = code
    return response


def ok(data=None):
    """Response for HTTP `OK`."""
    return json_response(200, data)


def created(data=None):
    """Response for HTTP `CREATED`."""
    return json_response(201, data)


def bad_request(data=None):
    """Response for HTTP `BAD REQUEST`."""
    return json_response(400, data)


def conflict(data=None):
    """Response for HTTP `CONFLICT`."""
    return json_response(409, data)


def unauthorized(data=None):
    """Response for HTTP `UNAUTHORIZED`."""
    return json_response(401, data)
