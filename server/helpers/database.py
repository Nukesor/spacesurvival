from server.extensions import db
from server.models.user import User

def create_db():
    # Create uuid-ossp extension, if it doesn't exist
    db.session.execute('CREATE EXTENSION IF NOT EXISTS "uuid-ossp";')
    db.session.commit()

    db.reflect()
    db.drop_all()
    db.create_all()


def create_debug_user(app):
    with app.app_context():
        user = User(
            nickname = 'admin',
            email = 'admin@lol.de',
            password = 'hunter2',
        )

        for resource in user.pod.resources:
            resource.amount = resource.max_amount

        db.session.add(user)
        db.session.commit()

def new_debug_db():
    create_db()
    create_debug_user()
