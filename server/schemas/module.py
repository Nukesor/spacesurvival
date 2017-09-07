from server import ma
from server.models.module import Module

class ModuleSchema(ma.ModelSchema):
    class Meta:
        model = Module
        exclude = (
            "created_at",
            "updated_at",
        )
