module View exposing (..)

import Model exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Messages
import GridView exposing (grid)
import View.Auth exposing (auth)
import Styles.Background
import Html.CssHelpers


css =
    Html.CssHelpers.withNamespace Styles.Background.ns


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
            div [ css.class [ Styles.Background.Container ] ] [ div [ css.class [ Styles.Background.Background ] ] [] ]
    in
        div []
            [ background
            , currentView
            ]
