from webargs.flaskparser import use_args

from server import app, db, user_datastore
from server.validation.user import user_creation_fields
from server.responses import created


@app.route('/api/user/register', methods = ['POST'])
@use_args(user_creation_fields)
def register(args):

    user = user_datastore.create_user(
        nickname = args['nickname'],
        email = args['email'],
        password = args['password'],
    )

    db.session.add(user)
    db.session.commit()

    return created()
