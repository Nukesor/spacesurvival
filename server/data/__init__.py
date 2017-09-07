from marshmallow import fields, Schema


class Dependency(Schema):
    type = fields.Str()
    level = fields.Int()

class Resource(Schema):
    type = fields.Str()
    amount = fields.Int()
