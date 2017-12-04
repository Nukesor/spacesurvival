"""Process queues."""
from datetime import datetime

from server.extensions import db
from server.models import QueueEntry


def finished_entries():
    """Process finished entries."""
    queue_entries = db.session.query(QueueEntry) \
        .filter(QueueEntry.finishes_at <= datetime.now()) \
        .all()

    for entry in queue_entries:
        if entry.module:
            entry.module.level = entry.level
            db.session.add(entry.module)
            entry.module.pod.update_resources()

        elif entry.research:
            entry.research.level = entry.level
            entry.research.researched = True
            db.session.add(entry.research)

        queue = entry.queue
        db.session.delete(entry)
        queue = queue.next_entry()

    db.session.commit()
