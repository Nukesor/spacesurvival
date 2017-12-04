"""Module tests."""
import uuid
import json
import pytest

from tests.helper import auth_token
from server.extensions import db

from server.models import User, Pod, QueueEntry


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestBuildModule:
    """Test all module related api functionality."""

    def post(self, client, user, data):
        """Url for this test class."""
        response = client.post(
            f'/api/pod/{user.pod.id}/modules',
            data=json.dumps(data),
            headers=auth_token(user),
        )
        return response

    def test_module_creation(self, app, user, client):
        """Simple module creation."""
        # Normal new module request
        data = {'module_type': 'PlasmaGenerator', 'stationary': False,
                'x_pos': 1, 'y_pos': 1}
        response = self.post(client, user, data)

        user = db.session.query(User).get(user.id)
        assert response.status_code == 201
        assert len(user.pod.modules) == 1
        assert len(user.pod.queue.queue_entries) == 1

        assert user.pod.queue.queue_entries[0].type == 'PlasmaGenerator'
        assert user.pod.queue.queue_entries[0].level == 0
        assert user.pod.queue.queue_entries[0].finishes_at is not None

        response = client.get(f'/api/pod/{user.pod.id}/modules',
                              headers=auth_token(user))
        assert response.status_code == 200
        assert response.json[0]
        assert response.json[0]['type'] == 'PlasmaGenerator'
        assert response.json[0]['stationary'] is False
        assert response.json[0]['x_pos'] == 1
        assert response.json[0]['y_pos'] == 1
        assert response.json[0]['level'] == 0
        assert response.json[0]['finished'] is False

    def test_position_stationary_validation(self, app, user, client):
        """Ensure that you send stationary and position."""
        # Stationary with x_pos and y_pos
        data = {'module_type': 'PlasmaGenerator', 'stationary': True,
                'x_pos': 1, 'y_pos': 1}
        response = self.post(client, user, data)
        assert response.status_code == 422

        # Y pos exists, but x_pos is missing
        data = {'module_type': 'PlasmaGenerator', 'stationary': False,
                'x_pos': None, 'y_pos': 1}
        response = self.post(client, user, data)
        assert response.status_code == 422

    @pytest.mark.parametrize('payload', [
        {'module_type': 'PlasmaGenerator'},
        {'module_type': 'PlasmaGenerator', 'x_pos': 2, 'stationary': False},
        {'module_type': 'PlasmaGenerator', 'y_pos': 2},
    ])
    def test_missing_fields(self, app, user, client, payload):
        """Ensure that you send stationary and position."""
        # Stationary with x_pos and y_pos
        response = self.post(client, user, payload)
        assert response.status_code == 422

        # Y pos exists, but x_pos is missing
        data = {'module_type': 'PlasmaGenerator', 'stationary': False,
                'x_pos': None, 'y_pos': 1}
        response = self.post(client, user, data)
        assert response.status_code == 422

    def test_zero_module_position(self, app, user, client):
        """Ensure that you can't build a module twice."""
        # Normal new module request
        data = {'module_type': 'PlasmaGenerator', 'stationary': False,
                'x_pos': 0, 'y_pos': 0}
        response = self.post(client, user, data)
        assert response.status_code == 201
        assert len(user.pod.modules) == 1

    def test_position_already_exists(self, app, user, client):
        """Ensure that you can't build a module twice."""
        # Normal new module request
        data = {'module_type': 'PlasmaGenerator', 'stationary': False,
                'x_pos': 1, 'y_pos': 1}
        response = self.post(client, user, data)
        assert response.status_code == 201
        assert len(user.pod.modules) == 1

        response = self.post(client, user, data)
        message = response.get_data().decode('utf-8')
        assert response.status_code == 400
        assert "There already is a module" in message

    def test_stationary_already_exists(self, app, user, client):
        """Ensure that you can't build a module twice."""
        # Normal new module request
        data = {'module_type': 'PlasmaGenerator', 'stationary': True,
                'x_pos': None, 'y_pos': None}
        response = self.post(client, user, data)
        assert response.status_code == 201
        assert len(user.pod.modules) == 1

        response = self.post(client, user, data)
        message = response.get_data().decode('utf-8')
        assert response.status_code == 400
        assert "There already is a module" in message

    def test_too_few_resources(self, app, user, client):
        """Too few resources."""
        for resource in user.pod.resources:
            resource.amount = 10
        db.session.add(user)
        db.session.commit()

        data = {'module_type': 'PlasmaGenerator', 'stationary': True,
                'x_pos': None, 'y_pos': None}
        response = self.post(client, user, data)
        message = response.get_data().decode('utf-8')
        assert response.status_code == 400
        assert "Not enough resources" in message


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestUpgradeModule:
    """Test all module related api functionality."""

    def put(self, client, user, module_id):
        """Url for this test class."""
        response = client.put(
            f'/api/pod/{user.pod.id}/modules/{module_id}',
            headers=auth_token(user),
        )
        return response

    def test_module_upgrade(self, app, pod, client):
        """Simple module creation."""
        # Normal new module request
        module = pod.user.pod.modules[0]
        response = self.put(client, pod.user, module.id)

        pod = db.session.query(Pod).get(pod.id)
        assert response.status_code == 200
        assert len(pod.queue.queue_entries) == 1

    def test_module_upgrade_twice(self, app, pod, client):
        """Upgrade a module multiple times."""
        # Normal new module request
        module = pod.user.pod.modules[0]
        response = self.put(client, pod.user, module.id)
        assert response.status_code == 200

        response = self.put(client, pod.user, module.id)
        assert response.status_code == 200

        pod = db.session.query(Pod).get(pod.id)
        queue_entries = db.session.query(QueueEntry) \
            .order_by(QueueEntry.created_at.asc()) \
            .all()
        assert len(queue_entries) == 2
        assert queue_entries[0].level == 1
        assert queue_entries[1].level == 2

    def test_no_such_module(self, app, pod, client):
        """Upgrade a module multiple times."""
        # Normal new module request
        module_id = uuid.uuid4()
        response = self.put(client, pod.user, module_id)
        assert response.status_code == 400

    def test_too_few_resources(self, app, pod, client):
        """Too few resources."""
        for resource in pod.resources:
            resource.amount = 10
        db.session.add(pod)
        db.session.commit()

        module = pod.modules[0]
        response = self.put(client, pod.user, module.id)
        assert response.status_code == 400
