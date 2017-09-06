from webargs.flaskparser import use_args
from flask_security.utils import login_user

from server import app, db, user_datastore
from server.responses import created
from server.models.user import User
from server.validation.user import user_creation_fields


@app.route('/api/user/register', methods = ['POST'])
@use_args(user_creation_fields)
def register(args):

    user = db.session.query(User) \
        .filter(User.nickname == args['nickname']) \
        .one_or_none()

    if user is not None:
        return bad_request('This nickname is already taken.')

    user = db.session.query(User) \
        .filter(User.email == args['email']) \
        .one_or_none()

    if user is not None:
        return bad_request('This email is already taken.')

    user = user_datastore.create_user(
        nickname = args['nickname'],
        email = args['email'],
        password = args['password'],
    )

    db.session.add(user)
    db.session.commit()
    login_user(user)

    return created()
