"""Parse, validate and load the `module_data.json`."""
import sys
import json
from marshmallow import fields

from server.data import Dependency, Resource, BaseSchema as Schema


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
    shoots = fields.Nested(Shoots, allow_none=True)
    generates = fields.Nested(Resource, many=True, missing=list)
    consumes = fields.Nested(Resource, many=True, missing=list)


class Module(Schema):
    """The json representation of a full module.

    This class is only for deserializing the included `module_data.yml`.
    It shouldn't be used in any other context!
    """

    type = fields.Str(required=True)
    display_name = fields.Str(required=True)
    dependencies = fields.Nested(Dependency, many=True, required=True)
    levels = fields.Nested(ModuleLevel, many=True, required=True)


class Modules(Schema):
    """Modules wrapper for easy Module loading."""

    modules = fields.Nested(Module, many=True)


def load_modules(path):
    """Load the module data from a file."""
    try:
        with open(path, 'r') as stream:
            data = json.load(stream)

        modules = {}
        parsed = Modules().load(data).data.get('modules')
        for item in parsed:
            modules[item['type']] = item
        return modules
    except BaseException as exc:
        print(exc)
        sys.exit(1)
