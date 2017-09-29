import pytest

from server.models.user import User

@pytest.fixture(scope='function')
def user(app, user_factory):
    """Session-wide test `Flask` application."""

    user = user_factory.get()
    return user
