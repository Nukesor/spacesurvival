from server.extensions import ma
from server.models.research import Research

class ResearchSchema(ma.ModelSchema):
    class Meta:
        """Meta class."""

        strict = True
        model = Research
        exclude = (
            "created_at",
            "updated_at",
        )
