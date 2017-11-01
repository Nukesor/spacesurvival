"""Resource database model."""

import uuid
from server.extensions import db
from sqlalchemy import (
    func,
    Column,
    CheckConstraint,
    ForeignKeyConstraint,
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
        ForeignKeyConstraint(['pod_id'], ['pod.id']),
        ForeignKeyConstraint(['base_id'], ['base.id']),
        CheckConstraint(
            "(pod_id is NULL and base_id is not NULL) or "
            "(pod_id is not NULL and base_id is NULL)"
        ),
        CheckConstraint("amount >= 0 and amount <= max_amount"),
        CheckConstraint("max_amount > 0"),
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
        nullable=False,
    )

    def __init__(self, name):
        """Create a new Resource."""
        self.name = name
        self.amount = 0
        self.production = 0
        self.max_amount = 5000

    def get_by_name(resources, name):
        """Get a resource by name."""
        return next((r for r in resources if r.name == name), None)

    def enough_resources(resources, requirements) -> bool:
        """Check if there are enough resources for construction."""
        enough = True
        missing = {}
        for requirement in requirements:
            resource = Resource.get_by_name(resources, requirement.type)
            if resource is None:
                print(f'Missing resource: {requirement.type}')
            if resource.amount <= requirement.amount:
                enough = False
                missing[resource.name] = requirement.amount - resource.amount
        return enough, missing

    def subtract_resources(resources, requirements) -> bool:
        """Check if there are enough resources for construction."""
        for requirement in requirements:
            resource = Resource.get_by_name(resources, requirement.type)
            if (resource.amount - requirement.amount) <= 0:
                print(f"Can't afford resource {requirement.type}: {resource.amount} of {requirement.amount}")
                raise Exception
            else:
                resource.amount -= requirement.amount
            db.session.add(resource)
        db.session.commit()
        return True

    def add_resources(resources, requirements) -> bool:
        """Check if there are enough resources for construction."""
        for requirement in requirements:
            resource = Resource.get_by_name(resources, requirement.type)
            resource.amount += requirement.amount
            db.session.add(resource)
        db.session.commit()
        return True
