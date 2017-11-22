"""Data related tests."""
import pytest

from tests.helper import auth_token


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestModuleSchema:
    """Test the json module schema."""

    def get(self, client, user):
        """Url for this test class."""
        return client.get('/api/modules', headers=auth_token(user))

    def test_correct_schema(self, app, user, client):
        """Test that everything works fine (defaults etc.)."""
        response = self.get(client, user)

        assert 'generates' in response.json['LaserTurret']['levels'][0]
        assert 'consumes' in response.json['LaserTurret']['levels'][0]
        assert 'shoots' in response.json['LaserTurret']['levels'][0]
