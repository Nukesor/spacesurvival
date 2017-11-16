"""Module tests."""
import uuid
import json
import pytest

from tests.helper import auth_token
from server.extensions import db

from server.models.user import User, Pod


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestResearch:
    """Test all module related api functionality."""

    def post(self, client, user, data):
        """Url for this test class."""
        response = client.post(
            f'/api/pod/{user.pod.id}/researches',
            data=json.dumps(data),
            headers=auth_token(user),
        )
        return response

    def test_module_creation(self, app, user, client):
        """Simple research creation."""
        # Normal research request
        data = {'type': 'Plasma'}
        response = self.post(client, user, data)

        user = db.session.query(User).get(user.id)
        assert response.status_code == 200
        assert len(user.pod.researches) == 1
        assert len(user.pod.queue.queue_entries) == 1

    def test_upgrade_research(self, app, user, client):
        """Simple research upgrade."""
        # Normal research request
        data = {'type': 'Plasma'}
        response = self.post(client, user, data)
        assert response.status_code == 200
        response = self.post(client, user, data)
        assert response.status_code == 200

        user = db.session.query(User).get(user.id)
        assert len(user.pod.researches) == 1
        assert len(user.pod.queue.queue_entries) == 2

    def test_too_few_resources(self, app, user, client):
        """Too few resources."""
        for resource in user.pod.resources:
            resource.amount = 10
        db.session.add(user)
        db.session.commit()

        data = {'type': 'Plasma'}
        response = self.post(client, user, data)
        message = response.get_data().decode('utf-8')
        assert response.status_code == 400
        assert "Not enough resources" in message
