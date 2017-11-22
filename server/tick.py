"""Server entry point."""
from server.app import app # noqa
from server.worker.queue import QueueUpdater

if __name__ == '__main__':
    with app.app_context():
        updater = QueueUpdater()
        updater.run()
