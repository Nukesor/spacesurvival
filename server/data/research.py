import sys
import json
from marshmallow import fields, Schema

from server.data import Dependency, Resource
from server.data.types import ResearchTypes


class ResearchLevel(Schema):
    """The json representation of a module."""
    level = fields.Int()
    time = fields.Int(),
    resources = fields.Nested(Resource, many=True)

# This class is only for deserializing the included `research_data.yml`.
#
# It shouldn't be used in any other context!
class Research(Schema):
    """The json representation of a full research."""
    display_name = fields.Str()
    dependencies = fields.Nested(Dependency, many=True)
    current_level = fields.Int(default = 0),
    levels = fields.Nested(ResearchLevel, many=True)

def load_research():
    try:
        with open("server/data/research_data.json", 'r') as stream:
            data = json.load(stream)

        researches = {}
        for key, research in data.items():
            # Check if the key is a valid research Type
            if key not in ResearchTypes.__members__:
                print('Unknown ResearchType "{key} in json"')
                sys.exit(1)

            # Deserialize
            deserialized = Research().load(research)
            if len(deserialized[1]) > 0:
                print("Error deserializing json")
                print(deserialized[1])
                sys.exit(1)
            researches[key] = deserialized[1]

        researches = Research().load(data)
        return researches
    except Exception as e:
        print(e)
        sys.exit(1)
