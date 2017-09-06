from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_marshmallow import Marshmallow
from flask_mail import Mail
from flask_security import Security, SQLAlchemyUserDatastore

app = Flask(__name__, static_folder='../static')
app.config['DEBUG'] = True
app.config['SECRET_KEY'] = 'lolololol'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgres://localhost/browsergame'
db = SQLAlchemy(app)
ma = Marshmallow(app)

# After 'Create app'
app.config['MAIL_SERVER'] = 'smtp.example.com'
app.config['MAIL_PORT'] = 465
app.config['MAIL_USE_SSL'] = True
app.config['MAIL_USERNAME'] = 'username'
app.config['MAIL_PASSWORD'] = 'password'
mail = Mail(app)

# Security settings
app.config['SECURITY_PASSWORD_SALT'] = 'lolwat'
app.config['SECURITY_CONFIRMABLE'] = True
app.config['SECURITY_TRACKABLE'] = True

from server.models.user import User
from server.models.role import Role

user_datastore = SQLAlchemyUserDatastore(db, User, Role)
security = Security(app, user_datastore)

import server.models
import server.api

def run():
    app.run(host="0.0.0.0")
