import pytest

from server import create_app
from server.config import TestConfig
from server.helpers.database import create_db
from tests.fixtures import *


@pytest.fixture(scope='session')
def app(request):
    """Session-wide test `Flask` application."""

    app = create_app('testing')
    # Establish an application context before running the tests.
    ctx = app.app_context()
    ctx.push()

    def teardown():
        ctx.pop()

    request.addfinalizer(teardown)
    return app


@pytest.fixture(scope='session')
def db(app, request):
    from server import db as _db
    """Session-wide test database."""
    def teardown():
        _db.drop_all()

    _db.app = app
    _db.drop_all()
    create_db()
    _db.create_all()

    request.addfinalizer(teardown)
    return _db


@pytest.fixture(scope='function')
def session(db, request):
    """Creates a new database session for a test."""
    connection = db.engine.connect()
    transaction = connection.begin()

    options = dict(bind=connection, binds={})
    session = db.create_scoped_session(options=options)

    db.session = session

    def teardown():
        transaction.rollback()
        connection.close()
        session.remove()

    request.addfinalizer(teardown)
    return session
