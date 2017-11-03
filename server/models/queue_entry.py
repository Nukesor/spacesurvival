import uuid
from server.extensions import db
from sqlalchemy import (
    func,
    text,
    Column,
    CheckConstraint,
    ForeignKeyConstraint,
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
        ForeignKeyConstraint(
            ['queue_id'], ['queue.id'],
            deferrable=True, initially='DEFERRED'),
        ForeignKeyConstraint(['module_id'], ['module.id'],
            deferrable=True, initially='DEFERRED'),
        ForeignKeyConstraint(['research_id'], ['research.id'],
            deferrable=True, initially='DEFERRED'),
        CheckConstraint(
            "(research_id is NULL and module_id is not NULL) or "
            "(research_id is not NULL and module_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    queue_id = Column(UUID(as_uuid=True), nullable=False)
    module_id = Column(UUID(as_uuid=True))
    research_id = Column(UUID(as_uuid=True))

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
