"""Research schema for serialization and validation."""
from server.schemas import BaseSchema
from marshmallow import fields


class ResearchSchema(BaseSchema):
    """Research Schema."""

    id = fields.UUID()
    pod_id = fields.UUID()
    base_id = fields.UUID()

    type = fields.Str(required=True)
    level = fields.Int()
    researched = fields.Bool(required=True)
