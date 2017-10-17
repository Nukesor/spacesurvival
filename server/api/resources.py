"""All routes regarding resources."""

from server import user_bp
from server.extensions import db
from server.responses import ok
from server.models.resource import Resource
from server.schemas.resource import ResourceSchema


@user_bp.route('/api/pod/<pod_id>/resources', methods=['GET'])
def pod_resources(pod_id):
    """Get the resources of a specified pod."""
    pod = db.session.query(Resource) \
        .filter(Resource.pod_id == pod_id) \
        .all()
    schema = ResourceSchema()

    return ok(schema.dump(pod.resources).data)
