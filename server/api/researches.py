from flask import g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import created, ok
from server.models.research import Research
from server.schemas.research import ResearchSchema
from server.validation.research import research_creation_fields
from server.data.types import ResearchTypes


@user_bp.route('/api/researches', methods = ['GET'])
def get_research_meta():
    from server.data.data import research_data
    return ok(research_data)


# This route returns the list of all modules and
# their current levels for the pod of the current user.
@user_bp.route('/api/pod/<uuid:pod_id>/researches', methods = ['GET'])
def get_pod_research(pod_id):
    pod = db.session.query(Pod).get(pod_id)
    schema = ModuleSchema()

    return ok(schema.dump(pod.modules).data)


@user_bp.route('/api/pod/<uuid:pod_id>/new_research', methods = ['POST'])
@use_args(module_creation_fields)
def new_pod_research(args):

    # Check for valid module type
    module_type = args['module_type']
    if module_type not in ModuleTypes.__members__:
        return bad_request('Unknown Module type "{module_type}"')

    if args['stationary']:
        existing_module = db.session.query(Module) \
            .filter(Module.pod_id == g.current_user.pod.id) \
            .filter(Module.type == args['module_type']) \
            .first()

    else:
        existing_module = db.session.query(Module) \
            .filter(Module.pod_id == g.current_user.pod.id) \
            .filter(Module.x_pos == args['position_x']) \
            .filter(Module.y_pos == args['position_y']) \
            .first()

    if existing_module:
        return bad_request('There already is a module at this position')

    return ok(schema.dump(modules).data)
