"""Helper file to allow late loading of module and research meta data."""
from server.data.module import load_modules
from server.data.research import load_research

research_data = load_research()
module_data = load_modules()
