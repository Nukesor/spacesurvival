from server.extensions import db
from server.models.pod import Pod



def update_resources():
    pods = db.session.query(Pod).all()

    for pod in pods:
        pod.update_resources()
    db.sesion.commit()
