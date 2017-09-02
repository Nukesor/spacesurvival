from server import app, db
from server.validation.user import user_creation_fields
from server.models.roles import user_datastore


@app.route('/register', methods = ['POST'])
@use_args(user_creation_fields)
def register(args):

    user = user_datastore.create_user(
        nickname = args['nickname'],
        email = args['email'],
    )

    user.hash_password(args['password'])
    db.session.add(user)
    db.session.commit()

    return 
