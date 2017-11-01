"""All routes regarding authentication."""

from datetime import datetime
from sqlalchemy import or_
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.models import User
from server.responses import bad_request, ok
from server.validation.user import login_fields
from server.helpers.decorators import login_exempt


@user_bp.route('/api/auth/login', methods=['POST'])
@login_exempt
@use_args(login_fields)
def login(args):
    """Endpoint for login.

    Check if we can login with the credentials. We try to get the
    user by searching email and nickname for the given identifier.
    """
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

    if user.has_valid_auth_token:
        token = user.current_auth_token
    else:
        token = user.generate_auth_token()
    user.last_login_at = datetime.utcnow()
    db.session.add(user)
    db.session.commit()

    return ok({"token": token,
               "user_id": user.id})
