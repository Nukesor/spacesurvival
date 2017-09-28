import uuid
import hmac
from datetime import datetime

from flask import current_app
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

from server.extensions import db, passlib
from server.models import (
    Pod,
    Queue,
)


class User(db.Model):
    __tablename__ = 'user'

    __table_args__ = (
        UniqueConstraint("nickname"),
        UniqueConstraint("email"),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, nullable=False, default=uuid.uuid4)
    email = Column(EmailType, nullable=False)
    nickname = Column(String(255), nullable=False)
    password_hash = Column(String(255), nullable=False)
    active = Column(Boolean, nullable=False)
    confirmed_at = Column(DateTime)

    pod = relationship("Pod", uselist=False, back_populates="user")

    auth_token = Column(String(255))
    last_action = Column(DateTime)
    last_login_at = Column(DateTime)
    current_auth_token = db.Column(db.String(36), index=True)

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False
    )

    def __init__(self, nickname, email, password):
        self.email = email
        self.nickname = nickname
        self.hash_password(password.encode('utf-8'))
        self.active = False

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
        self.password_hash = passlib.pwd_context.hash(password)

    def verify_password(self, password):
        return passlib.pwd_context.verify(password, self.password_hash)

    @property
    def has_valid_auth_token(self):
        """Return whether or not the user has a valid auth token."""
        latest_valid_date = datetime.utcnow() - current_app.config['AUTH_TOKEN_TIMEOUT']
        return (self.last_action and
                self.last_action > latest_valid_date and
                self.current_auth_token)

    @staticmethod
    def get_user_from_login_token(token):
        """Get a `User` from a login token.
        A login token has this format:
            <user uuid>:<auth token>
        """
        user_id, auth_token = token.split(':')
        user = db.session.query(User).filter_by(id=user_id).first()
        if user and user.current_auth_token:
            if hmac.compare_digest(user.current_auth_token, auth_token):
                return user
        return None


    def generate_auth_token(self):
        """Generate an auth token and save it to the `current_auth_token` column."""
        new_auth_token = str(uuid.uuid4())
        self.current_auth_token = new_auth_token
        self.last_action = datetime.utcnow()
        db.session.add(self)
        db.session.commit()
        return new_auth_token
