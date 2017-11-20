"""Base schema."""
from marshmallow import Schema


class BaseSchema(Schema):
    """Base schema class."""

    class Meta:
        """Base meta class."""

        strict = True
