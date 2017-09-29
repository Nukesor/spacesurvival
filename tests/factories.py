import pytest
from server.models.user import User
from server.extensions import db

@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
@pytest.fixture(scope='function')
def user_factory(app):
    """Session-wide test `Flask` application."""

    class UserFactory():
        def __init__(self):
            self.count = 0

        def get(self):
            """ Create a new user."""
            user = User(
                nickname = f'test-{self.count}',
                email = f'test-{self.count}@admin.de',
                password = 'hunter2',
            )
            db.session.add(user)
            db.session.commit()

            return user

    return UserFactory()
