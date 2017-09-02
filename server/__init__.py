from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_marshmallow import Marshmallow
from flask_mail import Mail
from flask_security import Security, SQLAlchemyUserDatastore

app = Flask(__name__, static_url_path='static')
app.config['DEBUG'] = True
app.config['SECRET_KEY'] = 'lolololol'
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
app.config['SECURITY_CONFIRMABLE'] = True
app.config['SECURITY_TRACKABLE'] = True

user_datastore = SQLAlchemyUserDatastore(db, User, Role)
security = Security(app, user_datastore)

def run():
    app.run(host="0.0.0.0")
