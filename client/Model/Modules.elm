module Model.Modules exposing (..)

import Dict
import Model.Research exposing (ResearchId, ResourceId)


type alias SlotEntry =
    Maybe Module


type alias ModuleId =
    String


type alias Modules =
    Dict.Dict String Module


type alias Module =
    { name : String
    , dependencies : List ( ResearchId, Int )
    , levels : List ModuleLevel
    }


type alias ModuleLevel =
    { level : Int
    , consumes : List ( ResourceId, Int )
    , generates : List ( ResourceId, Int )
    , buildCosts : List ( ResourceId, Int )
    , shoots :
        Maybe Shoots
    , buildTime : Int
    }


type alias Shoots =
    { damage : Int
    , range : Int
    , rate : Int
    }


type alias UserModules =
    {}
