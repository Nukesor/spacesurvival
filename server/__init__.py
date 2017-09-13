import os
from flask import Flask, Blueprint
from flask_mail import Mail
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy
from flask_marshmallow import Marshmallow
from flask_security import Security, SQLAlchemyUserDatastore
from server.config import configs

# Extensions
db = SQLAlchemy()
ma = Marshmallow()
migrate = Migrate()
mail = Mail()

# Blueprints
user_bp = Blueprint('user', __name__)
admin_bp = Blueprint('admin', __name__)

# User security datastore 
from server.models.user import User
from server.models.role import Role
user_datastore = SQLAlchemyUserDatastore(db, User, Role)

def create_app(config='develop'):

    app = Flask(__name__, static_folder='../static')
    app.config.from_object(configs[config])
    security = Security(app, user_datastore)
    db.init_app(app)
    ma.init_app(app)
    migrate.init_app(app, db)
    mail.init_app(app)
    load_blueprints(app)

    return app


def load_blueprints(app):
    app.register_blueprint(user_bp)
    app.register_blueprint(admin_bp, url_prefix='/admin')


def run():
    config = os.getenv('SQLALCHEMY_DATABASE_URI', 'develop')
    app = create_app(config)
    app.run(host="0.0.0.0")


import server.api
import server.models
