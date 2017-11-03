import json
import pytest

from tests.helper import auth_token
from server.extensions import db

from server.models.user import User


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestModule:
    def get_url(self, user):
        return f'/api/pod/{user.pod.id}/new_module'

    def test_module_creation(self, app, user, client):
        data = {
            'module_type': 'PlasmaGenerator',
            'stationary': False,
            'position_x': 1,
            'position_y': 1
        }

        response = client.post(
            self.get_url(user),
            data=json.dumps(data),
            headers=auth_token(user)
        )
        assert response.status_code == 201

        user = db.session.query(User).get(user.id)

        assert len(user.pod.modules) == 1
