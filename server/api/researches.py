from flask import g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import created, ok, bad_request
from server.data.types import ResearchTypes
from server.schemas.research import ResearchSchema
from server.validation.research import research_creation_fields

from server.models import (
    Pod,
    QueueEntry,
    Research,
    Resource,
)


@user_bp.route('/api/researches', methods=['GET'])
def get_research_meta():
    """Get the research meta data."""
    from server.data.data import research_data
    return ok(research_data)


@user_bp.route('/api/pod/<uuid:pod_id>/researches', methods=['GET'])
def get_pod_research(pod_id):
    """Send the pod meta data combined with your researches."""
    pod = db.session.query(Pod).get(pod_id)
    schema = ResearchSchema()

    return ok(schema.dump(pod.researches).data)


@user_bp.route('/api/pod/<uuid:pod_id>/researches', methods=['POST'])
@use_args(research_creation_fields)
def begin_pod_research(args, pod_id):
    """Begin a new pod research."""
    from server.data.data import research_data

    pod = db.session.query(Pod).get(pod_id)
    if pod.user_id != g.current_user.id:
        return bad_request(f"Pod doesn't belong to current user.")

    # Check for valid research type
    research_type = args['type']
    if research_type not in ResearchTypes.__members__:
        return bad_request('Unknown research type "{research_type}"')

    research = db.session.query(Research) \
        .filter(Research.pod_id == g.current_user.pod.id) \
        .filter(Research.type == args['research_type']) \
        .first()

    next_level = 0
    if research:
        next_level = research.level + 1
        highest_queue_entry = db.session.query(QueueEntry) \
            .filter(QueueEntry.pod == pod) \
            .filter(QueueEntry.research == research) \
            .order_by(QueueEntry.level.desc()) \
            .first()
        if highest_queue_entry:
            next_level = highest_queue_entry.level + 1

    # Check if we have enough resources
    research_level = research_data[research_type]['levels'].get(next_level)
    if research_level is None:
        return bad_request("Max level reached.")
    requirements = research_level['resources']
    enough, missing = Resource.enough_resources(pod.resources, requirements)
    if not enough:
        return bad_request(f'Not enough resources: {missing}')

    # Subtract the resources from the pod and create a queue entry.
    Resource.subtract_resources(pod.resources, requirements)

    # Create a new research if we don't have it yet.
    if not research:
        research = Research(research_type, pod)

    # Create a new queue entry.
    queue_entry = QueueEntry(pod.queue, next_level, research_level['duration'], research=research)

    db.session.add(queue_entry)
    db.session.add(research)
    db.session.commit()

    return created()
