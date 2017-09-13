from flask import Response

def json_response(code, message):
    return Response(
        message,
        status=code,
        mimetype='application/json',
    )

def ok(message=''):
    return json_response(200, message)

def created(message=''):
    return json_response(201, message)

def bad_request(message=''):
    return json_response(400, message)
