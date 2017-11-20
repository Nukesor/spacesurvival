"""Module tests."""
import pytest

from tests.helper import auth_token


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestResources:
    """Test all resource related api functionality."""

    def url(self, user):
        """Url for this test class."""
        return f'/api/pod/{user.pod.id}/resources'

    def test_module_creation(self, app, user, client):
        """Simple research check."""
        response = client.get(self.url(user), headers=auth_token(user))
        assert response.status_code
