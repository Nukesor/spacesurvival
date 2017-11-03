"""Parse, validate and load the `module_data.json`."""
import sys
import json
from flask import current_app
from marshmallow import fields, Schema

from server.data import Dependency, Resource
from server.data.types import ModuleTypes


class Shoots(Schema):
    """Trait for a module."""

    rate = fields.Int()
    damage = fields.Int()
    range = fields.Int()


class ModuleLevel(Schema):
    """The json representation of a module."""

    level = fields.Int()
    resources = fields.Nested(Resource, many=True)
    duration = fields.Int()
    shoots = fields.Nested(Shoots)
    generates = fields.Nested(Resource, many=True)
    consumes = fields.Nested(Resource, many=True)


class Module(Schema):
    """The json representation of a full module.

    This class is only for deserializing the included `module_data.yml`.
    It shouldn't be used in any other context!
    """

    display_name = fields.Str()
    dependencies = fields.Nested(Dependency, many=True)
    levels = fields.Nested(ModuleLevel, many=True)


def load_modules():
    """Load the module data from a file."""
    try:
        with current_app.app_context():
            with open(current_app.config["MODULE_FILE_PATH"], 'r') as stream:
                data = json.load(stream)

        modules = {}
        for key, module in data.items():
            # Check if the key is a valid module Type
            if key not in ModuleTypes.__members__:
                print('Unknown ModuleType "{key} in json"')
                sys.exit(1)

            # Deserialize
            deserialized = Module().load(module)
            if len(deserialized[1]) > 0:
                print("Error deserializing json")
                print(deserialized[1])
                sys.exit(1)
            modules[key] = deserialized[0]

        return modules
    except Exception as e:
        print(e)
        sys.exit(1)
