from flask import g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import ok
from server.models.resource import Resource
from server.schemas.resource import ResourceSchema


@user_bp.route('/api/pod/<pod_id>/resources', methods = ['GET'])
def pod_resources(pod_id):
    pod = db.session.query(Pod).get(pod_id)
    schema = ResourceSchema()

    return ok(schema.dump(pod.resources).data)
