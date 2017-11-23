"""Queue schemas."""

from marshmallow import fields

from server.schemas import Schema


class QueueEntrySchema(Schema):
    """QueueEntry serialization validation schema."""

    id = fields.UUID()
    type = fields.Str(required=True)
    queue_id = fields.UUID()
    module_id = fields.UUID()
    research_id = fields.UUID()

    level = fields.Int()
    duration = fields.Int()


class QueueSchema(Schema):
    """Queue serialization validation schema."""

    id = fields.UUID()
    pod_id = fields.UUID()
    base_id = fields.UUID()

    slots = fields.Int()
    queue_entries = fields.Nested(QueueEntrySchema, many=True)
