from flask import Response
from flask import jsonify

def json_response(code, message):
    response = jsonify(message)
    response.status_code = code
    return response

def ok(message=''):
    """Response for HTTP `OK`."""
    return json_response(200, message)

def created(message=''):
    """Response for HTTP `CREATED`."""
    return json_response(201, message)

def bad_request(message=''):
    """Response for HTTP `BAD REQUEST`."""
    return json_response(400, message)

def conflict(message=''):
    """Response for HTTP `CONFLICT`."""
    return json_response(409, message)

def unauthorized(message="Unauthorized"):
    """Response for HTTP `UNAUTHORIZED`."""
    return json_response(401, message)
