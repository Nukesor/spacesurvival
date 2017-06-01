module Model.Research exposing (..)

import Dict


type alias Research =
    { name : String
    , currentLevel : Int
    , dependencies : List ( ResearchId, Int )
    , levels : List ResearchLevel
    }


type alias ResearchLevel =
    { level : Int
    , resources : List ( ResourceId, Int )
    }


type alias ResearchId =
    String


type alias ResourceId =
    String


type alias Researches =
    Dict.Dict ResearchId Research


availableForQueueing researches key research =
    List.all (dependencyFulfilled researches) research.dependencies


dependencyFulfilled researches ( key, level ) =
    case Dict.get key researches of
        Just research ->
            research.currentLevel == level

        Nothing ->
            Debug.log ("Research not found: " ++ key) False
