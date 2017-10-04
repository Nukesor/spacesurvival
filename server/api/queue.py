from flask import g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import ok
from server.models.queue import Queue
from server.schemas.queue import QueueSchema


@user_bp.route('/api/pod/<pod_id>/queue', methods = ['GET'])
def pod_queue(pod_id):
    pod = db.session.query(Pod).get(pod_id)
    schema = QueueSchema()

    return ok(schema.dump(pod.queue).data)
