from sqlalchemy import or_
from webargs.flaskparser import use_args
from flask_security.utils import login_user

from server import app, db
from server.validation.user import login_fields

@app.route('/login', methods=['GET', 'POST'])
@use_args(login_fields)
def login(args):

    identifier = args['identifier']
    password = args['password']

    user = db.session.query(User) \
        .filter(or_(User.name == identifier, User.email == identifier)) \
        .one_or_none()

    if user is None:
        bad_request('Unknown credentials.')

    valid_password = user.verify_password(password)
    login_user(user)

    return ok()
