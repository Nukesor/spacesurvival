"""All valid types for Research, Resources and Models."""
from enum import Enum


class ResearchTypes(Enum):
    """This class contains all types of valid researches.

    It's used to check against the names in `research_data.yml`, to validate types in requests
    and to guarantee database string integrity.
    """

    Plasma = 'Plasma'
    EnergyWeapons = 'EnergyWeapons'
    MiningEfficiency = 'MiningEfficiency'


class ResourceTypes(Enum):
    """This class contains all types of valid resources.

    It's used to check against the costs in `research_data.yml` and `module_data.yml`,
    to validate types in requests and to guarantee database string integrity.
    """

    Minerals = 'Minerals'
    Fuel = 'Fuel'
    Water = 'Water'


class ModuleTypes(Enum):
    """This class contains all types of valid modules.

    It's used to check against the names in `module_data.yml`, to validate types in requests
    and to guarantee database string integrity.
    """

    LaserTurret = 'LaserTurret'
    PlasmaGenerator = 'PlasmaGenerator'
