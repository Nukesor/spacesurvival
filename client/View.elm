module View exposing (..)

import Model exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Messages
import View.Background
import Html.CssHelpers
import Model.User exposing (..)
import Animation
import View.Login
import View.Register
import View.Layout


css : Html.CssHelpers.Namespace String class id msg
css =
    Html.CssHelpers.withNamespace View.Background.ns


view : Model -> Html.Html Messages.Msg
view model =
    let
        currentView =
            case model.user of
                LoggedIn user ->
                    View.Layout.view model

                _ ->
                    auth model
    in
        div []
            [ View.Background.view
            , currentView
            ]


auth : Model -> Html Messages.Msg
auth model =
    div [ class "dialog-container" ]
        [ div (List.concat [ Animation.render model.authDialogAnimation, [ class "dialog" ] ])
            (authForm model)
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
