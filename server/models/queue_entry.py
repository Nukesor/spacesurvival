import uuid
from server.extensions import db
from sqlalchemy import (
    func,
    text,
    Column,
    CheckConstraint,
    ForeignKeyConstraint,
)

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
        ForeignKeyConstraint(['module_id'], ['module.id']),
        ForeignKeyConstraint(['research_id'], ['research.id']),
        CheckConstraint(
            "(research_id is NULL and module_id is not NULL) or "
            "(research_id is not NULL and module_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    queue_id = Column(UUID(as_uuid=True), nullable=False)
    module_id = Column(UUID(as_uuid=True))
    research_id = Column(UUID(as_uuid=True))

    name = Column(String(255), nullable=False)
    level = Column(Integer, nullable=False)
    duration = Column(Integer, nullable=False)

    finishes_at = Column(DateTime, nullable=False)
    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False
    )
