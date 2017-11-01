from webargs import fields


# Need validator -> both positions or stationary.
module_creation_fields = {
    'module_type': fields.Str(required=True),
    'stationary': fields.Boolean(required=True),
    'position_x': fields.Integer(required=True),
    'position_y': fields.Integer(required=True),
}
