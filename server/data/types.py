from enum import Enum

# This class contains all types of valid researches.
# It's used to check against the names in `research_data.yml`, to validate types in requests
# and to guarantee database string integrity.
class ResearchTypes(Enum):
    Plasma = 'Plasma'
    EnergyWeapons = 'EnergyWeapons'
    MiningEfficiency = 'MiningEfficiency'

# This class contains all types of valid resources.
# It's used to check against the costs in `research_data.yml` and `module_data.yml`,
# to validate types in requests and to guarantee database string integrity.
class ResourceTypes(Enum):
    Iron = 'Minerals'
    Water = 'Fuel'

# This class contains all types of valid modules.
# It's used to check against the names in `module_data.yml`, to validate types in requests
# and to guarantee database string integrity.
class ModuleTypes(Enum):
    LaserTurret = 'LaserTurret'
    PlasmaGenerator = 'PlasmaGenerator'
