from server.extensions import ma
from server.models.module import Research

class ResearchSchema(ma.ModelSchema):
    class Meta:
        model = Research
        exclude = (
            "created_at",
            "updated_at",
        )