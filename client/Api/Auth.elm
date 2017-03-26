module Api.Auth exposing (..)

import Json.Encode as Encode
import Json.Decode as Decode
import Model exposing (..)
import Model.User exposing (..)
import Http
import Messages


dataDecoder =
    Decode.field "data"


userEncoder : User -> Encode.Value
userEncoder user =
    Encode.object
        [ ( "nickname", Encode.string user.nickname )
        , ( "email", Encode.string user.email )
        , ( "password", Encode.string user.password )
        ]


registerDecoder =
    dataDecoder <| Decode.map2 registeredUser (Decode.field "nickname" Decode.string) (Decode.field "email" Decode.string)


loginDecoder =
    Decode.map loggedInUser (dataDecoder Decode.string)


registeredUser nickname email =
    { token = Nothing
    , nickname = nickname
    , email = email
    , password = ""
    }


loggedInUser token =
    { token = Just token, nickname = "", email = "", password = "" }


register : Model -> Cmd Messages.Msg
register model =
    let
        request =
            Http.post "/api/user/register" (Http.jsonBody <| userEncoder model.user) registerDecoder
    in
        Http.send Messages.Registered request


login : Model -> Cmd Messages.Msg
login model =
    let
        request =
            Http.post "/api/auth/login" (Http.jsonBody <| userEncoder model.user) loginDecoder
    in
        Http.send Messages.LoggedIn request
