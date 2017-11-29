"""Pod database model."""

import uuid
from datetime import datetime, timedelta
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
        """Update the production of each resource."""
        return

    def update_resources(self):
        """Update the current resource state in the db."""
        for resource in self.resources:
            time_diff = datetime.now() - resource.last_update
            diff_in_seconds = time_diff.total_seconds()
            resource_diff = diff_in_seconds * resource.production / 3600

            resource.amount += resource_diff
            if resource.amount < 0:
                resource.amount = 0
            elif resource.amount > resource.max_amount:
                resource.amount = resource.max_amount
            resource.last_update = datetime.now()

            db.session.add(resource)

    def get_by_name(self, name):
        """Get a resource by name."""
        resource = next((r for r in self.resources if r.name == name), None)
        if resource is None:
            raise LookupError(f'No resource with name {name} found.')
        return resource

    def enough_resources(self, requirements) -> bool:
        """Check if there are enough resources for construction."""
        enough = True
        missing = {}
        self.update_resources()
        for requirement in requirements:
            resource = self.get_by_name(requirement['type'])
            amount = requirement['amount']
            if resource is None:
                print(f'Missing resource: {requirement["type"]}')
            if resource.amount <= amount:
                enough = False
                missing[resource.name] = amount - resource.amount
        return enough, missing

    def subtract_resources(self, requirements) -> bool:
        """Check if there are enough resources for construction."""
        for requirement in requirements:
            resource = self.get_by_name(requirement['type'])
            amount = requirement['amount']
            if (resource.amount - amount) <= 0:
                print(f"Can't afford resource {requirement['type']}: {resource.amount} of {amount}")
                raise Exception
            else:
                resource.amount -= amount
            db.session.add(resource)
        db.session.commit()
        return True

    def add_resources(self, requirements) -> bool:
        """Check if there are enough resources for construction."""
        for requirement in requirements:
            resource = self.get_by_name(requirement['type'])
            amount = requirement['amount']
            resource.amount += amount
            db.session.add(resource)
        db.session.commit()
        return True
