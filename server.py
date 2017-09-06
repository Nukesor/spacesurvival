#!/bin/env python
from server import app, db

db.reflect()
db.drop_all()
db.create_all()

app.run(host="0.0.0.0")
