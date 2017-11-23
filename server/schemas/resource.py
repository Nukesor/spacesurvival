"""Resource schema."""
from marshmallow import fields
from server.schemas import Schema


class ResourceSchema(Schema):
    """Resource schema."""

    id = fields.UUID()
    pod_id = fields.UUID()
    base_id = fields.UUID()

    name = fields.Str(required=True)
    amount = fields.Int()
    production = fields.Int()
    max_amount = fields.Int()
    empty_at = fields.DateTime()
    last_update = fields.DateTime()
