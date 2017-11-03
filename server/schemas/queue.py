from server.extensions import ma
from server.models.queue import Queue
from server.models.queue_entry import QueueEntry


class QueueEntrySchema(ma.ModelSchema):
    class Meta:
        model = QueueEntry
        exclude = (
            "created_at",
            "updated_at",
        )


class QueueSchema(ma.ModelSchema):
    queue_entries = ma.Nested(QueueEntrySchema, many=True)
    class Meta:
        model = Queue
        exclude = (
            "created_at",
            "updated_at",
        )
