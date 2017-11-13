"""Parse, validate and load the `research_data.json`."""
import sys
import json
from marshmallow import fields

from server.data import Dependency, Resource, BaseSchema as Schema


class ResearchLevel(Schema):
    """The json representation of a module."""

    level = fields.Int(required=True)
    duration = fields.Int(required=True)
    resources = fields.Nested(Resource, many=True, required=True)


class Research(Schema):
    """The json representation of a full research.

    This class is only for deserializing the included `research_data.yml`.
    It shouldn't be used in any other context!
    """

    id = fields.Str()
    display_name = fields.Str(required=True)
    dependencies = fields.Nested(Dependency, many=True, required=True)
    levels = fields.Nested(ResearchLevel, many=True, required=True)


class Researches(Schema):
    """Researches wrapper for easier research loading."""

    researches = fields.Nested(Research, many=True, required=True)


def load_research(path) -> Research:
    """Load the research data from a file."""
    try:
        with open(path, 'r') as stream:
            data = json.load(stream)

        return Researches().load(data).data.get('researches')

    except Exception as e:
        print(e)
        sys.exit(1)
