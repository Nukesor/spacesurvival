"""Server entry point."""
from uwsgidecorators import timer
from server.app import app # noqa
from server.worker.queue import finished_entries


@timer(1)
def update_queues(arg):
    """Update queue related stuff."""
    with app.app_context():
        finished_entries()
