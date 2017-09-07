module Model.Resources exposing (..)

import Dict exposing (Dict)
import Extra.Dict exposing (insertDedupe)
import Maybe exposing (andThen)


type alias Resources =
    List Resource


type alias ResourceId =
    String


type alias Resource =
    { amount : Int
    , id : String
    , maxAmount : Int
    , name : ResourceId
    }


formatAmount : Int -> String
formatAmount amount =
    toString ((toFloat amount) / 1000000)


resourcesExist : Resources -> List ( ResourceId, Int ) -> Bool
resourcesExist existing mods =
    List.foldl
        (\( id, val ) previousSatisfied ->
            previousSatisfied && val >= (resourceValue existing id)
        )
        True
        mods


resourceValue : Resources -> ResourceId -> Int
resourceValue resources id =
    resources
        |> List.filter (\res -> res.id == id)
        |> List.head
        |> Maybe.map .amount
        |> Maybe.withDefault 0


foldModifications : List (List ( ResourceId, Int )) -> Dict ResourceId Int
foldModifications mods =
    let
        updateDict ( id, val ) dict =
            insertDedupe (+) id val dict
    in
        mods
            |> List.concat
            |> List.foldl updateDict Dict.empty


applyTick : (Int -> Int -> Int) -> Dict ResourceId Int -> Resources -> Resources
applyTick fn modifications resources =
    let
        findModification resource =
            Dict.get resource.name modifications
    in
        List.map
            (\resource ->
                findModification resource
                    |> Maybe.map (\val -> { resource | amount = fn resource.amount val })
                    |> Maybe.withDefault resource
            )
            resources
