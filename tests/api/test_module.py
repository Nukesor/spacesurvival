"""Module tests."""
import json
import pytest

from tests.helper import auth_token
from server.extensions import db

from server.models.user import User


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestModule:
    """Test all module related api functionality."""

    def get_url(self, user):
        """Url for this test class."""
        return f'/api/pod/{user.pod.id}/new_module'

    def test_module_creation(self, app, user, client):
        """Simple module creation."""
        data = {
            'module_type': 'PlasmaGenerator',
            'stationary': False,
            'position_x': 1,
            'position_y': 1,
        }

        response = client.post(
            self.get_url(user),
            data=json.dumps(data),
            headers=auth_token(user),
        )

        user = db.session.query(User).get(user.id)

        assert response.status_code == 201
        assert len(user.pod.modules) == 1
