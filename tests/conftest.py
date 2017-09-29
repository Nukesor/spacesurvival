import pytest
from retry import retry
from sqlalchemy.exc import InternalError
from sqlalchemy_utils.functions import database_exists, create_database, drop_database

from server import create_app
from server.config import TestConfig
from server.helpers.database import create_db
from server.extensions import db

from tests.fixtures import *
from tests.factories import *
from tests.helper import *


@pytest.fixture(scope='session')
def app(request):
    """Application with testing config."""
    app = create_app('testing')
    return app


@pytest.fixture(scope='session')
#@retry(tries=5, delay=5)
def dbmodels(app):
    """Database and models.

    If the database defined in `SQLALCHEMY_DATABASE_URI` already exists, it is
    dropped and re-created.
    """
    with app.app_context():
        # If there is an existing database, make sure to drop it and re-create
        # it in order to make sure that we're getting a clean testrun.
        db.drop_all()
        db.create_all()


@pytest.fixture(scope='function')
def dbtransaction(app, request, request_ctx, monkeypatch):
    """Temporary DB transaction.

    Use this if you want to operate on the real database but don't want changes to actually affect
    it outside of this test. This works using SQLAlchemy transactions.

    Transactions made outside of the session scope are not rolled back.
    """
    with app.app_context():
        connection = db.engine.connect()
        transaction = connection.begin()

        # Patch Flask-SQLAlchemy to use our connection
        monkeypatch.setattr(db, 'get_engine', lambda *args: connection)

        # Explicitly remove the session so that we'll get a new session every time we go here.
        db.session.remove()

        def teardown():
            # Since we are not committing things to the database directly when
            # testing, initially deferred constraints are not checked. The
            # following statement makes the DB check these constraints. We are
            # executing this command AFTER the tests and NOT BEFORE, because
            # within a transaction the DB is allowed to take temporarily
            # invalid state. Read
            # https://www.postgresql.org/docs/current/static/sql-set-constraints.html
            # for details.

            try:
                connection.execute('SET CONSTRAINTS ALL IMMEDIATE')
            except InternalError:
                # This is the case when we are doing something in the tests
                # that we expect it to fail by executing the statement above.
                # In this case, the transaction will be in an already failed
                # state, executing further SQL statements are ignored and doing
                # so raises an exception.
                pass

            transaction.rollback()
            connection.close()
        request.addfinalizer(teardown)

        return db
