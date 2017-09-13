
class DevConfig:
    DEBUG = True
    SECRET_KEY = 'lolololol'
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame-dev'

    MAIL_PORT = 465
    MAIL_USE_SSL = True
    MAIL_USERNAME = 'username'
    MAIL_PASSWORD = 'password'

    SECURITY_PASSWORD_SALT = 'lolwat'
    SECURITY_CONFIRMABLE = True
    SECURITY_TRACKABLE = True

    MODULE_FILE_PATH = "server/data/module_data.json"
    RESEARCH_FILE_PATH = "server/data/research_data.json"


class TestConfig:
    DEBUG = True
    TESTING = True
    SECRET_KEY = 'lolololol'
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame-test'

    MAIL_PORT = 465
    MAIL_USE_SSL = True
    MAIL_USERNAME = 'username'
    MAIL_PASSWORD = 'password'

    SECURITY_PASSWORD_SALT = 'lolwat'
    SECURITY_CONFIRMABLE = True
    SECURITY_TRACKABLE = True

    MODULE_FILE_PATH = "data/module_data.json"
    RESEARCH_FILE_PATH = "data/research_data.json"

class ProdConfig:
    DEBUG = True
    SECRET_KEY = 'lolololol'
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame'

    MAIL_PORT = 465
    MAIL_USE_SSL = True
    MAIL_USERNAME = 'username'
    MAIL_PASSWORD = 'password'

    SECURITY_PASSWORD_SALT = 'lolwat'
    SECURITY_CONFIRMABLE = True
    SECURITY_TRACKABLE = True

    MODULE_FILE_PATH = "server/data/module_data.json"
    RESEARCH_FILE_PATH = "server/data/research_data.json"

configs = {
    'develop': DevConfig,
    'testing': TestConfig,
    'production': ProdConfig,
}
