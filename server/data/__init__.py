"""Nested Schemas used by all multiple Schemas."""
from marshmallow import fields, post_load
from server.schemas import Schema


class Dependency(Schema):
    """The representation of a research dependency."""

    type = fields.Str(required=True)
    level = fields.Int(required=True)


class Resource(Schema):
    """The representation of a cost."""

    type = fields.Str(required=True)
    amount = fields.Int(required=True)

    @post_load
    def normalize_resource(self, data):
        """Convert resources to the internal representation (*1 mio)."""
        data['amount'] = data['amount'] * 1000 * 1000
        return data
