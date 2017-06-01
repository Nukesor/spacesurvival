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


availableForQueueing : Dict.Dict ResearchId Research -> String -> Research -> Bool
availableForQueueing researches id research =
    List.all (dependencyFulfilled researches) research.dependencies


dependencyFulfilled : Dict.Dict String Research -> ( ResearchId, Int ) -> Bool
dependencyFulfilled researches ( id, level ) =
    case Dict.get id researches of
        Just research ->
            research.currentLevel == level

        Nothing ->
            Debug.log ("Research not found: " ++ id) False
