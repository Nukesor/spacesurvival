"""Queue schemas."""

from server.extensions import ma
from server.models.queue import Queue
from server.models.queue_entry import QueueEntry


class QueueEntrySchema(ma.ModelSchema):
    """QueueEntry serialization validation schema."""

    class Meta:
        """Meta class."""

        strict = True
        model = QueueEntry
        exclude = (
            "created_at",
            "updated_at",
        )


class QueueSchema(ma.ModelSchema):
    """Queue serialization validation schema."""

    queue_entries = ma.Nested(QueueEntrySchema, many=True)

    class Meta:
        """Meta class."""

        strict = True
        model = Queue
        exclude = (
            "created_at",
            "updated_at",
        )
