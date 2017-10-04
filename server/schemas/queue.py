from server.extensions import ma
from server.models.queue import Queue

class QueueSchema(ma.ModelSchema):
    class Meta:
        model = Queue
        exclude = (
            "created_at",
            "updated_at",
        )
