import json
import pytest

from server import User

class TestAuthentication:
    #@pytest.mark.parametrize('tax, result', [(7, 140), (19, 380)])
    def test_user_creation(self, client, session, db):
        data = {
            'nickname': 'test',
            'email': 'test@test.de',
            'password': 'testtest',
        }
        response = client.post(
            '/api/user/register', data=json.dumps(data),
            content_type='application/json',
        )
        assert response.status_code == 201
        user = session.query(User) \
            .filter(User.nickname == 'test') \
            .filter(User.email == 'test@test.de') \
            .one_or_none()
        assert user
        assert user.verify_password('testtest')
