import uuid
from sqlalchemy.orm import relationship
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

from server.extensions import db


class Module(db.Model):
    __tablename__ = 'module'

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

    type = Column(String(255), nullable=False)
    level = Column(Integer, nullable=False)
    stationary = Column(Boolean, nullable=False)
    x_pos = Column(Integer)
    y_pos = Column(Integer)

    pod = relationship("Pod", back_populates="modules")
    base = relationship("Base", back_populates="modules")

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )
