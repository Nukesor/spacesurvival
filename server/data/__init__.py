"""Nested Schemas used by all multiple Schemas."""
from marshmallow import fields, Schema


class Dependency(Schema):
    """The representation of a research dependency."""

    type = fields.Str()
    level = fields.Int()


class Resource(Schema):
    """The representation of a cost."""

    type = fields.Str()
    amount = fields.Int()
