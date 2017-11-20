"""Authentication related tests."""
import json
import pytest

from server.extensions import db
from server.models.user import User


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestUserCreation:
    """User creation tests."""

    def test_user_creation(self, app, client):
        """A user an be created."""
        data = {
            'nickname': 'admin',
            'email': 'admin@admin.de',
            'password': 'hunter2',
        }
        response = client.post(
            '/api/user/register', data=json.dumps(data),
            content_type='application/json',
        )
        assert response.status_code == 201
        user = db.session.query(User) \
            .filter(User.nickname == 'admin') \
            .filter(User.email == 'admin@admin.de') \
            .one_or_none()
        assert user
        assert user.verify_password('hunter2')
        assert not user.verify_password('hunter1337')

    def test_cant_reuse_credentials(self, app, client):
        """Credentials are unique."""
        data = {
            'nickname': 'admin',
            'email': 'admin@admin.de',
            'password': 'hunter2',
        }
        response = client.post(
            '/api/user/register', data=json.dumps(data),
            content_type='application/json',
        )
        assert response.status_code == 201
        response = client.post(
            '/api/user/register', data=json.dumps(data),
            content_type='application/json',
        )
        assert response.status_code == 409

    @pytest.mark.parametrize('email', ['test', 'test@de'])
    def test_invalid_email(self, app, client, email):
        """Can't use invalid emails."""
        data = {
            'nickname': 'test',
            'email': email,
            'password': 'testtest',
        }
        response = client.post(
            '/api/user/register', data=json.dumps(data),
            content_type='application/json',
        )
        assert response.status_code == 422


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestAuthentication:
    """Authentication related tests."""

    def test_login(self, app, client, user):
        """Login works with nickname and mail."""
        for identifier in [user.nickname, user.email]:
            data = {
                'identifier': identifier,
                'password': 'hunter2',
            }
            response = client.post(
                '/api/auth/login', data=json.dumps(data),
                content_type='application/json',
            )
            assert response.status_code == 200
