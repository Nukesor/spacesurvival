import sys
import json
from marshmallow import fields, Schema

from server.data import Dependency, Resource
from server.data.types import ModuleTypes

class Shoots(Schema):
    rate = fields.Int()
    damage = fields.Int()
    range = fields.Int()

class ModuleLevel(Schema):
    """The json representation of a module."""
    level = fields.Int()
    resources = fields.Nested(Resource, many=True)
    time = fields.Int()
    shoots = fields.Nested(Shoots)
    generates = fields.Nested(Resource, many=True)
    consumes = fields.Nested(Resource, many=True)

# This class is only for deserializing the included `module_data.yml`.
#
# It shouldn't be used in any other context!
class Module(Schema):
    """The json representation of a full module."""
    display_name = fields.Str()
    dependencies = fields.Nested(Dependency, many=True)
    levels = fields.Nested(ModuleLevel, many=True)

def load_modules():
    try:
        with open("server/data/module_data.json", 'r') as stream:
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
            modules[key] = deserialized[1]

        modules = Module().load(data)
        return modules
    except Exception as e:
        print(e)
        sys.exit(1)
