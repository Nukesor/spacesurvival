"""Research model."""

import uuid
from server.extensions import db
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


class Research(db.Model):
    """Research model for pods and bases."""

    __tablename__ = 'research'
    __table_args__ = (
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)",
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    pod_id = Column(UUID(as_uuid=True), ForeignKey('pod.id'))
    base_id = Column(UUID(as_uuid=True), ForeignKey('base.id'))

    type = Column(String(255), nullable=False)
    level = Column(Integer, nullable=False)
    researched = Column(Boolean, nullable=False, default=False)

    pod = relationship("Pod", back_populates="researches")
    base = relationship("Base", back_populates="researches")

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def __init__(self, research_type, pod, level=0):
        """Create a new research."""
        self.type = research_type
        self.pod = pod
        self.level = level
