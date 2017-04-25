module View.Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Messages
import Model
import Model.User exposing (..)


view model =
    case model.user of
        Registering user ->
            [ input
                [ type_ "text", placeholder "Username", onInput (updateNickname user), value user.nickname ]
                []
            , input [ type_ "email", placeholder "E-Mail", onInput (updateEmail user), value user.email ] []
            , input
                [ type_ "password", placeholder "Password", onInput (updatePassword user), value user.password ]
                []
            , a [ onClick <| Messages.ChangeAuthView Model.Login, href "#" ] [ text "back to login" ]
            , button [ onClick Messages.Register ] [ text "Sign Up!" ]
            ]

        _ ->
            []


updateNickname user name =
    Messages.UpdateUser <| Registering { user | nickname = name }


updateEmail user email =
    Messages.UpdateUser <| Registering { user | email = email }


updatePassword user pw =
    Messages.UpdateUser <| Registering { user | password = pw }