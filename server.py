#!/bin/env python
from server import app, db

# Create uuid-ossp extension, if it doesn't exist
db.session.execute('CREATE EXTENSION IF NOT EXISTS "uuid-ossp";')
db.session.commit()

db.reflect()
db.drop_all()
db.create_all()

app.run(host="0.0.0.0")
