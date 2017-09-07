from server import ma
from server.models.user import User

class UserSchema(ma.ModelSchema):
    class Meta:
        model = User
        exclude = (
            "password_hash",
            "created_at",
            "updated_at",
        )
