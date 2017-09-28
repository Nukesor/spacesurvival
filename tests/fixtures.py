import pytest
from server.models.user import User

@pytest.fixture(scope='function')
def user(app, session):
    """Session-wide test `Flask` application."""

    user = User(
        nickname = 'admin',
        email = 'admin@admin.de',
        password = 'hunter2',
    )
    session.add(user)
    session.commit()
    return user
