from server.extensions import ma
from server.models.resource import Resource

class ResourceSchema(ma.ModelSchema):
    class Meta:
        model = Resource
        exclude = (
            "created_at",
            "updated_at",
        )
