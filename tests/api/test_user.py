"""User related tests."""
import json
import pytest


@pytest.mark.usefixtures('dbmodels', 'dbtransaction')
class TestUserSettings:
    """User settings."""

    def url(self, client, email):
        return
