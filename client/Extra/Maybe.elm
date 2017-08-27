module Extra.Maybe exposing (..)


isJust maybe =
    case maybe of
        Just _ ->
            True

        Nothing ->
            False
