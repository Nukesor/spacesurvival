"""Queue tests."""
import json
import pytest

from tests.helper import auth_token
from server.extensions import db

from server.models.user import User


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestQueue:
    """Test all queue related api functionality."""

    def research_upgrade(self, client, user, data):
        """Url for this test class."""
        response = client.post(
            f'/api/pod/{user.pod.id}/researches',
            data=json.dumps(data),
            headers=auth_token(user),
        )
        return response

    def get_queue(self, client, user):
        """Url for this test class."""
        response = client.get(
            f'/api/pod/{user.pod.id}/queue',
            headers=auth_token(user),
        )
        return response

    def delete_entry(self, client, user, entry):
        """Url for this test class."""
        response = client.delete(
            f'/api/pod/{user.pod.id}/queue/entry/{entry.id}',
            headers=auth_token(user),
        )
        return response

    def test_queue_entry_creation(self, app, user, client):
        """Simple queue creation."""
        # Normal queue request
        data = {'type': 'Plasma'}
        response = self.research_upgrade(client, user, data)

        user = db.session.query(User).get(user.id)
        assert response.status_code == 200
        assert len(user.pod.queue.queue_entries) == 1
        queue = user.pod.queue

        response = self.get_queue(client, user)
        assert len(response.json['queue_entries']) == 1
        assert response.json['queue_entries'][0]['level'] == 0
        assert response.json['queue_entries'][0]['queue_id'] == str(queue.id)

    def test_queue_entry_deletion(self, app, user, client):
        """Simple queue upgrade."""
        data = {'type': 'Plasma'}
        self.research_upgrade(client, user, data)

        response = self.delete_entry(client, user, user.pod.queue.queue_entries[0])
        assert response.status_code == 200

        response = self.get_queue(client, user)
        assert len(response.json['queue_entries']) == 0
