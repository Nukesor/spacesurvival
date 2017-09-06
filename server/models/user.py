from flask_security import UserMixin
from flask_security.utils import hash_password, verify_password
from sqlalchemy_utils import EmailType
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy import func, text, Column, UniqueConstraint
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

    __table_args__ = (
        UniqueConstraint("nickname"),
        UniqueConstraint("email"),
    )

    id = Column(UUID, primary_key=True, server_default=text("uuid_generate_v4()"))
    email = Column(EmailType)
    nickname = Column(String(255))
    password_hash = Column(String(255))
    active = Column(Boolean)
    confirmed_at = Column(DateTime, nullable=True)
    roles = relationship('Role', secondary=roles_users,
                            backref=db.backref('user', lazy='dynamic'))

    pod = relationship("Pod", uselist=False, back_populates="user")

    last_login_at = Column(DateTime, nullable=True)
    current_login_at = Column(DateTime, nullable=True)
    last_login_ip = Column(String(255), nullable=True)
    current_login_ip = Column(String(255), nullable=True)
    login_count = Column(Integer, default=0)

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )

    def __init__(self, nickname, email, password, active, roles):
        self.email = email
        self.nickname = nickname
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
