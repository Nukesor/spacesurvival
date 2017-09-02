module Extra.Dict exposing (..)

import Dict exposing (Dict)


insertDedupe : (v -> v -> v) -> comparable -> v -> Dict comparable v -> Dict comparable v
insertDedupe combine key value dict =
    let
        with maybeValue =
            case maybeValue of
                Just oldValue ->
                    Just (combine oldValue value)

                Nothing ->
                    Just value
    in
        Dict.update key with dict
