from server import db
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


class Research(db.Model):
    __tablename__ = 'research'

    __table_args__ = (
        ForeignKeyConstraint(['pod_id'], ['pod.id']),
        ForeignKeyConstraint(['base_id'], ['base.id']),
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)"
        ),
    )

    id = Column(UUID, primary_key=True, server_default=text("uuid_generate_v4()"))
    pod_id = Column(UUID, nullable=True)
    base_id = Column(UUID, nullable=True)

    name = Column(String(255))
    level = Column(Integer)

    pod = relationship("Pod", back_populates="researches")
    base = relationship("Base", back_populates="researches")

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
