from server.extensions import ma
from server.models.resource import Resource

class ResourceSchema(ma.ModelSchema):
    class Meta:
        """Meta class."""

        strict = True
        model = Resource
        exclude = (
            "created_at",
            "updated_at",
        )
