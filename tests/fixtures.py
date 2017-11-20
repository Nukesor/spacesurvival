"""Database test fixtures."""
import pytest

from server.models import Module
from server.extensions import db


@pytest.fixture(scope='function')
def user(app, user_factory):
    """Create a user."""
    user = user_factory.get()
    return user


@pytest.fixture(scope='function')
def pod(app, user_factory):
    """Create a pod."""
    user = user_factory.get()
    module = Module('PlasmaGenerator', user.pod, 0, False, 1, 1)
    db.session.add(module)
    db.session.commit()
    return user.pod
