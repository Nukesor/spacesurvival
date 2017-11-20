"""Pod database model."""

import uuid
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy import (
    func,
    Column,
    ForeignKey,
)

from sqlalchemy.types import (
    String,
    DateTime,
)

from server.extensions import db
from server.models.queue import Queue
from server.models.resource import Resource
from server.data.types import ResourceTypes


class Pod(db.Model):
    """Pod model."""

    __tablename__ = 'pod'

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(255), nullable=False)
    user_id = Column(UUID(as_uuid=True), ForeignKey('user.id', deferrable=True, initially='DEFERRED'), index=True, nullable=False)
    base_id = Column(UUID(as_uuid=True), ForeignKey('base.id'), index=True)

    user = relationship("User", back_populates="pod")

    resources = relationship("Resource")
    queue = relationship("Queue", uselist=False, back_populates="pod")
    modules = relationship("Module", back_populates="pod")
    researches = relationship("Research", back_populates="pod")

    created_at = Column(DateTime, server_default=func.now(), nullable=False)
    updated_at = Column(
        DateTime, server_default=func.now(),
        onupdate=func.current_timestamp(),
        nullable=False,
    )

    def __init__(self, nickname):
        """Create a new pod."""
        self.name = "{nickname}'s pod"
        self.queue = Queue()
        resources = []
        for resource in ResourceTypes:
            resources.append(Resource(resource.name))
        self.resources = resources

    def update_resource_production(self):
        return
