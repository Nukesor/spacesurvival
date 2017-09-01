from sqlalchemy import (
    Column,
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


class Resource(Base):
    __tablename__ = 'resource'

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
    amount = Column(BigInteger)
    production = Column(BigInteger)
    max_amount = Column(BigInteger)

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
