"""All api routes regarding modules."""

from flask import g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import created, ok, bad_request
from server.models.pod import Pod
from server.models.module import Module
from server.schemas.module import ModuleSchema
from server.validation.module import module_creation_fields
from server.data.types import ModuleTypes


@user_bp.route('/api/modules', methods=['GET'])
def get_module_meta():
    """Get the meta information about all modules."""
    from server.data.data import module_data
    return ok(module_data)


# This route returns the list of all modules and
# their current levels for the pod of the current user.
@user_bp.route('/api/pod/<uuid:pod_id>/modules', methods=['GET'])
def get_pod_modules(pod_id):
    """Get all modules and their information from a specific pod."""
    pod = db.session.query(Pod).get(pod_id)
    schema = ModuleSchema()

    return ok(schema.dump(pod.modules).data)


@user_bp.route('/api/pod/<uuid:pod_id>/new_module', methods=['POST'])
@use_args(module_creation_fields)
def new_pod_module(args):
    """Place a new module on the pod grid."""
    # Check for valid module type
    module_type = args['module_type']
    stationary = args['stationary']
    x_pos = args['position_x']
    y_pos = args['position_y']
    if module_type not in ModuleTypes.__members__:
        return bad_request('Unknown Module type "{module_type}"')

    if stationary:
        existing_module = db.session.query(Module) \
            .filter(Module.pod_id == g.current_user.pod.id) \
            .filter(Module.stationary == True) \
            .filter(Module.type == module_type) \
            .first()

    else:
        existing_module = db.session.query(Module) \
            .filter(Module.pod_id == g.current_user.pod.id) \
            .filter(Module.x_pos == x_pos) \
            .filter(Module.y_pos == y_pos) \
            .first()

    if existing_module:
        return bad_request('There already is a module at this position')

    module = Module(module_type, g.current_user.pod,
                    0, stationary, x_pos, y_pos)

    schema = ModuleSchema()
    return created(schema.dump(module).data)
