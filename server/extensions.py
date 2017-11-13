"""Helper module for extension initialization."""
from flask_mail import Mail
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy
from flask_marshmallow import Marshmallow
from server.helpers.passlib import Passlib

# Extensions
db = SQLAlchemy()
ma = Marshmallow()
migrate = Migrate()
mail = Mail()
passlib = Passlib()
