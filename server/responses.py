"""Response helper for this app."""
from flask import jsonify


def json_response(code, message, payload):
    """Create a simple json response."""
    if payload is not None:
        data = {
            'message': message,
            'payload': payload,
        }
    else:
        data = {
            'message': message,
        }
    response = jsonify(data)
    response.status_code = code
    return response


def ok(message='', payload=None):
    """Response for HTTP `OK`."""
    return json_response(200, message, payload)


def created(message='', payload=None):
    """Response for HTTP `CREATED`."""
    return json_response(201, message, payload)


def bad_request(message='', payload=None):
    """Response for HTTP `BAD REQUEST`."""
    return json_response(400, message, payload)


def conflict(message='', payload=None):
    """Response for HTTP `CONFLICT`."""
    return json_response(409, message, payload)


def unauthorized(message="Unauthorized", payload=None):
    """Response for HTTP `UNAUTHORIZED`."""
    return json_response(401, message, payload)
