from server import db
from sqlalchemy import (
    func,
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
            "(research_id is not NULL or module_id is not NULL) and"
            "not(research_id is not NULL and module_id is not NULL)"
        ),
    )

    id = Column(UUID, primary_key=True)
    queue_id = Column(UUID, nullable=True)
    module_id = Column(UUID, nullable=True)
    research_id = Column(UUID, nullable=True)

    name = Column(String(255))
    level = Column(Integer)
    duration = Column(Integer)

    finishes_at = Column(DateTime)
    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
