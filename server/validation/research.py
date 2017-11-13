"""Research related web argparsing."""

from webargs import fields


research_creation_fields = {
    'type': fields.Str(required=True),
}
