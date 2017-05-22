module View exposing (..)

import Model exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Messages
import View.Grid exposing (grid)
import Styles.Background
import Html.CssHelpers
import Model.User exposing (..)
import Animation
import View.Login
import View.Register


css : Html.CssHelpers.Namespace String class id msg
css =
    Html.CssHelpers.withNamespace Styles.Background.ns


view : Model -> Html.Html Messages.Msg
view model =
    let
        currentView =
            case model.user of
                LoggedIn user ->
                    div [ class "grid-container" ] [ grid model ]

                _ ->
                    auth model

        background =
            div
                [ css.class [ Styles.Background.Container ] ]
                [ div
                    [ css.class [ Styles.Background.Background ] ]
                    []
                ]
    in
        div []
            [ background
            , currentView
            ]


auth : Model -> Html Messages.Msg
auth model =
    div [ class "dialog-container" ]
        [ div (List.concat [ Animation.render model.authDialogAnimation, [ class "dialog" ] ])
            [ Html.form [ onSubmit Messages.Login ] <|
                authForm model
            ]
        ]


authForm : Model -> List (Html Messages.Msg)
authForm model =
    case model.user of
        LoggingIn _ ->
            View.Login.view model

        Registering _ ->
            View.Register.view model

        _ ->
            []
