"""Base schema."""
from marshmallow import Schema as MarshmallowSchmema


class Schema(MarshmallowSchmema):
    """Base schema class."""

    class Meta:
        """Base meta class."""

        strict = True
