"""Module schema for serialization and validation."""
from server.schemas import BaseSchema
from marshmallow import validates_schema, ValidationError, fields


class ModuleSchema(BaseSchema):
    """Module Schema."""

    id = fields.UUID()
    pod_id = fields.UUID()
    base_id = fields.UUID()

    type = fields.Str(required=True)
    level = fields.Int()
    stationary = fields.Bool(required=True)
    x_pos = fields.Int(allow_none=True)
    y_pos = fields.Int(allow_none=True)
    finished = fields.Bool()

    @validates_schema
    def position_or_stationary(self, data):
        """Either x_pos_x, y_pos or stationary."""
        if (data['x_pos'] and not data['y_pos']) or \
           (not data['x_pos'] and data['y_pos']):
                raise ValidationError('x and y position needed',
                                      ['x_pos', 'y_pos'])
        if (data['x_pos'] and data['y_pos'] and data['stationary']) or \
           (not data['x_pos'] and not data['y_pos'] and not data['stationary']):
            raise ValidationError('Position or stationary',
                                  ['x_pos', 'y_pos', 'stationary'])
