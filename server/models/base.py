from server import db
from sqlalchemy.orm import relationship
from sqlalchemy import (
    func,
    Column,
    ForeignKeyConstraint,
)

from sqlalchemy.types import (
    Boolean,
    String,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Base(db.Model):
    __tablename__ = 'base'

    __table_args__ = ()

    id = Column(UUID, primary_key=True)
    name = Column(String(255))
    user_id = Column(UUID)
    base_id = Column(UUID, nullable=True)

    resources = relationship("Resource")
    queue = relationship("Queue", uselist=False, back_populates="base")
    modules = relationship("Module", back_populates="base")
    researches = relationship("Research", back_populates="base")

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
