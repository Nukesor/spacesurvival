module Model.Queue exposing (..)

import Dict
import Model.Research exposing (ResearchId, Researches)
import Time.DateTime exposing (DateTime, addSeconds)


type alias ResearchData =
    { id : String
    , finishesAt : Maybe DateTime
    , level : Int
    , duration : Int
    , researchId : ResearchId
    , researchUuid : String
    }


type alias ModuleData =
    { id : String
    , finishesAt : Maybe DateTime
    , level : Int
    , duration : Int
    , moduleId : String
    , moduleUuid : String
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
                        { research | currentLevel = Just (max (Maybe.withDefault 0 research.currentLevel) data.level) }
                    )
                )
                researches

        _ ->
            researches


unfinishedEntries : DateTime -> Queue -> Queue
unfinishedEntries currentDate =
    List.filter (not << isFinished currentDate)


isFinished : DateTime -> Entry -> Bool
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


secondsBetween : DateTime -> DateTime -> Int
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
