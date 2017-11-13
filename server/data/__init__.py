"""Nested Schemas used by all multiple Schemas."""
from marshmallow import fields, Schema

class BaseSchema(Schema):
    class Meta:
        strict = True


class Dependency(BaseSchema):
    """The representation of a research dependency."""

    type = fields.Str(required=True)
    level = fields.Int(required=True)


class Resource(BaseSchema):
    """The representation of a cost."""

    type = fields.Str(required=True)
    amount = fields.Int(required=True)
