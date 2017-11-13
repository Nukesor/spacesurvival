import os
from flask import Flask, Blueprint

from server.config import configs
from server.data import data
from server.data.module import load_modules
from server.data.research import load_research
from server.extensions import db, mail, ma, migrate, passlib


# Blueprints
user_bp = Blueprint('user', __name__)
admin_bp = Blueprint('admin', __name__)


def create_app(config='develop'):
    """App factory."""
    app = Flask(
        __name__,
        static_folder='../static',
        static_url_path='/static-native',
    )
    config = configs[config]
    app.config.from_object(config)
    db.init_app(app)
    ma.init_app(app)
    migrate.init_app(app, db)
    mail.init_app(app)
    passlib.init_app(app)

    from server.handlers import register_handlers
    register_handlers(app)

    register_blueprints(app)

    data.module_data = load_modules(getattr(config(), "MODULE_FILE_PATH"))
    data.research_data = load_research(getattr(config(), "RESEARCH_FILE_PATH"))

    return app


def register_blueprints(app):
    """Register all blueprints."""
    app.register_blueprint(user_bp)
    app.register_blueprint(admin_bp, url_prefix='/admin')


def run():
    """Run the app on all IPs."""
    config = os.getenv('SQLALCHEMY_DATABASE_URI', 'develop')
    app = create_app(config)
    app.run(host="0.0.0.0")


import server.api # noqa
import server.models # noqa

