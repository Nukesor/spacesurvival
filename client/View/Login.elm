module View.Login exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Messages
import Model
import Model.User exposing (..)


view : Model.Model -> List (Html Messages.Msg)
view model =
    case model.user of
        LoggingIn user ->
            [ p [] [ text "Please log in or register to play!" ]
            , input [ type_ "text", placeholder "Username", onInput (updateIdentifier user), value user.identifier ] []
            , input
                [ type_ "password", placeholder "Password", onInput (updatePassword user), value user.password ]
                []
            , a [ onClick (toRegister user), href "#" ] [ text "Create Account" ]
            , button [ onClick Messages.Login ] [ text "Log in" ]
            ]

        _ ->
            []


toRegister : { a | password : String } -> Messages.Msg
toRegister user =
    Messages.UpdateUser <| Registering { email = "", nickname = "", password = user.password }


updateIdentifier : LoginData -> String -> Messages.Msg
updateIdentifier user name =
    Messages.UpdateUser <| LoggingIn { user | identifier = name }


updatePassword : LoginData -> String -> Messages.Msg
updatePassword user password =
    Messages.UpdateUser <| LoggingIn { user | password = password }
