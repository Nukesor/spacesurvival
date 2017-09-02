from webargs import fields

login_fields = {
    'identifier': fields.Str(required=True)
    'password': fields.Str(required=True)
}

user_creation_fields = {
    'nickname': fields.Str(required=True)
    'email': fields.Email(required=True)
    'password': fields.Str(required=True)
}
