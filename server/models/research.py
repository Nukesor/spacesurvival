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


class Research(db.Model):
    __tablename__ = 'research'

    __table_args__ = (
        ForeignKeyConstraint(['pod_id'], ['pod.id']),
        ForeignKeyConstraint(['base_id'], ['base.id']),
        CheckConstraint(
            "(pod_id is not NULL or base_id is not NULL) and"
            "not(pod_id is not NULL and pod_id is not NULL)"
        ),
    )

    id = Column(UUID, primary_key=True)
    pod_id = Column(UUID, nullable=True)
    base_id = Column(UUID, nullable=True)

    name = Column(String(255))
    level = Column(Integer)

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
