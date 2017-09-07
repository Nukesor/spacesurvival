from sqlalchemy import or_
from webargs.flaskparser import use_args
from flask_security.utils import login_user

from server import app, db
from server.models import User
from server.responses import bad_request, ok
from server.validation.user import login_fields

# Endpoint for login.
#
# Check if we can login with the credentials.
# We try to get the user by searching email and nickname for the given identifier.
@app.route('/api/auth/login', methods=['GET', 'POST'])
@use_args(login_fields)
def login(args):

    identifier = args['identifier']
    password = args['password']

    # Check if the user exists
    user = db.session.query(User) \
        .filter(or_(User.nickname == identifier, User.email == identifier)) \
        .one_or_none()
    if user is None:
        return bad_request('Unknown credentials or wrong password.')

    # Validate password
    valid_password = user.verify_password(password)
    if not valid_password:
        return bad_request('Unknown credentials or wrong password.')

    login_user(user)

    return ok()
