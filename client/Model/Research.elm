module Model.Research exposing (..)

import Dict
import List exposing (length)


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


atMaxLevel : Research -> Bool
atMaxLevel research =
    let
        maxLevel =
            length research.levels
    in
        research.currentLevel
            >= maxLevel


updateable : Researches -> String -> Bool
updateable researches key =
    case Dict.get key researches of
        Just research ->
            List.all (dependencyFulfilled researches) research.dependencies
                && not (atMaxLevel research)

        _ ->
            False


dependencyFulfilled : Dict.Dict String Research -> ( ResearchId, Int ) -> Bool
dependencyFulfilled researches ( id, level ) =
    case Dict.get id researches of
        Just research ->
            research.currentLevel >= level

        Nothing ->
            Debug.log ("Research not found: " ++ id) False
