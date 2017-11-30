"""All routes regarding queues."""

from flask import g
from server import user_bp
from server.extensions import db
from server.responses import ok, bad_request
from server.schemas.queue import QueueSchema
from server.models import Pod, QueueEntry, Queue


@user_bp.route('/api/pod/<uuid:pod_id>/queue', methods=['GET'])
def pod_queue(pod_id):
    """Get the queue of the specified pod."""
    pod = db.session.query(Pod).get(pod_id)
    schema = QueueSchema()

    return ok(schema.dump(pod.queue).data)


@user_bp.route('/api/pod/<uuid:pod_id>/queue/entry/<uuid:entry_id>', methods=['DELETE'])
def remove_queue_entry(pod_id, entry_id):
    """Remove a specific queue entry."""
    pod = db.session.query(Pod).get(pod_id)
    if pod.user_id != g.current_user.id:
        return bad_request("Pod doesn't belong to current user.")

    """Get the queue of the specified pod."""
    queue_entry = db.session.query(QueueEntry) \
        .join(Queue) \
        .filter(Queue.pod == pod) \
        .filter(QueueEntry.id == entry_id) \
        .one_or_none()

    if queue_entry is None:
        return bad_request("Queue entry doesn't exist")

    db.session.delete(queue_entry)
    pod.queue.next_entry()
    db.session.commit()

    return ok()
