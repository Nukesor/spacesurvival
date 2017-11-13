"""Parse, validate and load the `module_data.json`."""
import sys
import json
from flask import current_app
from marshmallow import fields

from server.data import Dependency, Resource, BaseSchema as Schema
from server.data.types import ModuleTypes


class Shoots(Schema):
    """Trait for a module."""

    rate = fields.Int(required=True)
    damage = fields.Int(required=True)
    range = fields.Int(required=True)


class ModuleLevel(Schema):
    """The json representation of a module."""

    level = fields.Int(required=True)
    current_level = fields.Int(default=0)
    resources = fields.Nested(Resource, many=True, required=True)
    duration = fields.Int(required=True)
    shoots = fields.Nested(Shoots)
    generates = fields.Nested(Resource, many=True)
    consumes = fields.Nested(Resource, many=True)


class Module(Schema):
    """The json representation of a full module.

    This class is only for deserializing the included `module_data.yml`.
    It shouldn't be used in any other context!
    """

    display_name = fields.Str(required=True)
    dependencies = fields.Nested(Dependency, many=True, required=True)
    levels = fields.Nested(ModuleLevel, many=True, required=True)


class Modules(Schema):
    modules = fields.Nested(Module, many=True)


def load_modules(path):
    """Load the module data from a file."""
    try:
        with open(path, 'r') as stream:
            data = json.load(stream)

        return Modules().load(data).data.get('modules')
    except Exception as e:
        print(e)
        sys.exit(1)
