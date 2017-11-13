"""Process queues."""
from datetime import datetime

from server.extensions import db
from server.models.queue import Queue


def finished_entries():
    """Process finished entries."""
    queue_entries = db.session.query(Queue) \
        .filter(Queue.finishes_at < datetime.now()) \
        .all()

    for entry in queue_entries:
        if entry.module:
            entry.module.level += 1
            db.session.add(entry.module)
            entry.module.pod.update_resources()

        elif entry.research:
            entry.research.level += 1
            db.session.add(entry.research)

        db.session.remove(entry)
    db.session.commit()
