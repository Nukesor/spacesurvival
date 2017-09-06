from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy import (
    func,
    text,
    Column,
    ForeignKeyConstraint,
)

from sqlalchemy.types import (
    Boolean,
    String,
    DateTime,
)

from server import db
from server.models.queue import Queue
from server.models.resource import Resource
from server.data.types import ResourceTypes

class Pod(db.Model):
    __tablename__ = 'pod'

    __table_args__ = (
        ForeignKeyConstraint(
            ['user_id'], ['user.id'],
            deferrable=True, initially='DEFERRED'),
        ForeignKeyConstraint(['base_id'], ['base.id']),
    )

    id = Column(UUID, primary_key=True, server_default=text("uuid_generate_v4()"))
    name = Column(String(255))
    user_id = Column(UUID)
    base_id = Column(UUID, nullable=True)

    user = relationship("User", back_populates="pod")

    resources = relationship("Resource")
    queue = relationship("Queue", uselist=False, back_populates="pod")
    modules = relationship("Module", back_populates="pod")
    researches = relationship("Research", back_populates="pod")

    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp()
    )

    def __init__(self, nickname):
        self.name = "{nickname}'s pod"
        self.queue = Queue()
        resources = []
        for resource in ResourceTypes:
            resources.append(Resource(resource.name))
        self.resources = resources
