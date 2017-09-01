from sqlalchemy import (
    Column,
)

from sqlalchemy.types import (
    Boolean,
    String,
    Integer,
    DateTime,
)
from sqlalchemy_utils import EmailType
from sqlalchemy.dialects.postgresql import UUID


class User(Base):
    __tablename__ = 'user'

    id = Column(UUID, primary_key=True)
    name = Column(String(255))
    nickname = Column(String(255))
    email = Column(EmailType)

    password = Column(String(255))
    auth = Column(String(255))

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
