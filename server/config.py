from datetime import timedelta


class BaseConfig:
    DEBUG = False
    SECRET_KEY = 'lolololol'
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame'
    AUTH_TOKEN_TIMEOUT = timedelta(days=365)

    MAIL_PORT = 465
    MAIL_USE_SSL = True
    MAIL_USERNAME = 'username'
    MAIL_PASSWORD = 'password'

    PASSLIB_SCHEMES = ["argon2"]
    SECURITY_CONFIRMABLE = True
    SECURITY_TRACKABLE = True

    MODULE_FILE_PATH = "server/data/module_data.json"
    RESEARCH_FILE_PATH = "server/data/research_data.json"

    CORS_ALLOW_ORIGIN = ''
    CORS_ALLOW_METHODS = ''
    CORS_ALLOW_HEADERS = ''


class DevConfig(BaseConfig):
    DEBUG = False
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame-dev'


class TestConfig(BaseConfig):
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame-test'


class ProdConfig(BaseConfig):
    SQLALCHEMY_DATABASE_URI = 'postgres://localhost/browsergame'
    AUTH_TOKEN_TIMEOUT = 30 * 12 * 30 * 24 * 3600


configs = {
    'develop': DevConfig,
    'testing': TestConfig,
    'production': ProdConfig,
}
