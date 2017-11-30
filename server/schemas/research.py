"""Research schema for serialization and validation."""
from server.schemas import Schema
from marshmallow import fields


class ResearchSchema(Schema):
    """Research Schema."""

    id = fields.UUID()
    pod_id = fields.UUID()
    base_id = fields.UUID()

    type = fields.Str(required=True)
    level = fields.Int()
