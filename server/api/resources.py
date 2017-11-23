"""All routes regarding resources."""

from flask import g

from server import user_bp
from server.extensions import db
from server.responses import ok, bad_request
from server.models.pod import Pod
from server.schemas.resource import ResourceSchema


@user_bp.route('/api/pod/<pod_id>/resources', methods=['GET'])
def pod_resources(pod_id):
    """Get the resources of a specified pod."""
    pod = db.session.query(Pod).get(pod_id)
    if pod.user_id != g.current_user.id:
        return bad_request(f"Pod doesn't belong to current user.")

    schema = ResourceSchema()

    return ok(schema.dump(pod.resources, many=True).data)
