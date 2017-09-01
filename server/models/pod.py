from sqlalchemy import (
    Column,
    ForeignKeyConstraint,
)

from sqlalchemy.types import (
    Boolean,
    String,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Pod(Base):
    __tablename__ = 'pod'

    __table_args__ = (
        ForeignKeyConstraint(
            ['user_id'], ['user.id'],
            deferrable=True, initially='DEFERRED'),
        ForeignKeyConstraint(['base_id'], ['base.id']),
    )

    id = Column(UUID, primary_key=True)
    user_id = Column(UUID)
    base_id = Column(UUID, nullable=True)

    name = Column(String(255), primary_key=True)

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
