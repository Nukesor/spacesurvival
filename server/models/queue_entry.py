import uuid
from server.extensions import db
from sqlalchemy import (
    func,
    text,
    Column,
    CheckConstraint,
    ForeignKey,
)
from sqlalchemy.orm import relationship

from sqlalchemy.types import (
    Boolean,
    String,
    Integer,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class QueueEntry(db.Model):
    __tablename__ = 'queue_entry'

    __table_args__ = (
        CheckConstraint(
            "(research_id is NULL and module_id is not NULL) or "
            "(research_id is not NULL and module_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    queue_id = Column(UUID(as_uuid=True), ForeignKey('queue.id'), nullable=False)
    module_id = Column(UUID(as_uuid=True), ForeignKey('module.id'))
    research_id = Column(UUID(as_uuid=True), ForeignKey('research.id'))

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
        nullable=False
    )

    def __init__(self, queue, level, duration, module=None, research=None):
        self.queue = queue
        self.level = level
        self.duration = duration
        self.module = module
        self.research = research
