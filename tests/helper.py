
def auth_token(user):
    return [
        ('Authorization', user.get_login_token()),
        ('Content-Type', 'application/json'),
    ]
