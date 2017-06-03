module Model.Queue exposing (..)

import Time.DateTime exposing (DateTime)


type alias ResearchData =
    { createdAt : DateTime
    , id : String
    , researchId : String
    , level : Int
    , finishesAt : Maybe DateTime
    }


type alias ModuleData =
    { createdAt : DateTime
    , id : String
    , moduleId : String
    , name : String
    , level : Int
    , finishesAt : Maybe DateTime
    }


type Entry
    = ResearchEntry ResearchData
    | ModuleEntry ModuleData


type alias Queue =
    List Entry


inQueue : String -> Int -> Queue -> Bool
inQueue id level queue =
    List.any
        (\entry ->
            case entry of
                ResearchEntry data ->
                    data.researchId == id && data.level == level

                _ ->
                    False
        )
        queue
