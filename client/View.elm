module View exposing (..)

import Model exposing (..)
import Animation
import Html exposing (..)
import Html.Attributes exposing (..)
import Messages
import GridView exposing (grid)
import View.Auth exposing (auth)


view : Model -> Html.Html Messages.Msg
view model =
    let
        currentView =
            case model.user.token of
                Just token ->
                    div [ class "grid-container" ] [ grid model ]

                Nothing ->
                    auth model

        background =
            div [ class "bg-container" ] [ div [ class "bg" ] [] ]
    in
        div []
            [ background
            , currentView
            ]
