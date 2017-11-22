"""Process queues."""
from datetime import datetime

from server.extensions import db
from server.models import QueueEntry


class QueueUpdater():
    """Class for updating queue."""

    def run(self):
        """Tick."""
        self.finished_entries()

    def finished_entries(self):
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
                entry.research.level += 1
                db.session.add(entry.research)

            entry.queue.next_entry()
            db.session.delete(entry)
        db.session.commit()
