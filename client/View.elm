module View exposing (..)

import Model exposing (..)
import Components.Modal
import Animation
import Html exposing (..)
import Html.Attributes exposing (..)
import Messages


view : Model -> Html.Html Messages.Msg
view model =
    div []
        [ div [ class "dialog-container" ]
            (List.map Components.Modal.view model.modals)
        , div [ class "bg-container" ] [ div [ class "bg" ] [] ]
        ]
