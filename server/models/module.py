"""The file for the Module database model."""

import uuid
from sqlalchemy.orm import relationship
from sqlalchemy import (
    func,
    Column,
    CheckConstraint,
    ForeignKey,
)
from sqlalchemy.types import (
    Boolean,
    String,
    Integer,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID

from server.extensions import db


class Module(db.Model):
    """The database model for all Modules."""

    __tablename__ = 'module'
    __table_args__ = (
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    pod_id = Column(UUID(as_uuid=True), ForeignKey('pod.id'))
    base_id = Column(UUID(as_uuid=True), ForeignKey('base.id'))

    type = Column(String(255), nullable=False)
    level = Column(Integer, nullable=False)
    stationary = Column(Boolean, nullable=False)
    x_pos = Column(Integer)
    y_pos = Column(Integer)
    finished = Column(Boolean, nullable=False, default=False)

    pod = relationship("Pod", back_populates="modules")
    base = relationship("Base", back_populates="modules")

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def __init__(self, type, pod, level, stationary, x_pos=None, y_pos=None):
        """Create a new module."""
        self.type = type
        self.pod = pod
        self.level = level
        self.stationary = stationary
        self.x_pos = x_pos
        self.y_pos = y_pos
