module Model.Resources exposing (..)

import Maybe exposing (andThen)


type alias Resources =
    List Resource


type alias ResourceId =
    String


type alias Resource =
    { amount : Int
    , id : ResourceId
    , maxAmount : Int
    , name : String
    }


applyTick : (Int -> Int -> Int) -> Resources -> List ( ResourceId, Int ) -> Resources
applyTick fn resources modifications =
    let
        findModification resource =
            modifications
                |> List.filter (\( id, val ) -> id == resource.id)
                |> List.head
    in
        List.map
            (\resource ->
                findModification resource
                    |> Maybe.map (\( id, val ) -> { resource | amount = fn resource.amount val })
                    |> Maybe.withDefault resource
            )
            resources
