from server import db

from flask_security import UserMixin
from flask_security.utils import hash_password, verify_password
from sqlalchemy_utils import EmailType
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID

from sqlalchemy import Column, func
from sqlalchemy.types import (
    Boolean,
    String,
    Integer,
    DateTime,
)
from server import db
from server.models import (
    Pod,
    Queue,
    roles_users,
)


class User(db.Model, UserMixin):
    __tablename__ = 'user'

    id = Column(UUID, primary_key=True)
    name = Column(String(255))
    nickname = Column(String(255))
    email = Column(EmailType)

    active = Column(Boolean)
    password_hash = Column(String(255))
    confirmed_at = Column(DateTime, nullable=True)
    roles = relationship('Role', secondary=roles_users,
                            backref=db.backref('user', lazy='dynamic'))

    pod = relationship("Pod", uselist=False, back_populates="user")

    last_login_at = Column(DateTime)
    current_login_at = Column(DateTime)
    last_login_ip = Column(String(255))
    current_login_ip = Column(String(255))
    login_count = Column(Integer)

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )

    def __init__(self, nickname, email, password, active, roles):
        self.nickname = nickname
        self.email = email
        self.hash_password(password)
        self.active = active
        self.roles = roles

        self.pod = Pod(nickname)

    # Flask login 
    def get_id(self):
        return self.id

    def is_active(self):
        return self.active

    def is_anonymous():
        return False

    def is_authenticated():
        return True

    def hash_password(self, password):
        self.password_hash = hash_password(password)

    def verify_password(self, password):
        return verify_password(password, self.password_hash)
