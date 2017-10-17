"""All routes regarding an user."""

from flask import jsonify, g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.helpers.decorators import login_exempt
from server.responses import created, conflict
from server.models.user import User
from server.schemas.user import UserSchema
from server.validation.user import user_creation_fields


@user_bp.route('/api/user/<uuid:user_id>', methods=['GET'])
def info(user_id):
    """Get the info about a specific user."""
    db.session.query(User).get(user_id)
    schema = UserSchema()
    return jsonify(schema.dump(g.current_user).data)


@user_bp.route('/api/user/register', methods=['POST'])
@use_args(user_creation_fields)
@login_exempt
def register(args):
    """Register a new user."""
    user = db.session.query(User) \
        .filter(User.nickname == args['nickname']) \
        .one_or_none()

    if user is not None:
        return conflict('This nickname is already taken.')

    user = db.session.query(User) \
        .filter(User.email == args['email']) \
        .one_or_none()

    if user is not None:
        return conflict('This email is already taken.')

    user = User(
        nickname=args['nickname'],
        email=args['email'],
        password=args['password'],
    )

    db.session.add(user)
    db.session.commit()

    return created()
