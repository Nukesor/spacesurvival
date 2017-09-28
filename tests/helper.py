
def auth_token(user):
    return [('Authentication', user.get_auth_token())]
