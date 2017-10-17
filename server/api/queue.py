"""All routes regarding queues."""

from server import user_bp
from server.extensions import db
from server.responses import ok
from server.models.pod import Pod
from server.schemas.queue import QueueSchema


@user_bp.route('/api/pod/<pod_id>/queue', methods=['GET'])
def pod_queue(pod_id):
    """Get the queue of the specified pod."""
    pod = db.session.query(Pod).get(pod_id)
    schema = QueueSchema()

    return ok(schema.dump(pod.queue).data)
