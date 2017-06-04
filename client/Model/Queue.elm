module Model.Queue exposing (..)

import Dict
import Model.Research exposing (ResearchId, Researches)
import Time.DateTime exposing (DateTime)


type alias ResearchData =
    { createdAt : DateTime
    , id : String
    , researchId : ResearchId
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


applyQueue : Researches -> Queue -> Researches
applyQueue researches queue =
    List.foldl applyQueueEntry researches queue


applyQueueEntry : Entry -> Researches -> Researches
applyQueueEntry entry researches =
    case entry of
        ResearchEntry data ->
            Dict.update data.researchId
                (Maybe.map
                    (\research ->
                        { research | currentLevel = max research.currentLevel data.level }
                    )
                )
                researches

        _ ->
            researches


unfinishedEntries : DateTime -> Queue -> Queue
unfinishedEntries currentDate =
    List.filter (not << isFinished currentDate)


isFinished currentDate entry =
    timeToCompletion entry currentDate
        |> Maybe.map (\time -> time <= 0)
        |> Maybe.withDefault False


timeToCompletion : Entry -> DateTime -> Maybe Int
timeToCompletion entry currentDate =
    let
        time =
            (\data ->
                Maybe.map
                    (\finishesAt ->
                        max (secondsBetween finishesAt currentDate) 0
                    )
                    data.finishesAt
            )
    in
        case entry of
            ResearchEntry data ->
                time data

            ModuleEntry data ->
                time data


secondsBetween a b =
    (Time.DateTime.delta a b).seconds


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
