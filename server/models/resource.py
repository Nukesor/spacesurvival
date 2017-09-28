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
    BigInteger,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Resource(db.Model):
    __tablename__ = 'resource'

    __table_args__ = (
        ForeignKeyConstraint(['pod_id'], ['pod.id']),
        ForeignKeyConstraint(['base_id'], ['base.id']),
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)"
        ),
    )

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    pod_id = Column(UUID(as_uuid=True))
    base_id = Column(UUID(as_uuid=True))

    name = Column(String(255), nullable=False)
    amount = Column(BigInteger, nullable=False)
    production = Column(BigInteger, nullable=False)
    max_amount = Column(BigInteger, nullable=False)

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False
    )

    def __init__(self, name):
        self.name = name
        self.amount = 0
        self.production = 0
        self.max_amount = 5000
