from server import db
from sqlalchemy import (
    func,
    Column,
    ForeignKeyConstraint,
)

from sqlalchemy.types import (
    Boolean,
    String,
    DateTime,
)
from sqlalchemy.dialects.postgresql import UUID


class Pod(db.Model):
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

    name = Column(String(255))
    modules = relationship("Module", backref="user")
    queue = relationship("Queue", uselist=False, back_populates="pod")
    researches = relationship("Research", backref="pod")
    resources = relationship("Resource", backref="pod")

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )
