from flask import jsonify, g
from webargs.flaskparser import use_args

from server import user_bp
from server.extensions import db
from server.responses import created, ok
from server.models.module import Module
from server.schemas.module import ModuleSchema
from server.data.types import ModuleTypes
from server.validation.module import module_creation_fields


@user_bp.route('/api/module', methods = ['GET'])
def get_module_meta():
    schema = ModuleSchema()
    return jsonify(schema.dump(g.current_user).data)


@user_bp.route('/api/modules/pod', methods = ['GET'])
def get_pod_modules():

    modules = g.current_user.pod.modules
    schema = ModuleSchema()

    return jsonify(schema.dump(modules).data)


@user_bp.route('/api/modules/pod/new', methods = ['POST'])
@use_args(module_creation_fields)
def new_pod_module(args):

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

    return jsonify(schema.dump(modules).data)
