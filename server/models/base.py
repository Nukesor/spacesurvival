import uuid
from sqlalchemy.orm import relationship
from sqlalchemy import (
    func,
    text,
    Column,
    ForeignKeyConstraint,
)

from sqlalchemy.types import (
    Boolean,
    String,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID

from server.extensions import db


class Base(db.Model):
    __tablename__ = 'base'

    __table_args__ = ()

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(255))
    user_id = Column(UUID(as_uuid=True), nullable=False)

    resources = relationship("Resource")
    queue = relationship("Queue", uselist=False, back_populates="base")
    modules = relationship("Module", back_populates="base")
    researches = relationship("Research", back_populates="base")

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False
    )
