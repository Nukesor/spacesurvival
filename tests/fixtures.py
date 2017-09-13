import pytest
from server import user_datastore

@pytest.fixture(scope='function')
def user(app, session):
    """Session-wide test `Flask` application."""

    user = user_datastore.create_user(
        nickname = 'admin',
        email = 'admin@admin.de',
        password = 'hunter2',
    )
    session.add(user)
    session.commit()
    return user
