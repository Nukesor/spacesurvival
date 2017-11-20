from server.extensions import ma
from server.models.user import User

class UserSchema(ma.ModelSchema):
    class Meta:
        """Meta class."""

        strict = True
        model = User
        exclude = (
            "password_hash",
            "created_at",
            "updated_at",
        )
