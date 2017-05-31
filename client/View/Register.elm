module View.Register exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Messages
import Model
import Model.User exposing (..)


view : Model.Model -> List (Html Messages.Msg)
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
            , a [ onClick <| Messages.UpdateUser (LoggingIn { identifier = "", password = "" }), href "#" ] [ text "back to login" ]
            , button [ onClick Messages.Register ] [ text "Sign Up!" ]
            ]

        _ ->
            []


updateNickname : RegisterData -> String -> Messages.Msg
updateNickname user name =
    Messages.UpdateUser <| Registering { user | nickname = name }


updateEmail : RegisterData -> String -> Messages.Msg
updateEmail user email =
    Messages.UpdateUser <| Registering { user | email = email }


updatePassword : RegisterData -> String -> Messages.Msg
updatePassword user pw =
    Messages.UpdateUser <| Registering { user | password = pw }
