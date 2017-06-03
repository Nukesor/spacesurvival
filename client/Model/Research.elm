module Model.Research exposing (..)

import Dict
import List exposing (length)
import Model.Queue exposing (Queue, inQueue)


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


atMaxLevel : Queue -> Research -> Bool
atMaxLevel queue research =
    let
        maxLevel =
            length research.levels
    in
        research.currentLevel
            >= maxLevel


updateable : Queue -> Researches -> Research -> Bool
updateable queue researches research =
    List.all (dependencyFulfilled researches) research.dependencies && not (atMaxLevel queue research)


dependencyFulfilled : Dict.Dict String Research -> ( ResearchId, Int ) -> Bool
dependencyFulfilled researches ( id, level ) =
    case Dict.get id researches of
        Just research ->
            research.currentLevel >= level

        Nothing ->
            Debug.log ("Research not found: " ++ id) False
