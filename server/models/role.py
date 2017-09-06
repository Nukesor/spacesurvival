from server import db
from flask_security import RoleMixin
from sqlalchemy import (
    func,
    Column,
    ForeignKey,
)

from sqlalchemy.types import String
from sqlalchemy.dialects.postgresql import UUID

from server import db


roles_users = db.Table('roles_users',
        db.Column('user_id', UUID, ForeignKey('user.id')),
        db.Column('role_id', UUID, ForeignKey('role.id')))

class Role(db.Model, RoleMixin):
    id = Column(UUID, primary_key=True)
    name = Column(String(80), unique=True)
    description = Column(String(255))
