"""Everything regarding queue entries."""
import uuid
from datetime import datetime
from server.extensions import db
from sqlalchemy import (
    func,
    Column,
    CheckConstraint,
    ForeignKey,
)
from sqlalchemy.orm import relationship

from sqlalchemy.types import (
    Integer,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class QueueEntry(db.Model):
    """Queue Entry model."""

    __tablename__ = 'queue_entry'
    __table_args__ = (
        CheckConstraint(
            "(research_id is NULL and module_id is not NULL) or "
            "(research_id is not NULL and module_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    queue_id = Column(UUID(as_uuid=True), ForeignKey('queue.id'), index=True, nullable=False)
    module_id = Column(UUID(as_uuid=True), ForeignKey('module.id'), index=True)
    research_id = Column(UUID(as_uuid=True), ForeignKey('research.id'), index=True)

    level = Column(Integer, nullable=False)
    duration = Column(Integer, nullable=False)

    queue = relationship("Queue", back_populates="queue_entries")
    module = relationship("Module")
    research = relationship("Research")

    started_at = Column(DateTime)
    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def __init__(self, queue, level, duration, module=None, research=None):
        """Create a new queue entry."""
        if len(queue.queue_entries) == 0:
            self.started_at = datetime.now()
        self.queue = queue
        self.level = level
        self.duration = duration
        self.module = module
        self.research = research
