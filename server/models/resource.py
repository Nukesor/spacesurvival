"""Resource database model."""

import uuid
from server.extensions import db
from sqlalchemy import (
    func,
    Column,
    CheckConstraint,
    ForeignKey,
)

from sqlalchemy.types import (
    String,
    BigInteger,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Resource(db.Model):
    """Resource model."""

    __tablename__ = 'resource'
    __table_args__ = (
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)",
        ),
        CheckConstraint("amount >= 0 and amount <= max_amount"),
        CheckConstraint("max_amount > 0"),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    pod_id = Column(UUID(as_uuid=True), ForeignKey('pod.id'), index=True)
    base_id = Column(UUID(as_uuid=True), ForeignKey('base.id'), index=True)

    name = Column(String(255), nullable=False)
    amount = Column(BigInteger, nullable=False)
    production = Column(BigInteger, nullable=False)
    max_amount = Column(BigInteger, nullable=False)
    empty_at = Column(DateTime)
    last_update = Column(DateTime, server_default=func.now(), nullable=False)

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def __init__(self, name):
        """Create a new Resource."""
        self.name = name
        self.amount = 0
        self.production = 0
        self.max_amount = 5000
