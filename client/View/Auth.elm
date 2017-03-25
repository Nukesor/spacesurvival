module View.Auth exposing (auth)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Model exposing (Model, AuthView)
import Animation
import Messages


auth : Model -> Html Messages.Msg
auth model =
    div [ class "dialog-container" ]
        [ div (List.concat [ Animation.render model.authDialogAnimation, [ class "dialog" ] ])
            [ Html.form [ onSubmit Messages.Login ] <|
                authForm
                    model
            ]
        ]


authForm model =
    case model.authView of
        Model.Register ->
            [ input
                [ type_ "text", placeholder "Username", onInput (updateNickname model.user) ]
                []
            , input [ type_ "text", placeholder "E-Mail", onInput (updateEmail model.user) ] []
            , input
                [ type_ "password", placeholder "Password", onInput (updatePassword model.user) ]
                []
            , a [ onClick <| Messages.ChangeAuthView Model.Login, href "#" ] [ text "back to login" ]
            , button [ onClick Messages.Register ] [ text "Sign Up!" ]
            ]

        Model.Login ->
            [ p [] [ text "Please log in or register to play!" ]
            , input [ type_ "text", placeholder "E-Mail", onInput (updateEmail model.user) ] []
            , input
                [ type_ "password", placeholder "Password", onInput (updatePassword model.user) ]
                []
            , a [ onClick <| Messages.ChangeAuthView Model.Register, href "#" ] [ text "Create Account" ]
            , button [ onClick Messages.Login ] [ text "Log in" ]
            ]


updateEmail user email =
    Messages.UpdateUser { user | email = email }


updatePassword user password =
    Messages.UpdateUser { user | password = password }


updateNickname user nickname =
    Messages.UpdateUser { user | nickname = nickname }
