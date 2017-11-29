"""Queue model."""
import uuid
from datetime import datetime, timedelta
from server.extensions import db
from sqlalchemy.orm import relationship
from sqlalchemy import (
    func,
    Column,
    CheckConstraint,
    ForeignKey,
)

from sqlalchemy.types import (
    Integer,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Queue(db.Model):
    """Queue model."""

    __tablename__ = 'queue'
    __table_args__ = (
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)",
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    pod_id = Column(UUID(as_uuid=True), ForeignKey('pod.id'), index=True)
    base_id = Column(UUID(as_uuid=True), ForeignKey('base.id'), index=True)

    slots = Column(Integer, default=2, nullable=False)
    pod = relationship("Pod", back_populates="queue")
    base = relationship("Base", back_populates="queue")
    queue_entries = relationship("QueueEntry", back_populates="queue", order_by='QueueEntry.created_at')

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def next_entry(self):
        """Activate the next queue entry."""
        if len(self.queue_entries) == 0:
            return
        next_entry = self.queue_entries[0]
        if next_entry and next_entry.finishes_at is None:
            finishes_at = datetime.now() + timedelta(seconds=next_entry.duration)
            next_entry.finishes_at = finishes_at
            db.session.add(next_entry)
