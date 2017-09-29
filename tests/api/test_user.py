import json
import pytest

from server.extensions import db
from server.models.user import User


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestAuthentication:
    @pytest.mark.parametrize('email', ['test', 'test@de'])
    def test_invalid_email(self, app, client, email):
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


    def test_user_info(self, app, client, user):
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
